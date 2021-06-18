with open("nestest.log") as f:
    data = f.read().strip().splitlines()

lines = []

for line in data:
    s = [x.strip() for x in line.split(" ")]
    l = []
    for item in s:
        if item.startswith("PPU:"):
            break
        l.append(item)
    lines.append(l)

for i, line in enumerate(lines[:-1]):
    lines[i][-1] = lines[i+1][-1]
    lines[i][-2] = lines[i+1][-2]
    lines[i][-3] = lines[i+1][-3]
    lines[i][-4] = lines[i+1][-4]
    lines[i][-5] = lines[i+1][-5]
    lines[i][[i for i, n in enumerate(lines[i]) if n == ''][1]:-5] = []
    lines[i].pop(1)
    print(lines[i])

with open("nestest-try.log", "w") as f:
    f.write("\n".join(map(lambda l: ' '.join(l), lines)))
