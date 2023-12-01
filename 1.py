data = open("1_input.txt", "r").read().splitlines()

digits = [[int(d) for d in line if d.isdigit()] for line in data]
print("part 1:", sum(10 * d[0] + d[-1] for d in digits))

nrs = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
map = {}
for v, nr in enumerate(nrs):
    map[nr] = v
    map[str(v)] = v
sum2 = 0
for line in data:
    d = [v for i in range(len(line)) for key, v in map.items() if line[i:].startswith(key)]
    sum2 += 10*d[0] + d[-1]
print("part 2:", sum2)
