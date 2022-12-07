#!/usr/bin/env bash

INPUT="example"

# Modify the $INPUT by regex
cp $INPUT $INPUT.copy
sed -i '/$ ls/d' $INPUT.copy # delete lines containing `ls`
sed -i '/$ cd \//d' $INPUT.copy # stay out of my root directory!
sed -i '/^[0-9]/s/^/fallocate -l /g' $INPUT.copy # allocate file with given size
sed -i '/dir /d' $INPUT.copy # delete output of `ls` that are dir
sed -i 's/^$ cd \([a-z]*\)/mkdir \1 \&\& cd \1/g' $INPUT.copy # create directories before going into them
sed -i 's/mkdir  && cd ../cd ../g' $INPUT.copy # but don't create parent directories

# Generate the directory tree with files
chmod +x $INPUT.copy
rm -rf test/
mkdir -p test/dir_only test/all
cd test/all
../../$INPUT.copy
du -b . > ../../results_all_files

# Generate the directory tree without files
cd ../..
cp $INPUT.copy $INPUT.copy.dir_only
sed -i '/^fallocate/d' $INPUT.copy.dir_only
cd test/dir_only
../../$INPUT.copy.dir_only
du -b . > ../../results_dir_only
cd ../..


# Compute
grep -o '^[0-9]*' results_all_files > sizes.full
grep -o '^[0-9]*' results_dir_only > sizes.empty
paste -d '-' sizes.full sizes.empty > main.jl
sed -i '1s/^/S = [/' main.jl
echo ']' >> main.jl
echo '@show S[S.<=100000] |> sum'  >> main.jl
echo 'R=sort(S)'  >> main.jl
echo 'idx=findfirst(>(8381165), R)' >> main.jl
echo '@show R[idx]' >> main.jl
julia main.jl

# Cleanup
rm -rf test/
rm main.jl
