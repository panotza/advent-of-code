digitWords = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]

def findDigitWord(s: str):
    for i, dw in enumerate(digitWords):
        if s.startswith(dw):
            return i + 1
    return None

with open("01.txt", "r") as f:
    sum = 0
    for line in f.readlines():
        digits = []
        for i, c in enumerate(line):
            if c.isdigit():
                digits.append(int(c))
            else:
                n = findDigitWord(line[i:])
                if n != None:
                    digits.append(n)
        sum += (digits[0] * 10) + digits[-1]
    print(sum)