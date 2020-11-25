import re

def count_words(sentence):
    sentence = re.sub(r"[^\w\d'\_\s]+", ' ', sentence)
    sentence = sentence.lower()
    word_list = sentence.split()
    word_dict = {}
    for w in word_list:
        while w.endswith("'") and w.startswith("'"):
            w = w[1:-1]
        if w in word_dict:
            word_dict[w] += 1
        else:
            word_dict[w] = 1
    print(word_dict)
    return word_dict
