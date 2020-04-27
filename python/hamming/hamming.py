def distance(strand_a, strand_b):
    if len(strand_a) != len(strand_b):
        raise ValueError("These strings are not the same length")
    not_equal = lambda i: strand_a[i] != strand_b[i]
    distance = [1 for i in range(0, len(strand_a)) if not_equal(i)]
    return sum(distance)