import argparse
from collections import namedtuple
from pathlib import Path
import re


Beam = namedtuple('Beam', 'i j dir')
Square = namedtuple('Square', 'i j c dirs')


def handleLine(line, i, ctx=None):
    for j, c in enumerate(line):
        ctx['grid'][(i, j)] = Square(i, j, c, set())


def nextBeams(beam, square):
    i, j, c = square.i, square.j, square.c
    if c == '.':
        return [Beam(i, j, beam.dir)]
    if c == '\\':
        dir = {'n': 'w', 's': 'e', 'e': 's', 'w': 'n', }[beam.dir]
        return [Beam(i, j, dir)]
    if c == '/':
        dir = {'n': 'e', 's': 'w', 'e': 'n', 'w': 's', }[beam.dir]
        return [Beam(i, j, dir)]
    if c == '|':
        if beam.dir in 'ns':
            return [Beam(i, j, beam.dir)]
        return [Beam(i, j, 'n'), Beam(i, j, 's')]
    if c == '-':
        if beam.dir in 'ew':
            return [Beam(i, j, beam.dir)]
        return [Beam(i, j, 'e'), Beam(i, j, 'w')]


def nextIj(beam: Beam):
    i, j, dir = beam.i, beam.j, beam.dir
    i += {'n': -1, 's': 1, 'w': 0, 'e': 0}[dir]
    j += {'n': 0, 's': 0, 'w': -1, 'e': 1}[dir]
    return i, j


def calcNumEnergized(grid, beam):
    beams = [beam]
    while len(beams):
        beam: Beam = beams.pop()
        i, j = nextIj(beam)
        if (i, j) not in grid:
            continue
        square: Square = grid[(i, j)]
        if beam.dir in square.dirs:
            continue
        square.dirs.add(beam.dir)
        beams.extend(nextBeams(beam, square))
    result = sum(1 for s in grid.values() if s.dirs)
    for square in grid.values():
        square.dirs.clear()
    return result


def process1(ctx):
    grid: dict = ctx['grid']
    return calcNumEnergized(grid, Beam(0, -1, 'e'))


def process2(ctx):
    grid: dict = ctx['grid']
    score = 0
    maxI = max(i for i, _ in grid.keys())
    maxJ = max(j for _, j in grid.keys())
    for i in range(maxI+1):
        score = max(score,
                    calcNumEnergized(grid, Beam(i, -1, 'e')),
                    calcNumEnergized(grid, Beam(i, maxJ+1, 'w')))
    for j in range(maxJ+1):
        score = max(score,
                    calcNumEnergized(grid, Beam(-1, j, 's')),
                    calcNumEnergized(grid, Beam(maxI+1, j, 'n')))
    return score


def main(part, input):
    filename = Path(__file__).with_name(input)
    ctx = {'grid': {}}

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
