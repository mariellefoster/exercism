def is_valid(isbn):
    isbn = isbn.replace('-', '')
    if len(isbn) == 10 and (isbn[-1].isdigit() or isbn[-1] == "X") and isbn[:-1].isdigit():
        isbn_sum = sum([int(isbn[i]) * (10-i) for i in range(0, len(isbn)-1)])
        if isbn[-1] == "X":
            isbn_sum += 10
        else:
            isbn_sum += int(isbn[-1])
        return isbn_sum % 11 == 0
    else:
        return False