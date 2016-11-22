import sys

file = None

if len(sys.argv) > 1 and sys.argv[1] == "debug":
  file = open("debug2.txt")
else:
  file = open("rosalind_ba7a.txt")


def adj_get(i, j):
  if adj_mat[i][j] != adj_mat[j][i]:
    print("Error! Matrix not mirrored")
    return
  return adj_mat[i][j]

def adj_set(i, j, v):
  adj_mat[i][j] = v
  adj_mat[j][i] = v

data = file.readlines()
num_leaves = int(data[0])
num_nodes = int(data[-1][:data[-1].find("->")]) + 1
adj_mat = [[0 for i in range(num_nodes)] for i in range(num_nodes)]

for line in data[1:]:
  mark1 = line.find("->")
  mark2 = line.find(":")
  start = int(line[:mark1])
  end = int(line[mark1 + 2:mark2])
  weight = int(line[mark2 + 1:])
  adj_mat[start][end] = weight

for i in range(num_nodes):
  for j in range(i, num_nodes):
    limb = adj_get(i, j)
    if limb == 0: #test the comments
      continue
    else:
      for x in range(num_nodes):
        other_limb = adj_get(x, j)
        if x == i or other_limb == 0:
          continue
        elif adj_get(i, x) == 0:
          adj_set(i, x, other_limb + limb)

for i in adj_mat[:num_leaves]:
  for j in i[:num_leaves]:
    print(j, end = " ")
  print()