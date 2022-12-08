A = [
  3 0 3 7 3
  2 5 5 1 2
  6 5 3 3 2
  3 3 5 4 9
  3 5 3 9 0
]

function main()
  N, M = size(A)
  count = 2(N-1) + 2(M-1)
  best = 0
  for i = 2:N-1
    for j = 2:M-1
      here = A[i, j]
      left = @view A[i, reverse(1:j-1)]
      right = @view A[i, j+1:end]
      top = @view A[reverse(1:i-1), j]
      bottom = @view A[i+1:end, j]
      visible = any([
        all(here .> left)
        all(here .> right)
        all(here .> top)
        all(here .> bottom)
      ])
      if !visible
        continue
      end
      count += 1
      scores = 1
      for v in (left, right, top, bottom)
        idx = findfirst(>=(here), v)
        scores *= isnothing(idx) ? length(v) : length(1:idx)
      end
      if scores > best
        best = scores
      end
    end
  end
  count, best
end
