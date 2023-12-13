import argparse
from collections import namedtuple
from pathlib import Path


Pattern = namedtuple('Pattern', 'rows')


def handleLine(line, ctx=None):
    if not line:
        ctx['patterns'].append(Pattern(tuple()))
    else:
        pat = ctx['patterns'][-1]
        ctx['patterns'][-1] = Pattern(pat.rows + (line,))


def transpose(pattern):
    return Pattern(tuple(''.join(i) for i in zip(*pattern.rows)))


def checkHorizontalMirror(pattern, mirror):
    for i in range(mirror):
        i1 = mirror - i - 1
        i2 = mirror + i
        if i1 < 0 or i2 >= len(pattern.rows):
            return True
        if pattern.rows[i1] != pattern.rows[i2]:
            return False
    return True


def findHorizontalMirror(pattern):
    for i in range(1, len(pattern.rows)):
        if checkHorizontalMirror(pattern, i):
            return i
    return None


def diffCount(pattern, mirror):
    diff = 0
    for i in range(mirror):
        i1 = mirror - i - 1
        i2 = mirror + i
        if i1 < 0 or i2 >= len(pattern.rows):
            return diff
        row1 = pattern.rows[i1]
        row2 = pattern.rows[i2]
        diff += sum(len(set(c))-1 for c in tuple(zip(row1, row2)))
    return diff


def findHorizontalMirrorWithSmudge(pattern):
    for i in range(1, len(pattern.rows)):
        if diffCount(pattern, i) == 1:
            return i
    return None


def process1(ctx):
    hSum = 0
    vSum = 0
    for pat in ctx['patterns']:
        hSum += (findHorizontalMirror(pat) or 0)
        vSum += (findHorizontalMirror(transpose(pat)) or 0)
    return 100 * hSum + vSum


def process2(ctx):
    hSum = 0
    vSum = 0
    for pat in ctx['patterns']:
        hSum += (findHorizontalMirrorWithSmudge(pat) or 0)
        vSum += (findHorizontalMirrorWithSmudge(transpose(pat)) or 0)
    return 100 * hSum + vSum


def main(part, input):
    filename = Path(__file__).with_name(input)
    ctx = {'patterns': [Pattern(tuple())]}

    with open(filename, 'r') as f:
        for i, line in enumerate(f.readlines()):
            l = line.strip()
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
