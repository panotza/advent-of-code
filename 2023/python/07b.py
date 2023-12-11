from enum import IntEnum, auto
from functools import cmp_to_key


class HandType(IntEnum):
    FIVEOFAKIND = auto()
    FOUROFAKIND = auto()
    FULLHOUSE = auto()
    THREEOFAKIND = auto()
    TWOPAIR = auto()
    ONEPAIR = auto()
    HIGHCARD = auto()


strength = {
    "A": 13,
    "K": 12,
    "Q": 11,
    "T": 10,
    "9": 9,
    "8": 8,
    "7": 7,
    "6": 6,
    "5": 5,
    "4": 4,
    "3": 3,
    "2": 2,
    "J": 1,
}


def compareHand(a, b):
    ta = handType(a[0])
    tb = handType(b[0])
    if ta == tb:
        for c in zip(a[0], b[0]):
            if c[0] == c[1]:
                continue
            return strength[c[0]] - strength[c[1]]
    return tb - ta


def handType(hand: str) -> HandType:
    count = {}
    j = 0
    for h in hand:
        if h == "J":
            j += 1
        else:
            count[h] = count.get(h, 0) + 1
    n = len(count.keys())
    if n == 0 or n == 1:
        return HandType.FIVEOFAKIND
    elif n == 2:
        return HandType.FOUROFAKIND if max(count.values()) + j == 4 else HandType.FULLHOUSE
    elif n == 3:
        return HandType.THREEOFAKIND if max(count.values()) + j == 3 else HandType.TWOPAIR
    elif n == 4:
        return HandType.ONEPAIR
    elif n == 5:
        return HandType.HIGHCARD
    else:
        raise Exception("unreachble")


with open("07.txt") as f:
    xs = list(map(lambda x: (x[0], int(x[1].strip())), [line.split(" ") for line in f.readlines()]))
    xs.sort(key=cmp_to_key(compareHand))
    sum = 0
    for i, x in enumerate(xs):
        sum += x[1] * (i + 1)
    print(sum)