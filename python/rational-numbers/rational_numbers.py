from __future__ import division


class Rational:
    def __init__(self, n, d):
        self.n = None
        self.d = None

    def __eq__(self, other):
        return self.n == other.n and self.d == other.d

    def __repr__(self):
        return '{}/{}'.format(self.n, self.d)

    def __add__(self, other):
        self.n = (self.n * other.d + other.n * self.d)
        self.d = (self.d * other.d)
        self._reduce_by_gcd()
        return self


    def __sub__(self, other):
        pass

    def __mul__(self, other):
        pass

    def __truediv__(self, other):
        pass

    def __abs__(self):
        pass


    def __pow__(self, power):
        pass

    def __rpow__(self, base):
        pass

    def _reduce_by_gcd(self):
        if self.n > self.d:
            gcd = self._find_gcd(self.n, self.d)
        elif self.n > self.d:
            gcd = self._find_gcd(self.d, self.n)
        else:
            self.n, self.d = 1, 1

        self.n = int(self.n / gcd)
        self.d = int(self.d / gcd)

    def _find_gcd(self, a, b):
        if a == 0:
            return b
        r = int(a/b)*b - a
        return self._find_gcd(b, r)



