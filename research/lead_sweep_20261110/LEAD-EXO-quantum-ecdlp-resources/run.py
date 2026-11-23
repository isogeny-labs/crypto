import json, random, sys
def divsteps(f, g):
    delta = 1; steps = 0; swaps = 0
    while g != 0:
        if delta > 0 and (g & 1):
            delta, f, g = 1 - delta, g, (g - f) >> 1; swaps += 1
        elif g & 1:
            delta, f, g = 1 + delta, f, (g + f) >> 1
        else:
            delta, f, g = 1 + delta, f, g >> 1
        steps += 1
    return steps, swaps, abs(f)
def bingcd(a, b):  # a odd; steps of subtract/shift loop (each step = one cswap in CT form)
    steps = 0
    while b != 0:
        if b & 1:
            if a > b: a, b = b, a
            b -= a
        b >>= 1; steps += 1
    return steps, a
def by_bound(n): return (49 * n + 57) // 17
def run(sizes, N, seed):
    rows = []; ok = True
    for n in sizes:
        rng = random.Random(seed * 1000 + n)
        ds = []; ss = []; bs = []; viol = 0; gbad = 0; ctrl_gbad = 0
        for _ in range(N):
            m = rng.getrandbits(n) | (1 << (n - 1)) | 1
            x = rng.getrandbits(n) % m
            st, sw, g = divsteps(m, x); sg = st
            import math
            if g != math.gcd(m, x): gbad += 1
            if st > by_bound(n): viol += 1
            bst, bg = bingcd(m, x)
            if bg != math.gcd(m, x): gbad += 1
            ds.append(st); ss.append(sw / max(st, 1)); bs.append(bst)
        # null: swap fraction under random-bit model of "g odd and delta>0" = 1/2*P(delta>0)
        rows.append(dict(n=n, N=N, div_max=max(ds), div_mean=sum(ds) / N, bin_max=max(bs), bin_mean=sum(bs) / N,
                         by_bound=by_bound(n), bound_violations=viol, gcd_errors=gbad,
                         swap_frac_mean=sum(ss) / N, binary_cswaps_per_step=1.0))
    return rows
def main():
    smoke = "--smoke" in sys.argv
    sizes = [12] if smoke else [12, 14, 16, 18, 20, 22]
    N = 200 if smoke else 20000
    A = run(sizes, N, 1); B = run(sizes, N, 2)  # B = A/A arm (different seed)
    sf = sum(r["swap_frac_mean"] for r in A) / len(A)
    sf2 = sum(r["swap_frac_mean"] for r in B) / len(B)
    viol = sum(r["bound_violations"] + r["gcd_errors"] for r in A + B)
    aa = abs(sf - sf2)
    if viol or aa > 0.01: verdict = "inconclusive" if aa > 0.01 and not viol else "falsified"
    else: verdict = "consistent_with_prediction" if sf < 0.35 else "falsified"
    print(json.dumps(dict(lead="LEAD-EXO-quantum-ecdlp-resources", smoke=smoke, seeds=[1, 2], sizes=sizes, samples=N,
        statistic_swap_fraction=sf, aa_swap_fraction=sf2, aa_diff=aa, threshold=0.35,
        positive_control="gcd equals math.gcd and steps<=BY bound", bound_violations_plus_gcd_errors=viol,
        rows=A, verdict=verdict)))
main()
