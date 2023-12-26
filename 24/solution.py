import argparse
from collections import defaultdict, namedtuple
import itertools
import math
from pathlib import Path
import random
import re


Stone = namedtuple('Stone', 'id px py pz vx vy vz')


def handleLine(line, i, ctx=None):
    m = re.match(
        r'(-?\d+), *(-?\d+), *(-?\d+) *@ *(-?\d+), *(-?\d+), *(-?\d+)', line)
    stone = Stone(i, *map(int, m.groups()))
    ctx['stones'].append(stone)


def flatten(stone):
    return Stone(*stone[0:3], 0, *stone[4:6], 0)


def calcT(stone, other):
    if stone.vx * other.vy == stone.vy * other.vx:
        return None  # parallel
    t = ((other.py - stone.py) + (other.vy / other.vx) *
         (stone.px - other.px)) / (stone.vy - stone.vx * other.vy / other.vx)
    return t


def nearestT(px1, vx1, px2, vx2):
    dx = px2 - px1
    vx = vx1 - vx2
    return dx / vx if vx else None


def heuristic(stone, other):
    tx = nearestT(stone.px, stone.vx, other.px, other.vx)
    ty = nearestT(stone.py, stone.vy, other.py, other.vy)
    tz = nearestT(stone.pz, stone.vz, other.pz, other.vz)
    ts = list(filter(lambda x: x, (tx, ty, tz)))
    mt = sum(ts) / len(ts)
    sx, sy, sz = stoneAtT(stone, mt)
    ox, oy, oz = stoneAtT(other, mt)
    distance = math.pow(ox - sx, 2) + math.pow(oy -
                                               sy, 2) + math.pow(oz - sz, 2)
    return distance


def stoneAtT(stone, t):
    x = stone.px + t * stone.vx
    y = stone.py + t * stone.vy
    z = stone.pz + t * stone.vz
    return x, y, z


def process1(ctx):
    stones = [flatten(s) for s in ctx['stones']]
    xMin, xMax = (200000000000000, 400000000000000)
    if stones[0][0:2] == (19, 13):  # test case
        xMin, xMax = (7, 27)
    yMin, yMax = xMin, xMax
    score = 0
    for s1, s2 in itertools.combinations(stones, 2):
        t1 = calcT(s1, s2)
        t2 = calcT(s2, s1)
        if not t1 or not t2:  # parallel
            continue
        if t1 < 0 or t2 < 0:  # past
            continue
        x, y, _ = stoneAtT(s1, t1)
        if xMin <= x and x <= xMax and yMin <= y and y <= yMax:
            score += 1
    return score


def inferRock(stone1: Stone, t1: int, stone2: Stone, t2: int):
    s1xyz = [stone1[i+1] + stone1[i+4] * t1 for i in range(3)]
    s2xyz = [stone2[i+1] + stone2[i+4] * t2 for i in range(3)]
    vxyz = [(s2xyz[i] - s1xyz[i]) / (t2 - t1) for i in range(3)]
    pxyz = [s1xyz[i] - t1 * vxyz[i] for i in range(3)]
    return Stone('r', *pxyz, *vxyz)


def searchRanges(t1Range, t2Range, stones):
    scoreMap = {}
    for t1 in t1Range:
        for t2 in t2Range:
            if t1 == t2:
                continue
            rock = inferRock(stones[0], t1, stones[1], t2)
            score = heuristic(rock, stones[2])
            scoreMap[(t1, t2)] = score
    lowest = (None, 10**50)
    for key in sorted(scoreMap):
        if scoreMap[key] < lowest[1]:
            lowest = (key, scoreMap[key])
    return lowest


def process2(ctx):
    stones = ctx['stones']
    t1Min, t1Max = 0, 4000000000000
    t2Min, t2Max = t1Min, t1Max
    while True:
        t1Span = t1Max - t1Min
        t2Span = t2Max - t2Min
        # print(t1Min, t1Max, t2Min, t2Max)
        t1Step = round(t1Span / 20) or 1
        t2Step = round(t2Span / 20) or 1
        t1Range = range(t1Min, t1Max, t1Step)
        t2Range = range(t2Min, t2Max, t2Step)
        (bestT1, bestT2), score = searchRanges(t1Range, t2Range, stones)
        if score == 0:
            break
        if t1Step == 1 and t2Step == 1:
            print('oops')  # finished iterating but didn't find a solution
            break
        t1Min = max(bestT1 - round(t1Span / 6), 0)
        t1Max = bestT1 + round(t1Span / 6) + 1
        t2Min = max(bestT2 - round(t2Span / 6), 0)
        t2Max = bestT2 + round(t2Span / 6) + 1
    rock = inferRock(stones[0], bestT1, stones[1], bestT2)
    result = round(sum(rock[1:4]))
    return result


def main(part, input):
    filename = Path(__file__).with_name(input)
    ctx = {'stones': []}

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
