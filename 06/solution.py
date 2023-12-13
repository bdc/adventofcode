import argparse
from collections import defaultdict, namedtuple
import functools
from pathlib import Path
import re
import math


Race = namedtuple('Race', 't d')


def handleLine(line, ctx=None):
    m = re.match(r'(?P<label>\w+): +(?P<values>.*?)$', line)
    label = m.group('label').lower()
    values = [int(i) for i in m.group('values').split()]
    ctx[label] = values


def numWaysToWin(race):
    t1 = (race.t - math.sqrt(math.pow(race.t, 2) - 4*race.d)) / 2
    a = math.floor(t1 + 1)
    return race.t - 2*a + 1


def process1(ctx):
    races = (Race(*p) for p in zip(*ctx.values()))
    nums = map(numWaysToWin, races)
    result = functools.reduce(lambda a, b: a * b, nums, 1)
    return result


def kern(nums):
    return int(functools.reduce(lambda a, b: str(a) + str(b), nums, ''))


def process2(ctx):
    race = Race(*map(kern, ctx.values()))
    return numWaysToWin(race)


def main(part, input):
    filename = Path(__file__).with_name(input)
    ctx = {}

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
