import argparse
from collections import namedtuple
import functools
from pathlib import Path
import re


Row = namedtuple('Row', 'line widths')


def handleLine(line, ctx=None):
    m = re.match('^(?P<line>[^ ]+) (?P<widths>[\d,]+)', line)
    widths = tuple(int(i) for i in m.group('widths').split(','))
    ctx['rows'].append(Row(m.group('line'), widths))


def unfold(row):
    return Row('?'.join([row.line] * 5), row.widths * 5)


@functools.lru_cache
def countFits(line, widths):
    n = 0
    if not widths:
        ret = 0 if '#' in line else 1
        return ret
    if not line:
        ret = 0 if widths else 1
        return ret
    if sum(widths) + len(widths) - 1 > len(line):
        ret = 0
        return ret
    pivot = int(len(line) / 2)
    for wI, wVal in enumerate(widths):
        if line[pivot] != '#':
            n1 = countFits(line[0:pivot], widths[0:wI])
            n2 = countFits(line[pivot+1:], widths[wI:])
            n += n1*n2
        if line[pivot] != '.':
            for i in range(wVal):
                lr = splitRow(line, pivot-i, pivot-i+wVal)
                if not lr:
                    continue
                l, r = lr
                n1 = countFits(l, widths[0:wI])
                n2 = countFits(r, widths[wI+1:])
                n += n1*n2
    if line[pivot] != '#':
        n1 = countFits(line[0:pivot], widths)
        n2 = countFits(line[pivot+1:], tuple())
        n += n1*n2
    return n


def splitRow(line, i, j):
    if '.' in line[i:j]:
        return None
    if i < 0:
        return None
    if j > len(line):
        return None
    l = line[0:i]
    r = line[j:]
    if l:
        if l[-1] == '#':
            return None
        l = l[0:-1]
    if r:
        if r[0] == '#':
            return None
        r = r[1:]
    return l, r


def process1(ctx):
    ns = (countFits(row.line, row.widths) for row in ctx['rows'])
    return sum(ns)


def process2(ctx):
    s = 0
    for i, row in enumerate(ctx['rows']):
        row = unfold(row)
        n = countFits(row.line, row.widths)
        s += n
    return s


def main(part, input):
    filename = Path(__file__).with_name(input)
    ctx = {'rows': []}

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
