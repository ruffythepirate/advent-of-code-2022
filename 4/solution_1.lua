
function getInterval(str) 
  for k, v in string.gmatch(str, "(%d+)%-(%d+)") do
    return {min=tonumber(k), max=tonumber(v)}
  end
end

function isContained(a, b) 
  if a.min == b.min or a.max == b.max then
    return true
  end
  if ((a.min < b.min) ~= (a.max < b.max)) then
    return true
  end
  return false
end


local containedPairs = 0
for line in io.lines() do
  if line == "" then
    break
  end
  for first, second in string.gmatch(line, "(.*),(.*)") do
    local firstInt = getInterval(first)
    local secondInt = getInterval(second)
    if isContained(firstInt, secondInt) then
      containedPairs = containedPairs + 1
    end
  end
end
print(containedPairs)
