import base64
import numpy as np

# I LIKE MY SAGEMATH IMPLEMENTATION MORE THAN THIS VIBECODED SLOT MACHINE VERSION!!!

class GF:
    P = None
    M = None
    MODULUS_COEFFS = None  # np.ndarray of shape (M+1,)

    def __init__(self, val):
        if isinstance(val, GF):
            self.coeffs = val.coeffs.copy()
        elif isinstance(val, (list, tuple, np.ndarray)):
            coeffs = np.array(val, dtype=np.int64) % self.P
            if len(coeffs) < self.M:
                coeffs = np.pad(coeffs, (0, self.M - len(coeffs)))
            else:
                coeffs = coeffs[:self.M]
            self.coeffs = coeffs
        else:
            # Convert raw integer to base-P coefficients
            v = int(val) % (self.P ** self.M)
            coeffs = np.zeros(self.M, dtype=np.int64)
            for i in range(self.M):
                coeffs[i] = v % self.P
                v //= self.P
            self.coeffs = coeffs

    @property
    def val(self):
        val = 0
        p_pow = 1
        for c in self.coeffs:
            val += int(c) * p_pow
            p_pow *= self.P
        return val

    def __add__(self, other):
        if isinstance(other, Poly):
            return NotImplemented
        if not isinstance(other, GF):
            other = self.__class__(other)
        return self.__class__((self.coeffs + other.coeffs) % self.P)

    __radd__ = __add__

    def __sub__(self, other):
        if isinstance(other, Poly):
            return NotImplemented
        if not isinstance(other, GF):
            other = self.__class__(other)
        return self.__class__((self.coeffs - other.coeffs) % self.P)

    def __rsub__(self, other):
        if isinstance(other, Poly):
            return NotImplemented
        return self.__class__(other) - self

    def __mul__(self, other):
        if isinstance(other, Poly):
            return NotImplemented
        if not isinstance(other, GF):
            other = self.__class__(other)
        
        # Use np.convolve for polynomial multiplication
        res_coeffs = np.convolve(self.coeffs, other.coeffs) % self.P
        
        # Reduction modulo the irreducible polynomial MODULUS_COEFFS
        if self.M > 1:
            for i in range(2 * self.M - 2, self.M - 1, -1):
                if res_coeffs[i] != 0:
                    lead = res_coeffs[i]
                    inv_lc = pow(int(self.MODULUS_COEFFS[-1]), int(self.P - 2), int(self.P))
                    factor = (lead * inv_lc) % self.P
                    res_coeffs[i - self.M : i + 1] = (res_coeffs[i - self.M : i + 1] - factor * self.MODULUS_COEFFS) % self.P
                        
        return self.__class__(res_coeffs[:self.M])

    __rmul__ = __mul__

    def inv(self):
        if np.all(self.coeffs == 0):
            raise ZeroDivisionError("Division by zero in GF")
        res = self.__class__([1])
        base = self
        exp = (self.P ** self.M) - 2
        while exp > 0:
            if exp & 1:
                res = res * base
            base = base * base
            exp >>= 1
        return res

    def __truediv__(self, other):
        return self * other.inv()

    def __neg__(self):
        return self.__class__((-self.coeffs) % self.P)

    def __eq__(self, other):
        if not isinstance(other, GF):
            other = self.__class__(other)
        return np.array_equal(self.coeffs, other.coeffs)

    def to_poly_str(self):
        terms = []
        for i, c in enumerate(self.coeffs):
            if c != 0:
                if i == 0:
                    term = f"{c}"
                elif i == 1:
                    term = "x" if c == 1 else f"{c}*x"
                else:
                    term = f"x^{i}" if c == 1 else f"{c}*x^{i}"
                terms.append(term)
        if not terms:
            return "0"
        return " + ".join(reversed(terms))

    def __str__(self):
        return self.to_poly_str()

    def __repr__(self):
        if self.M == 1:
            return f"GF({self.P})({self.coeffs[0]})"
        if self.P == 2:
            return f"GF(2^{self.M})({hex(self.val)})"
        return f"GF({self.P}^{self.M})({self.coeffs.tolist()})"

    def __int__(self):
        return self.val


