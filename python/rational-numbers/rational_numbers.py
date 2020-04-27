from __future__ import division
import math

class Rational:
    def __init__(self, n, d):
        self.n = n
        self.d = d
        self._reduce_by_gcd()
        if self.d < 0:
            self._fix_sign()

    def __eq__(self, other):
        return self.n == other.n and self.d == other.d

    def __repr__(self):
        return '{}/{}'.format(self.n, self.d)

    def __add__(self, other):
        # (a1 * b2 + a2 * b1) / (b1 * b2)
        return Rational(self.n * other.d + other.n * self.d,
                        self.d * other.d)

    def __sub__(self, other):
        # (a1 * b2 - a2 * b1) / (b1 * b2)
        return Rational(self.n * other.d - other.n * self.d,
                        self.d * other.d)

    def __mul__(self, other):
        return Rational(self.n * other.n,
                        self.d * other.d)

    def __truediv__(self, other):
        # (a1 * b2) / (a2 * b1)
        return Rational(self.n * other.d,
                        self.d * other.n)

    def __abs__(self):
        return Rational(abs(self.n), abs(self.d))

    def __pow__(self, power):
        # `r^n = (b^m)/(a^m)`, where `m = |n|`
        return Rational(self.n ** abs(power),
                        self.d ** abs(power))

    def __rpow__(self, base):
        ans = (base ** (self.n)) ** (1/self.d)
        return round(ans, 8)

    def _fix_sign(self):
        self.d = abs(self.d)
        self.n *= -1

    def _reduce_by_gcd(self):
        gcd = 1
        if self.n > self.d:
            gcd = self._find_gcd(self.n, self.d)
        elif self.n < self.d:
            gcd = self._find_gcd(self.d, self.n)
        else:
            self.n, self.d = 1, 1
            return
        self.n = int(self.n / gcd)
        self.d = int(self.d / gcd)

    def _find_gcd(self, a, b):
        if a == 0:
            return b
        if a == b or b == 0:
            return a
        r = int(a/b)*b - a
        return abs(self._find_gcd(b, r))
