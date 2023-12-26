import argparse
from collections import namedtuple
from pathlib import Path
import re


Workflow = namedtuple('Workflow', 'name steps default')
Part = namedtuple('Part', 'x m a s')
Step = namedtuple('Step', 'attr op val dest')
Node = namedtuple('Node', 'name stepIndex children')


def handleLine(line, i, ctx=None):
    m = re.match(r'{x=(\d+),m=(\d+),a=(\d+),s=(\d+)}', line)
    if m:
        ctx['parts'].append(Part(*(int(n) for n in m.groups())))
        return
    m = re.match(r'(\w+){(.*),(\w+)}', line)
    name = m.group(1)
    steps = parseSteps(m.group(2))
    ctx['workflows'][name] = Workflow(name, steps, m.group(3))


def parseSteps(line):
    steps = []
    for s in line.split(','):
        m = re.match(r'(\w)(.)(\d+):(\w+)', s)
        steps.append(Step(m.group(1), m.group(2), int(m.group(3)), m.group(4)))
    return tuple(steps)


def opGt(n, v): return n > v
def opLt(n, v): return n < v


def evaluateStep(part: Part, step: Step):
    n = getattr(part, step.attr)
    v = step.val
    op = opGt if step.op == '>' else opLt
    return op(n, v)


def evaluateWorkflow(part: Part, wf: Workflow):
    for step in wf.steps:
        if evaluateStep(part, step):
            return step.dest
    return wf.default


def process1(ctx):
    acceptedParts = set()
    for part in ctx['parts']:
        wfName = 'in'
        while wfName not in ['A', 'R']:
            wf = ctx['workflows'][wfName]
            wfName = evaluateWorkflow(part, wf)
        if wfName == 'A':
            acceptedParts.add(part)

    return sum(sum(p) for p in acceptedParts)


def invertStep(step):
    if step.op == '<':
        return Step(step.attr, '>', step.val - 1, step.dest)
    if step.op == '>':
        return Step(step.attr, '<', step.val + 1, step.dest)


def copyXmas(xmas):
    if not xmas:
        return {k: {'<': 4001, '>': 0} for k in 'xmas'}
    return {k: v.copy() for k, v in xmas.items()}


def narrowXmas(xmas, step, truth=True):
    xmas = copyXmas(xmas)
    if not truth:
        step = invertStep(step)
    op = min if step.op == '<' else max
    xmas[step.attr][step.op] = op(xmas[step.attr][step.op], step.val)
    return xmas


def calcXmas(xmas):
    spans = [a['<'] - a['>'] - 1 for a in xmas.values()]
    if len(list(filter(lambda i: i <= 0, spans))) > 0:
        return 0
    return spans[0] * spans[1] * spans[2] * spans[3]


def traverse(name, wfs, xmas=None):
    if name == 'A':
        return calcXmas(xmas)
    if name == 'R':
        return 0
    score = 0
    for step in wfs[name].steps:
        score += traverse(step.dest, wfs, narrowXmas(xmas, step))
        xmas = narrowXmas(xmas, step, False)
    score += traverse(wfs[name].default, wfs, xmas)
    return score


def process2(ctx):
    return traverse('in', ctx['workflows'])


def main(part, input):
    filename = Path(__file__).with_name(input)
    ctx = {'parts': [], 'workflows': {}}

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
