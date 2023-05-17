# %%
import itertools
from functools import lru_cache

# %%
def generate_pandigital_numbers():
    '''GPT Generated'''
    digits = '0123456789'
    pandigital_numbers = [''.join(p) for p in itertools.permutations(digits)]
    return pandigital_numbers

pandigitals = [int(n) for n in generate_pandigital_numbers() if len(str(int(n))) == 10]

# %%
@lru_cache(maxsize=100_000)
def find_divisors(n: int):
    def look(n):
        for i in range(2, int(n**0.5) + 1):
            if n % i == 0:
                yield i

                if i != n // i:
                    yield n // i
    return sorted(list(look(n)))

# %%
def number_has_property(m):
    word = [int(c) for c in str(m)]
    prime_numbers = [2,3,5,7,11,13,17]

    for i in range(1, len(word)):
        sub_digits = word[i:i + 3]

        if len(sub_digits) < 3:
            break

        number = sub_digits[0] * 100 + sub_digits[1] * 10 + sub_digits[2]
        divisors = find_divisors(number)

        if len(divisors) == 0:
            return False

        prime = prime_numbers.pop(0)
        if prime not in divisors:
            return False

    return True

# %%
sum(n for n in pandigitals if number_has_property(n))


