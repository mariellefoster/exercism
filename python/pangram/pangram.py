def is_pangram(sentence):
    sentence = sentence.lower()
    alpha_set = set()
    for c in sentence:
        if c.isalpha():
            alpha_set.add(c)
    if len(alpha_set) == 26:
        return True
    return False