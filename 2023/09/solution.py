import argparse
from collections import namedtuple
from pathlib import Path


Sequence = namedtuple('Sequence', 'vals')


def handleLine(line, ctx=None):
    seq = Sequence(list(map(int, line.split())))
    ctx['sequences'].append(seq)


def reduce(seq):
    vals = [seq.vals[i+1] - seq.vals[i] for i in range(len(seq.vals) - 1)]
    return Sequence(vals)


def findNextValue(seq):
    if all(not val for val in seq.vals):
        return 0
    return findNextValue(reduce(seq)) + seq.vals[-1]


def findPrevValue(seq):
    if all(not val for val in seq.vals):
        return 0
    return seq.vals[0] - findPrevValue(reduce(seq))


def process1(ctx):
    return sum(findNextValue(seq) for seq in ctx['sequences'])


def process2(ctx):
    return sum(findPrevValue(seq) for seq in ctx['sequences'])


def main(part, input):
    filename = Path(__file__).with_name(input)
    ctx = {'sequences': []}

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
