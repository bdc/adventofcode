import argparse
import itertools
from pathlib import Path


def handleLine(line, i, ctx=None):
    for j, c in enumerate(line):
        ctx['grid'][(i, j)] = c
    ctx['w'] = max(ctx['w'], j + 1)
    ctx['h'] = max(ctx['h'], i + 1)


def move(grid, start, direction):
    fromI, fromJ = start
    dirI, dirJ = direction
    while True:
        to = fromI + dirI, fromJ + dirJ
        if to not in grid:
            return
        if grid[to] != '.':
            return
        grid[to] = grid[(fromI, fromJ)]
        grid[(fromI, fromJ)] = '.'
        fromI, fromJ = to


def moveAll(ctx, direction):
    h, w = ctx['h'], ctx['w']
    grid: dict = ctx['grid']
    iter = itertools.product(range(h), range(w))
    if sum(direction) > 0:
        iter = reversed(list(iter))
    for ij in iter:
        if grid[ij] == 'O':
            move(grid, ij, direction)


def spin(ctx):
    for direction in ((-1, 0), (0, -1), (1, 0), (0, 1)):
        moveAll(ctx, direction)


def spinUntilLoop(ctx):
    seen = {}
    for i in range(1000):
        spin(ctx)
        s = serialize(ctx)
        if s in seen:
            return i, i - seen[s]
        seen[s] = i


def getScore(ctx):
    h, _ = ctx['h'], ctx['w']
    grid: dict = ctx['grid']
    score = sum(h - i for i, j in grid.keys() if grid[i, j] == 'O')
    return score


def serialize(ctx):
    result = ''
    for i in range(ctx['h']):
        result += str(''.join(ctx['grid'][i, j] for j in range(ctx['w']))) + '\n'
    return result


def process1(ctx):
    moveAll(ctx, (-1, 0))
    return getScore(ctx)


def process2(ctx):
    i, loop = spinUntilLoop(ctx)
    n = (1000000000 - i - 1) % loop
    for i in range(n):
        spin(ctx)
    return getScore(ctx)


def main(part, input):
    filename = Path(__file__).with_name(input)
    ctx = {'grid': {}, 'w': 0, 'h': 0}

    with open(filename, 'r') as f:
        for i, line in enumerate(f.readlines()):
            l = line.strip()
            if l:
                handleLine(l,i, ctx)

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
