GIFTS = ['twelve Drummers Drumming, ',
         'eleven Pipers Piping, ',
         'ten Lords-a-Leaping, ',
         'nine Ladies Dancing, ',
         'eight Maids-a-Milking, ',
         'seven Swans-a-Swimming, ',
         'six Geese-a-Laying, ',
         'five Gold Rings, ',
         'four Calling Birds, ',
         'three French Hens, ',
         'two Turtle Doves, ',
         'a Partridge in a Pear Tree.']

DAYS = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth"]

def recite(start_verse, end_verse):
    end_verse, start_verse = end_verse - 1, start_verse - 1
    starting_string = "On the %s day of Christmas my true love gave to me: " % DAYS[start_verse]
    if start_verse == end_verse:
        start_verse = 0
        GIFTS[11] = "and " + GIFTS[11]
    print ([starting_string] + GIFTS[start_verse:end_verse])
    return [starting_string] + GIFTS[start_verse:end_verse]