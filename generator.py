#!/usr/bin/env python3
import random
import sys
from typing import Callable, Dict, Tuple
from collections.abc import Generator

RANGE_X = 1000
RANGE_Y = 1000

UPPER_X = 100
LOWER_X = -100
UPPER_Y = 100
LOWER_Y = -100
DQUADRANT = 10

DWEIGHT = 1000
WEIGHT_MIN = 100
N = 1000
V_MAX = 100

PATTERN_GENERATOR_T = Generator[Tuple[int, int, int, int], None, None]
PATTERN_T = Callable[[], PATTERN_GENERATOR_T]
PATTERN_MAP: Dict[str, PATTERN_T] = {}


def pattern_mapping(map: Dict[str, PATTERN_T]):

    def wrapper(func: PATTERN_T):
        nonlocal map
        pattern_name = func.__name__.replace('generate_', '')
        map[pattern_name] = func
        return func

    return wrapper


@pattern_mapping(PATTERN_MAP)
def speed_clump() -> PATTERN_GENERATOR_T:
    for _ in range(N):
        x = random.randint(-DQUADRANT, DQUADRANT)
        y = random.randint(-DQUADRANT, DQUADRANT)
        vx = random.random() * (V_MAX * 2) - V_MAX
        vy = random.random() * (V_MAX * 2) - V_MAX
        yield x, y, vx, vy


@pattern_mapping(PATTERN_MAP)
def generate_random() -> PATTERN_GENERATOR_T:

    for _ in range(N):
        x = random.randint(-RANGE_X, RANGE_X)
        y = random.randint(-RANGE_Y, RANGE_Y)
        yield (x, y, 0, 0)


@pattern_mapping(PATTERN_MAP)
def generate_square() -> PATTERN_GENERATOR_T:
    for _ in range(N):
        quadrant = random.randint(1, 4)
        if quadrant == 1:
            x = random.randint(LOWER_X, UPPER_X)
            y = random.randint(UPPER_Y-DQUADRANT, UPPER_Y)
        elif quadrant == 2:
            x = random.randint(UPPER_X-DQUADRANT, UPPER_X)
            y = random.randint(LOWER_Y, UPPER_Y)
        elif quadrant == 3:
            x = random.randint(LOWER_X, UPPER_X)
            y = random.randint(LOWER_Y, LOWER_Y+DQUADRANT)
        elif quadrant == 4:
            x = random.randint(LOWER_X, LOWER_X+DQUADRANT)
            y = random.randint(LOWER_Y, UPPER_Y)
        yield (x, y, 0, 0)


def help():
    print("Usage: generator.py [mode] output_file")
    print(f"Available modes: {', '.join(PATTERN_MAP.keys())}")


def main():
    args = sys.argv[1:]

    if len(args) == 1 and args[0] == 'help' or len(args) not in [1,2]:
        help()
        return

    mode = 'random' if len(args) == 1 else args[0]
    output_file = args[-1] # last argument (even if mode is not specified)

    if mode not in PATTERN_MAP:
        help()
        return

    pattern = PATTERN_MAP[mode]

    with open(output_file, 'w') as f:

        for x,y,vx,vy in pattern():
            weight = random.randint(WEIGHT_MIN, DWEIGHT)
            f.write(f"{x},{y},{vx},{vy},{weight}\n")


if __name__ == '__main__':
    main()
