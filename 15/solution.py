import argparse
from collections import defaultdict, namedtuple
from pathlib import Path
import re


Lens = namedtuple('Lens', 'label value')


def handleLine(line, ctx=None):
    ctx['steps'] = line.split(',')


def hash(data):
    val = 0
    for c in data:
        val += ord(c)
        val *= 17
        val %= 256
    return val


def getIndexOfLambda(seq, predicate):
    for i, el in enumerate(seq):
        if predicate(el):
            return i


def scoreBox(boxNum, box):
    score = 0
    for i, lens in enumerate(box):
        score += (boxNum + 1) * (i + 1) * int(lens.value)
    return score


def process1(ctx):
    hashes = [hash(step) for step in ctx['steps']]
    return sum(hashes)


def process2(ctx):
    boxes = defaultdict(list)
    for step in ctx['steps']:
        label, op, value = re.match(r'(\w+)([-=])(\d*)', step).groups()
        boxId = hash(label)
        box = boxes[boxId]
        if op == '-':
            boxes[boxId] = list(filter(lambda lens: lens.label != label, box))
            continue
        boxLabels = set(lens.label for lens in box)
        if label in boxLabels:
            labelIndex = getIndexOfLambda(box, lambda lens: lens.label == label)
            box[labelIndex] = Lens(label, value)
        else:
            box.append(Lens(label, value))
    score = sum(scoreBox(boxId, boxes[boxId]) for boxId in boxes.keys())
    return score


def main(part, input):
    filename = Path(__file__).with_name(input)
    ctx = {'steps': []}

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
