from functools import cache

data = open("input.txt", "r").read().splitlines()
nr_winning = []

part1 = 0
for line in data:
    parts = line.split(":")[1].split("|")
    winning = set(int(x) for x in parts[0].split())
    my_nrs = set(int(x) for x in parts[1].split())
    my_winning = len(winning & my_nrs)
    nr_winning.append(my_winning)
    if my_winning > 0:
        part1 += 1 << (my_winning - 1)
print("Part 1:", part1)

@cache
def extra_copies(i):
    copies = 0
    for j in range(nr_winning[i]):
        copies += 1 + extra_copies(i + j + 1)
    return copies

part2 = len(data)
for i in range(len(data)):
    part2 += extra_copies(i)
print("Part 2:", part2)
