def count_words(sentence):
    low_sen = sentence.lower()
    word_list = low_sen.split()
    word_dict = {}
    for w in word_list:
        if w in word_dict:
            word_dict[w] += 1
        else:
            word_dict[w] = 1
    return word_dict
