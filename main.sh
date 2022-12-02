#/usr/bin/env bash

let max=0
let sum=0
cp input output
echo '' >> output
while read -r line
do
  if [[ ! -z $line ]]; then
    let sum=$sum+$line
  else
    if [[ $((sum > max)) ]]; then
      echo "sum=$sum, max=$max"
      let max=$sum
    fi
    let sum=0
  fi
done < output
