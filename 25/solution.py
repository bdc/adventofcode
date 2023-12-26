import argparse
from collections import defaultdict, namedtuple
from pathlib import Path
import re


def handleLine(line, i, ctx=None):
    m = re.match(r'(?P<nodeFrom>\w+): (?P<nodesTo>.*)', line)
    for nodeTo in m.group('nodesTo').split(' '):
        ctx['edges'][nodeTo].add(m.group('nodeFrom'))
        ctx['edges'][m.group('nodeFrom')].add(nodeTo)


def connectivity(n1, n2, edges):
    seen = set()
    count = 0
    while True:
        result = shortestPath(n1, n2, edges, seen)
        if not result:
            return count
        count += 1
        seen.update(result)


def shortestPath(n1, n2, edges, seen):
    newSeen = set()
    active = [(n1, [n1])]
    while active:
        n, path = active.pop(0)
        if n == n2:
            return set(path[1:-1])
        for nn in edges[n]:
            if nn in seen or nn in newSeen:
                continue
            newSeen.add(nn)
            active.append((nn, path + [nn]))  
    return None


def process1(ctx):
    edges = ctx['edges']
    nodes = list(edges.keys())
    pathNodeCount = defaultdict(int)
    # n1 = 'cmg'
    # n2 = 'hfx'
    # print(n1, n2, connectivity(n1, n2, edges))
    dataset = []
    for i, n1 in enumerate(nodes):
        print(i)
        for n2 in nodes:
            if n1 == n2:
                continue
            if n2 in edges[n1]:
                continue
            dataset.append((n1, n2, connectivity(n1, n2, edges)))
    groups = []
    for n1, n2, c in sorted(dataset):
        if c <= 3:
            continue
        added = False
        for group in groups:
            if n1 in group or n2 in group:
                group.update((n1, n2))
                added = True
                break
        if not added:
            groups.append(set((n1, n2)))
    for group in groups:
        print(len(group), group)
    print(len(nodes) - sum(len(group) for group in groups), 'solo')
    

        


    # print(list((a,b,c) for a,b,c in dataset if c > 3))
    
    # print(len(nodes))
    # print(sorted(pathNodeCount.items(), key=lambda item: item[1]))
    # data = [(edge[0], len(edge[1])) for edge in edges.items()]
    # map = defaultdict(int)
    # for k, v in data:
    #     map[v] += 1
    # print(map)
    return 1


def process2(ctx):
    return 2


def main(part, input):
    filename = Path(__file__).with_name(input)
    ctx = {'edges': defaultdict(set)}

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
