import argparse
from collections import defaultdict, namedtuple
import datetime
from pathlib import Path


Edge = namedtuple('Edge', 'ij1 ij2 d')


def handleLine(line, i, ctx=None):
    for j, c in enumerate(line):
        ctx['grid'][(i, j)] = c


def getAdjacents(ij, grid, slippery=False):
    i, j = ij
    candidates = [(i+1, j, '.v'), (i-1, j, '.^'),
                  (i, j+1, '.>'), (i, j-1, '.<')]
    if slippery:
        if grid[ij] == '<':
            candidates = [(i, j-1, '.<')]
        if grid[ij] == '>':
            candidates = [(i, j+1, '.>')]
        if grid[ij] == '^':
            candidates = [(i-1, j, '.^')]
        if grid[ij] == 'v':
            candidates = [(i+1, j, '.v')]
    candidates = filter(lambda ijc: ijc[0:2] in grid, candidates)
    if slippery:
        candidates = filter(lambda ijc: grid[ijc[0:2]] in ijc[2], candidates)
    else:
        candidates = filter(lambda ijc: grid[ijc[0:2]] != '#', candidates)
    return [(i, j) for i, j, _ in candidates]


def findNextNodes(node, grid):
    start = node[:]
    seen = {start}
    active = [(start, 0)]
    edges = []
    while active:
        ij, distance = active.pop(0)
        nextIjs = getAdjacents(ij, grid, slippery=True)
        for nextIj in nextIjs:
            if nextIj in seen:
                continue
            seen.add(nextIj)
            if grid[nextIj] in '<>^v':
                edges.append(Edge(start, nextIj, distance+1))
                edges.append(Edge(nextIj, start, distance+1))
            else:
                active.append((nextIj, distance+1))
    return edges


def calcMaxDist(end, start, nodeToEdgeMap):
    if end == start:
        return 0
    return max(calcMaxDist(edge.ij1, start, nodeToEdgeMap) + edge.d for edge in nodeToEdgeMap[end])


def process1(ctx):
    grid = ctx['grid']
    maxI = max(i for i, _ in grid.keys())
    maxJ = max(j for _, j in grid.keys())
    start = 0, 1
    end = maxI, maxJ-1
    grid[end] = 'v'

    allEdges = set()
    nodeToEdgeMap = defaultdict(list)
    active = [start]
    seen = set()
    while active:
        ij = active.pop(0)
        if ij in seen:
            continue
        seen.add(ij)
        edges = findNextNodes(ij, grid)
        for edge in edges:
            if edge in allEdges:
                continue
            if (edge[1], edge[0], edge[2]) in allEdges:
                continue
            allEdges.add(edge)
            nodeToEdgeMap[edge.ij2].append(edge)
            active.append(edge.ij2)
    score = calcMaxDist(end, start, nodeToEdgeMap)
    return score


def findAllNodes(grid):
    nodes = []
    for key in grid:
        if grid[key] == '#':
            continue
        adjs = getAdjacents(key, grid)
        if len(adjs) > 2:
            nodes.append(key)
    return nodes


def findAllEdges(nodes, grid):
    edges = []
    for node in nodes:
        seen = {node}
        active = [(node, 0)]
        while active:
            square, dist = active.pop(0)
            adjs = getAdjacents(square, grid)
            for adj in adjs:
                if adj in seen:
                    continue
                seen.add(adj)
                if adj == '#':
                    continue
                if adj in nodes:
                    edges.append(Edge(node, adj, dist + 1))
                    continue
                active.append((adj, dist + 1))
    return edges

_n = 0

def getMaxDist(start, end, edges, visited=tuple()):
    global _n
    _n += 1
    visited = set(visited).union((start,))
    if start == end:
        return 0
    best = -9999
    for edge in (e for e in edges if e.ij1 == start and e.ij2 not in visited):
        score = getMaxDist(edge.ij2, end, edges, visited) + edge.d
        best = max(best, score)
    return best


def process2(ctx):
    grid = ctx['grid']
    maxI = max(i for i, _ in grid.keys())
    maxJ = max(j for _, j in grid.keys())
    start = 0, 1
    end = maxI, maxJ-1
    nodes = [start, end] + findAllNodes(grid)
    edges = findAllEdges(nodes, grid)
    # for node in nodes:
    #     print(node, sum(1 for e in edges if e.ij1 == node))
    # print(datetime.datetime.now())
    result = getMaxDist(start, end, edges)
    # print(result, _n)
    # print(datetime.datetime.now())
    return getMaxDist(start, end, edges)


def main(part, input):
    filename = Path(__file__).with_name(input)
    ctx = {'grid': defaultdict(str)}

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
