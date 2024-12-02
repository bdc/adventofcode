import argparse
from collections import defaultdict, namedtuple
from pathlib import Path
import re


MapEntry = namedtuple('MapEntry', 'dst src l')
Map = namedtuple('Map', 'a b entries')
Range = namedtuple('Range', 'a l')


def handleLine(line, ctx=None):
    if re.match(r'seeds:', line):
        ctx['seeds'] = [int(i) for i in line[7:].split(' ')]
        return

    m = re.match(r'(\w+)\-to\-(\w+) map:', line)
    if m:
        ctx['maps'].append(Map(m.group(1), m.group(2), []))
        return

    m = re.match(r'(\d+) (\d+) (\d+)', line)
    if m:
        ctx['maps'][-1].entries.append(MapEntry(*(int(i) for i in m.groups())))


def convert(num, map):
    for e in map.entries:
        if e.src <= num and e.src + e.l > num:
            return num + e.dst - e.src
    return num


def process1(ctx):
    nums = ctx['seeds']
    for m in ctx['maps']:
        nums = [convert(n, m) for n in nums]
    return min(nums)


def toRanges(nums):
    return [Range(*nums[i:i+2]) for i in range(0, len(nums), 2)]


def transposeRange(range, mapEntry):
    return Range(range.a + mapEntry.dst - mapEntry.src, range.l)


def convertRange(r, m):
    for e in m.entries:
        minMatch = max(r.a, e.src)
        maxMatch = min(r.a + r.l, e.src + e.l)
        if maxMatch > minMatch:
            r1 = Range(r.a, e.src - r.a)
            r2 = Range(minMatch, maxMatch - minMatch)
            r3 = Range(e.src + e.l, r.a + r.l - e.src - e.l)
            return convertRange(r1, m) + [transposeRange(r2, e)] + convertRange(r3, m)
    return [r] if r.l > 0 else []


def process2(ctx):
    nums = ctx['seeds']
    ranges = toRanges(nums)
    for m in ctx['maps']:
        nextRanges = []
        for r in ranges:
            nextRanges.extend(convertRange(r, m))
        ranges = nextRanges
    return min(r.a for r in ranges)


def main(part, input):
    filename = Path(__file__).with_name(input)
    ctx = {
        'seeds': [],
        'maps': [],
    }

    with open(filename, 'r') as f:
        for i, line in enumerate(f.readlines()):
            l = line.strip()
            if l:
                handleLine(l, ctx)

    if part == 1:
        result = process1(ctx)
    else:
        result = process2(ctx)
    return result


def init():
    parser = argparse.ArgumentParser()
    parser.add_argument('--input', type=str, default='input.txt')
    args = parser.parse_args()
    result = main(1, args.input)
    print(result)
    result = main(2, args.input)
    print(result)


init()
