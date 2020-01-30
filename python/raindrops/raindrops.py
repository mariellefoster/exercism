def convert(number):
    sounds = {
        3: "Pling",
        5: "Plang",
        7: "Plong",
    }

    return ("".join([sound for fact, sound in sounds.items() if number % fact == 0 ])
            or str(number)
    )
