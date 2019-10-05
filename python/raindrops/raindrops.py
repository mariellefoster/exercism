def convert(number):
    final_str = ""
    if number % 3 == 0:
        final_str += "Pling"
    if number % 5 == 0:
        final_str += "Plang"
    if number % 7 == 0:
        final_str += "Plong"
    if final_str == "":
        final_str += str(number)
    return final_str

