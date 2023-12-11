import re

class Parser:
    curr = 0

    def __init__(self, input):
        self.input = input

    def parse(self):
        category = []
        while c := self.next():
            if c.isalpha():
                self.back()
                x = self.drainCategory()
                category.append(x)
        return category

    def drainCategory(self):
        cate = ""
        while c := self.next():
            if c.isalpha() or c == "-":
                cate = cate + c
            elif c == ":":
                break
            elif c.isspace():
                pass
            else:
                err = format(f"invalid category format: unexpected {c}")
                raise Exception(err)
        return {
            "cate": cate,
            "mappings": self.drainMapping(),
        } 
    
    def drainMapping(self):
        cs = ""
        digits = []
        mappings = []
        while c := self.next():
            if c.isdigit():
                cs = cs + c
            elif c == " ":
                if len(cs) > 0:
                    digits.append(int(cs))
                    cs = ""
            elif c.isspace():
                if len(cs) > 0:
                    digits.append(int(cs))
                    cs = ""
                if len(digits) > 0:
                    mappings.append(digits)
                    digits = []
            elif c.isalpha():
                self.back()
                break
            else:
                raise Exception("invalid category format")
        if len(cs) > 0:
            digits.append(int(cs))
        if len(digits) > 0:
            mappings.append(digits)
        return mappings

    def next(self) -> str | None:
        if self.curr < len(self.input):
            c = self.input[self.curr]
            self.curr += 1
            return c
        else:
            return None

    def back(self):
        self.curr -= 1

def map(seed, mappings: [[int]]) -> int:
    for m in mappings:
        dst, src, range = m
        srcTo = src + range - 1
        dstTo = dst + range - 1

        if seed < dst or seed > dstTo:
            continue
        
        offset = seed - dst
        expect = src + offset

        if expect < src or expect > srcTo:
            continue
        
        return expect
    return seed

with open("05.txt") as f:
    p = Parser(f.read())
    xs = p.parse()

    rangeList = []
    seeds = xs[0]["mappings"][0]
    for i in range(0, len(seeds), 2):
        range_from = seeds[i]
        range_to = range_from + seeds[i+1] - 1
        rangeList.append((range_from, range_to))

    # iterate seed in reverse order
    xs = xs[1:]
    xs.reverse()
    s = 0
    while True:
        seed = s
        for x in xs:
            seed = map(seed, x["mappings"])
        for ss in rangeList:
            if seed >= ss[0] and seed <= ss[1]:
                print("ans", s, seed)
                exit(0)
        s += 1
    # 219529182