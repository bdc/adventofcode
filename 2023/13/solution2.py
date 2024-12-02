from pathlib import Path


def transpose(rows):
    return tuple(''.join(i) for i in zip(*rows))


def diffCount(rows, mirror):
    diff = 0
    for i in range(mirror):
        i1 = mirror - i - 1
        i2 = mirror + i
        if i1 < 0 or i2 >= len(rows):
            return diff
        diff += sum(len(set(c))-1 for c in zip(rows[i1], rows[i2]))
    return diff


def match(rows, diff=0):
    for i in range(1, len(rows)):
        if diffCount(rows, i) == diff:
            return i
    return 0


def calc(patterns, diff):
    hSum = sum(match(pat, diff) for pat in patterns)
    vSum = sum(match(transpose(pat), diff) for pat in patterns)
    return 100 * hSum + vSum


def main():
    filename = Path(__file__).with_name('input.txt')
    patterns = [tuple()]

    with open(filename, 'r') as f:
        for line in f.readlines():
            l = line.strip()
            if l:
                patterns[-1] = patterns[-1] + (l,)
            else:
                patterns.append(tuple())

    print(calc(patterns, 0))
    print(calc(patterns, 1))


main()
