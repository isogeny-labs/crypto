"""Paired statistics: bootstrap CI, exact sign-flip permutation test, Holm correction.

Python prototype under a task-level exception to the no-Python rule (PROTOCOL.md s.1).
"""
from __future__ import annotations

import itertools

import numpy as np


def bootstrap_ci(diffs, n_boot: int = 10000, alpha: float = 0.05, seed: int = 20261026):
    """Percentile CI of the mean of paired differences (resampling pairs with replacement)."""
    d = np.asarray(diffs, dtype=float)
    n = len(d)
    if n == 0:
        return {"n": 0, "mean": None, "lo": None, "hi": None}
    rng = np.random.default_rng(seed)
    idx = rng.integers(0, n, size=(n_boot, n))
    means = d[idx].mean(axis=1)
    lo, hi = np.quantile(means, [alpha / 2, 1 - alpha / 2])
    return {"n": n, "mean": float(d.mean()), "lo": float(lo), "hi": float(hi),
            "sd": float(d.std(ddof=1)) if n > 1 else None, "n_boot": n_boot}


def sign_flip_pvalue(diffs, max_exact_n: int = 20):
    """Two-sided paired permutation (sign-flip) test of mean = 0, statistic |mean|.
    Exact (all 2^n sign vectors) for n <= max_exact_n."""
    d = np.asarray(diffs, dtype=float)
    n = len(d)
    if n == 0:
        return {"p": None, "method": "none", "n": 0}
    if n > max_exact_n:
        raise ValueError("use the sampled variant for large n")
    signs = np.array(list(itertools.product((1.0, -1.0), repeat=n)))
    means = np.abs((signs * d).mean(axis=1))
    obs = abs(d.mean())
    p = float(np.mean(means >= obs - 1e-12))
    return {"p": p, "method": f"exact sign-flip, all {2 ** n} flips", "n": n}


def holm(pvals: list[float]) -> list[float]:
    """Holm-Bonferroni adjusted p-values (same order as the input)."""
    m = len(pvals)
    order = sorted(range(m), key=lambda i: pvals[i])
    adj = [0.0] * m
    running = 0.0
    for rank, i in enumerate(order):
        v = (m - rank) * pvals[i]
        running = max(running, v)
        adj[i] = min(1.0, running)
    return adj


def pooled_stratified_bootstrap(strata, n_boot: int = 10000, alpha: float = 0.05,
                                seed: int = 20261027):
    """Mean over strata (equal weights) of the per-stratum mean difference; seeds are
    resampled within each stratum, strata are fixed."""
    arrs = [np.asarray(s, dtype=float) for s in strata if len(s)]
    if not arrs:
        return {"n_strata": 0}
    rng = np.random.default_rng(seed)
    acc = np.zeros(n_boot)
    for a in arrs:
        idx = rng.integers(0, len(a), size=(n_boot, len(a)))
        acc += a[idx].mean(axis=1)
    acc /= len(arrs)
    lo, hi = np.quantile(acc, [alpha / 2, 1 - alpha / 2])
    return {"n_strata": len(arrs), "mean": float(np.mean([a.mean() for a in arrs])),
            "lo": float(lo), "hi": float(hi), "n_boot": n_boot}
