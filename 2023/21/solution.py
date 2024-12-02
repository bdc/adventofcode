import argparse
from collections import defaultdict
from pathlib import Path


def handleLine(line, i, ctx=None):
    for j, c in enumerate(line):
        ctx['grid'][(i, j)] = c


def getAdjacents(ij):
    i, j = ij
    return [(i+1, j), (i-1, j), (i, j+1), (i, j-1),]


def printGrid(grid):
    maxI = max(i for i, _ in grid)
    maxJ = max(j for _, j in grid)
    for i in range(maxI + 1):
        line = ''
        for j in range(maxJ + 1):
            line += grid[(i, j)]
        print(line)


def process1(ctx):
    grid = ctx['grid']
    s = [ij for ij in grid if grid[ij] == 'S'][0]
    active = [s]
    steps = 64
    for i in range(steps+1):
        nextActive = []
        for ij in active:
            if ij not in grid:
                continue
            if grid[ij] not in '.S':
                continue
            grid[ij] = 'o' if i % 2 else 'e'
            nextActive.extend(getAdjacents(ij))
        active = nextActive
    result = sum(1 for ij in grid if grid[ij] == 'e')
    return result


def calcAt(iterArr, grid):
    vals = []
    side = max(i for i, _ in grid) + 1
    bigGrid = defaultdict(str)
    s = [ij for ij in grid if grid[ij] == 'S'][0]
    active = [s]
    for iter in range(iterArr[-1] + 1):
        if not active:
            break
        nextActive = []
        for ij in active:
            i, j = ij
            if ij in bigGrid:
                val = bigGrid[ij]
            else:
                val = grid[(i % side, j % side)]
            if val not in '.S':
                continue
            bigGrid[ij] = 'o' if iter % 2 else 'e'
            nextActive.extend(getAdjacents(ij))
        active = nextActive
        if iter in iterArr:
            char = 'o' if iter % 2 else 'e'
            score = sum(1 for ij in bigGrid if bigGrid[ij] == char)
            vals.append(score)
    return vals


def calcQuadratic(ys):
    y1, y2, y3 = ys
    a = int((y1 - 2*y2 + y3) / 2)
    b = y2 - y1 - a
    c = y1
    return a, b, c


def process2(ctx):
    grid = ctx['grid']
    side = max(i for i, _ in grid) + 1
    lastIter = 26501365
    iters = [lastIter % (2*side) + i * (2*side) for i in range(3)]
    vals = calcAt(iters, grid)

    a,b,c = calcQuadratic(vals)
    x = int(lastIter / (2*side))
    return a*x*x + b*x + c


def main(part, input):
    filename = Path(__file__).with_name(input)
    ctx = {'grid': {}}

    with open(filename, 'r') as f:
        for i, line in enumerate(f.readlines()):
            l = line.strip()
            if l:
                handleLine(l, i, ctx)

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
