import re

def count_words(sentence):
    sentence = re.sub(r"[^\w\d'\s]+", '', sentence)
    sentence = sentence.lower()
    word_list = sentence.split()
    word_dict = {}
    for w in word_list:
        w
        if w in word_dict:
            word_dict[w] += 1
        else:
            word_dict[w] = 1
    return word_dict
