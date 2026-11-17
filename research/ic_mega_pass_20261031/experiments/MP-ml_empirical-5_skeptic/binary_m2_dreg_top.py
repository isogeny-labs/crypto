"""Binary Weil descent of S_3 (m=2, ell = k-dim F_2-subspace V, N = 2k Boolean
unknowns, n equations): exact d_reg of the top-form ideal
F^top = (x_i^2) + (degree-2 parts of the descended equations), the Salizzoni
envelope d_reg(F^top)+1, and the measured degrees on the same instance:
  D_f4  = msolve -v 2 max F4 step degree (affine system + field equations),
  D*    = Boolean Macaulay refutation degree (least D with 1 in M_D).
Also: an a_6 sweep at fixed (V, z): F^top does not depend on a_6 (the tops are
the bilinear forms Tr(c X1 X2), Tr(c z) = 0), so if D varies with a_6 the
solving degree is not a function of (cell, F^top).
"""
import sys, os, json, random, subprocess, re, time, itertools
import numpy as np

OUT = os.path.dirname(os.path.abspath(__file__))
TMP = os.environ.get("TMPDIR", "/tmp")

IRR = {3: 0b1011, 4: 0b10011, 5: 0b100101, 6: 0b1000011, 7: 0b10000011, 8: 0b100011101,
       9: 0b1000010001, 10: 0b10000001001, 11: 0b100000000101, 12: 0b1000001010011,
       13: 0b10000000011011, 14: 0b100010001000011, 15: 0b1000000000000011,
       16: 0b10001000000001011, 17: 0b100000000000001001, 19: 0b10000000000000100111}


def gmul(a, b, n):
    m = IRR[n]; r = 0
    while b:
        if b & 1: r ^= a
        b >>= 1; a <<= 1
        if (a >> n) & 1: a ^= m
    return r


def gpow(a, e, n):
    r = 1
    while e:
        if e & 1: r = gmul(r, a, n)
        a = gmul(a, a, n); e >>= 1
    return r


def check_irreducible(n):
    # x^(2^n) == x mod f and x^(2^d) != x for proper divisors d: use order test on x
    x = 2
    y = x
    for i in range(1, n + 1):
        y = gmul(y, y, n)
        if y == x:
            return i == n
    return False


def trace(a, n):
    t = 0; x = a
    for _ in range(n):
        t ^= x; x = gmul(x, x, n)
    return t


# polynomials over F_{2^n} in Boolean variables: dict{monomask: coeff}
def pmul(A, B, n):
    C = {}
    for ma, ca in A.items():
        for mb, cb in B.items():
            m = ma | mb
            v = gmul(ca, cb, n)
            if v: C[m] = C.get(m, 0) ^ v
    return {m: c for m, c in C.items() if c}


def padd(*Ps):
    C = {}
    for Pp in Ps:
        for m, c in Pp.items(): C[m] = C.get(m, 0) ^ c
    return {m: c for m, c in C.items() if c}


def build_system(n, k, a6, z, basis):
    x1 = {1 << j: basis[j] for j in range(k)}
    x2 = {1 << (k + j): basis[j] for j in range(k)}
    xr = {0: z}
    e2 = padd(pmul(x1, x2, n), pmul(x1, xr, n), pmul(x2, xr, n))
    e3 = pmul(pmul(x1, x2, n), xr, n)
    S = padd(pmul(e2, e2, n), e3, {0: a6})
    eqs = []
    for j in range(n):
        f = frozenset(m for m, c in S.items() if (c >> j) & 1)
        if f: eqs.append(f)
    return eqs


def popcount(x): return bin(x).count("1")


def mono_str(m, N):
    return "*".join(f"x{i}" for i in range(N) if (m >> i) & 1) or "1"


def poly_str(f, N):
    return "+".join(mono_str(m, N) for m in sorted(f, key=lambda t: (-popcount(t), t)))


def msolve_run(lines, N, tag, extra, timeout=600):
    path = os.path.join(TMP, f"b_{tag}.ms"); out = os.path.join(TMP, f"b_{tag}.out")
    with open(path, "w") as f:
        f.write(",".join(f"x{i}" for i in range(N)) + "\n2\n" + ",\n".join(lines) + "\n")
    t0 = time.time()
    try:
        r = subprocess.run(["msolve", *extra, "-f", path, "-o", out], capture_output=True, text=True, timeout=timeout)
    except subprocess.TimeoutExpired:
        return None, None, None
    return r.stdout, open(out).read(), time.time() - t0


