#!/usr/bin/python
import sys
debug = True
sys.setrecursionlimit(100000)

def LCS(X, Y):
    m = len(X)
    n = len(Y)
    # An (m+1) times (n+1) matrix
    C = [[0] * (n + 1) for _ in range(m + 1)]
    for i in range(1, m+1):
        for j in range(1, n+1):
            if X[i-1] == Y[j-1]:
                C[i][j] = C[i-1][j-1] + 1
            else:
                C[i][j] = max(C[i][j-1], C[i-1][j])
    return C

def backTrack(C, X, Y, i, j):
    if i == 0 or j == 0:
        return ""
    elif X[i-1] == Y[j-1]:
        return backTrack(C, X, Y, i-1, j-1) + X[i-1]
    else:
        if C[i][j-1] > C[i-1][j]:
            return backTrack(C, X, Y, i, j-1)
        else:
            return backTrack(C, X, Y, i-1, j)

if len(sys.argv) > 1 and sys.argv[1] == "debug":
    debug = True

if debug:
    f = open("resources/debug.txt")
    for i in range(0, 3):
        a = f.readline().strip()
        b = f.readline().strip()
        C = LCS(a, b)
        m = len(a)
        n = len(b)
        print(backTrack(C, a, b, m, n))
else:
    f = open("resources/rosalind_ba5c.txt")
    a = f.readline()
    b = f.readline()
    C = LCS(a, b)
    m = len(a)
    n = len(b)
    print(backTrack(C, a, b, m, n))

