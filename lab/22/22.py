import sys

def chromosome_to_cycle(p):
  nodes = []

  for i in p:
    if (i > 0):
      nodes.append(2*i-1)
      nodes.append(2*i)
    else:
      nodes.append(-2*i)
      nodes.append(-2*i-1)
  return nodes

def colored_edges(genome):
  res = []
  for p in genome:
    s = chromosome_to_cycle(p)
    for i in range(len(s)//2):
      head = 1 + 2 * i
      tail = (2 + 2 * i) % len(s)
      res.append((s[head],s[tail]))
  return res

debug = False
if len(sys.argv) > 1 and sys.argv[1] == "debug":
  debug = True

file = open("rosalind_ba6h.txt")
sets = 1
if debug:
  file = open("debug.txt")
  sets = 2

for _ in range(sets):
  a = file.readline().strip()
  a = a.split(")(")
  a[0] = ''.join(c for c in a[0] if c not in "()")
  a[-1] = ''.join(c for c in a[-1] if c not in "()")
  for i in range(len(a)):
    a[i] = list(map(int, a[i].split()))

  res = colored_edges(a)
  print(res)