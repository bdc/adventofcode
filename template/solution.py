import argparse
import os
from pathlib import Path


def handleLine(line, ctx=None):
  print(line)

def process(ctx):
  return 1

def main(part, input):
  filename = Path(__file__).with_name(input)
  ctx = {}

  with open(filename, 'r') as f:
    for i, line in enumerate(f.readlines()):
      l = line.strip()
      if l:
        handleLine(l, ctx)

  result = process(ctx)
  return result


def init():
  parser = argparse.ArgumentParser()
  parser.add_argument('--part', type=int, default=1)
  parser.add_argument('--input', type=str, default='test.txt')
  args = parser.parse_args()
  result = main(args.part, args.input)
  print(result)


init()
