import argparse
from collections import defaultdict, namedtuple
from pathlib import Path


Node = namedtuple('Node', 'i j c dist')


def handleLine(line, i, ctx=None):
    for j, c in enumerate(line):
        node = Node(i, j, c, [])
        ctx['grid'][(i, j)] = node


def findSCoords(grid: dict):
    for ij, node in grid.items():
        if node.c == 'S':
            return ij


def getAdjacentCoords(grid, start):
    c = grid[start].c
    i, j = start
    ijN = i-1, j
    ijS = i+1, j
    ijW = i, j-1
    ijE = i, j+1
    if c == '.':
        return []
    if c == '-':
        return [ijW, ijE]
    if c == '|':
        return [ijN, ijS]
    if c == 'L':
        return [ijN, ijE]
    if c == 'F':
        return [ijS, ijE]
    if c == '7':
        return [ijW, ijS]
    if c == 'J':
        return [ijN, ijW]
    if c == 'S':
        return [ij for ij in [ijN, ijS, ijW, ijE] if start in getAdjacentCoords(grid, ij)]


def getAdjacentNodes(grid, node):
    ijs = getAdjacentCoords(grid, (node.i, node.j))
    return [grid[ij] for ij in ijs]


def process1(ctx):
    grid = ctx['grid']
    node = grid[findSCoords(grid)]
    dist = 0
    node.dist.append(dist)
    while True:
        dist += 1
        nodes = [n for n in getAdjacentNodes(grid, node) if not n.dist]
        if not nodes:
            return int(dist / 2)
        node = nodes[0]
        node.dist.append(dist)


def process2(ctx):
    grid = ctx['grid']
    # Depends on the input file. Could be determined programmatically, but isn't.
    sDigit = '7'
    iMax, jMax = max(grid)
    count = 0
    for i in range(iMax):
        numBounds = 0
        c0 = None
        for j in range(jMax):
            node = grid[(i, j)]

            if node.dist:
                c1 = node.c if node.c != 'S' else sDigit
                if c1 == '|':
                    numBounds += 1
                if c1 in 'FJ7L':
                    if not c0:
                        c0 = c1
                    else:
                        if c0 + c1 in ['FJ', 'L7']:
                            numBounds += 1
                        c0 = None
            else:
                count += numBounds % 2
    return count


def main(part, input):
    filename = Path(__file__).with_name(input)
    ctx = {'grid': defaultdict(lambda: Node(-10, -10, '.', []))}

    with open(filename, 'r') as f:
        for i, line in enumerate(f.readlines()):
            l = line.strip()
            if l:
                handleLine(l, i, ctx)

    result = process1(ctx)
    if part == 2:
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
