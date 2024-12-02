import argparse
from collections import namedtuple
import itertools
from pathlib import Path
import re


Edge = namedtuple('Edge', 'dir len hex', defaults=(None,))


def handleLine(line, i, ctx=None):
    m = re.match(r'(\w) (\d+) \(#(.{6})\)', line)
    edge = Edge(m.group(1), int(m.group(2)), m.group(3))
    ctx['edges'].append(edge)


def decodeHex(hex):
    dir = 'RDLU'[int(hex[5])]
    len = int(hex[0:5], 16)
    return dir, len


def getArea(edges):
    j, sum = 0, 1
    for edge in edges:
        dir, len = edge.dir, edge.len
        if dir == 'R':
            j += len
            sum += len
        if dir == 'L':
            j -= len
        if dir == 'D':
            sum += (j + 1) * len
        if dir == 'U':
            sum -= j * len
    return sum


def process1(ctx):
    edges = ctx['edges']
    return getArea(edges)


def process2(ctx):
    edges = [Edge(*decodeHex(edge.hex)) for edge in ctx['edges']]
    return getArea(edges)


def main(part, input):
    filename = Path(__file__).with_name(input)
    ctx = {'edges': []}

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
