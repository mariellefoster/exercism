import re

def count_words(sentence):
    sentence = re.sub(r"[^A-Za-z0-9\d'\s]+", ' ', sentence)
    word_list = sentence.lower().split()
    word_dict = {}
    for w in word_list:
        while w.endswith("'") and w.startswith("'"):
            w = w[1:-1]
        word_dict[w] = word_dict.get(w, 0) + 1
    return word_dict
