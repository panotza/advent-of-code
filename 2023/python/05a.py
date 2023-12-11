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

        if seed < src or seed > srcTo:
            continue
        
        offset = seed - src
        expect = dst + offset

        if expect < dst or expect > dstTo:
            continue

        return expect
    return seed

with open("05.txt") as f:
    p = Parser(f.read())
    xs = p.parse()

    locations = []
    seeds = xs[0]["mappings"][0]
    for s in seeds:
        for x in xs[1:]:
           s = map(s, x["mappings"])
        locations.append(s)
    print("ans", min(locations))