def hilbert_from_leading_ideal(gb_text, N):
    """Standard monomials of the leading ideal by degree (only squarefree monomials
    can be standard since x_i^2 is in the ideal)."""
    mm = re.search(r"\[(.*?)\]:", gb_text, re.S)
    lms = []
    for term in mm.group(1).split(","):
        term = term.strip()
        if not term: continue
        mask = 0; sq = False
        for part in term.split("*"):
            part = part.strip()
            if "^" in part:
                v, e = part.split("^"); e = int(e)
            else:
                v, e = part, 1
            if v == "1": continue
            i = int(v[1:])
            if e >= 2: sq = True
            mask |= 1 << i
        if not sq: lms.append(mask)
    lms = sorted(set(lms))
    hf = [0] * (N + 2)
    # squarefree monomials not divisible by any squarefree leading monomial
    all_m = np.arange(1 << N, dtype=np.int64)
    std = np.ones(1 << N, dtype=bool)
    for lm in lms:
        std &= (all_m & lm) != lm
    pc = np.array([popcount(int(m)) for m in range(1 << N)])
    for d in range(N + 1):
        hf[d] = int(np.sum(std & (pc == d)))
    dreg = next(d for d in range(N + 2) if hf[d] == 0)
    return hf[:N + 1], dreg, len(lms)


def boolean_macaulay_refutation_degree(eqs, N, Dmax):
    """Least D with 1 in the row space of the Boolean Macaulay matrix M_D
    (rows m*f, Boolean product, deg m <= D-2); None if not reached by Dmax."""
    monos_by_deg = {d: [m for m in range(1 << N) if popcount(m) == d] for d in range(Dmax + 1)}
    for D in range(2, Dmax + 1):
        cols = [m for d in range(D + 1) for m in monos_by_deg[d]]
        col_idx = {m: i for i, m in enumerate(cols)}
        pivots = {}
        one_bit = 1 << col_idx[0]
        for f in eqs:
            for d in range(D - 1):
                for m in monos_by_deg[d]:
                    row = 0
                    for t in f:
                        row ^= 1 << col_idx[m | t]
                    # reduce
                    while row:
                        hb = row.bit_length() - 1
                        if hb in pivots:
                            row ^= pivots[hb]
                        else:
                            pivots[hb] = row
                            break
        # is 1 (column index of monomial 0) in the row space? reduce the vector e_1
        v = one_bit
        while v:
            hb = v.bit_length() - 1
            if hb in pivots: v ^= pivots[hb]
            else: break
        if v == 0:
            return D
    return None


def count_solutions(eqs, N):
    A = np.arange(1 << N, dtype=np.int64)
    ok = np.ones(1 << N, dtype=bool)
    for f in eqs:
        val = np.zeros(1 << N, dtype=np.int64)
        for m in f:
            val ^= ((A & m) == m).astype(np.int64)
        ok &= (val == 0)
    return int(ok.sum())


def analyse_instance(n, k, a6, z, basis, tag, measure=True):
    N = 2 * k
    eqs = build_system(n, k, a6, z, basis)
    degs = [max(popcount(m) for m in f) for f in eqs]
    tops = [frozenset(m for m in f if popcount(m) == 2) for f in eqs if max(popcount(m) for m in f) == 2]
    umask = (1 << k) - 1
    bilinear = all(all(popcount(m & umask) == 1 for m in tp) for tp in tops)
    # rank of the span of the tops over F_2
    quad_monos = [m for m in range(1 << N) if popcount(m) == 2]
    qidx = {m: i for i, m in enumerate(quad_monos)}
    piv = {}
    for tp in tops:
        row = 0
        for m in tp: row ^= 1 << qidx[m]
        while row:
            hb = row.bit_length() - 1
            if hb in piv: row ^= piv[hb]
            else: piv[hb] = row; break
    top_rank = len(piv)
    rec = {"tag": tag, "n": n, "k": k, "N": N, "n_eqs": len(eqs), "eq_degrees": sorted(set(degs)),
           "tops_bilinear": bilinear, "top_span_rank": top_rank, "expected_rank_n_minus_1": n - 1}
    # exact d_reg(F^top) via msolve leading ideal over F_2
    hom = [f"x{i}^2" for i in range(N)] + [poly_str(tp, N) for tp in tops]
    so, gb, wt = msolve_run(hom, N, tag + "_top", ["-g", "1"], timeout=900)
    if gb is not None:
        hf, dreg, nlm = hilbert_from_leading_ideal(gb, N)
        rec.update({"HF_R_mod_Ftop": hf, "d_reg_Ftop": dreg, "salizzoni_bound": dreg + 1, "ftop_gb_wall_s": round(wt, 2), "n_sqfree_leading_monomials": nlm})
    if measure:
        rec["num_boolean_solutions"] = count_solutions(eqs, N)
        aff = [poly_str(f, N) for f in eqs] + [f"x{i}^2+x{i}" for i in range(N)]
        so, gb, wt = msolve_run(aff, N, tag + "_aff", ["-v", "2", "-g", "2"], timeout=900)
        if so is not None:
            steps = [int(m.group(1)) for m in re.finditer(r"^\s*(\d+)\s+\d+\s+\d+\s+\d+ x \d+", so, re.M)]
            rec["D_f4_max_step"] = max(steps) if steps else None
            rec["f4_steps"] = steps
            rec["affine_wall_s"] = round(wt, 2)
        if N <= 14 and rec["num_boolean_solutions"] == 0:
            rec["D_star_macaulay"] = boolean_macaulay_refutation_degree(eqs, N, 5)
        if "d_reg_Ftop" in rec and rec.get("D_f4_max_step") is not None:
            rec["gap_bound_minus_Df4"] = rec["salizzoni_bound"] - rec["D_f4_max_step"]
    return rec


