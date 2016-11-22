import sys
import math
sys.setrecursionlimit(1000000)

AAcids = ['A', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'Y']

weights = [
[ 4, 0,-2,-1,-2, 0,-2,-1,-1,-1,-1,-2,-1,-1,-1, 1, 0, 0,-3,-2],
[ 0, 9,-3,-4,-2,-3,-3,-1,-3,-1,-1,-3,-3,-3,-3,-1,-1,-1,-2,-2],
[-2,-3, 6, 2,-3,-1,-1,-3,-1,-4,-3, 1,-1, 0,-2, 0,-1,-3,-4,-3],
[-1,-4, 2, 5,-3,-2, 0,-3, 1,-3,-2, 0,-1, 2, 0, 0,-1,-2,-3,-2],
[-2,-2,-3,-3, 6,-3,-1, 0,-3, 0, 0,-3,-4,-3,-3,-2,-2,-1, 1, 3],
[ 0,-3,-1,-2,-3, 6,-2,-4,-2,-4,-3, 0,-2,-2,-2, 0,-2,-3,-2,-3],
[-2,-3,-1, 0,-1,-2, 8,-3,-1,-3,-2, 1,-2, 0, 0,-1,-2,-3,-2, 2],
[-1,-1,-3,-3, 0,-4,-3, 4,-3, 2, 1,-3,-3,-3,-3,-2,-1, 3,-3,-1],
[-1,-3,-1, 1,-3,-2,-1,-3, 5,-2,-1, 0,-1, 1, 2, 0,-1,-2,-3,-2],
[-1,-1,-4,-3, 0,-4,-3, 2,-2, 4, 2,-3,-3,-2,-2,-2,-1, 1,-2,-1],
[-1,-1,-3,-2, 0,-3,-2, 1,-1, 2, 5,-2,-2, 0,-1,-1,-1, 1,-1,-1],
[-2,-3, 1, 0,-3, 0, 1,-3, 0,-3,-2, 6,-2, 0, 0, 1, 0,-3,-4,-2],
[-1,-3,-1,-1,-4,-2,-2,-3,-1,-3,-2,-2, 7,-1,-2,-1,-1,-2,-4,-3],
[-1,-3, 0, 2,-3,-2, 0,-3, 1,-2, 0, 0,-1, 5, 1, 0,-1,-2,-2,-1],
[-1,-3,-2, 0,-3,-2, 0,-3, 2,-2,-1, 0,-2, 1, 5,-1,-1,-3,-3,-2],
[ 1,-1, 0, 0,-2, 0,-1,-2, 0,-2,-1, 1,-1, 0,-1, 4, 1,-2,-3,-2],
[ 0,-1,-1,-1,-2,-2,-2,-1,-1,-1,-1, 0,-1,-1,-1, 1, 5, 0,-2,-2],
[ 0,-1,-3,-2,-1,-3,-3, 3,-2, 1, 1,-3,-2,-2,-3,-2, 0, 4,-3,-1],
[-3,-2,-4,-3, 1,-2,-2,-3,-3,-2,-1,-4,-4,-2,-3,-3,-2,-3,11, 2],
[-2,-2,-3,-2, 3,-3, 2,-1,-2,-1,-1,-2,-3,-1,-2,-2,-2,-1, 2, 7]]

open_cost = 11
ext_cost = 1

# makes things easier for myself
def blosum62(x):
  return dict(zip(AAcids, x))

score = blosum62(map(blosum62, weights))

def affine_gap_gen_matrix(X, Y, score):
  lx = len(X)
  ly = len(Y)
  # gen matrix, first row and col are 0, each 'edge' represents a matching
  mats = {"mid": [[0] * (ly + 1)] + [[0] + [-math.inf] * ly for _ in range(lx)],
  "top": [[0] + [-open_cost - x * ext_cost for x in range(ly)]] + [[-math.inf] * (ly + 1) for _ in range (lx)],
  "bot": [[0] + [-math.inf] * ly] + [[-open_cost - x * ext_cost] + [-math.inf] * ly for x in range(lx)]}
  """
  mid
  0 0         0         0
  0 -math.inf -math.inf -math.inf
  0 -math.inf -math.inf -math.inf

  top
  0         -open_cost -open_cost-1*ext_cost -open_cost-2*ext_cost
  -math.inf -math.inf  -math.inf             -math.inf
  -math.inf -math.inf  -math.inf             -math.inf

  bot
  0                     -math.inf -math.inf -math.inf
  -open_cost            -math.inf -math.inf -math.inf
  -open_cost-1*ext_cost -math.inf -math.inf -math.inf
  -open_cost-2*ext_cost -math.inf -math.inf -math.inf
  """
  for xi in range(1, lx + 1):
    for yi in range(1, ly + 1):
      mats["bot"][xi][yi] = max(mats["bot"][xi - 1][yi] - ext_cost,
                                mats["mid"][xi - 1][yi] - open_cost)
      mats["top"][xi][yi] = max(mats["top"][xi][yi - 1] - ext_cost,
                                mats["mid"][xi][yi - 1] - open_cost)
      mats["mid"][xi][yi] = max(mats["bot"][xi][yi],
                                mats["top"][xi][yi],
                                mats["mid"][xi - 1][yi - 1] + score[X[xi - 1]][Y[yi - 1]])
  return mats

def affine_gap_backtrack(X, Y, mats, xi, yi, level):
  curr = mats[level][xi][yi]
  if xi == 0 or yi == 0:
    return (X[0:xi] + "-" * yi, Y[0:yi] + "-" * xi)
  if level == "mid":
    if curr == mats["bot"][xi][yi]:
      return affine_gap_backtrack(X, Y, mat, xi, yi, "bot")
    elif curr == mats["top"][xi][yi]:
      return affine_gap_backtrack(X, Y, mat, xi, yi, "top")
    elif curr == mats["mid"][xi][yi]:
      (a, b) = affine_gap_backtrack(X, Y, mat, xi - 1, yi - 1, "mid")
      return (a + X[xi - 1], b + Y[yi - 1])
  elif level == "bot":
    if curr == mats["bot"][xi - 1][yi] - ext_cost:
      (a, b) = affine_gap_backtrack(X, Y, mat, xi - 1, yi, "bot")
      return (a + X[xi - 1], b + "-")
    elif curr == mats["mid"][xi - 1][yi] - open_cost:
      (a, b) = affine_gap_backtrack(X, Y, mat, xi - 1, yi, "mid")
      return (a + X[xi - 1], b + "-")
  elif level == "top":
    if curr == mats["top"][xi][yi - 1] - ext_cost:
      (a, b) = affine_gap_backtrack(X, Y, mat, xi, yi - 1, "top")
      return (a + "-", b + Y[yi - 1])
    elif curr == mats["mid"][xi][yi - 1] - open_cost:
      (a, b) = affine_gap_backtrack(X, Y, mat, xi, yi - 1, "mid")
      return (a + "-", b + Y[yi - 1])

debug = False
if len(sys.argv) > 1 and sys.argv[1] == "debug":
  debug = True

file = open("resources/rosalind_ba5j.txt")
sets = 1
if debug:
  file = open("resources/debug.txt")
  sets = 2

for _ in range(sets):
  a = file.readline().strip()
  b = file.readline().strip()
  mat = affine_gap_gen_matrix(a, b, score)
  print(mat["mid"][len(a)][len(b)])
  (a, b) = affine_gap_backtrack(a, b, mat, len(a), len(b), "mid")
  print("{0}\n{1}".format(a, b))