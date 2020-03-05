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


YACHT = (lambda dice : 50 if len(set(dice)) == 1 else 0)
ONES = (lambda dice : side_count(1, dice) )
TWOS = (lambda dice : side_count(2, dice) )
THREES = (lambda dice : side_count(3, dice) )
FOURS = (lambda dice : side_count(4, dice) )
FIVES = (lambda dice : side_count(5, dice) )
SIXES = (lambda dice : side_count(6, dice) )
FULL_HOUSE = (lambda dice : sum(dice) if len(set(dice)) == 2 and
                    dice.count(dice[0]) in (2,3) else 0 )
FOUR_OF_A_KIND = (lambda dice : sorted(dice)[2]*4 if len(set(dice)) <= 2
                    and dice.count(dice[0]) in (1, 4, 5) else 0)
LITTLE_STRAIGHT = (lambda dice : 30 if sorted(dice) == [1,2,3,4,5] else 0)
BIG_STRAIGHT = (lambda dice : 30 if sorted(dice) == [2,3,4,5,6] else 0 )
CHOICE = (lambda dice : sum(dice) )

def side_count(side, dice):
    return side * dice.count(side)


def score(dice, category):  
    return category(dice)
