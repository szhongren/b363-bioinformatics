#!/usr/bin/python

DEBUG = True
filename = "debug.txt"
sets = 7

nucleotides = ["A", "C", "G", "T"]

if DEBUG == False:
    filename = "rosalind_ba1b.txt"
    sets = 1

def hamming_dist(pattern1, pattern2):
    hamming = 0
    for (i, j) in zip(pattern1, pattern2):
        if i != j:
            hamming += 1
    return hamming

def neighbors(pattern, d):
    if d == 0:
        return [pattern]
    if len(pattern) == 1:
        return nucleotides
    neighborhood = []
    suffix_neighbors = neighbors(pattern[1:], d)
    for suffix_neighbor in suffix_neighbors:
        if hamming_dist(suffix_neighbor, pattern[1:]) < d:
            for n in nucleotides:
                neighborhood.append(n + suffix_neighbor)
        else:
            neighborhood.append(pattern[0] + suffix_neighbor)
    return neighborhood

def freq_words_with_mismatches_complements(genome, k, d):
    freq_patterns = []
    neighborhoods = []
    for i in range(0, len(genome) - k):
        neighborhoods.append(neighbors(genome[i:k], d))
    return neighborhoods

f = open(filename, 'r')
for i in range(0, sets):
    genome = f.readline().rstrip()
    k_d = f.readline().split()
    k = int(k_d[0])
    d = int(k_d[1])
    print("{}, {}, {}\n------------------------------".format(genome, k, d))
    for test in freq_words_with_mismatches_complements(genome, k, d):
        print test
