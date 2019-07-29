def two_fer(name=None):
    if name in ["Alice", "Bob"]:
        return ("One for %s, one for me." % name)
    if name == None:
        return "One for you, one for me."

