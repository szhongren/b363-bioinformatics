import sys
import math
from enum import Enum

sys.setrecursionlimit(1000000)
class direction(Enum):
  right = 0
  down = 1
  diag = 2

open_cost = 5
ext_cost = 1
sigma = 5

def middle_column_score(v, w, scoring_matrix, sigma):
    '''Returns the score of the middle column for the alignment of v and w.'''

    # Initialize the score columns.
    S = [[i*j*sigma for j in range(-1, 1)] for i in range(len(v)+1)]
    S[0][1] = -sigma
    backtrack = [0]*(len(v)+1)

    # Fill in the Score and Backtrack matrices.
    for j in range(1, len(w)//2+1):
        for i in range(0, len(v)+1):
            if i == 0:
                S[i][1] = -j*sigma
            else:
                scores = [S[i-1][0] + scoring_matrix[v[i-1]][w[j-1]], S[i][0] - sigma, S[i-1][1] - sigma]
                S[i][1] = max(scores)
                backtrack[i] = scores.index(S[i][1])

        if j != len(w)/2:
            S = [[row[1]]*2 for row in S]

    return [row[1] for row in S], backtrack

def linear_space_alignment(X, Y, top, bottom, left, right, weights):
  if left == right:
    return ("-" * (bottom - top), Y[top:bottom])
  if top == bottom:
    return (X[left:right], "-" * (right - left))
  middle = (left + right) // 2
  midNode = middle_node(X, Y, top, bottom, left, right, weights)
  midEdge = middle_edge(X, Y, top, bottom, left, right, weights)
  (frontA, frontB) = linear_space_alignment(X, Y, top, midNode, left, middle, weights)
  frontA += '#' # concat the middle value with the front
  frontB += '#'
  if midEdge == direction.right or midEdge == direction.diag:
    middle += 1
  if midEdge == direction.down or midEdge == direction.diag:
    midNode += 1
  (backA, backB) = linear_space_alignment(X, Y, midNode, bottom, middle, right, weights)
  return (frontA + backA, frontB + backB)

def middle_node(X, Y, top, bottom, left, right, weights):
  pass

def middle_edge(X, Y, top, bottom, left, right, weights):
  pass

debug = False
if len(sys.argv) > 1 and sys.argv[1] == "debug":
  debug = True

blosum = open("resources/blosum62.txt")
AAcids = blosum.readline().split()
weights = [map(int, blosum.readline()[1:].split()) for _ in range(len(AAcids))]

# makes things easier for myself

def key_to_val(x):
  return dict(zip(AAcids, x))

score = key_to_val(map(key_to_val, weights))

file = open("resources/rosalind_ba5l.txt")
sets = 1
if debug:
  file = open("resources/debug.txt")
  sets = 2

for _ in range(sets):
  a = file.readline().strip()
  b = file.readline().strip()

sigma = 5
print(middle_column_score("ACGGAA", "ATTCAA", score, 5))