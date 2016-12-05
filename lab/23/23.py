import sys

debug = False
if len(sys.argv) > 1 and sys.argv[1] == "debug":
  debug = True

file = open("rosalind_ba6i.txt")
sets = 1
if debug:
  file = open("debug.txt")
  sets = 2

for _ in range(sets):
  a = file.readline().strip()
  a = a.split("), (")
  a[0] = ''.join(c for c in a[0] if c not in "()")
  a[-1] = ''.join(c for c in a[-1] if c not in "()")
  for i in range(len(a)):
      a[i] = list(map(int, a[i].split(", ")))

  for x in a:
      print(x)
  print()