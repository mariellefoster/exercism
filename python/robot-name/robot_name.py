from random import randrange, seed

class Robot:
    def __init__(self):
        self.name = ""
        self._new_name()

    def reset(self):
        self._new_name()

    def _new_name(self):
        seed()
        ch_name = [chr(randrange(65, 90)) for i in range(2)]
        num_name = [str(randrange(0, 9)) for i in range(3)]
        self.name = "".join(ch_name + num_name)