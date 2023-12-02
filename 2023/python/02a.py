configuration = {
    "red": 12,
    "green": 13,
    "blue": 14
}

def is_possible(cubeInfomation):
    for s in cubeInfomation.split(";"): # sets
        for c in s.split(","): # cubes
            x = c.strip().split(" ")
            n = int(x[0])
            color = x[1]
            if n > configuration[color]:
                return False
    return True

with open("02.txt", "r") as f:
    ans = 0
    for line in f.readlines(): # each game
        content = line.split(":")
        id = int(content[0].removeprefix("Game "))
        if is_possible(content[1]):
            ans += id
    print(ans)