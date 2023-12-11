import re
import math

with open("06.txt", "r") as f:
    ts = re.findall("\d+", f.readline())
    ds = re.findall("\d+", f.readline())
    sum = 1
    for race in zip(ts,ds):
        t, d = race
        t = int(t)
        d = int(d)
        total = 0
        for i in range(1, math.floor(t / 2) + 1):
            r = i * (t - i)
            if r > d:
               total += len(set([i, t - i]))
        sum *= total
    print(sum)