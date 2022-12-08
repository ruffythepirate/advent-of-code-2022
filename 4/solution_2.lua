
function getInterval(str) 
  for k, v in string.gmatch(str, "(%d+)%-(%d+)") do
    return {min=tonumber(k), max=tonumber(v)}
  end
end

function isOverlap(a, b) 
  if isInInterval(a.min, b) or isInInterval(a.max, b) then
    return true
  end
  if isInInterval(b.min, a) or isInInterval(b.max, a) then
    return true
  end
  return false
end

function isInInterval(num, interval)
  if num >= interval.min and num <= interval.max then
    return true
  end
  return false
end


local overlapPairs = 0
for line in io.lines() do
  if line == "" then
    break
  end
  for first, second in string.gmatch(line, "(.*),(.*)") do
    local firstInt = getInterval(first)
    local secondInt = getInterval(second)
    if isOverlap(firstInt, secondInt) then
      overlapPairs = overlapPairs + 1
    end
  end
end
print(overlapPairs)
