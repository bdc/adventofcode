import argparse
from collections import defaultdict, namedtuple
from pathlib import Path


def handleLine(i, line, ctx=None):
    for j, c in enumerate(line):
        ctx['grid'][(i,j)] = c


def getExpandedSpaces(grid):
    galaxies = [(i, j) for (i, j) in grid.keys() if grid[(i, j)] == '#']
    maxI = max(i for i, _ in grid.keys())
    maxJ = max(j for _, j in grid.keys())
    galaxyIs = set(i for i, _ in galaxies)
    galaxyJs = set(j for _, j in galaxies)
    emptyIs = set(range(maxI)).difference(galaxyIs)
    emptyJs = set(range(maxJ)).difference(galaxyJs)
    return (emptyIs, emptyJs)


def getDist(a, b, skips, expansion=1):
    d = abs(a - b)
    m = min(a, b)
    n = max(a, b)
    d += len(list(x for x in skips if m < x and x < n)) * (expansion - 1)
    return d


def sumDists(grid, expansion):
    galaxies = [(i, j) for (i, j) in grid.keys() if grid[(i, j)] == '#']
    dist = 0
    emptyIs, emptyJs = getExpandedSpaces(grid)
    for i in range(len(galaxies) - 1):
        i1, j1 = galaxies[i]
        for j in range(i + 1, len(galaxies)):
            i2, j2 = galaxies[j]
            distI = getDist(i1, i2, emptyIs, expansion=expansion)
            distJ = getDist(j1, j2, emptyJs, expansion=expansion)
            dist += distI + distJ
    return dist


def process1(ctx):
    grid = ctx['grid']
    return sumDists(grid, 2)


def process2(ctx):
    grid = ctx['grid']
    return sumDists(grid, 1000000)


def main(part, input):
    filename = Path(__file__).with_name(input)
    ctx = {'grid': defaultdict()}

    with open(filename, 'r') as f:
        for i, line in enumerate(f.readlines()):
            l = line.strip()
            if l:
                handleLine(i, l, ctx)

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
