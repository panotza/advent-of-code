def sum_cube(cubeInfomation):
    info = {}
    sum = 0
    for s in cubeInfomation.split(";"): # sets
        for c in s.split(","): # cubes
            x = c.strip().split(" ")
            n = int(x[0])
            color = x[1]
            info[color] = max(info.get(color, 0), n)
    sum = info.get("red", 0) * info.get("green", 0) * info.get("blue", 0)
    return sum

with open("02.txt") as f:
    ans = 0
    for line in f.readlines():
        content = line.split(":")
        ans += sum_cube(content[1])
    print(ans)