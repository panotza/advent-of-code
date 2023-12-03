with open("01.txt", "r") as f:
    sum = 0
    for line in f.readlines():
        digits = [int(c) for c in line if c.isdigit()]
        print(digits)
        sum += (digits[0] * 10) + digits[-1]
    print(sum)