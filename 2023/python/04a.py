import re

with open("04.txt") as f:
    sum = 0
    for line in f.readlines():
        idx = line.find(":")
        if idx == -1:
            raise Exception("pattern not found")
        ns = line[idx+1:].split("|")
        winingNumbers = set(re.findall("\d+", ns[0]))
        haveNumbers = re.findall("\d+", ns[1])
        matchedStake = 0
        for n in haveNumbers:
            if n in winingNumbers:
                matchedStake += 1
        point = 2 ** (matchedStake - 1) if matchedStake > 0 else 0
        sum += point
    print("sum point", sum)