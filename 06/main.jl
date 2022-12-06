function f(s, index::Int=4)
  if allunique(first(s, 4))
    return index
  else
    f(Iterators.peel(s) |> last, index+1)
  end
end

function main()
  for line in readlines("input")
    @show f(line)
  end
end
