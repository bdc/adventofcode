import argparse
from collections import namedtuple
from pathlib import Path


Node = namedtuple('Node', 'val')


def handleLine(line, i, ctx=None):
    graph = ctx['graph']
    graph2 = ctx['graph2']
    for j, c in enumerate(line):
        for d in 'nswe':
            for p in range(3):
                graph[(i, j, d, p)] = Node(int(c))
            for p in range(10):
                graph2[(i, j, d, p)] = Node(int(c))


def move(i, j, d, p, go):
    goL = {'n': 'w', 'w': 's', 's': 'e', 'e': 'n'}
    goR = {'n': 'e', 'e': 's', 's': 'w', 'w': 'n'}
    if go == 'l':
        d = goL[d]
        p = 0
    elif go == 'r':
        d = goR[d]
        p = 0
    else:
        p = p + 1

    dist = 1 if go == 'f' else 1
    if d == 'n':
        i -= dist
    if d == 's':
        i += dist
    if d == 'w':
        j -= dist
    if d == 'e':
        j += dist
    return i, j, d, p


def getAdjacents1(key, graph):
    adjacents = [
        move(*key, 'l'),
        move(*key, 'r'),
        move(*key, 'f'),
    ]
    return list(filter(lambda c: c in graph, adjacents))


def getAdjacents2(key, graph):
    _, _, _, p = key

    adjacents = [
        move(*key, 'l') if p > 2 else None,
        move(*key, 'r') if p > 2 else None,
        move(*key, 'f') if p < 10 else None,
    ]
    return list(filter(lambda c: c in graph, adjacents))


def printGrid(graph, visited):
    maxI = max(i for i, _, _, _ in graph)
    maxJ = max(j for _, j, _, _ in graph)
    vs = {(i, j): 0 for i, j, _, _ in visited}
    for ij in vs:
        vs[ij] = {v: d for (i, j, d, _), v in visited.items()
                  if (ij == (i, j))}
        vs[ij] = vs[ij][min(vs[ij])]
    for i in range(maxI+1):
        row = ''
        for j in range(maxJ+1):
            if (i, j) in vs:
                row += {'n': '^', 's': 'v', 'w': '<', 'e': '>'}[vs[(i, j)]]
            else:
                row += str(graph[i, j, 'n', 0].val)
        print(row)


def getBest(graph, part=1):
    adj = getAdjacents1 if part == 1 else getAdjacents2
    maxI = max(i for i, _, _, _ in graph)
    maxJ = max(j for _, j, _, _ in graph)
    active = [(0, 0, 'e', -1), (0, 0, 's', -1)]
    visited = {key: 0 for key in active}
    count = 0
    best = (maxI + maxJ) * 10
    while active:
        count += 1
        if not count % 1000000:
            print(best, len(visited), len(active))
        key = active.pop()
        nextKeys = adj(key, graph)
        for nextKey in nextKeys:
            score = visited[key] + graph[nextKey].val
            if score >= best:
                continue
            if nextKey in visited and visited[nextKey] <= score:
                continue
            visited[nextKey] = score
            if nextKey[0:2] == (maxI, maxJ):
                if part == 2 and nextKey[3] < 3:
                    pass
                else:
                    best = min(best, score)
                    print(best, len(visited), len(active))
            active.append(nextKey)
    return best


def process1(ctx):
    graph = ctx['graph']
    best = getBest(graph, part=1)
    return best


def process2(ctx):
    graph = ctx['graph2']
    best = getBest(graph, part=2)
    return best


def main(part, input):
    filename = Path(__file__).with_name(input)
    ctx = {'graph': {}, 'graph2': {}}

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
