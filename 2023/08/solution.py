import argparse
from collections import namedtuple
import itertools
import math
from pathlib import Path
import re


Node = namedtuple('Node', 'name l r')


def handleLine(line, ctx=None):
    if re.match(r'^[LR]+$', line):
        ctx['lr'] = line
        return
    if not line:
        return
    node = Node(*re.findall(r'\w{3}', line))
    ctx['nodes'][node.name] = node


def process1(ctx):
    node = ctx['nodes']['AAA']
    for i, lr in enumerate(itertools.cycle(ctx['lr']), 1):
        node = ctx['nodes'][getattr(node, lr.lower())]
        if node.name == 'ZZZ':
            return i


def getCycleSize(node: Node, ctx):
    # This works in a simple way due to certain properties of the input data
    for i, lr in enumerate(itertools.cycle(ctx['lr']), 1):
        node = ctx['nodes'][getattr(node, lr.lower())]
        if node.name.endswith('Z'):
            return i


def process2(ctx):
    nodes = [node for node in ctx['nodes'].values() if node.name.endswith('A')]
    cycleSizes = [getCycleSize(node, ctx) for node in nodes]
    result = math.lcm(*cycleSizes)
    return result


def main(part, input):
    filename = Path(__file__).with_name(input)
    ctx = {'lr': '', 'nodes': {}}

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
