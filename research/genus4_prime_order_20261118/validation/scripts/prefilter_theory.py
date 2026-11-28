#!/usr/bin/env python3
"""C2 (mathematics), the validator's numeric test on independent seeds.

For random monic squarefree f of degree 9 over small primes p (validator RNG, seed 'val4po-theory'),
#J(F_p) = L(1) from the point counts N_k = #C(F_{p^k}), k = 1..4 (vectorised character sums over
F_{p^k}; Newton's identities; functional equation a_{2g-i} = p^(g-i) a_i).  No BSGS, nothing from the
producer.  Checks, for every curve:
  - reducible f  =>  #J even      (the prefilter's premise)
  - irreducible f => #J odd       (so the prefilter is exactly the parity test)
  - 2^(m-1) | #J, m = number of irreducible factors  (#J[2](F_p) = 2^(m-1))
  - the 2-torsion certificate (w, 0) of every reducible f is valid, non-zero and [2](w,0) = 0
    in the validator's own Cantor arithmetic.
Writes out/prefilter_theory.json."""
import json, random, sys, os
import numpy as np
sys.path.insert(0, os.path.dirname(__file__))
from jac import Jac, trim
from polyfp import squarefree, n_factors, reducible_certificate, check_certificate
from common import dump

G = 4


def irreducible_modulus(p, k, rnd):
    while True:
        m = [rnd.randrange(p) for _ in range(k)] + [1]
        if k == 1 or (squarefree(m, p) and reducible_certificate_any(m, p) is None):
            return m


def reducible_certificate_any(m, p):
    # degree-k irreducibility via DDF (k <= 4)
    from polyfp import ddf
    parts = ddf(m, p)
    return None if (len(parts) == 1 and parts[0][0] == len(m) - 1) else parts


class Fq:
    """F_{p^k} = F_p[t]/(m(t)), all q elements as an (q, k) array of base-p digits."""
    def __init__(self, p, k, rnd):
        self.p, self.k = p, k
        self.m = irreducible_modulus(p, k, rnd)
        q = p ** k
        idx = np.arange(q, dtype=np.int64)
        self.all = np.stack([(idx // p ** i) % p for i in range(k)], axis=1)
        sq = self.mul(self.all, self.all)
        self.is_sq = np.zeros(q, dtype=bool)
        self.is_sq[self.enc(sq)] = True

    def enc(self, a):
        return sum(a[:, i] * self.p ** i for i in range(self.k))

    def mul(self, a, b):
        p, k = self.p, self.k
        n = a.shape[0]
        c = np.zeros((n, 2 * k - 1), dtype=np.int64)
        for i in range(k):
            for j in range(k):
                c[:, i + j] += a[:, i] * b[:, j]
        c %= p
        for t in range(2 * k - 2, k - 1, -1):     # reduce t^t with t^k = -sum m_i t^i
            top = c[:, t].copy()
            for i in range(k):
                c[:, t - k + i] -= top * self.m[i]
            c[:, t] = 0
            c %= p
        return c[:, :k] % p

    def char_sum(self, f):
        acc = np.zeros_like(self.all)
        for coef in reversed(f):
            acc = self.mul(acc, self.all)
            acc[:, 0] = (acc[:, 0] + coef) % self.p
        e = self.enc(acc)
        chi = np.where(e == 0, 0, np.where(self.is_sq[e], 1, -1))
        return int(chi.sum())


def jac_order(fields, f, p):
    s = [fields[k].char_sum(f) + 0 for k in range(1, G + 1)]          # N_k - p^k - 1 = sum chi
    a = [1]
    for k in range(1, G + 1):
        num = sum(s[i - 1] * a[k - i] for i in range(1, k + 1))
        assert num % k == 0
        a.append(num // k)
    full = a + [p ** (G - i) * a[i] for i in range(G - 1, -1, -1)]
    return sum(full), a


def main():
    rnd = random.Random("val4po-theory")
    res = {"method": __doc__, "per_p": {}, "fail": []}
    for p, n in ((11, 120), (13, 120), (17, 100), (19, 80), (23, 60), (29, 40)):
        fields = {k: Fq(p, k, rnd) for k in range(1, G + 1)}
        stats = {"curves": 0, "reducible": 0, "irreducible": 0, "even": 0, "odd": 0, "certs_checked": 0, "m_hist": {}}
        while stats["curves"] < n:
            f = [rnd.randrange(p) for _ in range(2 * G + 1)] + [1]
            if not squarefree(f, p):
                continue
            stats["curves"] += 1
            N, _ = jac_order(fields, f, p)
            m = n_factors(f, p)
            stats["m_hist"][m] = stats["m_hist"].get(m, 0) + 1
            cert = reducible_certificate(f, p, rnd)
            red = cert is not None
            stats["reducible" if red else "irreducible"] += 1
            stats["even" if N % 2 == 0 else "odd"] += 1
            if red != (N % 2 == 0):
                res["fail"].append({"p": p, "f": f, "N": N, "reducible": red})
            if N % (2 ** (m - 1)) != 0:
                res["fail"].append({"p": p, "f": f, "N": N, "m": m, "why": "2^(m-1) does not divide #J"})
            if red:
                J = Jac(p, f)
                D = (trim(cert[1]), ())
                ok = check_certificate(f, p, cert) and J.valid(D) and not J.is_zero(D) and J.is_zero(J.add(D, D))
                stats["certs_checked"] += 1
                if not ok:
                    res["fail"].append({"p": p, "f": f, "cert": cert, "why": "2-torsion certificate"})
        res["per_p"][str(p)] = stats
        print(p, stats, flush=True)
    res["ok"] = not res["fail"]
    dump("prefilter_theory.json", res)
    print("ok", res["ok"], "fails", len(res["fail"]))


if __name__ == "__main__":
    main()
