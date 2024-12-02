import argparse
from collections import defaultdict, namedtuple
import functools
from pathlib import Path
from enum import Enum


Hand = namedtuple('Hand', 'cards bid')


class HandType(Enum):
    FIVE_OF_A_KIND = 1
    FOUR_OF_A_KIND = 2
    FULL_HOUSE = 3
    THREE_OF_A_KIND = 4
    TWO_PAIR = 5
    ONE_PAIR = 6
    HIGH_CARD = 7


def handleLine(line, ctx=None):
    data = line.split(' ')
    ctx['hands'].add(Hand(data[0], int(data[1])))


def getHandType(hand, wildcard=None):
    ctr = defaultdict(int)
    wilds = 0
    for card in hand.cards:
        if card == wildcard:
            wilds += 1
        else:
            ctr[card] += 1
    sig = sorted(ctr.values())
    if not sig:
        sig = [0]
    sig[-1] += wilds
    if sig == [5]:
        return HandType.FIVE_OF_A_KIND
    if sig == [1, 4]:
        return HandType.FOUR_OF_A_KIND
    if sig == [2, 3]:
        return HandType.FULL_HOUSE
    if sig == [1, 1, 3]:
        return HandType.THREE_OF_A_KIND
    if sig == [1, 2, 2]:
        return HandType.TWO_PAIR
    if sig == [1, 1, 1, 2]:
        return HandType.ONE_PAIR
    return HandType.HIGH_CARD


def comparison(handTypeFn, cardRanks):
    def compare(hand1: Hand, hand2: Hand):
        t1 = handTypeFn(hand1)
        t2 = handTypeFn(hand2)
        if t1.value != t2.value:
            return 1 if t1.value > t2.value else -1
        for i in range(5):
            c1 = hand1.cards[i]
            c2 = hand2.cards[i]
            if c1 == c2:
                continue
            i1 = cardRanks.index(c1)
            i2 = cardRanks.index(c2)
            return 1 if i1 > i2 else -1
        return 0
    return functools.cmp_to_key(compare)


def calculateWinnings(keyFn, hands):
    ranked = sorted(hands, key=keyFn, reverse=True)
    winnings = 0
    for i, hand in enumerate(ranked):
        rank = i + 1
        winnings += rank * hand.bid
    return winnings


def process1(ctx):
    keyFn = comparison(getHandType, 'AKQJT98765432')
    return calculateWinnings(keyFn, ctx['hands'])


def process2(ctx):
    handFn = functools.partial(getHandType, wildcard='J')
    keyFn = comparison(handFn, 'AKQT98765432J')
    return calculateWinnings(keyFn, ctx['hands'])


def main(part, input):
    filename = Path(__file__).with_name(input)
    ctx = {'hands': set()}

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