def random_subspace(n, k, rng):
    while True:
        basis = [rng.randrange(1, 1 << n) for _ in range(k)]
        # check independence
        piv = {}
        ok = True
        for b in basis:
            row = b
            while row:
                hb = row.bit_length() - 1
                if hb in piv: row ^= piv[hb]
                else: piv[hb] = row; break
            if row == 0: ok = False; break
        if ok: return basis


if __name__ == "__main__":
    for n in IRR: assert check_irreducible(n), n
    rng = random.Random(1031)
    results = {"dreg_scan": [], "a6_sweep": None}
    # 1. d_reg(F^top) vs measured D on the Semaev diagonal k = ceil(n/2) and off it
    cells = [(n, (n + 1) // 2) for n in (11, 13, 15, 17)] + [(13, 5), (15, 5), (17, 5), (17, 6), (19, 6), (16, 4), (17, 4), (19, 4)]
    for (n, k) in cells:
        for draw in range(2):
            basis = random_subspace(n, k, rng)
            a6 = rng.randrange(1, 1 << n); z = rng.randrange(1, 1 << n)
            rec = analyse_instance(n, k, a6, z, basis, f"n{n}k{k}d{draw}", measure=(2 * k <= 16))
            print(json.dumps(rec), flush=True)
            results["dreg_scan"].append(rec)
    json.dump(results, open(os.path.join(OUT, "binary_m2_results.json"), "w"), indent=1)
    # 2. a6 sweep at n=11, k=6 with fixed (V, z): F^top fixed, does D move?
    n, k = 11, 6
    basis = random_subspace(n, k, rng); z = rng.randrange(1, 1 << n)
    sweep = {"n": n, "k": k, "basis": basis, "z": z, "rows": []}
    tops_seen = set()
    for a6 in rng.sample(range(1, 1 << n), 120):
        eqs = build_system(n, k, a6, z, basis)
        N = 2 * k
        tops = frozenset(frozenset(m for m in f if popcount(m) == 2) for f in eqs)
        tops_seen.add(tops)
        nsol = count_solutions(eqs, N)
        row = {"a6": a6, "num_solutions": nsol}
        if nsol == 0:
            row["D_star_macaulay"] = boolean_macaulay_refutation_degree(eqs, N, 5)
            aff = [poly_str(f, N) for f in eqs] + [f"x{i}^2+x{i}" for i in range(N)]
            so, gb, wt = msolve_run(aff, N, f"sweep_{a6}", ["-v", "2", "-g", "2"], timeout=300)
            steps = [int(m.group(1)) for m in re.finditer(r"^\s*(\d+)\s+\d+\s+\d+\s+\d+ x \d+", so, re.M)] if so else []
            row["D_f4_max_step"] = max(steps) if steps else None
        sweep["rows"].append(row)
    sweep["distinct_Ftop_sets_seen"] = len(tops_seen)
    from collections import Counter
    sweep["D_star_distribution_unsat"] = dict(Counter(str(r.get("D_star_macaulay")) for r in sweep["rows"] if r["num_solutions"] == 0))
    sweep["D_f4_distribution_unsat"] = dict(Counter(str(r.get("D_f4_max_step")) for r in sweep["rows"] if r["num_solutions"] == 0))
    sweep["n_sat"] = sum(1 for r in sweep["rows"] if r["num_solutions"] > 0)
    results["a6_sweep"] = sweep
    print(json.dumps({k_: v for k_, v in sweep.items() if k_ != "rows"}), flush=True)
    json.dump(results, open(os.path.join(OUT, "binary_m2_results.json"), "w"), indent=1)
