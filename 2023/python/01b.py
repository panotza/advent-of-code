digitLetter = {
    "one": "1",
    "two": "2",
    "three": "3",
    "four": "4",
    "five": "5",
    "six": "6",
    "seven": "7",
    "eight": "8",
    "nine": "9"
}

for i in range(1, 10):
    s = str(i)
    digitLetter[s] = s

wordWindowLength = 5

def findDigit(word):
    acc = ""
    for c in word:
        acc = acc + c
        n = digitLetter.get(acc)
        if n != None:
            return n
    return None

with open("01.txt", "r") as f:
    input = f.read()
    sum = 0
    for line in input.splitlines():
        n = ""
        for i in range(0, len(line)):
            word = line[i:i+wordWindowLength]
            m = findDigit(word)
            if m != None:
                n = n + m
                break
        for i in range(len(line) - 1, -1, -1):
            word = line[i:i+wordWindowLength]
            m = findDigit(word)
            if m != None:
                n = n + m
                break
        if len(n) == 1:
            raise Exception("no found in ", line)
        else:
            sum += int(n)
    print(sum)