import argparse
from collections import defaultdict, namedtuple
from pathlib import Path
import re


Brick = namedtuple('Brick', 'id minZ maxZ xyz1 xyz2')


def handleLine(line, i, ctx=None):
    coords = line.split('~')
    xyz1 = tuple(int(i) for i in coords[0].split(','))
    xyz2 = tuple(int(i) for i in coords[1].split(','))
    if xyz2[2] < xyz1[2]:
        swap = xyz1
        xyz1 = xyz2
        xyz2 = swap
    minZ = xyz1[2]
    maxZ = xyz2[2]
    id = chr(ord('A') + i)
    brick = Brick(id, minZ, maxZ, xyz1, xyz2)
    ctx['bricks'].append(brick)


def brickToCubes(brick):
    cubes = []
    for x in range(brick.xyz1[0], brick.xyz2[0] + 1):
        for y in range(brick.xyz1[1], brick.xyz2[1] + 1):
            for z in range(brick.xyz1[2], brick.xyz2[2] + 1):
                cubes.append((x, y, z))
    return cubes


def downTo(brick, minZ):
    z1 = minZ
    z2 = brick.maxZ - brick.minZ + z1
    xyz1 = brick.xyz1[0:2] + (z1,)
    xyz2 = brick.xyz2[0:2] + (z2,)
    return Brick(brick.id, z1, z2, xyz1, xyz2)


def settle(brick, settledBricks):
    contactPairs = set()
    cubes = brickToCubes(brick)
    z = max(settledBricks[(x, y)].maxZ for x, y, _ in cubes)
    settledBrick = downTo(brick, z + 1)
    settledCubes = brickToCubes(settledBrick)
    for cube in settledCubes:
        if cube[0:2] in settledBricks and settledBricks[cube[0:2]].maxZ == z:
            contactPairs.add((settledBricks[cube[0:2]].id, brick.id))
        settledBricks[cube[0:2]] = settledBrick
    return contactPairs


def loadBearing(brickId1, supporting, supportedBy):
    supportingList = []
    for brickId2 in supporting[brickId1]:
        if len(supportedBy[brickId2]) == 1:
            supportingList.append(brickId2)
    return supportingList


def process1(ctx):
    settledBricks = defaultdict(
        lambda: Brick(None, 0, 0, (0, 0, 0), (0, 0, 0)))  # (x, y) -> cube
    supporting = defaultdict(set)
    supportedBy = defaultdict(set)
    for brick in sorted(ctx['bricks'], key=lambda x: x.minZ):
        contactPairs = settle(brick, settledBricks)
        for brickId1, brickId2 in contactPairs:
            supporting[brickId1].add(brickId2)
            supportedBy[brickId2].add(brickId1)
    score = 0
    for brick in ctx['bricks']:
        if not loadBearing(brick.id, supporting, supportedBy):
            score += 1
    return score


def process2(ctx):
    settledBricks = defaultdict(
        lambda: Brick(None, 0, 0, (0, 0, 0), (0, 0, 0)))  # (x, y) -> cube
    supporting = defaultdict(set)
    supportedBy = defaultdict(set)
    for brick in sorted(ctx['bricks'], key=lambda x: x.minZ):
        contactPairs = settle(brick, settledBricks)
        for brickId1, brickId2 in contactPairs:
            supporting[brickId1].add(brickId2)
            supportedBy[brickId2].add(brickId1)
    score = 0
    for brick1 in ctx['bricks']:
        active = [brick1.id]
        fallingBricks = set(active)
        while active:
            brickId1 = active.pop(0)
            for brick2 in ctx['bricks']:
                if brick2.id == brickId1:
                    continue
                if supportedBy[brick2.id].issubset(fallingBricks):
                    if brick2.id not in fallingBricks:
                        fallingBricks.add(brick2.id)
                        active.append(brick2.id)
        score += len(fallingBricks) - 1
    return score


def main(part, input):
    filename = Path(__file__).with_name(input)
    ctx = {'bricks': []}

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
