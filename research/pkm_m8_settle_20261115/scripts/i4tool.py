#!/usr/bin/env python3
"""I4 certificate tool (r7/m8 round 2).

Given a degree-truncated msolve basis G (MSOLVE_TRUNC_DEG=c, -g 2 output) and the input system:
  summary : parse G, check LT = first printed term under DRL (first variable largest), constant check (c),
            delta_c = #monomials of degree <= c divisible by no LT(G)  (rank deficit lower bound),
            pair count: pairs with deg lcm <= c after product criterion and strict chain criterion.
  singular: write a Singular script that checks (a) every input generator reduces to 0 mod G and
            (b) every listed S-pair reduces to 0 mod G (all pairs, or a seeded sample of K).

Usage:
  i4tool.py summary  SYS.ms G.out c  > summary.json
  i4tool.py singular SYS.ms G.out c OUT.sing RESULT.txt [K SEED]
  i4tool.py selftest                  (delta counter vs brute force on random monomial ideals)
"""
import itertools
import json
import random
import re
import sys
from functools import lru_cache

TERM = re.compile(r"([+-]?\d+)((?:\*[A-Za-z_][A-Za-z0-9_]*\^\d+)*)")


def parse_poly(s, vidx, nv):
    s = s.strip().rstrip(",").replace(" ", "")
    poly = []
    # split on + that start a new term (coefficients are nonnegative in msolve output; input may have '-')
    for tok in re.split(r"(?<=[0-9A-Za-z_])(?=[+-])", s):
        if not tok:
            continue
        tok = tok.lstrip("+")
        parts = tok.split("*")
        coef = None
        e = [0] * nv
        for part in parts:
            if re.fullmatch(r"-?\d+", part):
                coef = (coef or 1) * int(part)
            else:
                neg = part.startswith("-")
                if neg:
                    part = part[1:]
                    coef = -(coef or 1)
                if "^" in part:
                    v, k = part.split("^")
                    e[vidx[v]] += int(k)
                else:
                    e[vidx[part]] += 1
        if coef is None:
            coef = 1
        poly.append((tuple(e), coef))
    return poly


def read_ms(path):
    lines = [ln.strip() for ln in open(path) if ln.strip()]
    names = [v.strip() for v in lines[0].split(",")]
    p = int(lines[1])
    vidx = {v: i for i, v in enumerate(names)}
    body = " ".join(lines[2:])
    polys = [parse_poly(q, vidx, len(names)) for q in body.split(",") if q.strip()]
    return names, p, polys


def read_out(path):
    txt = open(path).read()
    hdr = {}
    for ln in txt.splitlines():
        if ln.startswith("#variable order:"):
            hdr["vars"] = [v.strip() for v in ln.split(":", 1)[1].split(",")]
        if ln.startswith("#field characteristic:"):
            hdr["p"] = int(ln.split(":", 1)[1])
    body = txt[txt.index("["):txt.rindex("]") + 1].strip("[]")
    names = hdr["vars"]
    vidx = {v: i for i, v in enumerate(names)}
    polys = [parse_poly(q, vidx, len(names)) for q in body.split(",") if q.strip()]
    return names, hdr["p"], polys


def drl_key(e):
    # DRL with first variable largest: compare total degree, then the LAST variable with smaller exponent wins
    return (sum(e), tuple(-x for x in reversed(e)))


def lt(poly):
    return max(poly, key=lambda t: drl_key(t[0]))[0]


def minimalize(lts):
    lts = sorted(set(lts), key=sum)
    out = []
    for a in lts:
        if not any(all(x <= y for x, y in zip(b, a)) for b in out):
            out.append(a)
    return out


def count_standard(lts, nv, c):
    """#monomials of degree <= c in nv variables divisible by no element of lts."""
    lts = [t for t in minimalize(lts) if sum(t) <= c]

    @lru_cache(maxsize=None)
    def rec(gens, k, d):
        # gens: frozenset of exponent tuples over variables k..nv-1 ; count monomials in those vars of deg <= d
        if d < 0:
            return 0
        if any(sum(g) == 0 for g in gens):
            return 0
        if not gens:
            # C(nvars_left + d, d)
            n = nv - k
            from math import comb
            return comb(n + d, d)
        if k == nv:
            return 1
        total = 0
        # exponents of variable k appearing in gens
        for e in range(d + 1):
            sub = frozenset(minimalize([g[1:] for g in gens if g[0] <= e]))
            total += rec(sub, k + 1, d - e)
        return total

    return rec(frozenset(lts), 0, c)


def brute_standard(lts, nv, c):
    cnt = 0
    for d in range(c + 1):
        for comb_ in itertools.combinations_with_replacement(range(nv), d):
            e = [0] * nv
            for i in comb_:
                e[i] += 1
            if not any(all(x <= y for x, y in zip(b, e)) for b in lts):
                cnt += 1
    return cnt


