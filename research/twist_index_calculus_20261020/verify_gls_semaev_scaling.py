#!/usr/bin/env python3
"""Independent finite-field check of GLS abscissa scaling and Semaev S3.

The report gives the general algebraic proof. This script verifies the
identity on deterministic points of all three study fields and finds a
genuine group-relation counterexample to independently conjugating one
factor's abscissa. It is not a polynomial-system solver or a timing run.
"""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parent


class F2:
    def __init__(self, p, d):
        self.p, self.d = p, d
        self.zero, self.one = (0, 0), (1, 0)

    def const(self, n):
        return n % self.p, 0

    def add(self, x, y):
        return (x[0] + y[0]) % self.p, (x[1] + y[1]) % self.p

    def neg(self, x):
        return (-x[0]) % self.p, (-x[1]) % self.p

    def sub(self, x, y):
        return self.add(x, self.neg(y))

    def mul(self, x, y):
        return (
            (x[0] * y[0] + self.d * x[1] * y[1]) % self.p,
            (x[0] * y[1] + x[1] * y[0]) % self.p,
        )

    def pow(self, x, n):
        z = self.one
        while n:
            if n & 1:
                z = self.mul(z, x)
            x = self.mul(x, x)
            n >>= 1
        return z

    def inv(self, x):
        assert x != self.zero
        norm = (x[0] * x[0] - self.d * x[1] * x[1]) % self.p
        scale = pow(norm, self.p - 2, self.p)
        return (x[0] * scale) % self.p, (-x[1] * scale) % self.p

    def div(self, x, y):
        return self.mul(x, self.inv(y))

    def frob(self, x):
        return x[0], (-x[1]) % self.p

    def s3(self, a, b, x, y, z):
        # Same explicit polynomial as src/cryptanalysis/line_oracle.rs.
        t1 = self.mul(self.mul(self.sub(x, y), self.sub(x, y)), self.mul(z, z))
        inner = self.add(
            self.mul(self.add(x, y), self.add(self.mul(x, y), a)),
            self.mul(self.const(2), b),
        )
        t2 = self.mul(self.const(2), self.mul(inner, z))
        t3 = self.mul(self.sub(self.mul(x, y), a), self.sub(self.mul(x, y), a))
        t4 = self.mul(self.const(4), self.mul(b, self.add(x, y)))
        return self.sub(self.add(self.sub(t1, t2), t3), t4)

    def ec_add(self, u, v, a):
        if u is None:
            return v
        if v is None:
            return u
        x1, y1 = u
        x2, y2 = v
        if x1 == x2 and self.add(y1, y2) == self.zero:
            return None
        if u == v:
            num = self.add(self.mul(self.const(3), self.mul(x1, x1)), a)
            den = self.mul(self.const(2), y1)
        else:
            num, den = self.sub(y2, y1), self.sub(x2, x1)
        slope = self.div(num, den)
        x3 = self.sub(self.sub(self.mul(slope, slope), x1), x2)
        y3 = self.sub(self.mul(slope, self.sub(x1, x3)), y1)
        return x3, y3

    def ec_mul(self, n, point, a):
        out = None
        while n:
            if n & 1:
                out = self.ec_add(out, point, a)
            point = self.ec_add(point, point, a)
            n >>= 1
        return out


def main():
    curve_rows = [
        json.loads(line)
        for line in (ROOT / "gls_symmetry_results_v2.ndjson").read_text().splitlines()
        if '"kind":"curve"' in line
    ]
    assert [row["p"] for row in curve_rows] == [101, 211, 307]
    output = []
    for row in curve_rows:
        p, d = row["p"], row["d"]
        f = F2(p, d)
        c = tuple(row["c"])
        a, b = f.const(row["a"]), f.const(row["b_curve"])
        ac = f.mul(a, f.pow(c, 2))
        bc = f.mul(b, f.pow(c, 3))
        c4 = f.pow(c, 4)
        alpha = f.pow(c, p * p - 1 - (p - 1) // 2)
        assert f.pow(c, (p * p - 1) // 2) == f.neg(f.one)
        assert f.mul(f.pow(alpha, 2), f.frob(c)) == c

        for k in range(128):
            xs = [((k * m + t) % p, (k * n + t * t) % p)
                  for m, n, t in ((3, 5, 1), (7, 11, 2), (13, 17, 3))]
            scaled = f.s3(ac, bc, *(f.mul(c, x) for x in xs))
            plain = f.s3(a, b, *xs)
            assert scaled == f.mul(c4, plain)
            assert f.s3(a, b, *(f.frob(x) for x in xs)) == f.frob(plain)

        coords = row["generator"]
        g = ((coords[0], coords[1]), (coords[2], coords[3]))
        assert f.ec_mul(row["r"], g, ac) is None
        found = None
        for i in range(1, 25):
            for j in range(i + 1, 25):
                u, v = f.ec_mul(i, g, ac), f.ec_mul(j, g, ac)
                uv = f.ec_add(u, v, ac)
                if uv is None:
                    continue
                w = uv[0], f.neg(uv[1])
                xvals = [f.div(point[0], c) for point in (u, v, w)]
                if len(set(xvals)) != 3:
                    continue
                assert f.s3(a, b, *xvals) == f.zero
                assert f.s3(a, b, *(f.frob(x) for x in xvals)) == f.zero
                for point, x in zip((u, v, w), xvals):
                    transported = f.mul(f.pow(alpha, 2), f.frob(point[0]))
                    assert f.div(transported, c) == f.frob(x)
                one_conjugate = f.s3(a, b, f.frob(xvals[0]), xvals[1], xvals[2])
                if one_conjugate != f.zero:
                    found = {
                        "scalars": [i, j, -(i + j)],
                        "scaled_abscissas": [list(x) for x in xvals],
                        "single_factor_conjugate_s3": list(one_conjugate),
                    }
                    break
            if found:
                break
        assert found is not None
        output.append({"p": p, "r": row["r"], "checked_arbitrary_triples": 128,
                       "counterexample": found})
    (ROOT / "gls_semaev_scaling_v1.json").write_text(
        json.dumps({"schema": "gls-semaev-scaling/v1", "curves": output}, indent=2) + "\n"
    )
    print("S3 scaling, global Frobenius, and single-factor counterexamples: 3 curves OK")


if __name__ == "__main__":
    main()
