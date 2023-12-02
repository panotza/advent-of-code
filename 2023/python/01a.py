with open("01.txt", "r") as f:
    sum = 0
    for line in f.readlines():
        n = ""
        for c in line:
            if c.isdigit():
                n = n + c
                break
        for c in line[::-1]: 
            if c.isdigit():
                n = n + c
                break
        sum += int(n)
    print(sum)