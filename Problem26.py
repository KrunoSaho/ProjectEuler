# %%
from collections import namedtuple
import decimal as dec
import doctest

# %%
dec.setcontext(dec.Context(prec=4096, Emin=-42500000000, Emax=42500000000))

# %%
RecipData = namedtuple('RecipData', 'number recip pattern')

# %%
calculated_number = None
data = {f: RecipData(f, 
                                 calculated_number := dec.Decimal(1) / dec.Decimal(f), 
                                 ''.join([str(x) for x in calculated_number.as_tuple().digits])) 
                        for f in range(2, 1000)}

# %%
def find_mask_size(value: int, data: RecipData):
    pattern = data[value].pattern
    mask_i = 0
    mask_size = 4
    sub_pattern = pattern[mask_i:mask_i+mask_size]
    idx_delta = (0, 0)
    for i in range(0, len(pattern)):
        if sub_pattern == pattern[i:i+mask_size]:
            idx_delta = (idx_delta[1], i)
            a, b = idx_delta
            if b-a > 0:
                return b-a
    return -1

# %%
max([(find_mask_size(x, data), data[x].number) for x in data.keys()])


