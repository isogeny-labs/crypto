#!/usr/bin/env python3
"""Independent validator recompute for blind-spot item 01 (Gaudry k = 2 on E(F_{p^2})).

Validation gate (docs/VALIDATION_GATE.md, owner rule 2026-10-19), artifact 2
(validator report) and a draft of artifact 3 (rho reference descriptor).

Reads only committed raw data of research/blindspots_20261018/01-fp2-gaudry-k2/raw/
(never raw-aborted/, never replay/), the frozen cards, and git objects for the
instrument-source binding. Standard library only. Deterministic: the bootstrap is
seeded, no clock is read. Writes, next to this file:

    validator-report.json   (status / independent / raw_data / headline, plus detail)
    rho-reference.json      (draft rho_reference descriptor; not VALIDATION.json)

Usage:  python3 -I benchmarks/20261018-blindspots-fp2-gaudry-k2/validator_recompute.py
"""
from __future__ import annotations

import glob
import hashlib
import json
import math
import os
import random
import re
import subprocess
from collections import defaultdict
from decimal import ROUND_HALF_UP, Decimal

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, "..", ".."))
ITEM_REL = "research/blindspots_20261018/01-fp2-gaudry-k2"
ITEM = os.path.join(REPO, ITEM_REL)
RAW = os.path.join(ITEM, "raw")
SRC_BOUND_PATH = "examples/blindspots_01_fp2_gaudry.rs"          # path the cards bind
SRC_NOW = "research/blindspots_20261018/instruments/blindspots_01_fp2_gaudry.rs"
WEIGHTS = {"w0": 0.0, "w0.1": 0.1, "w1": 1.0}
BAND_KEYS = {"w0": "0", "w0.1": "0.1", "w1": "1"}
SIZES = [24, 28, 32, 36, 40]
FOURQ_LOG2_R = 246
# Matched rho per PROTOCOL.md "Decision rule (frozen)".
MATCHED = [
    ("torsion", "ic-a", "rho-neg"), ("torsion", "ic-b", "rho-neg"),
    ("torsion", "ic-c1", "rho-neg"), ("torsion", "ic-c2", "rho-neg"),
    ("prime", "ic-a", "rho-neg"),
    ("gls", "ic-gls-neg", "rho-neg"), ("gls", "ic-gls-psi", "rho-psi"),
    ("gls", "ic-gls-psi", "rho-neg"),   # reported as well by the protocol
]
STRONGEST_MATCHED = [m for m in MATCHED if m != ("gls", "ic-gls-psi", "rho-neg")]


def sha256(path: str) -> str:
    with open(path, "rb") as fh:
        return hashlib.sha256(fh.read()).hexdigest()


def rel_to_evidence(path: str) -> str:
    return os.path.relpath(path, HERE)


READ: dict[str, str] = {}   # abs path -> role


def note(path: str, role: str) -> str:
    READ.setdefault(path, role)
    return path


# --------------------------------------------------------------------------
# 1. Rows and row-level integrity (every derived field recomputed from counts)
# --------------------------------------------------------------------------

def load_rows():
    files = sorted(glob.glob(os.path.join(RAW, "*.jsonl")))
    rows = []
    for f in files:
        note(f, "raw run rows (blindspot01.v1)")
        with open(f) as fh:
            for line in fh:
                if line.strip():
                    r = json.loads(line)
                    r["_file"] = os.path.basename(f)
                    rows.append(r)
    return files, rows


def row_checks(rows):
    issues = []
    for r in rows:
        tag = f"{r['_file']}:{r['arm']}:i{r['idx']}:t{r['target']}"
        if r.get("schema") != "blindspot01.v1":
            issues.append(f"{tag}: schema {r.get('schema')}")
        nt = sum(r["natives"].values())
        if nt != r["native_total"]:
            issues.append(f"{tag}: native_total {r['native_total']} != sum(natives) {nt}")
        d = r["detail"]
        if r["arm"].startswith("rho"):
            g = d["group_adds"] + d["group_doubles"]
            if g != r["gae_priced"]:
                issues.append(f"{tag}: rho gae_priced {r['gae_priced']} != adds+doubles {g}")
        elif r["arm"].startswith("ic"):
            me = sum(v["mult_eq"] for v in d["phases"].values())
            g = me / d["c_add_mult_eq"]
            if abs(g - r["gae_priced"]) > 1e-9 * g:
                issues.append(f"{tag}: ic gae_priced {r['gae_priced']} != sum(phase mult_eq)/c_add {g}")
            if d["c_add_mult_eq"] != d["c_add_muls"] + d["inv_cost"] * d["c_add_invs"]:
                issues.append(f"{tag}: c_add != muls + inv_cost*invs")
            if d["inv_cost"] != 4:
                issues.append(f"{tag}: inv_cost {d['inv_cost']} != 4")
        for w, wv in WEIGHTS.items():
            t = r["gae_priced"] + wv * nt
            if abs(t - r["gae_total_by_weight"][w]) > 1e-9 * max(1.0, t):
                issues.append(f"{tag}: total {w} mismatch")
            s = t / math.sqrt(r["r"])
            if abs(s - r["s_by_weight"][w]) > 1e-9 * max(1.0, s):
                issues.append(f"{tag}: S {w} mismatch")
        if r["n"] != r["r"] * r["cofactor"]:
            issues.append(f"{tag}: n != r*cofactor")
        if abs(math.log2(r["r"]) - r["log2_r"]) > 1e-9:
            issues.append(f"{tag}: log2_r mismatch")
        if bool(r["verified"]) != bool(d.get("planted_matches")):
            issues.append(f"{tag}: verified != planted_matches")
    return issues


# --------------------------------------------------------------------------
# 2. Instances: primality, Hasse, and the automorphism / subfield checks that
#    decide which rho fold is the strongest available (from p and #E only)
# --------------------------------------------------------------------------

def is_prime(n: int) -> bool:
    if n < 2:
        return False
    small = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]
    for q in small:
        if n % q == 0:
            return n == q
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2
        s += 1
    for a in small:   # deterministic for n < 3.3e24
        x = pow(a, d, n)
        if x in (1, n - 1):
            continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1:
                break
        else:
            return False
    return True


def is_square(x: int) -> bool:
    return x >= 0 and math.isqrt(x) ** 2 == x


