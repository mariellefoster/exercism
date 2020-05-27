def is_armstrong_number(n):
    return n == sum([int(d)**len(str(n)) for d in str(n)])
