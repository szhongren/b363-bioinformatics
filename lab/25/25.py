import sys

class Mat(object):
    def __init__(self, file):
        buf = file.readlines()
        self.n = int(buf[0])
        self.D = buf[1: 1 + self.n]
        for i in range(len(self.D)):
            self.D[i] = self.D[i].split()
            self.D[i] = list(map(int, self.D[i]))

    def __str__(self):
        out = ""
        out += str(self.n) + '\n'
        for line in self.D:
            out += str(line) + '\n'
        return out

    def get_sums(self):
        return list(map(sum, self.D))

    def delete(self, i):
        pass

    def insert(self):
        pass

file = None
if len(sys.argv) > 1 and sys.argv[1] == "debug":
    file = open("debug.txt")
elif len(sys.argv) > 1 and sys.argv[1] == "debug1":
    file = open("debug1.txt")
else:
    file = open("rosalind_ba7e.txt")

D = Mat(file)
print(D)
print(D.get_sums())