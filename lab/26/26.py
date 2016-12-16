import sys
import math

file = None

if len(sys.argv) > 1 and sys.argv[1] == "debug":
    file = open("debug.txt")
elif len(sys.argv) > 1 and sys.argv[1] == "debug1":
    file = open("debug1.txt")
else:
    file = open("rosalind_ba8c.txt")

data = file.readlines()
km = list(map(int, data[0].split()))
k = km[0]
m = km[1]

points = data[1:]
for i in range(len(points)):
    points[i] = points[i].split()
    points[i] = list(map(float, points[i]))

def dist(pt1, pt2, m):
    sum = 0
    for i in range(m):
        sum += (pt1[i] - pt2[i])**2
    return math.sqrt(sum)

def get_center(cluster, m):
    sums = [0 for i in range(m)]
    for i in range(len(cluster)):
        for j in range(m):
            sums[j] += cluster[i][j]
    return [sums[i]/len(cluster) for i in range(m)]

def get_centers(k, m, pts, prev_res):
    clusters = [[] for _ in range(k)]
    for pt in pts:
        distances = [dist(pt, prev_res[i], m) for i in range(k)]
        min_dist = min(distances)
        clusters[distances.index(min_dist)].append(pt)
    return [get_center(clusters[i], m) for i in range(k)]

def lloyds_algorithm(k, m, pts):
    prev_res = []
    res = []
    for i in range(k):
        prev_res.append(None)
        res.append(pts[i])

    while prev_res != res:
        prev_res = res
        res = get_centers(k, m, pts, prev_res)
        pass
    return res

ans = lloyds_algorithm(k, m, points)
for center in ans:
    for coord in center:
        print(round(coord, 3), end=' ')
    print()
