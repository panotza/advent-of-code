import re

def calcMatchedCnt(winingNumbers: set[str], haveNumbers: [str]):
    matchedCnt = 0
    for n in haveNumbers:
        if n in winingNumbers:
            matchedCnt += 1
    return matchedCnt

def parseCard(s: str):
    s1, s2 = s.split(":")
    cardNo = re.findall("\d+", s1)[0]

    ns = s2.split("|")
    winingNumbers = set(re.findall("\d+", ns[0]))
    haveNumbers = re.findall("\d+", ns[1])
    return cardNo, winingNumbers, haveNumbers

with open("04.txt") as f:
    cards = f.readlines()
    receiveCardIds = [i for i, _ in enumerate(cards)]
    maxId = receiveCardIds[-1]

    curr = 0
    while curr < len(receiveCardIds):
        card = cards[receiveCardIds[curr]]
        cardNo, winingNumbers, haveNumbers = parseCard(card)
        matchedCnt = calcMatchedCnt(winingNumbers, haveNumbers)
        cardIdx = int(cardNo)
        for i in range(matchedCnt):
            idx = cardIdx + i
            if idx > maxId:
                break
            receiveCardIds.append(idx)

        curr += 1
    print("ans", len(receiveCardIds))