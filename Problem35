import random
import itertools as iter

def is_prime(n, k=5):
    '''Courtesy of ChatGPT'''
    randint = random.randint
    if n <= 3:
        return n == 2 or n == 3
    s = 0
    d = n - 1
    while d % 2 == 0:
        s += 1
        d //= 2
    for _ in range(k):
        a = randint(2, n - 2)
        x = pow(a, d, n)
        if x == 1 or x == n - 1:
            continue
        for r in range(s - 1):
            x = pow(x, 2, n)
            if x == n - 1:
                break
        else:
            return False
    return True


def separate_digits(num):
    '''Courtesy of ChatGPT'''
    digits = []
    while num > 0:
        digit = num % 10
        digits.append(digit)
        num //= 10
    digits.reverse()
    return digits

def merge_digits(digits):
    '''Courtesy of ChatGPT'''
    num = 0
    for digit in digits:
        num = num * 10 + digit
    return num


def rotate_list(lst, k):
    '''Courtesy of ChatGPT'''
    k = k % len(lst)
    return lst[-k:] + lst[:-k]


def all_combos_prime(d):
    digits = separate_digits(d)
    for c in [rotate_list(digits, i) for i in range(0,len(digits))]:
        num = merge_digits(c)
        yield is_prime(num)

xs = [i for i in range(1, 1_000_000) if all(all_combos_prime(i))]
xs, len(xs) # done in Jupyter Notebook
