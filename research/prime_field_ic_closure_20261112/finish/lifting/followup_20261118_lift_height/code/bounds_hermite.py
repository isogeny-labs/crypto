#!/usr/bin/env python3
"""Lifting union bound with the Hermite circuit cutoff (Lemma 3H).  [pcc:unverified]

Lemma 3H (this follow-up; paper proof in NOTE.md section 2):
  if P'_1..P'_r in E'(Q) are dependent, there is a circuit J (|J| = k+1, the points of J span
  rank k in E'(Q)/tors and every proper subset is independent) and a primitive n* supported on
  J with sum n*_i P'_i in E'(Q)_tors and |n*_i| <= (gamma_k * H / lambda)^(k/2) = (gamma_k rho)^(k/2).
  Its reduction is a point of <S> of order <= 12 < q, hence O.  So T (torsion) drops out.
Event count (support-refined, +-n identified):
  N(r, rho) = 1/2 * sum_{j=1..r} binom(r, j) * (2*C_{j-1})^j,   C_k = floor((gamma_k rho)^(k/2)), C_0 = 1
Per trial  Pr[dependent] <= N/q ;  E[#trials] >= q/(2N) ;  margin(b) = b/2 - log2(2N)  (bits over sqrt q).
The old box cutoff (bounds.py, PROOF.md Thm 1) is recomputed with the same convention for comparison:
  N_box = ((2C+1)^r - 1)/2 , C = C_sharp(r, rho, T).
"""
import math, json, sys, os
sys.path.insert(0, os.environ.get("LIFT_CODE", "/home/user/crypto/research/prime_field_ic_closure_20261112/finish/lifting/code"))
import bounds as B0   # original bounds.py (C_sharp, lam_petsche)

LOG2 = math.log2

# Hermite constants: exact gamma_k^k for k <= 8 (Korkine-Zolotarev k<=5, Blichfeldt k=6,7,8) [L-recalled]
_GK_POW = {1: 1.0, 2: 4.0 / 3.0, 3: 2.0, 4: 4.0, 5: 8.0, 6: 64.0 / 3.0, 7: 64.0, 8: 256.0}


def gamma(k):
    if k in _GK_POW:
        # stored as gamma_k^k except k=2 (gamma_2^2 = 4/3)
        if k == 2:
            return math.sqrt(4.0 / 3.0)
        return _GK_POW[k] ** (1.0 / k)
    # Blichfeldt upper bound gamma_k <= (2/pi) Gamma(2 + k/2)^(2/k) [L-recalled]
    return (2.0 / math.pi) * math.exp(math.lgamma(2.0 + k / 2.0) * 2.0 / k)


def log2_Ck(k, rho):
    """log2 of floor((gamma_k rho)^(k/2)); exact floor when small, real value when huge."""
    if k == 0:
        return 0.0
    v = (k / 2.0) * LOG2(gamma(k) * rho)
    if v < 50:
        return LOG2(max(1, math.floor(2.0 ** v)))
    return v


def log2_N_hermite(r, rho):
    terms = []
    for j in range(1, r + 1):
        lc = log2_Ck(j - 1, rho)
        terms.append(LOG2(math.comb(r, j)) + j * (1.0 + lc))   # binom * (2C)^j
    m = max(terms)
    return m + LOG2(sum(2.0 ** (t - m) for t in terms)) - 1.0   # the 1/2 for +-n


def log2_N_box(r, rho, T):
    lc = B0.log2_C_sharp(r, rho, T)
    # ((2C+1)^r - 1)/2
    return r * LOG2(2.0 * 2.0 ** lc + 1.0) - 1.0


def margin_from_log2N(log2N, bits):
    return bits / 2.0 - (1.0 + log2N)


def rho_max(r, bits, lo=1.0, hi=1e60):
    """largest rho with margin >= 0 under the Hermite cutoff (bisection in log space)."""
    if margin_from_log2N(log2_N_hermite(r, lo), bits) < 0:
        return None
    for _ in range(200):
        mid = math.sqrt(lo * hi)
        if margin_from_log2N(log2_N_hermite(r, mid), bits) >= 0:
            lo = mid
        else:
            hi = mid
    return lo


