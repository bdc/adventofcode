import argparse
from collections import defaultdict, namedtuple
import functools
from pathlib import Path
import re


Node = namedtuple('Node', 'name type state targets')
Pulse = namedtuple('Pulse', 'source target type')


def handleLine(line, i, ctx=None):
    m = re.match(r'([^\w]?)(\w+) -> (.*)', line)
    node = Node(m.group(2), m.group(1) or 'b', {}, m.group(3).split(', '))
    ctx['nodes'][node.name] = node


def initState(nodes):
    nodes['button'] = Node('button', 'b', {}, ['broadcaster'])
    for node in list(nodes.values()):
        initNode(node, nodes)


def initNode(node, nodes):
    if node.type == 'b':
        node.state['count'] = 0
    if node.type == '%':
        node.state['on'] = False
    if node.type == '&':
        for node2 in nodes.values():
            if node.name in node2.targets:
                node.state[node2.name] = 'l'
    for target in node.targets:
        if target not in nodes:
            nodes[target] = Node(target, 'b', {}, [])


def queuePulses(source, pulseType, ctx):
    nodes = ctx['nodes']
    pulses = ctx['pulses']
    for target in nodes[source].targets:
        pulses.append(Pulse(source, target, pulseType))


def checkForLoops(node, ctx):
    loops = ctx['loops']
    if node.name not in loops and any(v == 'l' for v in node.state.values()):
        count = ctx['nodes']['broadcaster'].state['count']
        loops[node.name] = count


def processNextPulse(ctx):
    pulses = ctx['pulses']
    pulse: Pulse = pulses.pop(0)
    nodes = ctx['nodes']
    node: Node = nodes[pulse.target]
    counter = ctx['counter']
    counter[pulse.type] += 1

    if node.type == 'b':  # broadcast or button
        if 'count' in node.state:
            node.state['count'] += 1
        queuePulses(node.name, pulse.type, ctx)
    if node.type == '%':
        if pulse.type == 'h':
            return
        on = node.state['on']
        node.state['on'] = not on
        nextPulseType = 'l' if on else 'h'
        queuePulses(node.name, nextPulseType, ctx)
    if node.type == '&':
        node.state[pulse.source] = pulse.type
        nextPulseType = 'h' if any(
            v == 'l' for v in node.state.values()) else 'l'
        queuePulses(node.name, nextPulseType, ctx)
        checkForLoops(node, ctx)


def pressButton(ctx):
    queuePulses('button', 'l', ctx)
    while ctx['pulses']:
        processNextPulse(ctx)


def calcScore(counter: dict):
    return functools.reduce(lambda a, b: a * b, counter.values(), 1)


def process1(ctx):
    initState(ctx['nodes'])
    for _ in range(1000):
        pressButton(ctx)
    return calcScore(ctx['counter'])


def process2(ctx):
    nodes = ctx['nodes']
    initState(nodes)
    while True:
        pressButton(ctx)
        if len(ctx['loops']) == sum(1 for n in ctx['nodes'].values() if n.type == '&'):
            break  # All conjunction modules have cycled at least once
    return calcScore(ctx['loops'])


def main(part, input):
    filename = Path(__file__).with_name(input)
    ctx = {'nodes': {}, 'pulses': [], 'counter': defaultdict(int), 'loops': {}}

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
