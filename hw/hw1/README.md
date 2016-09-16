1. FrequentWords(GAGTTAACGAACGCTTAAC, 3)

  All-kmers:
  - `GAG, AGT, GTT, TTA, TAA, AAC, ACG, CGA, GAA, AAC, ACG, CGC, GCT, CTT, TTA,
  TAA, AAC`

  Sorted:
  - `AAC, AAC, AAC, ACG, ACG, AGT, CGA, CGC, CTT, GAA, GAG, GCT, GTT, TAA, TAA,
  TTA, TTA`

  AAC and its complement, GTT occur 4 times in total

  TAA and its complement, TTA occur 4 times in total

  The probability of any 3-mer or its complement appearing in a random string
    is about 1/32, not taking repeated letters into account.

  Thus, 4 of them all appearing at the same time in a string of length 19 has
    the approximate probability
    ```
    (1/32)^4 * (31/32)^13 = 6.31e-7
    ```
    Thus this is significant.

2. ComputingFrequencies has to zero the hash table `(4^k)` and then make one
    pass through the DNA string `(|Text|*k)`, so total running time would be
    ```
    4k+|Text|k = (4+|Text|)k
    => |Text|k for large values of |Text|
    ```

  FrequentWords has a runtime of `|Text|^2*k`, so for all values of `|Text| and
    k >= 0`, ComputingFrequencies will be faster. Thus, this holds true for the
    Replication Origin Finding Problem.

3. No, as it includes all possible letters for each position in the generated
    string, so it will generate the string itself several times over.

  ```
  Input: A string P of length |P| and an alphabet A of size |A|;
  Neighbors(P)
    Q <- P
    Initialize S <- {P}
    for j <- 1 to |P|
      for k <- 1 to |A|
        if A[k] != P[j]
          Q[j] <- A[k]
          add Q into S
    output S
  ```

4. Let `h = hamming, l = |str|`
  ```
  GenNeighborsWithHamming(str, h)
    init neighbors <- empty list
    for replace <- all possible strings of length h
      for indices <- all possible combinations of indices with length h taking from 0:l
        neighbor = str
        for (i, char) <- list of tuples, where x_i = (indices[i], replace[i])
          neighbor[i] = char
        results += neighbor
    return neighbors
  ```
  This algorithm will take
  ```
  4^h * (l C h) * h
  => O(4^h * l! /(h!(l-h)!) * h)
  ```

5. For each k-mer in the string, check if the first `floor(k/2)` nucleotides
    complement the second `floor(k/2)` nucleotides
  ```
  FindPalindromes(str, k)
    palindromes <- empty list
    for i <- 0 to |str| - k
      if str[i:i+k/2] == complement(str[i+k/2:i+k])
        palindromes += str[i:i+k]
    return palindromes
  ```
  This algorithm will take `|str|k/2` operations, thus it will be `O(|str|k)`.

6. No, because there is no readily apparent minimum point. Also the end of
    the skew diagram sohuld match up with the start of the diagram because
    most bacterial genomes are circular. Thus, the replication origin could
    be anywhere after the lowest point that we can see on the graph.

7. Profile matrix
  ```
  0  | C | G | C | C | T | G | A | A | T | A |
  1  | C | G | A | G | A | A | A | G | T | T |
  2  | C | G | C | C | G | G | A | A | T | T |
  3  | G | G | C | A | T | G | A | A | T | A |
  4  | T | A | A | A | G | G | A | A | T | C |
  5  | T | A | A | T | T | T | A | A | T | T |
  6  | C | A | A | T | T | A | A | A | T | T |
  7  | G | A | C | A | T | G | A | A | T | C |
  8  | T | G | G | C | T | A | A | T | T | T |
  9  | C | A | A | C | T | G | A | A | T | T |

  A  | 0 | 5 | 5 | 3 | 1 | 3 |10 | 8 | 0 | 2 |
  C  | 5 | 0 | 4 | 4 | 0 | 0 | 0 | 0 | 0 | 2 |
  G  | 2 | 5 | 1 | 1 | 2 | 6 | 0 | 1 | 0 | 0 |
  T  | 3 | 0 | 0 | 2 | 7 | 1 | 0 | 1 |10 | 6 |

  A  | 0 |.5 |.5 |.3 |.1 |.3 | 1 |.8 | 0 |.2 |
  C  |.5 | 0 |.4 |.4 | 0 | 0 | 0 | 0 | 0 |.2 |
  G  |.2 |.5 |.1 |.1 |.2 |.6 | 0 |.1 | 0 | 0 |
  T  |.3 | 0 | 0 |.2 |.7 |.1 | 0 |.1 | 1 |.6 |
  ```
  Entropy = 10.438

  Pr(S_0|Profile) =   .000158
