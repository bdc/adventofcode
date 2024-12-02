import argparse
import re
from collections import defaultdict, namedtuple
from pathlib import Path

Draw = namedtuple('Draw', 'colors')
Game = namedtuple('Game', 'n draws')


def handleLine(line, ctx=None):
    groups = re.match(r'Game (\d+): (.*)', line).groups()
    gameNum = int(groups[0])
    draws = []
    drawstrs = groups[1].split('; ')
    for drawstr in drawstrs:
        colorstrs = drawstr.split(', ')
        draw = Draw(defaultdict(int))
        for colorstr in colorstrs:
            (n, color) = re.match(r'(\d+) (.*)', colorstr).groups()
            draw.colors[color] += int(n)
        draws.append(draw)
    ctx['games'].append(Game(gameNum, draws))


def process1(ctx):
    colors = {'red': 12, 'green': 13, 'blue': 14}
    return sum(g.n for g in ctx['games'] if isFeasible(g, colors))


def process2(ctx):
    powers = [getPower(g) for g in ctx['games']]
    return sum(powers)


def getPower(game):
    counts = getMinColorCount(game).values()
    power = 1
    for n in counts:
        power *= n
    return power


def getMinColorCount(game):
    minColors = defaultdict(int)
    for draw in game.draws:
        for color in draw.colors.keys():
            minColors[color] = max(minColors[color], draw.colors[color])
    return minColors


def isFeasible(game, colors):
    minColors = getMinColorCount(game)
    for color in minColors.keys():
        if colors[color] < minColors[color]:
            return False
    return True


def main(part, input):
    filename = Path(__file__).with_name(input)
    ctx = {'games': []}

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
    result = main(2, args.input)
    print(result)


init()
