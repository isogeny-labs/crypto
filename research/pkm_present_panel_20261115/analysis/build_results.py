#!/usr/bin/env python3
"""Post-run analysis of panel P (PROTOCOL.md v1, sections 4-6): tables, verdicts, figure, report.  [pcc:unverified]

Usage:  build_results.py --results results/<stamp> --out results/<stamp>/analysis [--protocol protocol.json]
                         [--strict-lb] [--seed 20261115]
Reads results/<stamp>/rows.jsonl (written by runner/run_plan.py) and, optionally, env/.  Never reads hidden.json.
Writes RESULTS.md, analysis.json, figures/presentation_work.{svg,png}, REPORT.typ (compiled to PDF only if
`typst` is on PATH; otherwise the fact is stated in RESULTS.md).

Implements section 5 as written; every place where the text leaves a choice is marked CHOICE in the code and
listed in analysis.json["choices"] so that the reader sees it (none of them is resolved silently).
"""
import argparse, hashlib, itertools, json, math, shutil, subprocess, sys
from collections import Counter, defaultdict
from pathlib import Path

import numpy as np

CENS = ("timeout", "as_rlimit", "crash")
STRATA = ("sat", "unsat")
BASE = "chain-tower"
CONTROL = "mono-tower"  # positive control for the statistic at m = 3 (protocol v1.2)
NOT_CAND = (BASE, "aa")
SYM = ("sym-tower", "sym-direct")
READINGS = ("w", "w_plus", "w_solve")
N_BOOT = 20000
MC_N = 200000
EXACT_MAX_N = 20
MIN_PAIRS = 8
ALPHA = 0.05
SEED = 20261115
TAG = "[pcc:unverified]"

CHOICES = [
    "sign-flip statistic: the sum of d_i (equivalently the mean), with +-inf entries imputed as +-K, K = 1 + 2 * sum of the "
    "finite |d_i| (so an infinite pair dominates lexicographically); zeros contribute nothing and are dropped from the "
    "enumeration; p = P(|S_flip| >= |S_obs|) over all 2^n_eff sign vectors.  A median-statistic flip test cannot reach "
    "p <= 0.0125 at n = 10 (flipping any subset of the 4 smallest |d_i| leaves the median unchanged), so it is not used.",
    "ties at exactly 0 count as 0 in the median (section 5 text) and are reported (n_zero); they are dropped from the "
    "sign-flip enumeration (red-team M3).",
    "censoring (B3): candidate censored + baseline completed = +inf; baseline censored + candidate completed = -inf; "
    "both censored = dropped and counted; pairs whose lower bound does not certify the sign are counted "
    "(n_lb_not_exceeding) and kept unless --strict-lb.",
    "a ratio with a zero count on exactly one side is dropped and counted (n_dropped_zero); both zero = 0.",
    "strata are the real labels sat / unsat (section 5); planted instances are controls and are not in C2.  A descriptive "
    "split by algebraic_count == 0 vs > 0 (red-team S9) is added to analysis.json only.",
    "k (Bonferroni) = number of non-baseline, non-aa arms with at least one non-not_applicable native row in the cell "
    "(mono-direct included).",
    "|median| > D_AA uses the largest per-curve D_AA of the cell; within_control_distribution uses per-curve medians against "
    "the per-curve D_AA.",
    "positive control (mono-tower, m = 3) is judged per (cell, stratum): supported_within_scope with a positive median "
    "there; if not detected, that stratum's other verdicts are inconclusive.  At m = 4 no control arm exists: verdicts are "
    "not gated by it and carry positive_control = 'not available at m=4'.",
    "w^+ and w^solve and the msolve proxy enter the verdict by sign of their own pooled medians (nonzero and equal to the "
    "primary sign), not by their own tests.",
    "C1 is semantics-adjusted: tree-tower is compared with the baseline minus tree-missable decompositions; chain-direct/aa "
    "with the baseline; monolithic arms must contain the baseline, their extras must be chain-missable, and their count must "
    "equal the classifier's real_count (the monolithic arm stands in for the non-public classifier set, so for m = 4, "
    "where none runs, a baseline deficiency is only counted as classifier_bound_only).  Exclusions are reported per cell "
    "(c1_semantics_exclusions).  C1 compares the unordered, sign-carrying sets of verified decompositions (a decomposition = a multiset of "
    "(abscissa, sign) elements; ordered arms collapse); sym arms are compared with the distinct-abscissa subset of the "
    "baseline's set.  Planted 'found' = the public planted_abscissae appear among the verified decompositions when the row "
    "carries them, else the list is non-empty (hidden.json is never read).  A null verified list (extraction skipped or "
    "censored, instrument status recorded) makes C1 / planted / negative checks not evaluable for that call.",
    "per-arm variety gate: the protocol gives no per-arm count formula from the public classifier fields, so the gate is "
    "partial: equal fp_rational_solutions within {chain-tower, tree-tower, chain-direct, aa} and within {sym-tower, sym-direct}, and "
    "equality with row['predicted_fp_rational'][arm] when the instrument supplies it.",
    "C3 pairs instances by (curve, label, draw rank) as written; cells of different t carry different target files, so "
    "this pairs instances of the same curve and label that are not the same target.",
]


class AnalysisError(Exception):
    pass


# ------------------------------------------------------------------ statistics
def fnum(x):
    if x is None:
        return None
    if isinstance(x, float) and math.isnan(x):
        return None
    if isinstance(x, float) and math.isinf(x):
        return "+inf" if x > 0 else "-inf"
    return x


def sanitize(o):
    if isinstance(o, dict):
        return {str(k): sanitize(v) for k, v in o.items()}
    if isinstance(o, (list, tuple)):
        return [sanitize(v) for v in o]
    if isinstance(o, (np.floating, np.integer)):
        o = o.item()
    if isinstance(o, np.bool_):
        return bool(o)
    return fnum(o)


