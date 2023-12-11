import re
import math

with open("06.txt", "r") as f:
    ts = re.findall("\d+", f.readline().replace(" ", ""))
    ds = re.findall("\d+", f.readline().replace(" ", ""))
    t = int(ts[0])
    d = int(ds[0])
    sum = 0
    for i in range(1, math.floor(t / 2) + 1):
        r = i * (t - i)
        if r > d:
            sum += len(set([i, t - i]))
    print(sum)