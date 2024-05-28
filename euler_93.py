# %%
import itertools as it
from functools import lru_cache
from typing import OrderedDict

# %%
@lru_cache
def fn_op(a, b, op):
    match op:
        case '+': return a + b
        case '-': return a - b
        case '*': return a * b
        case '/': return a / b if b != 0 else 0

# %%
def solve(stack) -> int:
    a, b = stack.pop(0), stack.pop(0)
    op = stack.pop(0)
    result = fn_op(a, b, op)
    
    while len(stack) > 0:
        a, b = result, stack.pop(0)
        op = stack.pop(0)
        result = fn_op(a, b, op)
        
    return abs(int(result)) if int(result) == result else 0 # type: ignore

# %%
def get_op_combos():
    ops = '+-*/'
    uniques = set()

    for o1, o2, o3 in it.product(ops, repeat=3):
        uniques.add(f"{o1}{o2}{o3}")

    return uniques


def create_stacks(x, y, z, w):
    digits = [x, y, z, w]
    uniques = set()

    for a, b, c, d in it.permutations(digits, 4):
        for o1, o2, o3 in get_op_combos():
            uniques.add((a, b, o1, c, o2, d, o3))
    
    yield from uniques

# %%
def generate_inputs():
    uniques = set()
    for a in range(0, 10):
        for b in range(0, 10):
            for c in range(0, 10):
                for d in range(0, 10):
                    if a < b < c < d: uniques.add((a, b, c, d))
    return uniques

# %%
def solve_all(single_stack=None):
    inputs = generate_inputs()
    
    if single_stack:
        inputs = [[int(x) for x in single_stack]]
    
    for u, v, w, p in inputs:
        stacks = create_stacks(u, v, w, p)
        values = {}

        for stack in stacks:
            result = solve(list(stack))
            if result > 0:
                fmt = ''.join(str(x) for x in stack)
                values[result] = fmt
        
        numerals = sorted(values.keys())
        for a, b in zip(numerals, numerals[1:]):
            if b - a != 1:
                code = f"{u}{v}{w}{p}"
                if single_stack == code:
                    print(f"{code}: {a}")
                    for k, v in OrderedDict(sorted(values.items())).items():
                        print(f"{k}: {v}")
                yield code, a
                break

print(max((result, digits) for digits, result in solve_all()))


