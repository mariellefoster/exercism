def is_valid(isbn):
    isbn = list(isbn.replace('-', ''))
    if len(isbn) != 10:
        return False

    if isbn[-1] == "X":
        isbn[-1] = '10'
    
    try:
        isbn_sum = sum([int(isbn[i]) * (10-i) for i in range(len(isbn))])
        return isbn_sum % 11 == 0
    except ValueError:
        return False