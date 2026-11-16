import sys
REF="/Volumes/SSD990-2/cryptanalysis/experiments/hashed-ecdl-tensor-reference"
sys.dont_write_bytecode=True
if REF not in sys.path: sys.path.insert(0,REF)
import random
from tensor_encoding import WordCircuit, BinaryCurveEncoding, field_power, bits_value
from ecc2k import BinaryField, challenge, parameters

class FreeCircuit(WordCircuit):
    """WordCircuit with the char-2 Schoenhage gadget parametrised by (J,k).
    J: ring B_J=F2[x]/(x^2J+x^J+1) (word 2J bits); k: block width, k<=J (2k-1<=2J).
    J=None -> prototype behaviour (k=J//4, J grown until L<=3J)."""
    J_FIXED = None
    K_FIXED = None
    def polynomial_multiply(self, left, right):
        if self.J_FIXED is None:
            return WordCircuit.polynomial_multiply(self, left, right)
        d = max(len(left), len(right))
        left = tuple(left) + self.constant(0, d - len(left))
        right = tuple(right) + self.constant(0, d - len(right))
        J, k = self.J_FIXED, self.K_FIXED
        assert 1 <= k <= J
        n, L = (d + k - 1) // k, 1
        while L < 2 * n - 1: L *= 3
        assert (3 * J) % L == 0, ("L must divide 3J", L, J)
        e, t = 0, L
        while t > 1: e += 1; t //= 3
        L1, L2 = 3 ** (e // 2), 3 ** ((e + 1) // 2)
        field = BinaryField(2 * J, (1 << (2 * J)) | (1 << J) | 1)
        root = field.reduce(1 << (3 * J // L))
        powers = [field_power(field, root, i) for i in range(L)]
        assert field_power(field, root, L) == 1 and (L == 1 or field_power(field, root, L // 3) != 1)
        add, multiply = (lambda a, b: a ^ b), field.mul
        def chunks(word):
            padded = word + self.constant(0, n * k - d)
            return [padded[i * k:(i + 1) * k] for i in range(n)] + [self.constant(0, k)] * (L - n)
        tr = [self.transform(chunks(w), L1, L2, 2 * J, powers, add, multiply, name="binary_fft") for w in (left, right)]
        prods = [self.compute((a, b), (2 * J,), lambda v: (multiply(*v),), "binary_ring_product")[0] for a, b in zip(*tr)]
        co = self.transform(prods, L1, L2, 2 * J, [powers[-i % L] for i in range(L)], add, multiply, name="binary_ifft")
        res = []
        for bit in range(2 * d - 1):
            terms = [co[j][bit - j * k] for j in range(2 * n - 1) if 0 <= bit - j * k < 2 * k - 1]
            res.append(self.xor_bit(*terms))
        self.mul_metadata.append({"type": "binary_polynomial", "J": J, "k": k, "L": L, "n": n})
        return tuple(res)

def canon_py(field, x, y):
    best = (x, y); cx, cy = x, y
    for _ in range(field.degree):
        cx, cy = field.square(cx), field.square(cy)
        if (cx, cy) < best: best = (cx, cy)
    return best

class FoldEncoding(BinaryCurveEncoding):
    """BinaryCurveEncoding with free (J,k) and optional Frobenius canonicalisation vertex."""
    def __init__(self, curve, J=None, k=None, fold=False):
        super().__init__(curve)
        self.circuit = FreeCircuit(); self.circuit.J_FIXED = J; self.circuit.K_FIXED = k
        self.fold = fold
    def hashed_membership(self, point, prime, alpha, beta, threshold):
        if not self.fold:
            return super().hashed_membership(point, prime, alpha, beta, threshold)
        c, f = self.circuit, self.field; b = f.degree
        cx, cy = c.word(b), c.word(b)
        # one vertex, 4*b ports: (x,y) -> (x',y') = tau^j(x,y), x' least rotation (normal basis)
        def pred(bits, b=b, f=f):
            return (bits_value(bits[2*b:3*b]), bits_value(bits[3*b:])) == canon_py(f, bits_value(bits[:b]), bits_value(bits[b:2*b]))
        c.builder.constraint(tuple(v for w in (point.x, point.y, cx, cy) for v in w), pred, "frobenius_canon")
        c.steps.append(((tuple(point.x), tuple(point.y)), (cx, cy), lambda v: canon_py(f, v[0], v[1])))
        canon = type(point)(point.infinity, cx, cy)
        return super().hashed_membership(canon, prime, alpha, beta, threshold)
