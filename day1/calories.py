with open('input') as f:
  lines = f.readlines()
  callories = []
  local_sum = 0
  for line in lines:
    if line == '\n':
      callories.append(local_sum)
      local_sum = 0
    else:
      local_sum += int(line.strip())

sorted_callories = sorted(callories, reverse=True)
print(sorted_callories[0])
print(sum(sorted_callories)[:3])
