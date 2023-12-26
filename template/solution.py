import argparse
from collections import defaultdict, namedtuple
from pathlib import Path
import re


def handleLine(line, i, ctx=None):
    print(line)


def process1(ctx):
    return 1


def process2(ctx):
    return 2


def main(part, input):
    filename = Path(__file__).with_name(input)
    ctx = {}

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
    parser.add_argument('--input', type=str, default='test.txt')
    args = parser.parse_args()
    result = main(1, args.input)
    print(result)
    result = main(2, args.input)
    print(result)


init()
