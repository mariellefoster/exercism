QUESTION = "Sure."
YELL = "Whoa, chill out!"
YELL_QUESTION = "Calm down, I know what I'm doing!"
SILENCE = "Fine. Be that way!"
ALL_ELSE = "Whatever."


def response(hey_bob):
    hey_bob = hey_bob.strip()
    if hey_bob == "":
        return SILENCE
    end_token = hey_bob[-1]
    if hey_bob.isupper():
        if end_token == "?":
            return YELL_QUESTION
        return YELL
    if end_token == "?":
        return QUESTION
    return ALL_ELSE
