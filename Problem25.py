# %%
from functools import cache
import doctest
import sys

# %%
def make_fib():
    data = [1,1]
    def fib(n):
        '''
        >>> fib(8)
        21
        >>> fib(12)
        144
        '''
        if n < 2:
            return 1
        if n >= len(data):
            m = fib(n-1) + fib(n-2)
            data.append(m)
        return data[n-1] + data[n-2]
    return fib

fib = make_fib()
[fib(n) for n in range(12)]

# %%
for n in range(2,9999):
    if len(str(fib(n))) >= 1000:
        print(n)
        break
fib(4781)


