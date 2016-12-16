import sys

def find_cycles(edges):
  cycles = []
  new_cycle = True
  start = None
  start_i = 0
  for i in range(len(edges)):
    if new_cycle:
      start = edges[i][0]
      new_cycle = False
      start_i = i
    else:
      if start % 2 == 0:
        if edges[i][1] == start - 1:
          cycles.append(edges[start_i: i + 1])
          new_cycle = True
      else:
        if edges[i][1] == start + 1:
          cycles.append(edges[start_i: i + 1])
          new_cycle = True
  return cycles

def cycle_to_chromosome(cycle):
  chromosome = []
  for edge in cycle:
    if edge[0] % 2 == 0:
      chromosome.append(edge[0] // 2)
    else:
      chromosome.append(-edge[0] // 2)
  return chromosome

def graph_to_genome(edges):
  res = []
  cycles = find_cycles(edges)
  for cycle in cycles:
    res.append(cycle_to_chromosome(cycle))
  return res

debug = False
if len(sys.argv) > 1 and sys.argv[1] == "debug":
  debug = True

file = open("rosalind_ba6i.txt")
sets = 1
if debug:
  file = open("debug.txt")
  sets = 2

for _ in range(sets):
  edges = file.readline().strip()
  edges = edges.split("), (")
  edges[0] = ''.join(c for c in edges[0] if c not in "()")
  edges[-1] = ''.join(c for c in edges[-1] if c not in "()")
  for i in range(len(edges)):
      edges[i] = list(map(int, edges[i].split(", ")))

  genome = graph_to_genome(edges)

  for gen in genome:
    print("(", end = '')
    for i in range(len(gen)):
      if gen[i] > 0:
        print("+", end = '')
      print(gen[i], end = '')
      if i != len(gen) - 1:
        print(" ", end = '')
    print(")", end = '')
  print()

