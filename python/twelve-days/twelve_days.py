GIFTS = ['twelve Drummers Drumming,',
         'eleven Pipers Piping,',
         'ten Lords-a-Leaping,',
         'nine Ladies Dancing,',
         'eight Maids-a-Milking,',
         'seven Swans-a-Swimming,',
         'six Geese-a-Laying,',
         'five Gold Rings,',
         'four Calling Birds,',
         'three French Hens,',
         'two Turtle Doves,',
         'a Partridge in a Pear Tree.']

DAYS = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth"]

def recite(start_verse, end_verse):
    final_list = []
    for day in range(start_verse, end_verse+1):
        gift_list = GIFTS[-(day):]
        gift_verse = (f"On the {DAYS[day-1]} day of Christmas my true love gave to me: ")
        if(day > 1):
            gift_list[-1] = "and " + gift_list[-1]
        gift_verse += " ".join(gift_list)
        final_list.append(gift_verse)
    return final_list