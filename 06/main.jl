function f(s::String)
  chars = [c for c in s]
  buf = Char[]
  i = 0
  while !isempty(chars)
    push!(buf, popfirst!(chars))
    i += 1
    if length(buf) <= 14
      continue
    end
    popfirst!(buf)
    if allunique(buf)
      println("found")
      @show buf
      @show i
      break
    end
  end
end


function main()
  for line in readlines("input")
    f(line)
  end
end
