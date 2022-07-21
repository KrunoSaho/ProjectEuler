# %%
import sympy

# %%
def prime_quadratic(a, b, n):
    return n*n + a*n + b

is_prime = sympy.isprime

# %%

factors = (0, 0)
last_product = 0
largest_n = 0

for a in range(-1001, 1001):
    for b in range(-1001, 1001):
        n = 0
        while is_prime(prime_quadratic(a, b, n)):
            if n > largest_n:
                largest_n = n
                last_product = a*b
                factors = (a, b)
            n += 1

factors, last_product, largest_n
