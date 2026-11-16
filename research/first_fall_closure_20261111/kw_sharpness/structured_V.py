"""Structured factor bases: (a) V inside a proper subfield, (b) V = span{1,theta,...,theta^{n'-1}} (low-degree polynomials)."""
import json, random, sys
import ffd
from kwgf import GF
import itertools

def analyse(n, npr, nus, F, c, a6, label):
    N = 3 * npr
    s = ffd.descend(F, nus, c, a6)
    M = ffd.masks(N)
    top = ffd.topform_analysis(s, N)
    ev = list(ffd.moore_e(F, nus).values())
    prods = {F.mul(F.mul(x, y), z) for x, y, z in itertools.combinations_with_replacement(ev, 3)}
    op, log = ffd.operational_first_fall(s, N, cmax=8)
    r = dict(label=label, n=n, npr=npr, nus=nus, dimV=top['dimV'], dimVbar=top['dimVbar'], dimE=ffd.rank_int(ev), dimE3=ffd.rank_int(list(prods)),
             Dff_top=top.get('Dff_top'), nullity7=top.get('nullity_deg7'), Dff_op=op, op_log=log)
    print({k: v for k, v in r.items() if k != 'op_log'}); return r

def subfield_basis(F, k, npr, rnd):
    """npr F_2-independent elements of the subfield GF(2^k) inside GF(2^n) (k | n)"""
    step = F.q1 // ((1 << k) - 1)
    elems = [0] + [int(F.exp[(j * step) % F.q1]) for j in range((1 << k) - 1)]
    while True:
        nus = rnd.sample(elems[1:], npr)
        if ffd.rank_int(nus) == npr: return nus

if __name__ == "__main__":
    res = []
    rnd = random.Random(99)
    for (n, k, npr) in [(12, 4, 4), (12, 6, 5), (12, 6, 6), (14, 7, 6), (15, 5, 5), (18, 6, 6)]:
        F, a2, a6, c = ffd.random_instance(n, rnd)
        nus = subfield_basis(F, k, npr, rnd)
        res.append(analyse(n, npr, nus, F, c, a6, "V inside GF(2^%d), n=%d" % (k, n)))
    # polynomial basis  1, theta, ..., theta^{n'-1}  with theta = x (primitive root of the defining polynomial)
    for (n, npr) in [(13, 5), (14, 5), (15, 5), (16, 6), (17, 6), (18, 6), (19, 7), (20, 7), (21, 7)]:
        F, a2, a6, c = ffd.random_instance(n, rnd)
        nus = [1 << j for j in range(npr)]
        res.append(analyse(n, npr, nus, F, c, a6, "polynomial basis 1,x,..,x^{n'-1}"))
    json.dump(res, open("structured_V_results.json", "w"), indent=1)
