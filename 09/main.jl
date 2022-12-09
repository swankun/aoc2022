using LinearAlgebra

function action!(u::Vector{T}, c::Char) where {T <: Real}
  if c == 'U'
    u[1] = 0
    u[2] = 1
  elseif c == 'L'
    u[1] = -1
    u[2] = 0
  elseif c == 'D'
    u[1] = 0
    u[2] = -1
  elseif c == 'R'
    u[1] = 1
    u[2] = 0
  end
end

function step!(x::Vector{T}, u::Vector{T}) where {T <: Real}
  steplen = norm(x[1:2]+u - x[3:4])
  if !any(steplen .≈ [sqrt(2), 1, 0])
    x[3:4] .= x[1:2]
  end
  x[1:2] .+= u
end 

function input(filename="example")
  x = [0; 0; 0; 0]
  u = [0; 0]
  traj = [1x]
  t = 1
  for line in eachline(filename)
    ustr, n = split(line, ' ')
    action!(u, only(ustr))
    foreach(1:parse(Int,n)) do _
      step!(x, u)
      push!(traj, copy(x))
      t += 1
    end
  end
  hcat(traj...)
end

function process(traj::Matrix{T}) where {T <: Real}
  map(vec, eachcol(traj[3:4, :])) |> unique
end