def instance_checks(rows):
    path = note(os.path.join(RAW, "instances.txt"), "instance list (p, n, cofactor, r)")
    inst = {}
    with open(path) as fh:
        for line in fh:
            if not line.strip():
                continue
            name = line.split()[0]
            kv = dict(t.split("=", 1) for t in line.split()[1:] if "=" in t)
            inst[name] = {k: kv[k] for k in ("p", "n", "cofactor", "r")}
    issues, per = [], {}
    for r in rows:
        e = inst.get(r["instance"])
        if e is None:
            issues.append(f"{r['instance']}: not in instances.txt")
            continue
        for k in ("p", "n", "cofactor", "r"):
            if int(e[k]) != int(r[k]):
                issues.append(f"{r['instance']}: {k} row {r[k]} != instances.txt {e[k]}")
        if r["instance"] in per:
            continue
        p, n, h, rr = r["p"], r["n"], r["cofactor"], r["r"]
        q = p * p
        T = q + 1 - n
        two_sq_base = is_square(T + 2 * p) and (T + 2 * p) <= 4 * p     # base change of E0/F_p
        two_sq_twist = is_square(2 * p - T) and (2 * p - T) <= 4 * p    # quadratic twist (GLS)
        per[r["instance"]] = {
            "family": r["family"], "bits": r["bits"], "p": p, "n": n, "r": rr, "cofactor": h,
            "p_prime": is_prime(p), "r_prime": is_prime(rr), "hasse_ok": abs(T) <= 2 * p,
            "ordinary": T % p != 0,
            # j = 1728 needs 4q - T^2 = 4 b^2; j = 0 needs 4q - T^2 = 3 b^2 (ordinary E, q = p^2)
            "j1728_possible": (4 * q - T * T) % 4 == 0 and is_square((4 * q - T * T) // 4),
            "j0_possible": (4 * q - T * T) % 3 == 0 and is_square((4 * q - T * T) // 3),
            # j in F_p (j != 0, 1728) needs T = t^2 - 2p (base change) or T = 2p - t^2 (twist)
            "j_in_Fp_base_change_possible": two_sq_base,
            "j_in_Fp_twist_possible": two_sq_twist,
        }
    return inst, per, issues


# --------------------------------------------------------------------------
# 3. Cells, gaps, fits, bootstrap
# --------------------------------------------------------------------------

def build_cells(rows):
    cell = defaultdict(lambda: defaultdict(list))
    for r in rows:
        cell[(r["family"], r["bits"], r["arm"])][r["idx"]].append(r)
    return cell


def curve_means(cell, fam, bits, arm, w, field="gae_total_by_weight"):
    c = cell[(fam, bits, arm)]
    return {i: sum(x[field][w] for x in c[i]) / len(c[i]) for i in sorted(c)}


def gap(cell, fam, bits, ic, rho, w, curves=None, variant="B"):
    """B (protocol): ratio of the means over curves of per-curve mean cost (GAE).
    A (sensitivity): ratio of the means over curves of per-curve mean S = cost/sqrt(r)."""
    field = "gae_total_by_weight" if variant == "B" else "s_by_weight"
    a = curve_means(cell, fam, bits, ic, w, field)
    b = curve_means(cell, fam, bits, rho, w, field)
    ids = curves if curves is not None else sorted(a)
    return (sum(a[i] for i in ids) / len(ids)) / (sum(b[i] for i in ids) / len(ids))


def mean_S(cell, fam, bits, arm, w):
    m = curve_means(cell, fam, bits, arm, w, "s_by_weight")
    return sum(m.values()) / len(m)


def ols(xs, ys):
    n = len(xs)
    mx, my = sum(xs) / n, sum(ys) / n
    sxx = sum((x - mx) ** 2 for x in xs)
    b = sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sxx
    return b, my - b * mx


def fit(cell, fam, arm, w, sizes):
    xs, ys = [], []
    for bits in sizes:
        for i, rs in sorted(cell[(fam, bits, arm)].items()):
            for x in rs:
                xs.append(math.log2(x["r"]))
                ys.append(math.log2(x["gae_total_by_weight"][w]))
    return ols(xs, ys)


def bootstrap_gap(cell, fam, bits, ic, rho, w, n=2000, seed=20261022, mode="paired"):
    """Clusters are curves; targets stay with their curve. 'paired' resamples one set of
    curves for both arms; 'independent' resamples the IC and rho curves separately."""
    rng = random.Random(f"{seed}:{fam}:{bits}:{ic}:{rho}:{w}:{mode}")
    a = curve_means(cell, fam, bits, ic, w)
    b = curve_means(cell, fam, bits, rho, w)
    ids = sorted(a)
    vals = []
    for _ in range(n):
        s = [rng.choice(ids) for _ in ids]
        s2 = s if mode == "paired" else [rng.choice(ids) for _ in ids]
        vals.append((sum(a[i] for i in s) / len(s)) / (sum(b[i] for i in s2) / len(s2)))
    vals.sort()
    return vals[int(0.025 * n)], vals[int(0.975 * n) - 1]


def decimals_of(s: str) -> int:
    return len(s.split(".")[1]) if "." in s else 0


def rounds_to(value: float, claimed: str) -> bool:
    q = Decimal(1).scaleb(-decimals_of(claimed))
    return Decimal(repr(value)).quantize(q, rounding=ROUND_HALF_UP) == Decimal(claimed)


def half_ulp_rel_tol(claimed: str) -> float:
    return 0.5 * 10 ** (-decimals_of(claimed)) / abs(float(claimed))


# --------------------------------------------------------------------------
# 4. Provenance: cards, receipts, outputs, instrument binding, same process
# --------------------------------------------------------------------------

def parse_arms(spec: str):
    out = {}
    for part in spec.split(","):
        arm, rng_ = part.split(":")
        ends = [int(x) for x in rng_.split("-")]
        lo, hi = ends[0], ends[-1]
        out[arm] = set(range(lo, hi + 1))
    return out


def git_blob_sha256(commit: str, path: str):
    try:
        blob = subprocess.run(["git", "-C", REPO, "show", f"{commit}:{path}"],
                              check=True, capture_output=True).stdout
    except subprocess.CalledProcessError:
        return None
    return hashlib.sha256(blob).hexdigest()


def provenance(rows):
    by_file = defaultdict(list)
    for r in rows:
        by_file[r["_file"]].append(r)
    out, issues, file_to_card, process = [], [], {}, {}
    for i in range(1, 12):
        card = note(os.path.join(ITEM, f"card-{i:02d}.yaml"), "frozen command card (binds, argv)")
        rec_p = note(os.path.join(RAW, f"receipt-card-{i:02d}.json"), "execution receipt")
        rec = json.load(open(rec_p))
        text = open(card).read()
        binds = re.findall(r"path: (\S+), sha256: ([0-9a-f]{64})", text)
        entry = {
            "card": f"card-{i:02d}.yaml", "card_sha256_matches_receipt": sha256(card) == rec["card_sha256"],
            "status": rec["status"], "all_commands_succeeded": all(c["succeeded"] for c in rec["commands"]),
            "commit": rec["commit"], "binds": [{"path": p, "sha256": h} for p, h in binds],
            "instrument_dirty_before_run": any("blindspots_01_fp2_gaudry" in x for x in rec["dirty_before"]),
            "outputs": {},
        }
        for p, h in binds:
            got = git_blob_sha256(rec["commit"], p)
            entry["bound_source_sha256_at_commit"] = got
            entry["bound_source_matches"] = got == h
        proto_now = sha256(note(os.path.join(ITEM, "PROTOCOL.md"), "frozen protocol (definitions, decision rule)"))
        entry["protocol_at_commit_equals_committed_protocol"] = (
            git_blob_sha256(rec["commit"], f"{ITEM_REL}/PROTOCOL.md") == proto_now)
        for c in rec["commands"]:
            for p, h in (c.get("outputs") or {}).items():
                ap = os.path.join(REPO, p)
                ok = os.path.isfile(ap) and sha256(ap) == h
                entry["outputs"][os.path.basename(p)] = ok
                file_to_card[os.path.basename(p)] = i
                argv = c["argv"]
                if "--arms" in argv:
                    arms = parse_arms(argv[argv.index("--arms") + 1])
                    got = defaultdict(set)
                    for r in by_file.get(os.path.basename(p), []):
                        got[r["arm"]].add(r["target"])
                    ic = sorted(a for a in arms if a.startswith("ic"))
                    rho = sorted(a for a in arms if a.startswith("rho"))
                    process[os.path.basename(p)] = {
                        "card": i, "command_id": c["id"], "ic_arms": ic, "rho_arms": rho,
                        "same_invocation": bool(ic) and bool(rho),
                        "rows_match_argv": {a: sorted(t) for a, t in got.items()} == {a: sorted(t) for a, t in arms.items()},
                        "binary": argv[2] if argv[0] == "prlimit" else argv[0],
                    }
        out.append(entry)
        for k in ("card_sha256_matches_receipt", "all_commands_succeeded", "bound_source_matches",
                  "protocol_at_commit_equals_committed_protocol"):
            if entry.get(k) is not True:
                issues.append(f"card {i:02d}: {k} is {entry.get(k)}")
        if entry["status"] != "completed":
            issues.append(f"card {i:02d}: status {entry['status']}")
        if entry["instrument_dirty_before_run"]:
            issues.append(f"card {i:02d}: instrument dirty before run")
        for f, ok in entry["outputs"].items():
            if not ok:
                issues.append(f"card {i:02d}: output {f} hash mismatch")
    for f in by_file:
        if f not in file_to_card:
            issues.append(f"{f}: no receipt names it as an output")
    for f, pr in process.items():
        if not pr["rows_match_argv"]:
            issues.append(f"{f}: rows do not match the command's --arms")
    src_now = os.path.join(REPO, SRC_NOW)
    note(src_now, "instrument source (read only to settle unit questions; not a number source)")
    return out, process, issues, sha256(src_now)


# --------------------------------------------------------------------------
# main
# --------------------------------------------------------------------------

def main():
    files, rows = load_rows()
    row_issues = row_checks(rows)
    inst, per_inst, inst_issues = instance_checks(rows)
    cards, process, prov_issues, src_sha_now = provenance(rows)
    cell = build_cells(rows)

    # ---- positive control, oracle cross-check, coverage
    ic_rows = [r for r in rows if r["arm"].startswith("ic")]
    rate_rows = [r for r in rows if r["arm"] == "control-shuffled-rate"]
    solve_rows = [r for r in rows if r["arm"] != "control-shuffled-rate"]
    K = {28: 400, 32: 60, 36: 30, 40: 10}
    k_dev = []
    for r in ic_rows:
        if r["arm"] in ("ic-a", "ic-b", "ic-gls-neg", "ic-gls-psi"):
            d = r["detail"]
            want = d["oracle_calls"] if r["bits"] == 24 else min(K[r["bits"]], d["oracle_calls"])
            if d["crosscheck_trials"] != want:
                k_dev.append(f"{r['_file']}:{r['arm']}:t{r['target']}")
    coverage_issues = []
    for (fam, arms_ic, rhos, ncurves) in [("torsion", ["ic-a", "ic-b", "ic-c1", "ic-c2"], ["rho-neg"], 6),
                                          ("prime", ["ic-a"], ["rho-neg"], 3),
                                          ("gls", ["ic-gls-neg", "ic-gls-psi"], ["rho-neg", "rho-psi"], 3)]:
        for bits in SIZES:
            for arm in arms_ic + rhos:
                c = cell[(fam, bits, arm)]
                want_t = {0, 1} if arm.startswith("ic") else set(range(8))
                if sorted(c) != list(range(ncurves)) or any({x["target"] for x in c[i]} != want_t for i in c):
                    coverage_issues.append(f"{fam} b{bits} {arm}: curves {sorted(c)}")
    unverified = [f"{r['_file']}:{r['arm']}:t{r['target']}" for r in solve_rows if not r["verified"]]

    # ---- gaps (protocol definition B) and sensitivity (A)
    gaps = {}
    for fam, ic, rho in MATCHED:
        for bits in SIZES:
            for w in WEIGHTS:
                gaps[(fam, ic, rho, bits, w)] = {
                    "B": gap(cell, fam, bits, ic, rho, w, variant="B"),
                    "A": gap(cell, fam, bits, ic, rho, w, variant="A"),
                }
    band_order_all = all(v["B"] > 1 and v["A"] > 1 for v in gaps.values())
    best40 = {w: min(((gaps[(f, i, r, 40, w)]["B"], f"{f}/{i} vs {r}") for f, i, r in STRONGEST_MATCHED))
              for w in WEIGHTS}
    smallest_any = min(((v["B"], f"{k[0]}/{k[1]} vs {k[2]} b{k[3]} {k[4]}") for k, v in gaps.items()
                        if (k[0], k[1], k[2]) in STRONGEST_MATCHED))

    def G(f, i, r, b, w):
        return gaps[(f, i, r, b, w)]["B"]

    # ---- headline (gated). Claimed strings copied from the item README (Verdict
    # and Numbers sections); the set was named by the task card before recompute.
    head_spec = [
        ("torsion ic-b / matched rho-neg, log2#E=40, w=0 (gap = ratio of curve means of GAE)", "50.0", G("torsion", "ic-b", "rho-neg", 40, "w0")),
        ("torsion ic-c2 / matched rho-neg, log2#E=40, w=0", "51.6", G("torsion", "ic-c2", "rho-neg", 40, "w0")),
        ("torsion ic-b / matched rho-neg, log2#E=40, w=1", "22.2", G("torsion", "ic-b", "rho-neg", 40, "w1")),
        ("torsion ic-c2 / matched rho-neg, log2#E=40, w=1", "14.1", G("torsion", "ic-c2", "rho-neg", 40, "w1")),
        (f"best ratio at log2#E=40, w=0 (min over IC arms vs matched, strongest rho; {best40['w0'][1]})", "50.0", best40["w0"][0]),
        (f"best ratio at log2#E=40, w=1 ({best40['w1'][1]})", "14.1", best40["w1"][0]),
        (f"best ratio at any size and weight ({smallest_any[1]})", "9.9", smallest_any[0]),
    ]
    headline, head_ok = [], True
    for name, claimed, rec in head_spec:
        ok = rounds_to(rec, claimed)
        head_ok &= ok
        headline.append({"name": name, "claimed": float(claimed), "recomputed": rec,
                         "rel_tol": half_ulp_rel_tol(claimed), "rounds_to_claimed": ok})

    # ---- supporting checks (README numbers outside the headline; reported, not gated)
    sup = []

    def chk(name, claimed, rec, kind="round"):
        if kind == "round":
            ok = rounds_to(rec, claimed)
        elif kind == "exact":
            ok = rec == claimed
        else:
            ok = kind
        sup.append({"name": name, "claimed": claimed, "recomputed": rec, "agrees": bool(ok)})

    S_claim = {"ic-a": ["233", "239", "275", "306", "337"], "ic-b": ["40.5", "39.7", "50.8", "42.1", "44.0"],
               "ic-c1": ["65.4", "75.0", "104", "102", "124"], "ic-c2": ["40.3", "40.1", "51.8", "43.3", "45.4"],
               "rho-neg": ["1.35", "1.14", "1.06", "0.89", "0.88"]}
    for arm, cl in S_claim.items():
        for bits, c in zip(SIZES, cl):
            chk(f"torsion S(w0) {arm} b{bits} (mean over curves of cost/sqrt(r))", c, mean_S(cell, "torsion", bits, arm, "w0"))
    for f, i, r, w, c in [("torsion", "ic-a", "rho-neg", "w0", "384"), ("torsion", "ic-a", "rho-neg", "w1", "116"),
                          ("torsion", "ic-c1", "rho-neg", "w0", "141"), ("torsion", "ic-c1", "rho-neg", "w1", "38.1"),
                          ("prime", "ic-a", "rho-neg", "w0", "187"), ("prime", "ic-a", "rho-neg", "w1", "57.1"),
                          ("gls", "ic-gls-neg", "rho-neg", "w0", "196"), ("gls", "ic-gls-neg", "rho-neg", "w1", "58.3"),
                          ("gls", "ic-gls-psi", "rho-neg", "w0", "104"), ("gls", "ic-gls-psi", "rho-neg", "w1", "32.9"),
                          ("gls", "ic-gls-psi", "rho-psi", "w0", "167"), ("gls", "ic-gls-psi", "rho-psi", "w1", "42.2")]:
        chk(f"gap {f}/{i} vs {r} b40 {w}", c, G(f, i, r, 40, w))
    chk("lever ic-a/ic-b S at b40 w0", "7.7", mean_S(cell, "torsion", 40, "ic-a", "w0") / mean_S(cell, "torsion", 40, "ic-b", "w0"))
    chk("lever ic-a/ic-b S at b40 w1", "5.2", mean_S(cell, "torsion", 40, "ic-a", "w1") / mean_S(cell, "torsion", 40, "ic-b", "w1"))
    chk("ic-c2 better than ic-b at b40 w1 (S ratio)", "1.6", mean_S(cell, "torsion", 40, "ic-b", "w1") / mean_S(cell, "torsion", 40, "ic-c2", "w1"))
    t40 = lambda arm, key: [x for i in cell[("torsion", 40, arm)] for x in cell[("torsion", 40, arm)][i]]
    nat = lambda arm: sum(x["native_total"] for x in t40(arm, 0)) / len(t40(arm, 0))
    filt = lambda arm: sum(x["natives"]["filter_nnz_touches_lower_bound"] for x in t40(arm, 0)) / len(t40(arm, 0))
    chk("README 'filter native touches' ic-b b40, as native_total (M)", "15.9", nat("ic-b") / 1e6)
    chk("README 'filter native touches' ic-c2 b40, as native_total (M)", "2.0", nat("ic-c2") / 1e6)
    sup.append({"name": "filter_nnz_touches_lower_bound alone, b40 mean per instance (ic-b, ic-c2)",
                "claimed": "15.9 M -> 2.0 M (README labels these 'the filter's native touches')",
                "recomputed": [filt("ic-b"), filt("ic-c2")], "agrees": False,
                "note": "label mismatch: 15.9 M and 2.0 M are native_total (all unpriced counters); the filter counter alone falls 14.0 M -> 348"})
    trials = lambda arm: sum(x["detail"]["trials"] for x in t40(arm, 0)) / len(t40(arm, 0))
    chk("ic-c1 trials / ic-b trials at b40 ('about 4 times')", "4", trials("ic-c1") / trials("ic-b"))
    chk("gls ic-gls-neg S(w0) b36", "319", mean_S(cell, "gls", 36, "ic-gls-neg", "w0"))
    chk("gls ic-gls-neg S(w0) b40", "222", mean_S(cell, "gls", 40, "ic-gls-neg", "w0"))
    chk("rows", 945, len(rows), "exact")
    chk("jsonl files", 32, len(files), "exact")
    chk("IC rows verified / IC rows", [330, 330], [sum(r["verified"] for r in ic_rows), len(ic_rows)], "exact")
    chk("solve rows verified / solve rows", [936, 936], [sum(r["verified"] for r in solve_rows), len(solve_rows)], "exact")
    chk("oracle cross-check decisions", 101415, sum(r["detail"].get("crosscheck_trials", 0) for r in ic_rows), "exact")
    chk("oracle cross-check disagreements", 0, sum(r["detail"].get("crosscheck_disagreements", 0) for r in ic_rows), "exact")

    # ---- negative control (shuffled base)
    ctl = []
    for r in rows:
        if not r["arm"].startswith("control"):
            continue
        a = cell[(r["family"], r["bits"], "ic-a")][r["idx"]]
        rate_a = sum(x["detail"]["decompositions_found"] for x in a) / sum(x["detail"]["trials"] for x in a)
        e = {"arm": r["arm"], "bits": r["bits"], "idx": r["idx"], "control_rate": r["detail"]["hit_rate"],
             "control_trials": r["detail"]["trials"], "ic_a_rate": rate_a, "diff": r["detail"]["hit_rate"] - rate_a}
        e["within_0.08"] = abs(e["diff"]) <= 0.08
        if r["arm"] == "control-shuffled-a":
            sa = [x for x in a if x["target"] == r["target"]][0]
            e["S_ctl_over_S_a"] = {w: r["s_by_weight"][w] / sa["s_by_weight"][w] for w in WEIGHTS}
            e["ge_10"] = all(v >= 10 for v in e["S_ctl_over_S_a"].values())
        ctl.append(e)
    full = [e for e in ctl if e["arm"] == "control-shuffled-a"]
    rate = [e for e in ctl if e["arm"] == "control-shuffled-rate"]
    worst = max(rate, key=lambda e: abs(e["diff"]))
    chk("control full-solve |rate diff| max ('within 0.03')", True, max(abs(e["diff"]) for e in full) <= 0.03, "exact")
    chk("control S_ctl/S_a min (w0, target 0)", "11.0", min(e["S_ctl_over_S_a"]["w0"] for e in full))
    chk("control S_ctl/S_a max (w0, target 0)", "52.7", max(e["S_ctl_over_S_a"]["w0"] for e in full))
    chk("rate-only cells within 0.08", 8, sum(e["within_0.08"] for e in rate), "exact")
    chk("worst rate-only cell |diff| (b40 i2)", "0.084", abs(worst["diff"]))
    chk("worst rate-only cell control rate", "0.310", worst["control_rate"])
    chk("worst rate-only cell ic-a rate", "0.394", worst["ic_a_rate"])
    chk("mean rate-only diff", "-0.027", sum(e["diff"] for e in rate) / len(rate))

    # ---- exponents (point estimates; README intervals are bootstrap and not gated)
    a_claim = [("torsion", "ic-a", "0.536", "0.536", "0.526", "0.530"), ("torsion", "ic-b", "0.507", "0.473", "0.484", "0.476"),
               ("torsion", "ic-c1", "0.556", "0.529", "0.546", "0.527"), ("torsion", "ic-c2", "0.510", "0.474", "0.509", "0.476"),
               ("torsion", "rho-neg", "0.456", "0.469", "0.473", "0.473"), ("prime", "ic-a", "0.528", "0.537", "0.506", "0.529"),
               ("prime", "rho-neg", "0.468", "0.477", "0.479", "0.482")]
    for fam, arm, a0, t0, a1, t1 in a_claim:
        chk(f"alpha {fam} {arm} all-size w0", a0, fit(cell, fam, arm, "w0", SIZES)[0])
        chk(f"alpha {fam} {arm} top-3 w0", t0, fit(cell, fam, arm, "w0", SIZES[2:])[0])
        chk(f"alpha {fam} {arm} all-size w1", a1, fit(cell, fam, arm, "w1", SIZES)[0])
        chk(f"alpha {fam} {arm} top-3 w1", t1, fit(cell, fam, arm, "w1", SIZES[2:])[0])
    for fam, arm, a0, t0 in [("gls", "ic-gls-neg", "0.572", "0.495"), ("gls", "ic-gls-psi", "0.567", "0.488"),
                             ("gls", "rho-neg", "0.488", "0.506"), ("gls", "rho-psi", "0.457", "0.445")]:
        chk(f"alpha {fam} {arm} all-size w0", a0, fit(cell, fam, arm, "w0", SIZES)[0])
        chk(f"alpha {fam} {arm} top-3 w0", t0, fit(cell, fam, arm, "w0", SIZES[2:])[0])
    for arm, a1 in [("ic-gls-neg", "0.556"), ("ic-gls-psi", "0.552")]:
        chk(f"alpha gls {arm} all-size w1", a1, fit(cell, "gls", arm, "w1", SIZES)[0])

    # ---- FourQ extrapolation point estimates (labelled extrapolation by the README)
    fq = {}
    for arm in ("ic-b", "ic-c2"):
        for w in ("w0", "w1"):
            for lab, sz in (("all", SIZES), ("top3", SIZES[2:])):
                bi, ai = fit(cell, "torsion", arm, w, sz)
                br, ar = fit(cell, "torsion", "rho-neg", w, sz)
                fq[f"{arm} {w} {lab}"] = (ai - ar) + (bi - br) * FOURQ_LOG2_R
            # rho at its analytic sqrt(pi r / 4) steps times its measured GAE per expected step at b40
            rr = [x for i in cell[("torsion", 40, "rho-neg")] for x in cell[("torsion", 40, "rho-neg")][i]]
            per_step = sum(x["gae_total_by_weight"][w] for x in rr) / sum(x["detail"]["expected_steps"] for x in rr)
            bi, ai = fit(cell, "torsion", arm, w, SIZES)
            rho_log2 = math.log2(math.sqrt(math.pi * 2.0 ** FOURQ_LOG2_R / 4.0) * per_step)
            fq[f"{arm} {w} rho-analytic"] = (ai + bi * FOURQ_LOG2_R) - rho_log2
    rng_ok = lambda vals, lo, hi: rounds_to(min(vals), lo) and rounds_to(max(vals), hi)
    sup.append({"name": "FourQ log2 gap, best arms (ic-b, ic-c2), all-size fit, w0", "claimed": "16.7 to 17.4",
                "recomputed": [fq["ic-b w0 all"], fq["ic-c2 w0 all"]], "agrees": rng_ok([fq["ic-b w0 all"], fq["ic-c2 w0 all"]], "16.7", "17.4")})
    sup.append({"name": "FourQ log2 gap, best arms, all-size fit, w1", "claimed": "6.9 to 11.6",
                "recomputed": [fq["ic-b w1 all"], fq["ic-c2 w1 all"]], "agrees": rng_ok([fq["ic-b w1 all"], fq["ic-c2 w1 all"]], "6.9", "11.6")})
    t3 = [fq[k] for k in fq if k.endswith("top3")]
    sup.append({"name": "FourQ log2 gap, best arms, top-3 fit (w0, w1)", "claimed": "4.7 to 7.0",
                "recomputed": t3, "agrees": rng_ok(t3, "4.7", "7.0")})
    sup.append({"name": "FourQ log2 gap with rho at analytic asymptote, ic-c2 (w1, w0)", "claimed": "5.8 to 7.8",
                "recomputed": [fq["ic-c2 w1 rho-analytic"], fq["ic-c2 w0 rho-analytic"]],
                "agrees": rng_ok([fq["ic-c2 w1 rho-analytic"], fq["ic-c2 w0 rho-analytic"]], "5.8", "7.8")})
    sup.append({"name": "FourQ log2 gap with rho at analytic asymptote, ic-b (w1, w0)", "claimed": "1.0 to 7.2",
                "recomputed": [fq["ic-b w1 rho-analytic"], fq["ic-b w0 rho-analytic"]],
                "agrees": rng_ok([fq["ic-b w1 rho-analytic"], fq["ic-b w0 rho-analytic"]], "1.0", "7.2")})

    # ---- bootstrap intervals (own seeded implementation; Monte Carlo, not gated)
    boot = {}
    for f, i, r, w, cl in [("torsion", "ic-b", "rho-neg", "w0", "42.0 to 60.3"), ("torsion", "ic-c2", "rho-neg", "w0", "43.3 to 62.0"),
                           ("torsion", "ic-b", "rho-neg", "w1", "19.7 to 25.5"), ("torsion", "ic-c2", "rho-neg", "w1", "12.0 to 16.9")]:
        boot[f"{f}/{i} vs {r} b40 {w}"] = {"readme": cl,
                                           "paired_2000": bootstrap_gap(cell, f, 40, i, r, w, mode="paired"),
                                           "independent_2000": bootstrap_gap(cell, f, 40, i, r, w, mode="independent")}
    boot["smallest lower bound at b40, any arm/weight"] = {
        "readme": "12.0",
        "paired_2000": min(bootstrap_gap(cell, f, 40, i, r, w, mode="paired")[0] for f, i, r in STRONGEST_MATCHED for w in WEIGHTS),
        "independent_2000": min(bootstrap_gap(cell, f, 40, i, r, w, mode="independent")[0] for f, i, r in STRONGEST_MATCHED for w in WEIGHTS)}
    boot["note"] = ("Monte Carlo, not gated. The README intervals agree with independent resampling of IC and rho "
                    "curves to within resampling noise; paired resampling (same curves for both arms) is narrower. "
                    "Every lower bound at b40 exceeds 1 under both.")
    boot["all_b40_lower_bounds_above_1"] = all(
        bootstrap_gap(cell, f, 40, i, r, w, mode=m)[0] > 1 for f, i, r in MATCHED for w in WEIGHTS
        for m in ("paired", "independent"))

    # ---- rho sanity and unit facts
    rho_rows = [r for r in rows if r["arm"].startswith("rho")]
    soe = {}
    for fam in ("torsion", "prime", "gls"):
        for arm in ("rho-neg", "rho-psi"):
            for bits in SIZES:
                v = [r["detail"]["steps_over_expected"] for r in rho_rows if (r["family"], r["arm"], r["bits"]) == (fam, arm, bits)]
                if v:
                    soe[f"{fam} {arm} b{bits}"] = sum(v) / len(v)
    tr40 = [x for i in cell[("torsion", 40, "rho-neg")] for x in cell[("torsion", 40, "rho-neg")][i]]
    dbl_share = sum(x["detail"]["group_doubles"] for x in tr40) / sum(x["gae_priced"] for x in tr40)
    neg_canon_free = all(r["detail"]["canonicalisation_fp_mult_eq"] == 0 for r in rho_rows if r["arm"] == "rho-neg")
    psi_rows = [r for r in rho_rows if r["arm"] == "rho-psi"]
    psi_canon_gae_share = sum(r["detail"]["canonicalisation_gae_if_priced_by_count"] for r in psi_rows) / sum(r["gae_priced"] for r in psi_rows)
    psi_gae_per_map = sum(r["detail"]["canonicalisation_gae_if_priced_by_count"] for r in psi_rows) / sum(r["natives"]["psi_maps_uncharged"] for r in psi_rows)
    folds_ok = all(r["detail"]["fold"] == (2 if r["arm"] == "rho-neg" else 4) for r in rho_rows)
    cadd = sorted({r["detail"]["c_add_mult_eq"] for r in rows if "c_add_mult_eq" in r["detail"]})

    # ---- instance facts for "strongest available"
    fam_inst = defaultdict(list)
    for v in per_inst.values():
        fam_inst[v["family"]].append(v)
    inst_summary = {}
    for fam, vs in fam_inst.items():
        inst_summary[fam] = {
            "curves": len(vs), "all_p_prime": all(v["p_prime"] for v in vs), "all_r_prime": all(v["r_prime"] for v in vs),
            "all_hasse_ok": all(v["hasse_ok"] for v in vs), "all_ordinary": all(v["ordinary"] for v in vs),
            "any_j0_or_j1728_possible": any(v["j0_possible"] or v["j1728_possible"] for v in vs),
            "any_j_in_Fp_base_change_possible": any(v["j_in_Fp_base_change_possible"] for v in vs),
            "any_j_in_Fp_twist_possible": any(v["j_in_Fp_twist_possible"] for v in vs),
            "all_j_in_Fp_twist_possible": all(v["j_in_Fp_twist_possible"] for v in vs),
            "j0_or_j1728_not_excluded": sorted(k for k, v in per_inst.items() if v["family"] == fam
                                               and (v["j0_possible"] or v["j1728_possible"])),
        }
    inst_summary["method"] = (
        "From p and #E only (curve coefficients are not in raw/). q = p^2, T = q + 1 - #E. Aut(E) > {+-1} needs "
        "j in {0, 1728}, which for ordinary E needs 4q - T^2 = 3b^2 (j = 0) or 4b^2 (j = 1728). j in F_p (j != 0, 1728) "
        "needs T = t^2 - 2p (base change) or T = 2p - t^2 (quadratic twist, GLS). Both tests are necessary conditions: "
        "a failed test excludes the structure; a passed test does not prove it.")

    # ---- band for the descriptor (torsion, log2#E = 40)
    def cell_mean(arm, w):
        m = curve_means(cell, "torsion", 40, arm, w)
        return sum(m.values()) / len(m)

    band_by_arm = {}
    for arm in ("ic-b", "ic-c2"):
        band_by_arm[arm] = {BAND_KEYS[w]: {"ic": cell_mean(arm, w), "rho": cell_mean("rho-neg", w),
                                           "ratio": cell_mean(arm, w) / cell_mean("rho-neg", w)} for w in WEIGHTS}
    b_vs_c2 = [cell_mean("ic-b", w) < cell_mean("ic-c2", w) for w in WEIGHTS]
    order_ic_vs_rho = all(v["ic"] > v["rho"] for a in band_by_arm.values() for v in a.values())

    same_process_40 = {f: p for f, p in process.items() if f.startswith("torsion-b40")}
    all_same_process = all(p["same_invocation"] and p["rows_match_argv"] for f, p in process.items()
                           if f.split("-")[0] in ("torsion", "prime", "gls"))

    all_issues = row_issues + inst_issues + prov_issues + coverage_issues + [f"unverified solve row {u}" for u in unverified]
    sup_fail = [s["name"] for s in sup if not s["agrees"]]
    status = "pass" if head_ok and not all_issues else "fail"

    raw_data = []
    for p, role in sorted(READ.items()):
        if p.endswith(".rs"):
            continue
        raw_data.append({"path": rel_to_evidence(p), "sha256": sha256(p), "role": role})
    summ = os.path.join(RAW, "summary.json")
    raw_data.append({"path": rel_to_evidence(summ), "sha256": sha256(summ),
                     "role": "producer summary; consulted once, read-only, to classify the prime-family 187 vs 188 discrepancy "
                             "(gaps['prime/ic-a-vs-rho-neg'].w0.per_size['40'].gap); never an input to any recomputed value"})
    readme = os.path.join(ITEM, "README.md")
    raw_data.append({"path": rel_to_evidence(readme), "sha256": sha256(readme), "role": "source of the claimed values"})

    report = {
        "schema": "validation-gate/validator-report/v1",
        "status": status,
        "independent": True,
        "item": ITEM_REL,
        "evidence_directory": "benchmarks/20261018-blindspots-fp2-gaudry-k2",
        "script": "validator_recompute.py",
        "raw_data": raw_data,
        "headline": headline,
        "headline_tolerance_rationale": (
            "The README prints every gap to 3 significant figures (50.0, 51.6, 22.2, 14.1, 9.9). rel_tol is half a unit in "
            "the last printed digit divided by the claimed value, the largest relative error a correctly rounded print can "
            "carry (1.0e-3 for 50.0 up to 5.1e-3 for 9.9). The exact test is rounds_to_claimed: the recomputed value, "
            "rounded half-up to the claim's decimals, equals the claim. Status requires that test, which is never looser "
            "than the gate's rel_tol check."),
        "gap_definition": (
            "PROTOCOL.md Metric: per size, ratio of the mean over curves of per-curve mean total cost (GAE at weight w) of "
            "the IC arm to that of its matched rho on the same curves. The ratio of mean S = cost/sqrt(r) (variant A) is "
            "reported as a sensitivity; it does not reproduce the README (e.g. torsion ic-b b40 w0 = %.4f)." % gaps[("torsion", "ic-b", "rho-neg", 40, "w0")]["A"]),
        "gap_sensitivity_variant_A_b40": {f"{f}/{i} vs {r} {w}": gaps[(f, i, r, 40, w)]["A"] for f, i, r in MATCHED for w in WEIGHTS},
        "gaps_all_cells_B": {f"{f}/{i} vs {r} b{b} {w}": gaps[(f, i, r, b, w)]["B"] for f, i, r in MATCHED for b in SIZES for w in WEIGHTS},
        "band_order_ic_above_rho_every_cell_every_weight": band_order_all,
        "supporting_checks": sup,
        "supporting_checks_disagreeing": sup_fail,
        "bootstrap_intervals_not_gated": boot,
        "integrity": {
            "row_issues": row_issues, "instance_issues": inst_issues, "provenance_issues": prov_issues,
            "coverage_issues": coverage_issues, "unverified_solve_rows": unverified,
            "crosscheck_K_deviations": k_dev, "rows": len(rows), "files": len(files),
        },
        "provenance": {"cards": cards, "process_per_output": process, "instrument_source_sha256_now": src_sha_now,
                       "instrument_source_now": SRC_NOW, "instrument_source_bound_path": SRC_BOUND_PATH},
        "negative_control": ctl,
        "rho_sanity": {"steps_over_expected_cell_means": soe, "torsion_b40_doubling_share_of_rho_gae": dbl_share,
                       "rho_neg_canonicalisation_fp_mult_eq_all_zero": neg_canon_free,
                       "rho_psi_canonicalisation_gae_if_priced_by_count_share": psi_canon_gae_share,
                       "rho_psi_gae_per_psi_map_if_priced_by_count": psi_gae_per_map, "folds_match_arm": folds_ok,
                       "c_add_mult_eq_values": cadd},
        "instances": inst_summary,
    }
    verdict = "passed" if status == "pass" else "failed"
    report["validation_report"] = {
        "id": "VAL-20261022-0cea85",
        "task_id": None,
        "task_note": "Dispatched by the main session for the validation gate; no TASK record was issued.",
        "run_ids": [f"{ITEM_REL}/raw/receipt-card-{i:02d}.json" for i in range(1, 12)],
        "artifact_checks": [
            f"{len(files)} raw/*.jsonl files, {len(rows)} rows; every file is an output named by exactly one receipt and hashes to it",
            "11 receipts: status completed, every command succeeded, card_sha256 equals the committed card",
            f"every card binds {SRC_BOUND_PATH} sha256 {src_sha_now}; the blob at each receipt's commit hashes to it; the "
            "instrument is absent from every dirty_before list",
            f"the bound path no longer exists at HEAD; the same bytes are at {SRC_NOW} (moved by commit 5bfcc5743f)",
            "PROTOCOL.md at every receipt commit is byte-identical to the committed PROTOCOL.md (frozen before card 01 ran)",
            "only raw/ read for numbers; raw-aborted/ and replay/ not read",
        ],
        "metric_recomputations": [
            "per row: native_total = sum(natives); gae_total(w) = gae_priced + w*native_total; S = total/sqrt(r); IC "
            "gae_priced = sum(phase mult_eq)/c_add; rho gae_priced = group_adds + group_doubles; n = h*r; 0 mismatches",
            "gaps for every matched pair, size and weight (ratio of curve means of GAE): headline set reproduces to the printed digit",
            "S table (%d values), exponents (%d point estimates), FourQ extrapolation ranges, lever ratios: %s" % (
                sum(1 for s in sup if s["name"].startswith("torsion S(w0)")),
                sum(1 for s in sup if s["name"].startswith("alpha ")),
                "reproduce" if all(s["agrees"] for s in sup if s["name"].startswith(("torsion S(w0)", "alpha ", "FourQ", "lever")))
                else "see supporting_checks_disagreeing"),
            "prime ic-a / rho-neg b40 w0: README 187, recomputed 187.537 (rounds to 188); the producer's summary.json "
            "holds the same 187.537, so the error is in the README transcription; not a headline",
            "README 'filter native touches 15.9 M to 2.0 M' are native_total (all unpriced counters); the filter counter "
            "alone falls 14.0 M to 348; label error, not a headline",
        ],
        "control_checks": [
            "positive: 330/330 IC rows and 936/936 solve rows recover the planted d (verified == planted_matches)",
            "oracle cross-check: 101,415 decisions, 0 disagreements; K per row matches the protocol (all oracle calls at "
            "24 bits, 400/60/30/10 at 28/32/36/40)",
            "negative (shuffled base): full solves at 24, 28 bits: |rate diff| <= 0.021, S_ctl/S_a 10.99 to 52.68 (>= 10 "
            "at every weight); rate-only 32-40 bits: 8 of 9 within 0.08, b40 i2 misses by 0.0837 (frozen criterion not "
            "met in that cell; disclosed by the producer)",
            "null-object rule (inventor-protocol 3): the rate claim is measured against a same-size random base; no "
            "correlation or bias claim needs a decay check",
        ],
        "heuristic_validation_checks": [
            "not a Dickman-type heuristic-validation run; the pre-registered prediction (E16: the ratio does not fall) "
            "and the decision rule were frozen before any card ran (PROTOCOL.md identical at all run commits)",
            "scale: log2#E 24 to 40, 6/3/3 curves a size, 2 IC and 8 rho targets a curve; FourQ (log2 r = 246) is a "
            "labelled extrapolation, not evidence",
        ],
        "cost_model_checks": [
            "unit declared: GAE; IC = counted F_p mult-eq (inv = 4) / c_add (24 on every row); rho = adds + doubles",
            "optimistic assumptions flagged by the producer: filter pass count is a lower bound (favours IC)",
            "unflagged: rho doublings priced at 1 GAE (1.8 percent of rho GAE at torsion b40; effect on gaps < 0.2 "
            "percent); rho-psi psi maps left unpriced (0.417 GAE a map if priced by count, 41 percent of rho-psi priced "
            "GAE), entering only through the band; favours rho-psi, GLS rows only",
            "memory is not reported next to time (limitation, item 5 of the cost-model checks)",
            "every row is a complete verified solve, so expected cost is the measured mean; no success-probability scaling applies",
            "AGENTS.md 5a: every unpriced counter is listed with its count for every arm, rho included; IC above rho at all "
            "three weights in every cell",
        ],
        "proof_architecture_checks": [],
        "speedup_ladder": "not applicable: no improvement is claimed (REGRESSES at the constant level)",
        "verdict": verdict,
        "limitations": [
            "README misprint (non-headline): prime ic-a / rho-neg b40 w0 printed 187, data give 187.54 (188)",
            "README label error (non-headline): 15.9 M -> 2.0 M are native_total, not the filter's touches",
            "negative control misses its frozen 0.08 rate criterion in 1 of 9 rate-only cells (b40 i2, 0.0837)",
            "binary sha256 not recorded; same binary rests on one process invocation per curve plus the bound source hash",
            "cards bind a path that no longer exists at HEAD (byte-identical file moved); a replay must account for it",
            "README bootstrap intervals use independent resampling of IC and rho curves; paired intervals are narrower",
            "rho steps / sqrt(pi r / 2A) cell means 0.94 to 1.19 in the cards (torsion 24-32 bits about 1.15; 40 bits "
            "0.98), against 1.0 to 1.1 cited from development probes; slow rho at small sizes shrinks the gap there",
            "gls-b24-i0: j = 0 not excluded by the trace test (4p - t^2 = 3*31^2); coefficients not in raw/",
            "no benchmarks/README.md row exists for this item at HEAD; claims taken from the item README",
            "toy scale (log2#E <= 40); gap intervals resample curves, not instruction streams",
            "no independent replay in this report (that is the separate replay certificate)",
        ],
        "artifact_paths": [
            "benchmarks/20261018-blindspots-fp2-gaudry-k2/validator-report.json",
            "benchmarks/20261018-blindspots-fp2-gaudry-k2/rho-reference.json",
            "benchmarks/20261018-blindspots-fp2-gaudry-k2/validator_recompute.py",
        ],
        "review_attestation": {
            "joints_owned": ["headline recompute from raw/", "rho reference descriptor (draft)"],
            "sources_read": [os.path.relpath(p, REPO) for p in sorted(READ)] + [
                f"{ITEM_REL}/raw/summary.json (keys only: top level, gaps['prime/ic-a-vs-rho-neg'], positive_control; "
                "after the validator's own values were computed)",
                f"{ITEM_REL}/README.md", "docs/VALIDATION_GATE.md", "scripts/validation_gate.py (lines 100-230)",
                "AGENTS.md (Standing workflow rules: validation gate; 5 and 5a; curve-structure rules)",
                f"{SRC_NOW} (ranges only: measure_c_add, Ops::gae, curve add/dbl counters, rho, run_rho_arm, load_rows)",
            ],
            "read_sibling_reports": False,
            "sibling_disclosure": ("Replay-card commits by another session (replay/PLAN.md, card-r01..r05, "
                                   "receipt-card-r01.json and replay/raw/*) appeared by file name in git log/status; "
                                   "none was opened."),
            "verdict": verdict,
        },
    }
    with open(os.path.join(HERE, "validator-report.json"), "w") as fh:
        json.dump(report, fh, indent=2, sort_keys=False)
        fh.write("\n")

    chosen = "ic-c2"
    strongest_torsion = (not inst_summary["torsion"]["any_j0_or_j1728_possible"]
                         and not inst_summary["torsion"]["any_j_in_Fp_base_change_possible"]
                         and not inst_summary["torsion"]["any_j_in_Fp_twist_possible"])
    rho_ref = {
        "draft": True,
        "note": "Draft rho_reference descriptor for VALIDATION.json (docs/VALIDATION_GATE.md); not VALIDATION.json itself.",
        "scope": ("Headline cell: torsion family (cofactor 4, 6 curves), log2#E = 40, best IC arms ic-b and ic-c2 against "
                  "matched rho-neg, 2 IC targets and 8 rho targets a curve."),
        "kind": "negation",
        "kind_by_family": {
            "torsion": "negation (rho-neg; Aut = {+-1}, j not in F_p by the trace test)",
            "prime": "negation (rho-neg; Aut = {+-1}, j not in F_p by the trace test)",
            "gls": ("negation_frobenius (rho-psi: classes {+-P, +-psiP}, psi the twisted p-power Frobenius). Only "
                    "ic-gls-psi / rho-psi is against the strongest available reference; the README's GLS rows "
                    "ic-gls-neg / rho-neg and ic-gls-psi / rho-neg are not. j = 0 (an order-6 automorphism, a larger "
                    "fold) is not excluded by the trace test for: %s." % (inst_summary["gls"]["j0_or_j1728_not_excluded"] or "none")),
        },
        "protocol": ("PROTOCOL.md 'Decision rule (frozen)': matched rho of the family, rho-neg (Van Oorschot-Wiener on "
                     "{P, -P}, r-adding walk, 16 jumps below 2^30 and 32 above, distinguished points with "
                     "floor(log2 r / 4) bits, one addition a walk start, short-cycle escape by doubling, answer verified) "
                     "for ic-a, ic-b, ic-c1, ic-c2, ic-gls-neg; rho-psi for ic-gls-psi"),
        "unit": ("GAE (group-addition equivalents). IC: counted F_p multiplication equivalents (inversion = 4) divided by "
                 "c_add, the measured mult-eq of one affine addition on the same curve (24 at inv-cost 4). Rho: counted "
                 "group additions plus doublings, 1 GAE each. Unpriced native operations of every arm priced at w GAE "
                 "each (w = 0, 0.1, 1)."),
        "binary_or_process": (
            "One process per curve: each torsion-b40-i{0..5}.jsonl came from a single invocation of the card-built "
            "binary target/blindspots-01/card-NN/b01 run with --arms 'ic-a:0-1,ic-b:0-1,ic-c1:0-1,ic-c2:0-1,rho-neg:0-7' "
            "(cards 05, 06, 07). Every card built that binary with the same rustc command from "
            f"{SRC_BOUND_PATH} sha256 {src_sha_now}, verified at each receipt's commit; the file now lives byte-identically "
            f"at {SRC_NOW}. The binary's own sha256 was not recorded (each card deletes it)."),
        "matches_protocol": True,
        "strongest_available": bool(strongest_torsion),
        "same_unit": True,
        "same_binary_or_process": bool(all_same_process),
        "gae_band": {k: {"ic": v["ic"], "rho": v["rho"]} for k, v in band_by_arm[chosen].items()},
        "order_holds_across_band": bool(order_ic_vs_rho),
        "band_arm": chosen,
        "band_cost_definition": ("Mean over the 6 torsion curves of the per-curve mean total cost per verified solve, GAE, "
                                 "gae_priced + w * native_total; the ratio of these is the protocol gap."),
        "band_ratio": {k: v["ratio"] for k, v in band_by_arm[chosen].items()},
        "gae_band_by_arm": band_by_arm,
        "best_arm_note": ("ic-b is cheaper than ic-c2 at w = 0 and dearer at w = 0.1 and 1 (ic-b < ic-c2: %s), so by "
                          "AGENTS.md 5a the two are unresolved against each other. ic-c2 is shown because it is best at two "
                          "of three weights and has the smallest ratio in the band; the IC-vs-rho order holds for both "
                          "arms at every weight." % b_vs_c2),
        "evidence": {
            "matches_protocol": "raw rows: arm rho-neg, fold 2, 8 targets on each of the 6 curves, all verified; argv per card",
            "strongest_available": inst_summary["torsion"],
            "same_unit": {"c_add_mult_eq_values": cadd, "rho_doubling_share_b40": dbl_share,
                          "rho_neg_canonicalisation_fp_mult_eq_all_zero": neg_canon_free},
            "same_binary_or_process": same_process_40,
        },
    }
    with open(os.path.join(HERE, "rho-reference.json"), "w") as fh:
        json.dump(rho_ref, fh, indent=2)
        fh.write("\n")

    print(f"status={status} headline_ok={head_ok} issues={len(all_issues)} supporting_disagree={sup_fail}")
    for h in headline:
        print(f"  {h['claimed']:>6} vs {h['recomputed']:.4f}  ok={h['rounds_to_claimed']}  {h['name']}")
    for k, v in band_by_arm[chosen].items():
        print(f"  band w={k}: ic={v['ic']:.6g} rho={v['rho']:.6g} ratio={v['ratio']:.4f}")
    for i in all_issues[:20]:
        print("  ISSUE", i)


if __name__ == "__main__":
    main()
