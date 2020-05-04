import re

def is_isogram(string):
    alpha_only_str = re.sub(r'[^A-Za-z]+', '', string.lower())
    iso = set(alpha_only_str)
    return len(iso) == len(alpha_only_str)