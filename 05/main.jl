"""
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 
"""
crates = [
  [char for char in "ZN"],
  [char for char in "MCD"],
  [char for char in "P"],
];

for line in readlines("example")
  qty, from, to = parse.(Int, getindex(split(line, " "), [2,4,6]))
  for _ in 1:qty
    push!(crates[to], pop!(crates[from]))
  end
end

@show res = [last(c) for c in crates] |> join


"""
        [M]     [B]             [N]
[T]     [H]     [V] [Q]         [H]
[Q]     [N]     [H] [W] [T]     [Q]
[V]     [P] [F] [Q] [P] [C]     [R]
[C]     [D] [T] [N] [N] [L] [S] [J]
[D] [V] [W] [R] [M] [G] [R] [N] [D]
[S] [F] [Q] [Q] [F] [F] [F] [Z] [S]
[N] [M] [F] [D] [R] [C] [W] [T] [M]
 1   2   3   4   5   6   7   8   9 
 """
crates = [
  [char for char in "NSDCVQT"],
  [char for char in "MFV"],
  [char for char in "FQWDPNHM"],
  [char for char in "DQRTF"],
  [char for char in "RFMNQHVB"],
  [char for char in "CFGNPWQ"],
  [char for char in "WFRLCT"],
  [char for char in "TZNS"],
  [char for char in "MSDJRQHN"],
];

for line in readlines("input")
  qty, from, to = parse.(Int, getindex(split(line, " "), [2,4,6]))
  tmp = Char[]
  for _ in 1:qty
    push!(tmp, pop!(crates[from]))
  end
  append!(crates[to], reverse(tmp))
end

@show res = [last(c) for c in crates] |> join
;