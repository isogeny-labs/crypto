"""Validator's own statistics (no third-party code): Student t quantile,
paired ratio-of-sums with a linearised (delta-method) standard error, the
independent-SE combination of section 5, and resampling intervals."""
import math
import random


def _betacf(a, b, x):
    MAXIT, EPS, FPMIN = 400, 3e-16, 1e-300
    qab, qap, qam = a + b, a + 1.0, a - 1.0
    c, d = 1.0, 1.0 - qab * x / qap
    d = 1.0 / (d if abs(d) > FPMIN else FPMIN)
    h = d
    for m in range(1, MAXIT + 1):
        m2 = 2 * m
        aa = m * (b - m) * x / ((qam + m2) * (a + m2))
        d = 1.0 + aa * d; d = FPMIN if abs(d) < FPMIN else d
        c = 1.0 + aa / c; c = FPMIN if abs(c) < FPMIN else c
        d = 1.0 / d; h *= d * c
        aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2))
        d = 1.0 + aa * d; d = FPMIN if abs(d) < FPMIN else d
        c = 1.0 + aa / c; c = FPMIN if abs(c) < FPMIN else c
        d = 1.0 / d; de = d * c; h *= de
        if abs(de - 1.0) < EPS:
            break
    return h


def betai(a, b, x):
    if x <= 0:
        return 0.0
    if x >= 1:
        return 1.0
    lbt = math.lgamma(a + b) - math.lgamma(a) - math.lgamma(b) + a * math.log(x) + b * math.log1p(-x)
    bt = math.exp(lbt)
    if x < (a + 1) / (a + b + 2):
        return bt * _betacf(a, b, x) / a
    return 1.0 - bt * _betacf(b, a, 1 - x) / b


def t_cdf(t, nu):
    x = nu / (nu + t * t)
    p = 0.5 * betai(nu / 2.0, 0.5, x)
    return 1 - p if t > 0 else p


def t_ppf(q, nu):
    lo, hi = 0.0, 50.0
    for _ in range(200):
        mid = (lo + hi) / 2
        if t_cdf(mid, nu) < q:
            lo = mid
        else:
            hi = mid
    return (lo + hi) / 2


def mean(xs):
    return sum(xs) / len(xs)


def ratio_of_sums(pairs):
    """pairs: list of (wall_a, gae_a, wall_b, gae_b). Returns
    R = (sum wall_b / sum gae_b) / (sum wall_a / sum gae_a) and the
    linearised standard error of ln R (delta method on the four sums,
    sample variance with n-1, divided by n)."""
    n = len(pairs)
    sa = sum(p[0] for p in pairs); ga = sum(p[1] for p in pairs)
    sb = sum(p[2] for p in pairs); gb = sum(p[3] for p in pairs)
    R = (sb / gb) / (sa / ga)
    ma, mga, mb, mgb = sa / n, ga / n, sb / n, gb / n
    u = [p[2] / mb - p[3] / mgb - p[0] / ma + p[1] / mga for p in pairs]
    mu = mean(u)
    s2 = sum((x - mu) ** 2 for x in u) / (n - 1)
    se = math.sqrt(s2 / n)
    return {"n": n, "ns_per_gae_a": sa / ga, "ns_per_gae_b": sb / gb, "ratio": R,
            "se_log": se, "dof": n - 1, "sum_wall_a": sa, "sum_wall_b": sb,
            "sum_gae_a": ga, "sum_gae_b": gb}


def ci_log(R, se, dof, level=0.95):
    t = t_ppf(0.5 + level / 2, dof)
    return [R * math.exp(-t * se), R * math.exp(t * se)]


def ci_linear(R, se, dof, level=0.95):
    t = t_ppf(0.5 + level / 2, dof)
    return [R * (1 - t * se), R * (1 + t * se)]


def corrected(r, aa, level=0.95):
    """Section 5: corrected = ratio / aa_ratio, the two standard errors of the
    log ratios combined as independent, Welch-Satterthwaite degrees of freedom."""
    C = r["ratio"] / aa["ratio"]
    v1, v2 = r["se_log"] ** 2, aa["se_log"] ** 2
    se = math.sqrt(v1 + v2)
    dof = (v1 + v2) ** 2 / (v1 * v1 / r["dof"] + v2 * v2 / aa["dof"])
    return {"ratio": C, "se_log": se, "dof": dof,
            "ci95_log": ci_log(C, se, dof, level), "ci95_linear": ci_linear(C, se, dof, level)}


def percentile(xs, q):
    xs = sorted(xs)
    k = (len(xs) - 1) * q
    f = math.floor(k); c = math.ceil(k)
    return xs[f] if f == c else xs[f] + (xs[c] - xs[f]) * (k - f)


def cluster_bootstrap(clusters, stat, B=4000, seed=654733):
    """clusters: list of cluster payloads; stat(list_of_payloads) -> float.
    Percentile interval from B resamples of whole clusters with replacement."""
    rng = random.Random(seed)
    n = len(clusters)
    vals = []
    for _ in range(B):
        vals.append(stat([clusters[rng.randrange(n)] for _ in range(n)]))
    return [percentile(vals, 0.025), percentile(vals, 0.975)], vals