def crossover_hermite(r, H_of_bits, D_of_bits, bmax=200000):
    def ok(b):
        rho = H_of_bits(b) / B0.lam_petsche(D_of_bits(b))
        return margin_from_log2N(log2_N_hermite(r, rho), b) > 0
    b = 16
    prev = 16
    while b <= bmax:
        if ok(b):
            lo, hi = prev, b
            while hi - lo > 1:
                mid = (lo + hi) // 2
                if ok(mid):
                    hi = mid
                else:
                    lo = mid
            return hi
        prev = b
        b = int(b * 1.1) + 1
    return None


# toy fits (Silverman normalisation, nats) from the R5 lifting lab out/bounds.md (r=2..6)
FITS_R5 = {2: ((-2.60, 0.686), (-57.4, 13.94)), 3: ((-2.82, 0.793), (-64.8, 17.84)),
           4: ((-3.10, 0.941), (-69.3, 22.24)), 5: ((-5.33, 1.448), (-117.7, 36.09)),
           6: ((-9.75, 2.748), (-251.8, 74.94))}


def main(out_json, extra_fits=None):
    res = {"gamma": {k: gamma(k) for k in range(1, 21)}}
    bits = 256
    rhos = [1.065, 1.25, 2.68, 4.0, 10.0, 100.0, 1e3, 1e4]
    rows = []
    for r in range(2, 21):
        row = {"r": r}
        for rho in rhos:
            row[f"herm_{rho:g}"] = margin_from_log2N(log2_N_hermite(r, rho), bits)
            if r <= 9 and rho <= 100:
                for T in (1, 16):
                    try:
                        row[f"box_T{T}_{rho:g}"] = margin_from_log2N(log2_N_box(r, rho, T), bits)
                    except OverflowError:
                        row[f"box_T{T}_{rho:g}"] = None
        row["rho_max_256"] = rho_max(r, 256)
        row["rho_max_128"] = rho_max(r, 128)
        row["rho_max_384"] = rho_max(r, 384)
        rows.append(row)
    res["rows"] = rows
    fits = dict(FITS_R5)
    if extra_fits:
        fits.update(extra_fits)
    unc = []
    for r, ((ha, hb), (da, db)) in sorted(fits.items()):
        Hf = lambda b, ha=ha, hb=hb: ha + hb * b
        Df = lambda b, da=da, db=db: da + db * b
        rho256 = Hf(256) / B0.lam_petsche(Df(256))
        unc.append({"r": r, "H256": Hf(256), "D256": Df(256), "rho_petsche_256": rho256,
                    "margin256_herm": margin_from_log2N(log2_N_hermite(r, rho256), 256),
                    "threshold_bits_herm": crossover_hermite(r, Hf, Df),
                    "rho_max_256": rho_max(r, 256),
                    "lambda_required_256": (Hf(256) / rho_max(r, 256)) if rho_max(r, 256) else None,
                    "lang_c_required_256": (Hf(256) / rho_max(r, 256) / Df(256)) if rho_max(r, 256) else None})
    res["unconditional"] = unc
    json.dump(res, open(out_json, "w"), indent=1)
    return res


if __name__ == "__main__":
    out = sys.argv[1]
    extra = None
    if len(sys.argv) > 2:
        ex = json.load(open(sys.argv[2]))
        extra = {int(k): (tuple(v["H"]), tuple(v["D"])) for k, v in ex.items()}
    r = main(out, extra)
    print("gamma_k:", {k: round(v, 4) for k, v in r["gamma"].items() if k <= 10})
    print("\nmargin at 256 bits (bits above sqrt q; >0 = excluded)")
    for row in r["rows"]:
        s = " ".join(f"{k}={v:7.1f}" if isinstance(v, float) else f"{k}={v}" for k, v in row.items() if k.startswith("herm") or k.startswith("box"))
        print(f"r={row['r']:2d} rho_max256={row['rho_max_256']} rho_max128={row['rho_max_128']} | {s}")
    print("\nunconditional (Petsche) with Hermite cutoff")
    for u in r["unconditional"]:
        print(u)
