import sys

def breakpoint_count(permutation):
  permutation = [0] + list(permutation) + [len(permutation)+1]
  res = map(lambda x, y: abs(x-y) != 1, permutation[1:], permutation[:-1])
  return sum(res)

debug = False
if len(sys.argv) > 1 and sys.argv[1] == "debug":
  debug = True


file = open("rosalindba6b.txt")
sets = 1
if debug:
  file = open("debug.txt")
  sets = 3

for _ in range(sets):
  permutation = file.readline()
  permutation = permutation[permutation.index('(') + 1: permutation.index(')')]
  permutation = list(map(int, permutation.strip().split()))
  print(breakpoint_count(permutation))