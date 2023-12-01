import argparse
from pathlib import Path
import re


lookup = {
    'one': 1,
    'two': 2,
    'three': 3,
    'four': 4,
    'five': 5,
    'six': 6,
    'seven': 7,
    'eight': 8,
    'nine': 9,
}


def handleLine1(line, ctx=None):
    d1 = int(re.search(r'(\d).*', line).groups()[0])
    d2 = int(re.search(r'.*(\d)', line).groups()[0])
    ctx["nums"].append(d1*10 + d2)


def handleLine2(line, ctx=None):
    digit = f"\d|{'|'.join(lookup.keys())}"
    d1 = re.search(f'({digit}).*', line).groups()[0]
    d1 = lookup[d1] if d1 in lookup else int(d1)
    d2 = re.search(f'.*({digit})', line).groups()[0]
    d2 = lookup[d2] if d2 in lookup else int(d2)
    ctx["nums"].append(d1*10 + d2)


def process(ctx):
    return sum(ctx["nums"])


def main(part, input):
    filename = Path(__file__).with_name(input)
    ctx = {"nums": []}

    with open(filename, 'r') as f:
        for i, line in enumerate(f.readlines()):
            l = line.strip()
            # if i > 5:
            #     break
            if l:
                if part == 1:
                    handleLine1(l, ctx)
                else:
                    handleLine2(l, ctx)

    result = process(ctx)
    return result


def init():
    parser = argparse.ArgumentParser()
    parser.add_argument('--part', type=int, default=2)
    parser.add_argument('--input', type=str, default='input.txt')
    args = parser.parse_args()
    result = main(args.part, args.input)
    print(result)


init()
