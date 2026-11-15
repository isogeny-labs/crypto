#!/usr/bin/env python3
"""Toy audit of the same-field genus-2 embedding obstruction (stdlib only)."""
import json, sys, random, math
def primes_in(lo, hi):
    return [n for n in range(lo, hi) if n > 2 and all(n % d for d in range(2, int(n**.5)+1))]
def nonres(p):
    sq = {x*x % p for x in range(1, p)}
    return next(n for n in range(2, p) if n not in sq)
def count_sextic(co, p):
    """co = [c0..c6]; return (N1, N2) = #C(F_p), #C(F_p^2), C: y^2=f(x)."""
    sq = [0]*p
    for x in range(1, p): sq[x*x % p] = 1
    def chi(v):
        v %= p
        return 0 if v == 0 else (1 if sq[v] else -1)
    f = lambda x: sum(c*pow(x, i, p) for i, c in enumerate(co)) % p
    N1 = sum(1 + chi(f(x)) for x in range(p))
    N1 += 1 + chi(co[6]) if co[6] % p else (1 if co[5] % p else 2)
    # F_{p^2} = F_p[s]/(s^2-n); element (a,b)
    n = nonres(p)
    mul = lambda u, v: ((u[0]*v[0] + n*u[1]*v[1]) % p, (u[0]*v[1] + u[1]*v[0]) % p)
    sqs = set(mul((a, b), (a, b)) for a in range(p) for b in range(p))
    N2 = 0
    for a in range(p):
        for b in range(p):
            x = (a, b); acc = (0, 0); xp = (1, 0)
            for c in co:
                t = mul((c % p, 0), xp); acc = ((acc[0]+t[0]) % p, (acc[1]+t[1]) % p); xp = mul(xp, x)
            N2 += 1 if acc == (0, 0) else (2 if acc in sqs else 0)
    N2 += 2 if co[6] % p else (1 if co[5] % p else 2)
    return N1, N2
def jac(N1, N2, p): return (N1*N1 + N2)//2 - p
def ell(a, b, c, d, p):
    sq = {x*x % p for x in range(1, p)}
    return 1 + sum(1 + (0 if v == 0 else (1 if v in sq else -1))
                   for v in ((a*x**3 + b*x*x + c*x + d) % p for x in range(p)))
def sqfree3(a, b, c, d, p):
    disc = (18*a*b*c*d - 4*b**3*d + b*b*c*c - 4*a*c**3 - 27*a*a*d*d) % p
    return a % p and d % p and disc
def main():
    smoke = '--smoke' in sys.argv
    rng = random.Random(20261110)
    ps = [11, 13] if smoke else primes_in(23, 70)
    per = 3 if smoke else 12
    pos_ok = pos_n = 0; neg_match = neg_n = 0; hasse_viol = 0; ratios = []; rows = []
    for p in ps:
        done = 0
        while done < per:
            a, b, c, d = (rng.randrange(p) for _ in range(4))
            if not sqfree3(a, b, c, d, p) or not sqfree3(d, c, b, a, p): continue
            e1, e2 = ell(a, b, c, d, p), ell(d, c, b, a, p)
            N1, N2 = count_sextic([d, 0, c, 0, b, 0, a], p)  # y^2 = a x^6+b x^4+c x^2+d
            J = jac(N1, N2, p)
            pos_n += 1; pos_ok += (J == e1*e2)
            if e2 < (math.sqrt(p)-1)**2 - 1e-9: hasse_viol += 1
            ratios.append(math.log(max(e2,1))/math.log(max(e1,2)))
            # negative control: generic sextic with odd terms, compare to E1*E2 of same coefficients
            co = [rng.randrange(p) for _ in range(7)]
            if co[6] and co[0]:
                M1, M2 = count_sextic(co, p)
                neg_n += 1; neg_match += (jac(M1, M2, p) == e1*e2)
            done += 1
        rows.append(p)
    pos_rate = pos_ok/pos_n
    neg_rate = neg_match/max(neg_n, 1)
    ok = pos_rate == 1.0 and hasse_viol == 0 and neg_rate < 0.5
    verdict = "consistent_with_prediction" if ok else ("falsified" if pos_rate < 1.0 or hasse_viol else "inconclusive")
    p_big = ps[-1]
    print(json.dumps({"lead": "LEAD-TRF-same-field-genus-g-embedding", "smoke": smoke, "seed": 20261110,
        "primes": rows, "curves_per_prime": per, "positive_control_identity_rate": pos_rate, "n_pos": pos_n,
        "hasse_cofactor_violations": hasse_viol, "negative_control_match_rate": neg_rate, "n_neg": neg_n,
        "mean_log_cofactor_over_log_N": sum(ratios)/len(ratios),
        "cost_model_log_ratio_IC_over_rho_at_p_max": math.log(p_big)/math.log(math.sqrt(p_big)),
        "note": "IC q^(2-2/g) at g=2 is q vs rho sqrt(q): exponent ratio 2 by construction",
        "verdict": verdict}))
main()
