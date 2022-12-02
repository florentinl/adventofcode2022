score_1 = 0
score_2 = 0

def calc_score(a, b):
  match (a - b) % 3:
    case 0:
      return 3 + b + 1
    case 1:
      return 0 + b + 1
    case 2:
      return 6 + b + 1

with open("day2/input") as f:
  rounds = f.readlines()
  for round in rounds:
    a, b = round.strip().split(" ")
    a, b = ord(a) - ord('A'), ord(b) - ord('X')
    score_1 += calc_score(a, b)
    score_2 += calc_score(a, (a + b - 1) % 3)

print(score_1)
print(score_2)