# Generalized Polynomial class over any field
class Poly:
    def __init__(self, coeffs, field=None):
        coeffs = list(coeffs)
        if field is None:
            if len(coeffs) > 0:
                field = type(coeffs[0])
            else:
                raise ValueError("Must specify field for empty polynomial")
        
        coeffs = [field(c) for c in coeffs]
        while coeffs and coeffs[-1] == 0:
            coeffs.pop()
        self.coeffs = np.array(coeffs, dtype=object)
        self.field = field

    @staticmethod
    def from_bytes(b, field):
        v = int.from_bytes(b, 'big')
        v = int(f"{v:0{field.M}b}"[::-1], 2)
        return Poly([field(v)], field=field)

    def to_bytes(self):
        val = self.constant_coefficient().val
        v = int(f"{val:0{self.field.M}b}"[::-1], 2)
        num_bytes = (self.field.M + 7) // 8
        return v.to_bytes(num_bytes, 'big')

    def degree(self):
        return len(self.coeffs) - 1

    def is_zero(self):
        return len(self.coeffs) == 0

    def constant_coefficient(self):
        return self.coeffs[0] if len(self.coeffs) > 0 else self.field(0)

    def leading_coefficient(self):
        return self.coeffs[-1] if len(self.coeffs) > 0 else self.field(0)

    def _coerce(self, other):
        if isinstance(other, Poly):
            if other.field != self.field:
                raise ValueError("Polynomials must be over the same field")
            return other
        return Poly([self.field(other)], field=self.field)

    def __add__(self, other):
        other = self._coerce(other)
        n = max(len(self.coeffs), len(other.coeffs))
        coeffs = [
            (self.coeffs[i] if i < len(self.coeffs) else self.field(0)) +
            (other.coeffs[i] if i < len(other.coeffs) else self.field(0))
            for i in range(n)
        ]
        return Poly(coeffs, field=self.field)

    __radd__ = __add__

    def __sub__(self, other):
        other = self._coerce(other)
        n = max(len(self.coeffs), len(other.coeffs))
        coeffs = [
            (self.coeffs[i] if i < len(self.coeffs) else self.field(0)) -
            (other.coeffs[i] if i < len(other.coeffs) else self.field(0))
            for i in range(n)
        ]
        return Poly(coeffs, field=self.field)

    def __rsub__(self, other):
        return self._coerce(other) - self

    def __mul__(self, other):
        other = self._coerce(other)
        if self.is_zero() or other.is_zero():
            return Poly([], field=self.field)
        coeffs = [self.field(0)] * (self.degree() + other.degree() + 1)
        for i, c1 in enumerate(self.coeffs):
            for j, c2 in enumerate(other.coeffs):
                coeffs[i + j] += c1 * c2
        return Poly(coeffs, field=self.field)

    __rmul__ = __mul__

    def __divmod__(self, other):
        other = self._coerce(other)
        if other.is_zero():
            raise ZeroDivisionError("division by zero polynomial")
        num, den = list(self.coeffs), other.coeffs
        deg_num, deg_den = len(num) - 1, len(den) - 1
        if deg_num < deg_den:
            return Poly([], field=self.field), self
        quot = [self.field(0)] * (deg_num - deg_den + 1)
        inv_leading = self.field(1) / den[-1]
        for i in range(deg_num - deg_den, -1, -1):
            if num[deg_den + i] == 0:
                continue
            q = num[deg_den + i] * inv_leading
            quot[i] = q
            for j in range(deg_den + 1):
                num[i + j] -= q * den[j]
        return Poly(quot, field=self.field), Poly(num, field=self.field)

    def gcd(self, other):
        other = self._coerce(other)
        f, g = self, other
        while not g.is_zero():
            _, r = divmod(f, g)
            f, g = g, r
        if not f.is_zero() and f.leading_coefficient() != f.field(1):
            f = Poly([c / f.leading_coefficient() for c in f.coeffs], field=f.field)
        return f