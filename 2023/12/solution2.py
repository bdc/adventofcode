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


@functools.lru_cache(maxsize=10000)
def calc(line, widths):
    if not widths:
        return 0 if '#' in line else 1
    if len(line) < sum(widths) + len(widths) - 1:
        return 0
    pivot = int(len(widths) / 2)
    width = widths[pivot]
    n = 0
    for i in range(len(line) - width + 1):
        if '.' in line[i:i+width]:
            continue
        if i > 0 and line[i-1] == '#':
            continue
        if i + width < len(line) and line[i+width] == '#':
            continue
        leftLine, rightLine = split(line, i, width)
        n1 = calc(leftLine, widths[0:pivot])
        n2 = calc(rightLine, widths[pivot+1:])
        n += n1 * n2
    return n


def split(line, i, width):
    line = ' ' + line + ' '
    left = line[0:i].replace(' ', '')
    right = line[i+width+2:].replace(' ', '')
    return left, right


def process1(ctx):
    total = 0
    for row in ctx['rows']:
        score = calc(*row)
        print(score)
        total += score
    return total


def process2(ctx):
    total = 0
    for i, row in enumerate(ctx['rows']):
        line = '?'.join([row.line] * 5)
        widths = row.widths * 5
        score = calc(line, widths)
        print(i, score)
        total += score
    return total


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
