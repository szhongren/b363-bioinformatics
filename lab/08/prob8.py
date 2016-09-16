#!/usr/bin/python

f = open('rosalind_ba1b.txt', mode='r')
for i in range(0, 1):
    genome = f.readline()
    k = int(f.readline())

    allKmers = []
    mostFrequent = []
    maxCount = 0

    for x in range(0, len(genome) - k):
        allKmers.append(genome[x: x + k])
    # print(allKmers, len(allKmers))
    allKmers.sort() # use radix sort here for O(len(genome)) time
    # print(allKmers, len(allKmers))

    curr = ""
    currCount = 1

    for i in range(len(allKmers)):
        if curr != allKmers[i]:
            # print(curr, currCount)
            if currCount > maxCount:
                mostFrequent = []
                mostFrequent.append(allKmers[i-1])
                maxCount = currCount
            elif currCount == maxCount:
                mostFrequent.append(allKmers[i-1])
            currCount = 1
        else:
            currCount += 1
        curr = allKmers[i]

    if currCount > maxCount:
        mostFrequent = []
        mostFrequent.append(allKmers[-1])
    elif currCount == maxCount:
        mostFrequent.append(allKmers[-1])
    print(' '.join(mostFrequent))
