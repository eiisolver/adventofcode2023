from dataclasses import dataclass

def overlaps(start0, end0, start1, end1):
    return start0 <= end1 and start1 <= end0

@dataclass
class RangeMap:
    dest: int
    src: int
    length: int

@dataclass
class Range:
    src: int
    length: int

def overlaps(r1, r2):
    return r1.src < r2.src + r2.length and r2.src < r1.src + r1.length

class Map:
    def __init__(self) -> None:
        self.ranges = []

    def get(self, v):
        for r in self.ranges:
            if r.src <= v < r.src + r.length:
                return r.dest + v - r.src
        return v
    
    def map_range(self, range: Range, output):
        # Split input in smaller ranges, each resulting range overlaps with max 1 map range
        split_input = [range]
        for r in self.ranges:
            temp = []
            for r2 in split_input:
                if overlaps(r, r2):
                    start = r2.src
                    end = start + r2.length
                    if start < r.src:
                        temp.append(Range(src=start, length=r.src - start))
                        start = r.src
                    if end > r.src + r.length:
                        start2 = r.src + r.length
                        temp.append(Range(src=start2, length=end - start2))
                        end = start2
                    temp.append(Range(src=start, length=end - start))
                else:
                    temp.append(r2)
            split_input = temp

        # Map the splitted input ranges to output.
        for r_in in split_input:
            mapped = False
            for r in self.ranges:
                if overlaps(r, r_in):
                    mapped = True
                    output.append(Range(src=r.dest + r_in.src - r.src, length=r_in.length))
            if not mapped:
                output.append(r_in)


data = open("input.txt", "r").read().splitlines()
seeds = [int(x) for x in data[0].split(":")[1].split()]
maps = []
curr_map = None
for line in data[2:]:
    if line.endswith(":"):
        curr_map = Map()
        maps.append(curr_map)
    elif line:
        lst = [int(x) for x in line.split()]

        curr_map.ranges.append(RangeMap(dest=lst[0], src=lst[1], length=lst[2]))

def loc(seed):
    for map in maps:
        seed = map.get(seed)
    return seed

v = 1e110
for seed in seeds:
    value = loc(seed)
    if value < v:
        v = value
print("Part 1:", v)

in_ranges = [Range(src, length) for src, length in zip(seeds[0::2], seeds[1::2])]
for map in maps:
    out_ranges = []
    for r in in_ranges:
        map.map_range(r, out_ranges)
    in_ranges = out_ranges
print("Part 2:", min(r.src for r in out_ranges))
