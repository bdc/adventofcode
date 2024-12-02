import argparse
from collections import namedtuple
from pathlib import Path
import re


# left and right are sets of ints
Card = namedtuple('Card', 'n left right')


def handleLine(line, ctx=None):
    split = int((line.index('|') - line.index(':') + 1) / 3)
    nums = [int(i) for i in re.findall(r'\d+', line)]
    card = Card(nums[0], set(nums[1:split]), set(nums[split:]))
    ctx['cards'].append(card)


def process1(ctx):
    score = 0
    for card in ctx['cards']:
        n = len(card.left.intersection(card.right))
        score += [0, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512][n]
    return score


def process2(ctx):
    nCards = [1 for c in ctx['cards']]
    for card in ctx['cards']:
        n = len(card.left.intersection(card.right))
        for i in range(card.n + 1, card.n + 1 + n):
            nCards[i] += nCards[card.n]
    score = sum(nCards)
    return score


def main(part, input):
    filename = Path(__file__).with_name(input)
    ctx = {'cards': []}

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
