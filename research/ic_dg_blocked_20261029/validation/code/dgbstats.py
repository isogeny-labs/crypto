"""Statistics for the XP-20261029-dg-blocked validator (validator's own code).

Student-t quantile from the regularised incomplete beta function (no scipy on
this host), the per-operation wall ratio with its pair-influence standard
error, an independent-on-the-log-scale combination, a dependence-aware joint
corrected ratio, and a stratified cluster bootstrap.
"""
import math

import numpy as np


def _betacf(a, b, x):
    fpmin, eps = 1e-300, 1e-15
    qab, qap, qam = a + b, a + 1.0, a - 1.0
    c, d = 1.0, 1.0 - qab * x / qap
    d = fpmin if abs(d) < fpmin else d
    d = 1.0 / d
    h = d
    for m in range(1, 1000):
        m2 = 2 * m
        aa = m * (b - m) * x / ((qam + m2) * (a + m2))
        d = 1.0 + aa * d
        d = fpmin if abs(d) < fpmin else d
        c = 1.0 + aa / c
        c = fpmin if abs(c) < fpmin else c
        d = 1.0 / d
        h *= d * c
        aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2))
        d = 1.0 + aa * d
        d = fpmin if abs(d) < fpmin else d
        c = 1.0 + aa / c
        c = fpmin if abs(c) < fpmin else c
        d = 1.0 / d
        de = d * c
        h *= de
        if abs(de - 1.0) < eps:
            break
    return h


def _betai(a, b, x):
    if x <= 0.0:
        return 0.0
    if x >= 1.0:
        return 1.0
    lbt = math.lgamma(a + b) - math.lgamma(a) - math.lgamma(b) + a * math.log(x) + b * math.log1p(-x)
    bt = math.exp(lbt)
    if x < (a + 1.0) / (a + b + 2.0):
        return bt * _betacf(a, b, x) / a
    return 1.0 - bt * _betacf(b, a, 1.0 - x) / b


def t_cdf(t, df):
    x = df / (df + t * t)
    tail = 0.5 * _betai(df / 2.0, 0.5, x)
    return 1.0 - tail if t > 0 else tail


def t975(df):
    lo, hi = 0.0, 64.0
    for _ in range(200):
        mid = 0.5 * (lo + hi)
        if t_cdf(mid, df) < 0.975:
            lo = mid
        else:
            hi = mid
    return 0.5 * (lo + hi)


def self_test():
    table = {1: 12.706204736, 2: 4.302652730, 5: 2.570581836, 10: 2.228138852,
             30: 2.042272456, 60: 2.000297822, 120: 1.979930405}
    worst = max(abs(t975(df) - v) for df, v in table.items())
    assert worst < 1e-6, worst
    return worst


def log_ci(est, se, df):
    if est is None or se is None or df is None or est <= 0:
        return None
    half = t975(df) * se / est
    return [est * math.exp(-half), est * math.exp(half)]


def per_op_ratio(pairs):
    """pairs: list of (a_wall, b_wall, a_gae, b_gae).  Returns the ratio of
    ns per GAE B/A, ns per GAE of each arm, the influence SE and df."""
    k = len(pairs)
    if k < 2:
        return None
    aw = sum(p[0] for p in pairs)
    bw = sum(p[1] for p in pairs)
    ag = sum(p[2] for p in pairs)
    bg = sum(p[3] for p in pairs)
    nsa, nsb = aw / ag, bw / bg
    rho = nsb / nsa
    u = [rho * (p[1] / bw - p[3] / bg - p[0] / aw + p[2] / ag) for p in pairs]
    ub = sum(u) / k
    se = math.sqrt(k / (k - 1) * sum((x - ub) ** 2 for x in u))
    return {"ns_per_gae_a": nsa, "ns_per_gae_b": nsb, "ratio": rho, "se": se,
            "df": k - 1.0, "n": k, "ci95": log_ci(rho, se, k - 1.0),
            "total_wall_ratio": bw / aw}


def divide_independent(n, d):
    s1, s2 = n["se"] / n["ratio"], d["se"] / d["ratio"]
    est = n["ratio"] / d["ratio"]
    v = s1 * s1 + s2 * s2
    df = v * v / (s1 ** 4 / n["df"] + s2 ** 4 / d["df"]) if v > 0 else min(n["df"], d["df"])
    se = est * math.sqrt(v)
    return {"ratio": est, "se": se, "df": df, "ci95": log_ci(est, se, df)}


def joint_corrected(units):
    """Dependence-aware delta method for ((Bw/Bg)/(Aw/Ag)) / ((Cw/Cg)/(A'w/A'g)).
    units: list of dicts with keys dec=(aw,bw,ag,bg) and aa=(aw,bw,ag,bg)
    (sums over the unit's admitted pairs; zeros where the unit has none)."""
    k = len(units)
    S = {}
    for key in ("dec", "aa"):
        S[key] = [sum(u[key][i] for u in units) for i in range(4)]
    dec = (S["dec"][1] / S["dec"][3]) / (S["dec"][0] / S["dec"][2])
    aa = (S["aa"][1] / S["aa"][3]) / (S["aa"][0] / S["aa"][2])
    est = dec / aa
    z = []
    for u in units:
        d, a = u["dec"], u["aa"]
        z.append(d[1] / S["dec"][1] - d[3] / S["dec"][3] - d[0] / S["dec"][0] + d[2] / S["dec"][2]
                 - (a[1] / S["aa"][1] - a[3] / S["aa"][3] - a[0] / S["aa"][0] + a[2] / S["aa"][2]))
    zb = sum(z) / k
    se_log = math.sqrt(k / (k - 1) * sum((x - zb) ** 2 for x in z))
    return {"ratio": est, "se_log": se_log, "df": k - 1.0, "n_units": k,
            "ci95": log_ci(est, est * se_log, k - 1.0)}


def cluster_bootstrap(strata, resamples, seed):
    """strata: list (one per curve) of arrays shape (n_targets, 8) holding per
    target dec (aw,bw,ag,bg) then aa (aw,bw,ag,bg).  Resample targets with
    replacement within each curve; return the corrected-ratio percentiles."""
    rng = np.random.default_rng(seed)
    out = np.empty(resamples)
    for i in range(resamples):
        tot = np.zeros(8)
        for arr in strata:
            idx = rng.integers(0, arr.shape[0], arr.shape[0])
            tot += arr[idx].sum(axis=0)
        dec = (tot[1] / tot[3]) / (tot[0] / tot[2])
        aa = (tot[5] / tot[7]) / (tot[4] / tot[6])
        out[i] = dec / aa
    lo, hi = np.percentile(out, [2.5, 97.5])
    return {"resamples": resamples, "seed": seed, "percentile_ci95": [float(lo), float(hi)],
            "sd": float(out.std(ddof=1)), "median": float(np.median(out))}
