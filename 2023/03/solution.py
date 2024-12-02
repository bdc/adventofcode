import argparse
import re
import itertools
import functools
from collections import defaultdict, namedtuple
from pathlib import Path


Num = namedtuple('num', 'val xy')
Tool = namedtuple('tool', 'symbol xy')


def fuzz(xy):
    x, y = xy
    xys = ((x+i, y+j)
           for i, j in itertools.product(range(-1, 2), range(-1, 2)))
    return xys


def handleLine(line, i, ctx=None):
    matches = re.finditer(r'(\d+)', line)
    for m in matches:
        num = Num(int(m.group()), (i, m.span()[0]))
        ctx['nums'].append(num)
    matches = re.finditer(r'([^\d\.])', line)
    for m in matches:
        tool = Tool(m.group(), (i, m.span()[0]))
        ctx['tools'].append(tool)
    map = defaultdict(set)
    for n in ctx['nums']:
        for i in range(len(str(n.val))):
            map[(n.xy[0], n.xy[1] + i)].add(n)
    for t in ctx['tools']:
        map[t.xy].add(t)
    ctx['map'] = map


def process1(ctx):
    sum = 0
    for n in ctx['nums']:
        xys1 = ((n.xy[0], n.xy[1]+i) for i in range(len(str(n.val))))
        xys2 = set(itertools.chain.from_iterable(fuzz(xy) for xy in xys1))
        for xy in xys2:
            if any(el.__class__ == Tool for el in ctx['map'][xy]):
                sum += n.val
    return sum


def process2(ctx):
    sum = 0
    for t in ctx['tools']:
        if t.symbol != '*':
            continue
        ns = set(el for el in itertools.chain.from_iterable(
            ctx['map'][xy] for xy in fuzz(t.xy)) if el.__class__ == Num)
        if len(ns) == 2:
            sum += functools.reduce(lambda x, y: x*y, (n.val for n in ns))
    return sum


def main(part, input):
    filename = Path(__file__).with_name(input)
    ctx = {
        'nums': [],
        'tools': [],
    }

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
