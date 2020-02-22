"""
This exercise stub and the test suite contain several enumerated constants.

Since Python 2 does not have the enum module, the idiomatic way to write
enumerated constants has traditionally been a NAME assigned to an arbitrary,
but unique value. An integer is traditionally used because it’s memory
efficient.
It is a common practice to export both constants and functions that work with
those constants (ex. the constants in the os, subprocess and re modules).

You can learn more here: https://en.wikipedia.org/wiki/Enumerated_type
"""


# Score categories.
# Change the values as you see fit.
YACHT = 'y'
ONES = 1
TWOS = 2
THREES = 3
FOURS = 4
FIVES = 5
SIXES = 6
FULL_HOUSE = 'fh'
FOUR_OF_A_KIND = 'f'
LITTLE_STRAIGHT = "ls"
BIG_STRAIGHT = "bs"
CHOICE = 'ch'


def score(dice, category):
    dice = sorted(dice)
    score = 0
    if isinstance(category, int):
        for d in dice:
            if d == category:
                score += d
            else:
                pass
    elif category in [LITTLE_STRAIGHT, BIG_STRAIGHT]:
        dice = sorted(set(dice))
        if len(dice) == 5:
            if LITTLE_STRAIGHT and dice[0] != 1 or \
                BIG_STRAIGHT and dice[4] != 6:
                return 30
    elif category == CHOICE:
        return sum(dice)
    elif category == FULL_HOUSE:
        if dice[0] == dice[2] and dice[3] == dice[4] or \
            dice[0] == dice[1] and dice[2] == dice[4]:
            if dice[0] != dice[4]:
                return sum(dice)
    elif category == YACHT:
        if dice[0] == dice[4]:
            return 50
    elif category == FOUR_OF_A_KIND:
        if dice[0] == dice[3] or dice[1] == dice[4]:
            return 4*dice[1]


    return score