def median_inf(vals):
    """Median allowing +-inf; the mean of two middle values +inf and -inf is undefined -> nan."""
    v = sorted(vals)
    n = len(v)
    if n == 0:
        return float("nan")
    if n % 2:
        return float(v[n // 2])
    a, b = v[n // 2 - 1], v[n // 2]
    if math.isinf(a) and math.isinf(b) and a != b:
        return float("nan")
    return float((a + b) / 2)


def signflip_p(d, seed=SEED):
    """Exact paired sign-flip test (see CHOICES[0]).  -> (p, method string, n_eff)."""
    d = [x for x in d if not (isinstance(x, float) and math.isnan(x))]
    nz = [x for x in d if x != 0]
    n = len(nz)
    if n == 0:
        return 1.0, "no nonzero pair", 0
    fin = [abs(x) for x in nz if math.isfinite(x)]
    K = 1.0 + 2.0 * sum(fin)
    v = np.array([(K if x > 0 else -K) if math.isinf(x) else x for x in nz], dtype=float)
    obs = abs(v.sum())
    tol = 1e-9 * max(1.0, float(np.abs(v).sum()))
    if n <= EXACT_MAX_N:
        total = 1 << n
        hit = 0
        bits = np.arange(n, dtype=np.uint32)
        for start in range(0, total, 1 << 16):
            idx = np.arange(start, min(total, start + (1 << 16)), dtype=np.uint32)
            sg = 1.0 - 2.0 * ((idx[:, None] >> bits) & 1)
            hit += int((np.abs(sg @ v) >= obs - tol).sum())
        return hit / total, f"exact enumeration of 2^{n} sign vectors", n
    rng = np.random.default_rng(seed)
    hit = 0
    for _ in range(MC_N // 10000):
        sg = rng.integers(0, 2, (10000, n)) * -2.0 + 1.0
        hit += int((np.abs(sg @ v) >= obs - tol).sum())
    return (1 + hit) / (1 + MC_N), f"Monte Carlo {MC_N} random sign vectors (n_eff={n}>20), seed {seed}, add-one", n


def boot_ci(d, level, seed):
    """Percentile bootstrap of the median, N_BOOT resamples.  -> dict."""
    d = np.array([x for x in d if not (isinstance(x, float) and math.isnan(x))], dtype=float)
    n = len(d)
    if n == 0:
        return {"lo": None, "hi": None, "level": level, "n": 0, "undetermined_share": None, "degenerate": None}
    rng = np.random.default_rng(seed)
    res = np.sort(d[rng.integers(0, n, (N_BOOT, n))], axis=1)
    with np.errstate(invalid="ignore"):
        if n % 2:
            med = res[:, n // 2]
        else:
            med = (res[:, n // 2 - 1] + res[:, n // 2]) / 2
    undet = float(np.isnan(med).mean())
    m = np.sort(med[~np.isnan(med)])
    if len(m) == 0:
        return {"lo": None, "hi": None, "level": level, "n": n, "undetermined_share": 1.0, "degenerate": None}
    a = 1 - level
    lo = float(m[int(math.floor(a / 2 * (len(m) - 1)))])
    hi = float(m[int(math.ceil((1 - a / 2) * (len(m) - 1)))])
    return {"lo": lo, "hi": hi, "level": level, "n": n, "undetermined_share": undet, "degenerate": bool(lo == hi)}


def sub_seed(*parts):
    h = hashlib.sha256(("|".join(map(str, parts)) + f"|{SEED}").encode()).digest()
    return int.from_bytes(h[:8], "big")


# ------------------------------------------------------------------ rows
def key_of(r):
    return (r["cell"], r["curve"], r["instance"], r["arm"], r["engine"])


def load_rows(path):
    rows = {}
    for i, ln in enumerate(Path(path).read_text().splitlines(), 1):
        if not ln.strip():
            continue
        try:
            r = json.loads(ln)
        except json.JSONDecodeError as e:
            raise AnalysisError(f"{path}:{i}: {e}")
        for k in ("cell", "curve", "instance", "arm", "engine", "censored", "label", "m", "t"):
            if k not in r:
                raise AnalysisError(f"{path}:{i}: row lacks '{k}'")
        rows.setdefault(key_of(r), r)
    return rows


def need(r, fields):
    miss = [f for f in fields if r.get(f) is None]
    if miss:
        raise AnalysisError(f"row {key_of(r)} (censored=no) lacks {miss}")


def reading(r, name):
    """-> (kind, x): kind in ok | cens | na | missing."""
    if r is None:
        return "missing", None
    c = r["censored"]
    if c in ("not_applicable", "not_run"):
        return "na", None
    if r["engine"] == "native":
        if c == "no":
            need(r, ["solve_field_multiplications", "extract_field_multiplications",
                     "construction_field_multiplications"])
            s, e, k = (r["solve_field_multiplications"], r["extract_field_multiplications"],
                       r["construction_field_multiplications"])
            return "ok", {"w": s + e, "w_solve": s, "w_plus": s + e + k}[name]
        lo = r.get("w_lo")
        if lo is not None and name == "w_plus":
            lo = lo + (r.get("construction_field_multiplications") or 0)
        return "cens", lo
    if c == "no":
        need(r, ["work_proxy"])
        return "ok", r["work_proxy"]
    return "cens", r.get("work_proxy_lo")


def pair_value(a, b, strict_lb=False):
    """d = log2(a/b) for candidate a, baseline b (kinds from reading()).  -> (d or None, tag)."""
    (ka, xa), (kb, xb) = a, b
    if "missing" in (ka, kb) or "na" in (ka, kb):
        return None, "na"
    if ka == "ok" and kb == "ok":
        if xa == 0 and xb == 0:
            return 0.0, "ok"
        if xa == 0 or xb == 0:
            return None, "dropped_zero"
        return math.log2(xa / xb), "ok"
    if ka == "cens" and kb == "cens":
        return None, "dropped_both_censored"
    if ka == "cens":
        if strict_lb and (xa is None or xa <= xb):
            return None, "dropped_lb"
        return float("inf"), "lb_not_exceeding" if (xa is None or xa <= xb) else "cens_pos"
    if strict_lb and (xb is None or xb <= xa):
        return None, "dropped_lb"
    return float("-inf"), "lb_not_exceeding" if (xb is None or xb <= xa) else "cens_neg"


# ------------------------------------------------------------------ decompositions (C1)
def elem_key(e):
    if isinstance(e, bool):
        raise AnalysisError(f"unrecognised decomposition element {e!r}")
    if isinstance(e, int):
        return (e, None)
    if isinstance(e, (list, tuple)) and len(e) == 2 and all(isinstance(z, int) for z in e):
        return (e[0], e[1])
    if isinstance(e, dict):
        x = e.get("x", e.get("abscissa"))
        if isinstance(x, int):
            return (x, e.get("sign", e.get("s")))
    raise AnalysisError(f"unrecognised decomposition element {e!r}: expected int, [x, sign] or {{x, sign}}")


def norm_set(decs):
    out = set()
    for d in decs:
        if not isinstance(d, (list, tuple)):
            raise AnalysisError(f"unrecognised decomposition {d!r}: expected a list of elements")
        out.add(tuple(sorted(elem_key(e) for e in d)))
    return out


def distinct(dec):
    return len({x for x, _ in dec}) == len(dec)


MONO_ARMS = ("mono-tower", "mono-direct")


def _net_zero(elems):
    net = {}
    for x, sg in elems:
        if sg not in (1, -1):
            raise AnalysisError(f"decomposition element without a +-1 sign: {(x, sg)}")
        net[x] = net.get(x, 0) + sg
    return all(v == 0 for v in net.values())


def chain_missable(dec):
    """True iff every ordering of the summands has an identity prefix sum (prefixes of length 2..m-1, the multiset of
    signed abscissae with equal-x opposite-sign pairs cancelled being empty)."""
    m = len(dec)
    for perm in set(itertools.permutations(dec)):
        if not any(_net_zero(perm[:k]) for k in range(2, m)):
            return False
    return True


def tree_missable(dec):
    """m = 4: every partition of the four summands into two pairs contains a cancelling pair."""
    if len(dec) != 4:
        return False
    for (a, b), (c, d) in (((0, 1), (2, 3)), ((0, 2), (1, 3)), ((0, 3), (1, 2))):
        if not (_net_zero([dec[a], dec[b]]) or _net_zero([dec[c], dec[d]])):
            return False
    return True


# ------------------------------------------------------------------ analysis
class Run:
    def __init__(self, rows, strict_lb=False):
        self.rows = rows
        self.strict_lb = strict_lb
        self.cells = []
        for r in sorted(rows.values(), key=lambda r: r.get("seq", 0)):
            if r["cell"] not in self.cells:
                self.cells.append(r["cell"])
        self.meta = {}
        for r in rows.values():
            self.meta.setdefault(r["cell"], {"m": r["m"], "t": r["t"]})
        self.curves = {c: sorted({r["curve"] for r in rows.values() if r["cell"] == c}) for c in self.cells}
        self.insts = {}
        for r in rows.values():
            self.insts.setdefault((r["cell"], r["curve"]), {})[r["instance"]] = r
        self.arms = {c: [] for c in self.cells}
        for r in sorted(rows.values(), key=lambda r: r.get("seq", 0)):
            if r["arm"] not in self.arms[r["cell"]]:
                self.arms[r["cell"]].append(r["arm"])

    def row(self, cell, curve, inst, arm, eng="native"):
        return self.rows.get((cell, curve, inst, arm, eng))

    def instances(self, cell, curve):
        d = self.insts.get((cell, curve), {})
        return sorted(d, key=lambda n: (d[n]["label"] == "planted", d[n].get("draw_index") or 0, n))

    def cand_arms(self, cell):
        out = []
        for a in self.arms[cell]:
            if a in NOT_CAND:
                continue
            if any(r["arm"] == a and r["engine"] == "native" and r["cell"] == cell
                   and r["censored"] not in ("not_applicable", "not_run") for r in self.rows.values()):
                out.append(a)
        return out

    # ---- controls: C1, classifier, negative, planted, variety
    def controls(self):
        res = {}
        for cell in self.cells:
            cr = {"c1_semantics_exclusions": {"chain_missable": 0, "chain_missable_known_instances": 0, "tree_missable": 0},
                  "classifier_bound_only": 0, "c1_evaluated": 0, "c1_not_evaluable": 0, "c1_failures": [], "classifier_failures": [],
                  "negative_failures": [], "planted_failures": [], "variety_failures": [],
                  "variety_gate": "partial (see choices)", "instances_checked": 0}
            arm_bad = defaultdict(list)
            for curve in self.curves[cell]:
                for inst in self.instances(cell, curve):
                    nat = {a: self.row(cell, curve, inst, a) for a in self.arms[cell]}
                    done = {a: r for a, r in nat.items() if r and r["censored"] == "no"}
                    base = done.get(BASE)
                    any_row = next(r for r in nat.values() if r is not None)
                    label = any_row["label"]
                    rc, rcd = any_row.get("real_count"), any_row.get("real_count_distinct")
                    cr["instances_checked"] += 1
                    where = f"{curve}/{inst}"
                    for a, r in list(done.items()):
                        if "verified_real_decompositions" not in r:
                            raise AnalysisError(f"row {key_of(r)} (censored=no) lacks verified_real_decompositions")
                        if r["verified_real_decompositions"] is None:  # extraction skipped / censored: not evaluable
                            cr["c1_not_evaluable_extraction"] = cr.get("c1_not_evaluable_extraction", 0) + 1
                            del done[a]
                    for a, r in done.items():  # negative control: no false relation
                        if rc == 0 and r["verified_real_decompositions"]:
                            cr["negative_failures"].append(f"{where} {a}: {len(r['verified_real_decompositions'])} "
                                                           f"verified on a real-UNSAT instance")
                        pab = r.get("planted_abscissae")
                        if label == "planted" and pab is not None:
                            want = sorted(pab)
                            found = any(sorted(x for x, _ in d) == want for d in norm_set(r["verified_real_decompositions"]))
                        else:
                            found = bool(r["verified_real_decompositions"])
                        if label == "planted" and not found:
                            cr["planted_failures"].append(f"{where} {a}: planted decomposition not found")
                            arm_bad[a].append(f"planted not found on {where}")
                    if base is None:
                        cr["c1_not_evaluable"] += 1
                        continue
                    sb = norm_set(base["verified_real_decompositions"])
                    sbd = {d for d in sb if distinct(d)}
                    mono = [done[x] for x in MONO_ARMS if x in done]
                    M = norm_set(mono[0]["verified_real_decompositions"]) if mono else None
                    # classifier gate.  The classifier's set is not public, only its counts; the monolithic arm (which
                    # represents every decomposition) stands in for it where it ran.  Distinct-abscissa decompositions are
                    # never chain-missable (a missable one contains a cancelling pair), so that count is exact.
                    bad = []
                    if rcd is not None and len(sbd) != rcd:
                        bad.append(f"distinct {len(sbd)} vs classifier {rcd}")
                    if M is not None:
                        nm = sum(chain_missable(d) for d in M)
                        if rc is not None and len(M) != rc:
                            bad.append(f"monolithic arm {len(M)} vs classifier {rc}")
                        if not sb <= M or len(sb) != len(M) - nm:
                            bad.append(f"baseline {len(sb)} vs monolithic {len(M)} minus {nm} chain-missable")
                        cr["c1_semantics_exclusions"]["chain_missable"] += nm
                        cr["c1_semantics_exclusions"]["chain_missable_known_instances"] += 1
                    else:
                        if rc is not None and len(sb) > rc:
                            bad.append(f"baseline {len(sb)} exceeds classifier {rc}")
                        if rc is not None and len(sb) < rc:
                            cr["classifier_bound_only"] += 1  # deficiency cannot be split into missable / bug here
                    if bad:
                        cr["classifier_failures"].append(f"{where}: " + "; ".join(bad))
                    for a, r in done.items():
                        if a == BASE:
                            continue
                        sa = norm_set(r["verified_real_decompositions"])
                        if a in SYM:
                            want, why = sbd, "distinct-abscissa subset of the baseline"
                        elif a == "tree-tower":
                            tm = {d for d in sb if tree_missable(d)}
                            want, why = sb - tm, f"baseline minus {len(tm)} tree-missable"
                            cr["c1_semantics_exclusions"]["tree_missable"] += len(tm)
                        elif a in MONO_ARMS:
                            ok = sb <= sa and all(chain_missable(d) for d in sa - sb) and (M is None or sa == M)
                            want, why = (sa if ok else sb), "baseline plus chain-missable extras (full classifier set)"
                        else:
                            want, why = sb, "baseline"
                        cr["c1_evaluated"] += 1
                        if sa != want:
                            cr["c1_failures"].append(f"{where} {a}: {len(sa)} decompositions vs expected {len(want)} ({why})")
                    for grp in (("chain-tower", "tree-tower", "chain-direct", "aa"), ("sym-tower", "sym-direct")):
                        have = {a: done[a].get("fp_rational_solutions") for a in grp if a in done
                                and done[a].get("fp_rational_solutions") is not None}
                        for a, n in have.items():
                            if n is None:
                                raise AnalysisError(f"{cell}/{where}/{a}: fp_rational_solutions missing")
                        if grp[0] in have and len(set(have.values())) > 1:
                            ref = have[grp[0]]
                            for a, n in have.items():
                                if n != ref and a != "aa" and a != grp[0]:
                                    arm_bad[a].append(f"fp_rational_solutions {n} != {ref} on {where}")
                                    cr["variety_failures"].append(f"{where} {a}: {n} != {ref}")
                    for a, r in done.items():
                        pred = (r.get("predicted_fp_rational") or {}).get(a)
                        if pred is not None and pred != r["fp_rational_solutions"]:
                            arm_bad[a].append(f"fp_rational_solutions {r['fp_rational_solutions']} != predicted {pred} on {where}")
                            cr["variety_failures"].append(f"{where} {a}: predicted {pred}")
            cr["cell_invalid"] = bool(cr["c1_failures"] or cr["classifier_failures"] or cr["negative_failures"])
            cr["arm_invalid"] = dict(arm_bad)
            res[cell] = cr
        return res

    # ---- A/A floor
    def d_aa(self):
        out = {}
        for cell in self.cells:
            for curve in self.curves[cell]:
                ds, nd, bad = [], 0, 0
                for inst in self.instances(cell, curve):
                    d, tag = pair_value(reading(self.row(cell, curve, inst, "aa"), "w"),
                                        reading(self.row(cell, curve, inst, BASE), "w"))
                    if d is None:
                        nd += tag != "na"
                        continue
                    if not math.isfinite(d):
                        bad += 1
                    ds.append(abs(d))
                out[(cell, curve)] = {"D_AA": (max(ds) if ds else None), "n": len(ds), "n_not_evaluable": nd,
                                      "n_censor_mismatch": bad}
        return out

    # ---- C2
    def c2(self, ctl, daa):
        verdicts = []
        for cell in self.cells:
            cands = self.cand_arms(cell)
            k = max(1, len(cands))
            m = self.meta[cell]["m"]
            level = 1 - ALPHA / k
            for arm in cands:
                for st in STRATA:
                    verdicts.append(self.one(cell, arm, st, k, level, ctl[cell], daa, m))
        # positive control gating (m = 3): second pass
        det = {}
        for v in verdicts:
            if v["arm"] == CONTROL:
                det[(v["cell"], v["stratum"])] = (v["supported"] and v["median"] is not None and v["median"] > 0)
        for v in verdicts:
            c = v["cell"]
            if self.meta[c]["m"] == 3 and (CONTROL in self.arms[c]):
                v["positive_control"] = "detected" if det.get((c, v["stratum"])) else "NOT detected"
                if v["arm"] != CONTROL and not det.get((c, v["stratum"])):
                    if v["verdict"] in ("supported_within_scope", "within_control_distribution"):
                        v["reasons"].append("positive control (mono-tower) not detected in this cell/stratum")
                        v["verdict"] = "inconclusive"
            else:
                v["positive_control"] = f"not available at m={self.meta[c]['m']}"
        for v in verdicts:
            if v["arm"] == CONTROL and v["verdict"] == "supported_within_scope" and v["median"] <= 0:
                v["verdict"] = "inconclusive"
                v["reasons"].append("supported with the wrong sign (control expects > 0)")
        return verdicts

    def pairs(self, cell, arm, st, name="w"):
        out = []
        for curve in self.curves[cell]:
            for inst in self.instances(cell, curve):
                ref = self.insts[(cell, curve)][inst]
                if ref["label"] != st:
                    continue
                eng = "msolve" if name == "proxy" else "native"
                nm = "proxy" if name == "proxy" else name
                d, tag = pair_value(reading(self.row(cell, curve, inst, arm, eng), nm),
                                    reading(self.row(cell, curve, inst, BASE, eng), nm), self.strict_lb)
                out.append((curve, inst, d, tag))
        return out

    def one(self, cell, arm, st, k, level, ctl, daa, m):
        P = self.pairs(cell, arm, st, "w")
        valid = [(c, i, d) for c, i, d, t in P if d is not None]
        ds = [d for _, _, d in valid]
        tags = Counter(t for _, _, d, t in P)
        v = {"cell": cell, "m": m, "t": self.meta[cell]["t"], "arm": arm, "stratum": st, "k": k,
             "n_instances": len([1 for _, _, _, t in P if t != "na"]), "n_valid": len(valid),
             "n_na": tags.get("na", 0), "n_dropped_both_censored": tags.get("dropped_both_censored", 0),
             "n_dropped_zero": tags.get("dropped_zero", 0) + tags.get("dropped_lb", 0),
             "n_censored_pairs": sum(1 for d in ds if math.isinf(d)), "n_pos_inf": sum(1 for d in ds if d == math.inf),
             "n_neg_inf": sum(1 for d in ds if d == -math.inf), "n_lb_not_exceeding": tags.get("lb_not_exceeding", 0),
             "n_zero": sum(1 for d in ds if d == 0), "reasons": [], "level": level}
        med = median_inf(ds) if ds else float("nan")
        v["median"] = med
        v["median_is_bound"] = bool(math.isinf(med))
        v["ratio_median"] = (2 ** med) if math.isfinite(med) else med
        v["point_estimate"] = bool(ds) and v["n_censored_pairs"] < len(ds) / 2
        pc = {}
        for c in self.curves[cell]:
            dc = [d for cc, _, d in valid if cc == c]
            pc[c] = {"n": len(dc), "median": median_inf(dc) if dc else None}
        v["per_curve"] = pc
        if ds:
            v["p"], v["p_method"], v["n_eff"] = signflip_p(ds, sub_seed(cell, arm, st, "p"))
            v["ci"] = boot_ci(ds, level, sub_seed(cell, arm, st, "ci"))
        else:
            v["p"], v["p_method"], v["n_eff"], v["ci"] = None, "no valid pair", 0, None
        for nm, key in (("w_plus", "median_w_plus"), ("w_solve", "median_w_solve"), ("proxy", "median_msolve_proxy")):
            dd = [d for _, _, d, _ in self.pairs(cell, arm, st, nm) if d is not None]
            v[key] = median_inf(dd) if dd else None
            v["n_" + key[7:]] = len(dd)
        floors = [daa[(cell, c)]["D_AA"] for c in self.curves[cell] if daa[(cell, c)]["D_AA"] is not None]
        v["D_AA"] = max(floors) if floors else None
        v["D_AA_per_curve"] = {c: daa[(cell, c)]["D_AA"] for c in self.curves[cell]}
        sgn = lambda x: None if x is None or (isinstance(x, float) and (math.isnan(x) or x == 0)) else (1 if x > 0 else -1)
        v["supported"] = False
        # ---- verdict (section 5), in this order
        if ctl["cell_invalid"]:
            v["verdict"] = "invalid_measurement"
            v["reasons"].append("C1/classifier/negative control failed in the cell: " + "; ".join(
                (ctl["c1_failures"] + ctl["classifier_failures"] + ctl["negative_failures"])[:3]))
        elif ctl["arm_invalid"].get(arm):
            v["verdict"] = "invalid_measurement"
            v["reasons"].append(f"arm invalid in this cell: {ctl['arm_invalid'][arm][0]}")
        elif len(valid) < MIN_PAIRS:
            v["verdict"] = "inconclusive"
            v["reasons"].append(f"fewer than {MIN_PAIRS} valid pairs ({len(valid)})")
        elif v["D_AA"] is None:
            v["verdict"] = "inconclusive"
            v["reasons"].append("A/A floor not evaluable")
        else:
            curves_with = [c for c in pc if pc[c]["n"]]
            within = all(pc[c]["median"] is not None and abs(pc[c]["median"]) <= (daa[(cell, c)]["D_AA"] or 0.0)
                         for c in curves_with) and bool(curves_with)
            if within:
                v["verdict"] = "within_control_distribution"
                v["reasons"].append("|per-curve median| <= D_AA on every curve")
            else:
                s0 = sgn(med)
                conds = {
                    f"p <= {ALPHA}/{k} = {ALPHA / k:.4g}": v["p"] is not None and v["p"] <= ALPHA / k,
                    "bootstrap interval excludes 0": bool(v["ci"] and v["ci"]["lo"] is not None and
                                                          (v["ci"]["lo"] > 0 or v["ci"]["hi"] < 0)),
                    "per-curve medians share the sign on every curve": s0 is not None and all(
                        sgn(pc[c]["median"]) == s0 for c in curves_with),
                    "|median| > D_AA": abs(med) > v["D_AA"] if not math.isnan(med) else False,
                    "same sign on w^+": s0 is not None and sgn(v["median_w_plus"]) == s0,
                    "same sign on w^solve": s0 is not None and sgn(v["median_w_solve"]) == s0,
                    "msolve proxy median has the same sign": s0 is not None and sgn(v["median_msolve_proxy"]) == s0,
                }
                v["conditions"] = conds
                if all(conds.values()):
                    v["verdict"] = "supported_within_scope"
                    v["supported"] = True
                else:
                    v["verdict"] = "inconclusive"
                    v["reasons"] += [f"failed: {c}" for c, ok in conds.items() if not ok]
        v["direction"] = None if sgn(med) is None else ("higher" if sgn(med) > 0 else "lower")
        return v

    # ---- C3
    def c3(self):
        out = []
        by = {}
        for cell in self.cells:
            by.setdefault(self.meta[cell]["m"], {})[self.meta[cell]["t"]] = cell
        for m, tc in sorted(by.items()):
            for t in sorted(tc):
                if t + 1 not in tc:
                    continue
                c0, c1 = tc[t], tc[t + 1]
                arms = [a for a in self.arms[c0] if a in self.arms[c1] and a != "aa"]
                for arm in arms:
                    for name in ("w", "w_plus", "proxy"):
                        eng = "msolve" if name == "proxy" else "native"
                        nm = "proxy" if name == "proxy" else name
                        recs = []
                        for curve in sorted(set(self.curves[c0]) & set(self.curves[c1])):
                            for label in ("sat", "unsat", "planted"):
                                i0 = [i for i in self.instances(c0, curve) if self.insts[(c0, curve)][i]["label"] == label]
                                i1 = [i for i in self.instances(c1, curve) if self.insts[(c1, curve)][i]["label"] == label]
                                for rank, (a, b) in enumerate(zip(i0, i1)):
                                    ka = reading(self.row(c1, curve, b, arm, eng), nm)
                                    kb = reading(self.row(c0, curve, a, arm, eng), nm)
                                    recs.append((curve, label, rank, secant(ka, kb)))
                        out.append(c3_summary(m, t, arm, name, recs))
        return out


def secant(num, den):
    """log2(num/den) as (kind, value): exact | lower | upper | dropped."""
    (kn, xn), (kd, xd) = num, den
    if "missing" in (kn, kd) or "na" in (kn, kd):
        return ("na", None)
    if kn == "ok" and kd == "ok":
        return ("exact", math.log2(xn / xd)) if xn and xd else ("dropped", None)
    if kn == "cens" and kd == "cens":
        return ("dropped", None)
    if kn == "cens":
        return ("lower", math.log2(xn / xd)) if xn and xd else ("dropped", None)
    return ("upper", math.log2(xn / xd)) if xn and xd else ("dropped", None)


def c3_summary(m, t, arm, name, recs):
    ex = [v for _, _, _, (k, v) in recs if k == "exact"]
    lo = [v for _, _, _, (k, v) in recs if k == "lower"]
    up = [v for _, _, _, (k, v) in recs if k == "upper"]
    allb = ex + lo + up
    per_label = {}
    for lab in ("sat", "unsat", "planted"):
        e = [v for _, l, _, (k, v) in recs if l == lab and k == "exact"]
        per_label[lab] = {"n_exact": len(e), "median": float(np.median(e)) if e else None}
    return {"m": m, "t_from": t, "t_to": t + 1, "arm": arm, "reading": name, "n_pairs": len([1 for r in recs if r[3][0] != "na"]),
            "n_exact": len(ex), "n_lower": len(lo), "n_upper": len(up),
            "n_dropped": len([1 for r in recs if r[3][0] == "dropped"]),
            "gamma_median_exact": float(np.median(ex)) if ex else None,
            "iqr_exact": [float(np.percentile(ex, 25)), float(np.percentile(ex, 75))] if ex else None,
            "gamma_median_with_bounds": float(np.median(allb)) if allb else None,
            "per_label": per_label, "gate": None}


# ------------------------------------------------------------------ output
def fmt(x, nd=3):
    if x is None:
        return "-"
    if isinstance(x, float):
        if math.isnan(x):
            return "undef"
        if math.isinf(x):
            return "+inf" if x > 0 else "-inf"
        return f"{x:.{nd}f}"
    return str(x)


def md_table(head, body):
    s = "| " + " | ".join(head) + " |\n|" + "|".join("---" for _ in head) + "|\n"
    for r in body:
        s += "| " + " | ".join(str(c) for c in r) + " |\n"
    return s


def figure(run, out_dir):
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt
    import matplotlib.ticker
    cols = {"chain-tower": "#1f4e79", "tree-tower": "#e08a00", "chain-direct": "#2a9d8f", "mono-direct": "#c0392b",
            "mono-tower": "#e377c2",
            "sym-tower": "#7b4fa3", "sym-direct": "#8c564b"}
    ms = sorted({v["m"] for v in run.meta.values()})
    fig, axes = plt.subplots(len(ms), 2, figsize=(11, 4.2 * len(ms)), squeeze=False, sharey=False)
    for i, m in enumerate(ms):
        cells = [c for c in run.cells if run.meta[c]["m"] == m]
        for j, st in enumerate(STRATA):
            ax = axes[i][j]
            arms = [a for a in ARM_DRAW if any(a in run.arms[c] for c in cells)]
            for arm in arms:
                xs, ys, cx, cy = [], [], [], []
                for c in sorted(cells, key=lambda c: run.meta[c]["t"]):
                    B = 2 ** run.meta[c]["t"]
                    vals = []
                    for curve in run.curves[c]:
                        for inst in run.instances(c, curve):
                            if run.insts[(c, curve)][inst]["label"] != st:
                                continue
                            k, x = reading(run.row(c, curve, inst, arm), "w")
                            if k == "ok" and x > 0:
                                vals.append(x)
                            elif k == "cens" and x:
                                cx.append(B)
                                cy.append(x)
                    if vals:
                        xs.append(B)
                        ys.append(float(np.median(vals)))
                        ax.scatter([B] * len(vals), vals, s=9, color=cols[arm], alpha=0.35, linewidths=0)
                if xs:
                    ax.plot(xs, ys, "-o", color=cols[arm], ms=4, lw=1.6, label=arm)
                if cx:
                    ax.scatter(cx, cy, marker="^", facecolors="none", edgecolors=cols[arm], s=34, label=f"{arm} (censored, lower bound)")
            # A/A muted
            xs, ys = [], []
            for c in sorted(cells, key=lambda c: run.meta[c]["t"]):
                vals = [reading(run.row(c, cu, inst, "aa"), "w")[1] for cu in run.curves[c] for inst in run.instances(c, cu)
                        if run.insts[(c, cu)][inst]["label"] == st and reading(run.row(c, cu, inst, "aa"), "w")[0] == "ok"]
                vals = [v for v in vals if v]
                if vals:
                    xs.append(2 ** run.meta[c]["t"])
                    ys.append(float(np.median(vals)))
            if xs:
                ax.plot(xs, ys, "--", color="#9a9a9a", lw=1.2, label="A/A (chain-tower rerun)")
            ax.set_xscale("log", base=2)
            ax.set_yscale("log")
            ax.set_xticks(sorted({2 ** run.meta[c]["t"] for c in cells}))
            ax.get_xaxis().set_major_formatter(matplotlib.ticker.ScalarFormatter())
            ax.set_xlabel("base size B = 2^t")
            ax.set_ylabel("work w (field multiplications)")
            ax.set_title(f"m = {m}, real-{st.upper()}")
            ax.grid(True, which="both", alpha=0.2)
            ax.legend(fontsize=6.5, loc="upper left")
    fig.suptitle("Presentation work vs B, median over instances and curves, dots = instances, "
                 "triangles = censored lower bounds  " + TAG, fontsize=9)
    fig.tight_layout()
    d = Path(out_dir) / "figures"
    d.mkdir(parents=True, exist_ok=True)
    fig.savefig(d / "presentation_work.svg")
    fig.savefig(d / "presentation_work.png", dpi=150)
    plt.close(fig)


ARM_DRAW = ["chain-tower", "tree-tower", "chain-direct", "mono-tower", "mono-direct", "sym-tower", "sym-direct"]


def run_summary(run):
    rows = run.rows.values()
    summ = {"rows": len(run.rows), "by_engine_status": {}, "by_cell": {}}
    for eng in ("native", "msolve"):
        summ["by_engine_status"][eng] = dict(Counter(r["censored"] for r in rows if r["engine"] == eng))
    for c in run.cells:
        summ["by_cell"][c] = dict(Counter(r["censored"] for r in rows if r["cell"] == c))
    summ["reruns"] = sum(1 for r in rows if r.get("rerun"))
    summ["rows_with_warnings"] = sum(1 for r in rows if r.get("warnings"))
    return summ


def build_report(run, ctl, daa, v, c3, out, results, protocol, typst_status):
    L = []
    L.append(f"# Panel P results: presentation dependence of the PKM Kummer relation finder {TAG}\n")
    L.append(f"Every number below is `[pcc:unverified]` (no pcc claim filed yet); nothing here reaches a leaderboard or "
             f"`benchmarks/README.md`. Source: `{results}/rows.jsonl`, produced by `runner/run_plan.py`; this file by "
             f"`analysis/build_results.py` (mechanical, no judgement added).\n")
    L.append("## Disclosures\n")
    L.append("- The seed-777 smoke (`research/pkm_present_panel_20261115/smoke/`, `MAXDEG=64`) is **development data**: it was seen "
             "before the freeze, is not part of these rows, and no row here comes from it.")
    L.append("- The metric here (native counted field multiplications of an F_p F4 solve plus extraction, and msolve's "
             "sum of rows x cols x density) is **not comparable** with the F_{p^3} predecessor's matrix cells "
             "(`gag_presentation_matched_20261031`); only the verdict vocabulary is shared (M5).")
    L.append("- Non-improvement sentence (S8): whatever C2 finds, no presentation effect at m <= 4 is a measured improvement of "
             "the index-calculus baseline; the panel changes no leaderboard row and makes no claim about m = 8.")
    L.append("- C3 secants are descriptive; no gate is applied (S7). Section 5a: all arms are priced in one native unit; no "
             "cross-method ranking and no rho comparison.")
    L.append(f"- Typst: {typst_status}\n")
    s = run_summary(run)
    L.append("## Run summary\n")
    L.append(md_table(["engine", "censored / status counts"], [[e, json.dumps(c)] for e, c in s["by_engine_status"].items()]))
    L.append(f"\nReruns after interrupted calls: {s['reruns']}; rows with warnings: {s['rows_with_warnings']}.\n")
    L.append(md_table(["cell", "rows by censored class"], [[c, json.dumps(x)] for c, x in s["by_cell"].items()]))
    hp = Path(results) / "env/hashes.json"
    if hp.is_file():
        L.append("\nHashes recorded by the runner: `" + json.dumps(json.loads(hp.read_text())) + "`\n")
    L.append("\n## Cells and instances\n")
    body = []
    for c in run.cells:
        for cu in run.curves[c]:
            insts = run.insts[(c, cu)].values()
            r0 = next(iter(insts))
            labs = Counter(i["label"] for i in insts)
            body.append([c, cu, r0.get("p"), r0.get("B"), r0.get("B_real"), r0.get("sym_applicable"),
                         labs.get("sat", 0), labs.get("unsat", 0), labs.get("planted", 0)])
    L.append(md_table(["cell", "curve", "p", "B", "B_real", "sym applicable", "SAT", "UNSAT", "planted"], body))
    L.append("\n## Controls (section 6)\n")
    body = []
    for c in run.cells:
        x = ctl[c]
        body.append([c, x["instances_checked"], x["c1_evaluated"], x["c1_not_evaluable"], len(x["c1_failures"]),
                     len(x["classifier_failures"]), len(x["negative_failures"]), len(x["planted_failures"]),
                     len(x["variety_failures"]), "INVALID" if x["cell_invalid"] else "ok",
                     json.dumps(x["c1_semantics_exclusions"])])
    L.append(md_table(["cell", "instances", "C1 arm-instance checks", "C1 not evaluable (baseline censored)", "C1 failures",
                       "classifier gate failures", "false relations (UNSAT)", "planted not found", "variety gate failures (partial)",
                       "cell", "c1_semantics_exclusions"], body))
    for c in run.cells:
        x = ctl[c]
        for kind in ("c1_failures", "classifier_failures", "negative_failures", "planted_failures", "variety_failures"):
            for line in x[kind][:5]:
                L.append(f"- {c} {kind}: {line}")
    L.append("\nLiteral A/A floor D_AA = max_i |log2(w_AA/w_chain)| per (cell, curve):\n")
    L.append(md_table(["cell", "curve", "D_AA", "pairs", "not evaluable", "censor mismatches"],
                      [[c, cu, fmt(daa[(c, cu)]["D_AA"]), daa[(c, cu)]["n"], daa[(c, cu)]["n_not_evaluable"],
                        daa[(c, cu)]["n_censor_mismatch"]] for c in run.cells for cu in run.curves[c]]))
    L.append("\nPositive control for the statistic (mono-tower, m = 3):\n")
    L.append(md_table(["cell", "stratum", "median log2 ratio", "verdict", "control"],
                      [[x["cell"], x["stratum"], fmt(x["median"]), x["verdict"], x["positive_control"]]
                       for x in v if x["arm"] == CONTROL]))
    L.append("\n## C2: median paired log2(w_A / w_chain-tower), pooled over curves (section 5)\n")
    L.append("Rule text. `supported_within_scope`: exact sign-flip p <= 0.05/k, bootstrap interval (20,000 resamples, level "
             "1 - 0.05/k) excludes 0, per-curve medians share the sign on every curve, |median| > D_AA, the same sign on w^+ "
             "and w^solve, and the msolve proxy median has the same sign. `within_control_distribution`: |median| <= D_AA on "
             "every curve. `invalid_measurement`: C1 failed on a completed instance of the cell. `inconclusive`: otherwise, "
             "including fewer than 8 valid pairs, the positive control not detected, or the co-gate disagreeing. Medians "
             "that are +-inf are bounds (a censored pair).\n")
    body = []
    for x in v:
        ci = x["ci"]
        body.append([x["cell"], x["arm"], x["stratum"], x["k"], x["n_valid"],
                     f"{x['n_pos_inf']}/{x['n_neg_inf']}/{x['n_dropped_both_censored']}/{x['n_dropped_zero']}",
                     fmt(x["median"]) + (" (bound)" if x["median_is_bound"] else ""),
                     fmt(x["ratio_median"], 4),
                     f"[{fmt(ci['lo'])}, {fmt(ci['hi'])}]" + (" degenerate" if ci.get("degenerate") else "") if ci else "-",
                     fmt(x["p"], 5), " / ".join(f"{c}:{fmt(p['median'])}" for c, p in x["per_curve"].items()),
                     f"{fmt(x['median_w_plus'])} / {fmt(x['median_w_solve'])} / {fmt(x['median_msolve_proxy'])}",
                     fmt(x["D_AA"]), x["verdict"], "; ".join(x["reasons"][:3])])
    L.append(md_table(["cell", "arm", "stratum", "k", "n valid", "+inf/-inf/both-cens/zero-dropped", "median log2",
                       "ratio 2^median", f"interval", "p", "per-curve medians", "median w^+ / w^solve / msolve proxy",
                       "D_AA", "verdict", "reasons"], body))
    n_tests = sum(1 for x in v if x["p"] is not None)
    L.append(f"\nNumber of verdict records: {len(v)}; with an exact test run: {n_tests}. Family-wise error is **not** controlled "
             f"beyond the per-cell Bonferroni (k per cell); each cell is its own family and the count of supported cells is "
             f"not a test (M2).\n")
    cnt = Counter(x["verdict"] for x in v)
    L.append("Verdict counts: " + json.dumps(dict(cnt)) + "\n")
    L.append("Sign-flip method per record: " + "; ".join(sorted({x["p_method"].split(" (")[0] for x in v if x["p_method"]})) + ".\n")
    L.append("\n### Pre-registered expectations vs outcome\n")
    exp = (protocol or {}).get("expectations_preregistered", {})
    body = []
    for arm, e in exp.items():
        xs = [x for x in v if x["arm"] == arm]
        sup = [x for x in xs if x["verdict"] == "supported_within_scope"]
        body.append([arm, e, len(xs), sum(1 for x in sup if x["direction"] == "lower"),
                     sum(1 for x in sup if x["direction"] == "higher"), len(xs) - len(sup)])
    L.append(md_table(["arm", "expected", "records", "supported lower", "supported higher", "not supported"], body))
    L.append("\n## Work readings per arm (completed calls, median over instances and curves)\n")
    body = []
    for c in run.cells:
        for arm in run.arms[c]:
            for st in STRATA:
                xs = {n: [] for n in READINGS}
                nc = 0
                for cu in run.curves[c]:
                    for inst in run.instances(c, cu):
                        if run.insts[(c, cu)][inst]["label"] != st:
                            continue
                        r = run.row(c, cu, inst, arm)
                        for n in READINGS:
                            kk, x = reading(r, n)
                            if kk == "ok":
                                xs[n].append(x)
                        if r and r["censored"] in CENS:
                            nc += 1
                if xs["w"] or nc:
                    body.append([c, arm, st, len(xs["w"]), nc] + [fmt(float(np.median(xs[n])) if xs[n] else None, 0) for n in READINGS])
    L.append(md_table(["cell", "arm", "stratum", "completed", "censored", "median w", "median w^+", "median w^solve"], body))
    L.append("\n## C3: secant gamma_A(t -> t+1) = median log2(w(t+1)/w(t)), nested bases, paired by (curve, label, draw rank)\n")
    L.append("Descriptive; no gate is applied. Lower/upper = pair is a bound because one side is censored. "
             "Pairing is by draw rank as the protocol says: different t cells have their own target files (see choices).\n")
    L.append(md_table(["m", "t->t+1", "arm", "reading", "n exact", "n lower", "n upper", "dropped", "gamma (exact median)", "IQR",
                       "median incl. bounds"],
                      [[x["m"], f"{x['t_from']}->{x['t_to']}", x["arm"], x["reading"], x["n_exact"], x["n_lower"], x["n_upper"],
                        x["n_dropped"], fmt(x["gamma_median_exact"]),
                        f"[{fmt(x['iqr_exact'][0])}, {fmt(x['iqr_exact'][1])}]" if x["iqr_exact"] else "-",
                        fmt(x["gamma_median_with_bounds"])] for x in c3]))
    L.append("\n## Figure\n\n![presentation work](figures/presentation_work.svg)\n")
    L.append("\n## Not run, stated (limits of scope)\n")
    for n in (protocol or {}).get("not_run", []):
        L.append(f"- {n}")
    L.append("\n## Analysis choices where the protocol text leaves room\n")
    for c in CHOICES:
        L.append(f"- {c}")
    (Path(out) / "RESULTS.md").write_text("\n".join(L) + "\n")


def typ_escape(s):
    return str(s).replace("\\", "\\\\").replace("#", "\\#").replace("$", "\\$").replace("*", "\\*").replace("_", "\\_") \
        .replace("<", "\\<").replace("@", "\\@").replace("`", "'")


def build_typ(run, v, daa, out, protocol):
    t = ['#set document(title: "Panel P: presentation dependence of the PKM relation finder")',
         "#set text(size: 9.5pt)", "#set page(margin: 1.6cm)",
         f"= Panel P results {typ_escape(TAG)}", "",
         "All numbers are unverified assertions. The seed-777 smoke is development data and not part of these rows. "
         "The metric (native counted field multiplications of an F_p F4 solve plus extraction; msolve matrix proxy) is not "
         "comparable with the F_{p^3} predecessor's matrix cells. No presentation effect at m <= 4 is a measured improvement "
         "of the index-calculus baseline; nothing is claimed for m = 8.", "",
         '#figure(image("figures/presentation_work.svg", width: 100%), caption: [Work vs B; triangles are censored lower bounds, grey dashed is the literal A/A.])',
         "", "== C2 verdicts", ""]
    head = ["cell", "arm", "stratum", "n", "median log2", "p", "verdict"]
    t.append(f"#table(columns: {len(head)}, " + ", ".join(f"[*{h}*]" for h in head) + ",")
    for x in v:
        t.append("  " + ", ".join(f"[{typ_escape(c)}]" for c in (x["cell"], x["arm"], x["stratum"], x["n_valid"], fmt(x["median"]),
                                                                  fmt(x["p"], 4), x["verdict"])) + ",")
    t.append(")")
    (Path(out) / "REPORT.typ").write_text("\n".join(t) + "\n")


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--results", type=Path, required=True)
    ap.add_argument("--out", type=Path, required=True)
    ap.add_argument("--protocol", type=Path, default=Path(__file__).resolve().parent.parent / "protocol.json")
    ap.add_argument("--strict-lb", action="store_true")
    a = ap.parse_args(argv)
    rows = load_rows(a.results / "rows.jsonl")
    if not rows:
        raise AnalysisError("rows.jsonl is empty")
    run = Run(rows, a.strict_lb)
    prot = json.loads(a.protocol.read_text()) if a.protocol.is_file() else None
    ctl = run.controls()
    daa = run.d_aa()
    v = run.c2(ctl, daa)
    c3 = run.c3()
    # descriptive S9 split (algebraic_count == 0 vs > 0), arms vs baseline, median only
    desc = []
    for cell in run.cells:
        for arm in run.cand_arms(cell):
            for lab, pred in (("alg0", lambda r: r.get("algebraic_count") == 0), ("alg>0", lambda r: (r.get("algebraic_count") or 0) > 0)):
                ds = []
                for cu in run.curves[cell]:
                    for inst in run.instances(cell, cu):
                        ref = run.insts[(cell, cu)][inst]
                        if ref["label"] == "planted" or not pred(ref):
                            continue
                        d, _ = pair_value(reading(run.row(cell, cu, inst, arm), "w"), reading(run.row(cell, cu, inst, BASE), "w"))
                        if d is not None:
                            ds.append(d)
                desc.append({"cell": cell, "arm": arm, "split": lab, "n": len(ds), "median": median_inf(ds) if ds else None})
    a.out.mkdir(parents=True, exist_ok=True)
    figure(run, a.out)
    typst = shutil.which("typst")
    typst_status = "not on PATH in this container; REPORT.typ was written but not compiled (no REPORT.pdf)"
    build_typ(run, v, daa, a.out, prot)
    if typst:
        r = subprocess.run([typst, "compile", str(a.out / "REPORT.typ"), str(a.out / "REPORT.pdf")], capture_output=True, text=True)
        typst_status = "compiled to REPORT.pdf" if r.returncode == 0 else f"typst compile failed: {r.stderr[-300:]}"
    build_report(run, ctl, daa, v, c3, a.out, a.results, prot, typst_status)
    doc = {"standing": TAG, "n_rows": len(rows), "summary": run_summary(run), "controls": ctl,
           "d_aa": {f"{c}/{cu}": x for (c, cu), x in daa.items()}, "c2": v, "c3": c3, "descriptive_algebraic_split": desc,
           "number_of_verdict_records": len(v), "number_of_exact_tests": sum(1 for x in v if x["p"] is not None),
           "settings": {"n_boot": N_BOOT, "seed": SEED, "alpha": ALPHA, "min_pairs": MIN_PAIRS, "exact_max_n": EXACT_MAX_N,
                        "mc_n": MC_N, "strict_lb": a.strict_lb}, "choices": CHOICES, "typst": typst_status}
    (a.out / "analysis.json").write_text(json.dumps(sanitize(doc), indent=1))
    print(f"analysis: {len(v)} verdict records -> {a.out}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except AnalysisError as e:
        print(f"build_results: {e}", file=sys.stderr)
        sys.exit(3)
