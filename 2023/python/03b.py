with open("03.txt") as f:
    content = f.read()
    size = content.find("\n")
    size += 1

    sum = 0
    for i, c in enumerate(content):
        if c == "*":
            # check L LT T TR R RB B BL
            l = -1 if i-1 < 0 else i-1
            lt = -1 if ((i-size)-1) < 0 else ((i-size)-1)
            t = -1 if i-size < 0 else i-size
            tr = -1 if ((i-size)+1) < 0 else ((i-size)+1)
            r = -1 if i+1 >= len(content) else i+1
            rb = -1 if i+size+1 >= len(content) else i+size+1
            b = -1 if i+size >= len(content) else i+size
            bl = -1 if i+size-1 >= len(content) else i+size-1
            neighbors = [l, lt, t, tr, r, rb, b, bl]
            visited = {}

            digits = []
            for i in neighbors:
                if i < 0:
                    continue
                if visited.get(i, False):
                    continue

                if content[i].isdigit():
                    # collect digit left
                    l = i
                    while l > 0:
                        l -= 1
                        if not content[l].isdigit():
                            l += 1
                            break
                    # collect digit right
                    r = i
                    while r < len(content):
                        r += 1
                        if not content[r].isdigit():
                            break
                    for j in range(l,r):
                        visited[j] = True

                    digits.append(int(content[l:r]))
            if len(digits) == 2:
                sum += digits[0] * digits[1]
    print(sum) # 81166799