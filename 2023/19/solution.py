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
        steps.append(Step(
            m.group(1),
            m.group(2),
            int(m.group(3)),
            m.group(4),
        ))
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


def populateNodeChildren(node, ctx):
    for wf in ctx['workflows'].values():
        for i, step in enumerate(wf.steps):
            if step.dest == node.name:
                node.children.append(Node(wf.name, i, []))
        if wf.default == node.name:
            node.children.append(Node(wf.name, i + 1, []))
            continue
    for child in node.children:
        if child.name == 'in':
            continue
        populateNodeChildren(child, ctx)


def printNode(node, depth=0):
    print('  '*depth, 'Node', node.name, node.stepIndex)
    for child in node.children:
        printNode(child, depth=depth+1)


def normalizeStepTruth(step, truth):
    if truth:
        return step
    if step.op == '<':
        return Step(step.attr, '>', step.val - 1, step.dest)
    if step.op == '>':
        return Step(step.attr, '<', step.val + 1, step.dest)


def calculatePossibilities(conditions):
    attrs = {
        'x': {'<': 4001, '>': 0},
        'm': {'<': 4001, '>': 0},
        'a': {'<': 4001, '>': 0},
        's': {'<': 4001, '>': 0},
    }
    for cond in conditions:
        step = normalizeStepTruth(*cond)
        op = min if step.op == '<' else max
        attrs[step.attr][step.op] = op(attrs[step.attr][step.op], step.val)
    spans = [a['<'] - a['>'] - 1 for a in attrs.values()]
    if len(list(filter(lambda i: i <= 0, spans))) > 0:
        return 0
    return spans[0] * spans[1] * spans[2] * spans[3]


def traverse(node, ctx, conditions=None):
    conditions = [i for i in conditions] if conditions else []
    if node.name != 'A':
        wf = ctx['workflows'][node.name]
        for i in range(node.stepIndex):
            conditions.append((wf.steps[i], False))
        if node.stepIndex < len(wf.steps):
            conditions.append((wf.steps[node.stepIndex], True))
    if node.name == 'in':
        return calculatePossibilities(conditions)
    return sum(traverse(child, ctx, conditions) for child in node.children)


def process2(ctx):
    node = Node('A', None, [])
    populateNodeChildren(node, ctx)
    p = traverse(node, ctx)
    return p


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