def pairs_after_criteria(L, c):
    """L: list of LT exponent tuples (index = basis index). Return list of (i, j) with deg lcm <= c,
    LTs not coprime, and not removable by the strict chain criterion
    (exists k: LT_k | lcm_ij, lcm_ik != lcm_ij, lcm_jk != lcm_ij)."""
    import numpy as np
    A = np.array(L, dtype=np.int16)
    n = len(L)
    out = []
    nprod = nchain = ndeg = 0
    deg = A.sum(1)
    for i in range(n):
        if n - i - 1 == 0:
            continue
        lc = np.maximum(A[i], A[i + 1:])
        dl = lc.sum(1)
        js = np.nonzero(dl <= c)[0]
        ndeg += int((dl > c).sum())
        for jj in js:
            j = i + 1 + int(jj)
            lcm = lc[jj]
            if not np.any((A[i] > 0) & (A[j] > 0)):
                nprod += 1
                continue
            div = np.all(A <= lcm, axis=1)
            div[i] = div[j] = False
            ks = np.nonzero(div)[0]
            red = False
            if ks.size:
                lik = np.maximum(A[ks], A[i])
                ljk = np.maximum(A[ks], A[j])
                ne_i = np.any(lik != lcm, axis=1)
                ne_j = np.any(ljk != lcm, axis=1)
                red = bool(np.any(ne_i & ne_j))
            if red:
                nchain += 1
                continue
            out.append((i, j))
    return out, dict(deg_gt_c=ndeg, product=nprod, chain=nchain, kept=len(out))


def poly_str(poly, names):
    terms = []
    for e, cf in poly:
        mon = "*".join(f"{names[i]}^{k}" if k > 1 else names[i] for i, k in enumerate(e) if k)
        terms.append(f"{cf}*{mon}" if mon else f"{cf}")
    return "+".join(terms).replace("+-", "-")


def mono_str(e, names):
    s = "*".join(f"{names[i]}^{k}" if k > 1 else names[i] for i, k in enumerate(e) if k)
    return s if s else "1"


def main():
    cmd = sys.argv[1]
    if cmd == "selftest":
        rnd = random.Random(7)
        for trial in range(60):
            nv = rnd.randint(2, 5)
            c = rnd.randint(1, 6)
            lts = [tuple(rnd.randint(0, 3) for _ in range(nv)) for _ in range(rnd.randint(0, 6))]
            lts = [t for t in lts if sum(t) > 0]
            a, b = count_standard(lts, nv, c), brute_standard(minimalize(lts), nv, c)
            assert a == b, (lts, nv, c, a, b)
        print("selftest count_standard vs brute: 60/60 PASS")
        return
    sysf, outf, c = sys.argv[2], sys.argv[3], int(sys.argv[4])
    names_in, p_in, F = read_ms(sysf)
    names, p, G = read_out(outf)
    assert names == names_in and p == p_in, "variable order / field mismatch"
    nv = len(names)
    L = []
    lt_first = True
    for g in G:
        t = lt(g)
        if t != g[0][0]:
            lt_first = False
        L.append(t)
    const = any(sum(t) == 0 for t in L)
    maxdeg = max(sum(e) for g in G for e, _ in g)
    if cmd == "summary":
        from math import comb
        total = comb(nv + c, c)
        delta = count_standard(L, nv, c)
        pairs, pst = pairs_after_criteria(L, c)
        print(json.dumps(dict(sys=sysf.split("/")[-1], c=c, nvars=nv, ngens_input=len(F), basis_size=len(G),
                              basis_max_deg=maxdeg, lt_is_first_term=lt_first, constant_in_basis=const,
                              lt_minimal=len(minimalize(L)) == len(set(L)),
                              dim_total=total, delta_c=delta, dim_W=total - delta, pairs=pst)))
    elif cmd == "singular":
        sing, resf = sys.argv[5], sys.argv[6]
        K = int(sys.argv[7]) if len(sys.argv) > 7 else 0
        seed = int(sys.argv[8]) if len(sys.argv) > 8 else 0
        pairs, pst = pairs_after_criteria(L, c)
        if K and len(pairs) > K:
            pairs = sorted(random.Random(seed).sample(pairs, K))
            mode = f"sample {K} of {pst['kept']} (seed {seed})"
        else:
            mode = f"all {len(pairs)}"
        with open(sing, "w") as f:
            f.write(f"ring r = ({p}), ({','.join(names)}), dp;\noption(redSB);\n")
            f.write("ideal G;\n")
            for k, g in enumerate(G):
                f.write(f"G[{k+1}] = {poly_str(g, names)};\n")
            f.write('attrib(G, "isSB", 1);\nideal F;\n')
            for k, q in enumerate(F):
                f.write(f"F[{k+1}] = {poly_str(q, names)};\n")
            f.write("int bad = 0; int k; poly s;\n")
            f.write("int nf = 0;\nfor (k = 1; k <= ncols(F); k++) { if (reduce(F[k], G) != 0) { nf++; } }\n")
            f.write(f'write("{resf}", "generators_nonzero_NF " + string(nf) + " of " + string(ncols(F)));\n')
            f.write("int np = 0;\n")
            for (i, j) in pairs:
                lcm = tuple(max(a, b) for a, b in zip(L[i], L[j]))
                mi = tuple(a - b for a, b in zip(lcm, L[i]))
                mj = tuple(a - b for a, b in zip(lcm, L[j]))
                f.write(f"s = ({mono_str(mi, names)})*G[{i+1}]/leadcoef(G[{i+1}]) - ({mono_str(mj, names)})*G[{j+1}]/leadcoef(G[{j+1}]); "
                        "if (reduce(s, G) != 0) { bad++; } np++;\n")
            f.write(f'write("{resf}", "pairs_checked " + string(np) + " nonzero_NF " + string(bad) + " mode {mode}");\n')
            f.write('write("' + resf + '", "constant_LT " + string(' + ("1" if const else "0") + '));\nquit;\n')
        print(json.dumps(dict(sing=sing, mode=mode, pairs=pst)))


if __name__ == "__main__":
    main()
