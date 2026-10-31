#!/usr/bin/env python3
"""LEAD-19: pair the raw, projected and subgroup-only rows of an
`ic boundary --base-arms raw,projected,subgroup` run and lay out the
comparison `research/lead19_projected_base/PROTOCOL.md` §8 froze.

usage: lead19_projected_base_compare.py <char2.json> <prime24.json>
       --frozen docs/ic/runs/ic-boundary-ledger-round5-2026-09-22.json
       --out research/lead19_projected_base/analysis.json
       [--md research/lead19_projected_base/analysis.md]

Python lays out recorded values only: every number below is read from
the run records or is a ratio, mean or resample of such numbers.  No
group operation is computed here.

Arms (the record's variant names):
  A   mitm_m2_negfold_walk[_balanced]            the raw base
  A'  A minus the class test (admissibility_adds + admissibility_doubles)
  B   mitm_m2_negfold_walk_projected[_balanced]  the cofactor-projected base
  C   mitm_m2_negfold_walk_subgroup[_balanced]   the subgroup-only base

The primary estimand is the per-repeat paired ratio S(A')/S(B) at the
matched rung, summarised by its geometric mean with a paired bootstrap
(10,000 resamples, seed 20261023) of the mean log-ratio.  The §5a band
re-prices the one unpriced native counter of the comparison (arm C's
`subgroup_trace_tests`) at 0, 0.1 and 1 GAE per operation.
"""
import json
import math
import random
import sys
from collections import OrderedDict, defaultdict

RAW = "mitm_m2_negfold_walk"
AP = "A'"
ARMS = OrderedDict([("A", ""), ("B", "_projected"), ("C", "_subgroup")])
RUNGS = OrderedDict([("std", ""), ("bal", "_balanced")])
PRICES = (0.0, 0.1, 1.0)
UNPRICED = ("subgroup_trace_tests",)
BOOT_RESAMPLES = 10_000
BOOT_SEED = 20261023
# The published rung of each primary row (docs/ic/LEADERBOARD.md Table B)
# and the dossier's class-zero census (LEAD-19 §1), keyed by degree.
PUBLISHED_RUNG = {15: "std", 21: "bal", 27: "bal"}
CENSUS_P1 = {15: 0.50, 21: 0.50, 27: 0.17}
PRIMARY = (15, 21, 27)


def arg(flag, default=None):
    if flag in sys.argv:
        return sys.argv[sys.argv.index(flag) + 1]
    return default


def degree(inst):
    return int(inst["curve"]["field"]["degree"]) if inst["regime"] == "char2" else None


def rows_by_name(inst):
    """Variant rows grouped by name, in repeat order (the ladder appends
    every variant of a repeat before the next repeat)."""
    out = defaultdict(list)
    for v in inst["variants"]:
        out[v["name"]].append(v)
    return out


def unpriced(row):
    return sum(row["factor_base"]["native"].get(k, 0) for k in UNPRICED) + sum(
        row["relations"]["native"].get(k, 0) for k in UNPRICED
    )


def s_at(row, w, sqrt_r, prime_arm=False):
    """S of a row at price `w` per unpriced native operation; for A' the
    class test's group operations are taken out first."""
    gae = row["total_gae"]
    if prime_arm:
        nat = row["factor_base"]["native"]
        gae -= nat.get("admissibility_adds", 0) + nat.get("admissibility_doubles", 0)
    return (gae + w * unpriced(row)) / sqrt_r


def gmean(xs):
    return math.exp(sum(math.log(x) for x in xs) / len(xs))


def bootstrap_ci(ratios, resamples=BOOT_RESAMPLES, seed=BOOT_SEED):
    """Percentile interval of the geometric mean of paired ratios."""
    logs = [math.log(x) for x in ratios]
    rng = random.Random(seed)
    n = len(logs)
    means = []
    for _ in range(resamples):
        means.append(sum(logs[rng.randrange(n)] for _ in range(n)) / n)
    means.sort()
    lo = means[int(0.025 * resamples)]
    hi = means[int(0.975 * resamples) - 1]
    return math.exp(lo), math.exp(hi)


def check_control(inst):
    """Stop check 2: on the prime-order control every arm is bit-identical
    to the raw row of the same repeat, in every counter of every phase."""
    by = rows_by_name(inst)
    problems = []
    for rung, suffix in RUNGS.items():
        raw = by.get(RAW + suffix, [])
        for arm, asuf in list(ARMS.items())[1:]:
            rows = by.get(RAW + asuf + suffix, [])
            if not raw or not rows:
                continue
            if len(rows) != len(raw):
                problems.append(f"{rung}/{arm}: {len(rows)} rows against {len(raw)}")
                continue
            for k, (a, b) in enumerate(zip(raw, rows)):
                for key in ("trials", "relations_found", "total_gae", "rank", "verified"):
                    if a[key] != b[key]:
                        problems.append(f"{rung}/{arm} repeat {k}: {key} {a[key]} vs {b[key]}")
                for phase in ("factor_base", "relations", "linear_algebra", "verify"):
                    if a[phase]["group_ops"] != b[phase]["group_ops"]:
                        problems.append(f"{rung}/{arm} repeat {k}: {phase}.group_ops differ")
                    if a[phase]["native"] != b[phase]["native"]:
                        problems.append(f"{rung}/{arm} repeat {k}: {phase}.native differ")
    return problems


def check_frozen(inst, frozen_inst):
    """Stop check 3: arm A, repeats 0 to 2, equals the frozen round-5
    counters of the same instance."""
    problems = []
    if frozen_inst is None:
        return ["no frozen instance with this curve name"]
    by = rows_by_name(inst)
    fby = rows_by_name(frozen_inst)
    for rung, suffix in RUNGS.items():
        name = RAW + suffix
        rows, frows = by.get(name, []), fby.get(name, [])
        if not frows:
            continue
        for k, f in enumerate(frows):
            if k >= len(rows):
                problems.append(f"{name} repeat {k}: missing in this run")
                continue
            for key in ("trials", "relations_found", "total_gae"):
                if rows[k][key] != f[key]:
                    problems.append(f"{name} repeat {k}: {key} {rows[k][key]} vs frozen {f[key]}")
    return problems


def analyse_instance(inst, frozen_inst):
    n = degree(inst)
    r = inst["r"]
    sqrt_r = math.sqrt(r)
    by = rows_by_name(inst)
    rho = inst["rho_s_mean"]
    out = OrderedDict()
    out["curve"] = inst["curve"]["name"]
    out["degree"] = n
    out["r"] = r
    out["cofactor"] = inst["cofactor"]
    out["rho_s_mean"] = rho
    out["frozen_check"] = check_frozen(inst, frozen_inst)
    census = {}
    for key in ("factor_base", "factor_base_balanced", "factor_base_projected",
                "factor_base_projected_balanced", "factor_base_subgroup",
                "factor_base_subgroup_balanced"):
        c = inst["curve"].get(key)
        if c and "class_census" in c:
            census[key] = {
                "abscissae": c["abscissae"],
                "signed_points": c["signed_points"],
                "class_zero_fraction": c["class_census"]["folded_pairs_class_zero_fraction"],
                "classes": c["class_census"]["classes"],
                "folded_pairs": c["class_census"]["folded_pairs"],
            }
    out["census"] = census
    rungs = OrderedDict()
    for rung, suffix in RUNGS.items():
        names = {arm: RAW + asuf + suffix for arm, asuf in ARMS.items()}
        if not all(by.get(nm) for nm in names.values()):
            continue
        rows = {arm: by[nm] for arm, nm in names.items()}
        reps = min(len(x) for x in rows.values())
        rec = OrderedDict()
        rec["repeats"] = reps
        rec["abscissae"] = {arm: rows[arm][0]["abscissae"] for arm in ARMS}
        rec["signed_points"] = {arm: rows[arm][0]["signed_points"] for arm in ARMS}
        rec["verified"] = all(rows[arm][k]["verified"] for arm in ARMS for k in range(reps))
        # Per-repeat S at the three prices; A' beside A.
        s = OrderedDict()
        for w in PRICES:
            s[str(w)] = OrderedDict(
                (arm, [s_at(rows[arm][k], w, sqrt_r) for k in range(reps)]) for arm in ARMS
            )
            s[str(w)]["A'"] = [s_at(rows["A"][k], w, sqrt_r, prime_arm=True) for k in range(reps)]
        rec["s_per_repeat"] = s
        rec["s_mean"] = OrderedDict(
            (w, OrderedDict((arm, sum(v) / len(v)) for arm, v in arms.items()))
            for w, arms in s.items()
        )
        rec["trials"] = {arm: [rows[arm][k]["trials"] for k in range(reps)] for arm in ARMS}
        rec["relations_found"] = {
            arm: [rows[arm][k]["relations_found"] for k in range(reps)] for arm in ARMS
        }
        pooled = {
            arm: sum(rec["relations_found"][arm]) / max(1, sum(rec["trials"][arm])) for arm in ARMS
        }
        rec["yield_pooled"] = pooled
        rec["yield_ratio_B_over_A"] = pooled["B"] / pooled["A"] if pooled["A"] else None
        rec["yield_ratio_C_over_A"] = pooled["C"] / pooled["A"] if pooled["A"] else None
        rec["yield_ratio_B_over_C"] = pooled["B"] / pooled["C"] if pooled["C"] else None
        rec["yield_over_ceiling_exact_mean"] = {
            arm: sum(rows[arm][k]["yield_over_ceiling_exact"] for k in range(reps)) / reps
            for arm in ARMS
        }
        rec["class_test_gae_removed_per_repeat"] = [
            rows["A"][k]["factor_base"]["native"].get("admissibility_adds", 0)
            + rows["A"][k]["factor_base"]["native"].get("admissibility_doubles", 0)
            for k in range(reps)
        ]
        # Paired ratios at each price, with the bootstrap at w = 0.
        ratios = OrderedDict()
        for w in PRICES:
            sw = s[str(w)]
            for arm in ("B", "C"):
                pr = [sw["A'"][k] / sw[arm][k] for k in range(reps)]
                entry = OrderedDict(per_repeat=pr, geometric_mean=gmean(pr))
                if w == 0.0:
                    entry["bootstrap_95"] = bootstrap_ci(pr)
                ratios[f"A'/{arm}@{w}"] = entry
                pa = [sw["A"][k] / sw[arm][k] for k in range(reps)]
                ratios[f"A/{arm}@{w}"] = OrderedDict(per_repeat=pa, geometric_mean=gmean(pa))
        rec["paired_ratios"] = ratios
        # Distance to the matched rho at the three prices (rho has no
        # unpriced native work, so its S is the same at every price).
        rec["gap_to_rho"] = OrderedDict(
            (w, OrderedDict((arm, rec["s_mean"][w][arm] / rho) for arm in ("A", "A'", "B", "C")))
            for w in rec["s_mean"]
        )
        # Does the A/B and A/C order hold across the band?
        rec["order_holds_across_band"] = {
            arm: all(
                (rec["s_mean"][str(w)]["A'"] > rec["s_mean"][str(w)][arm])
                == (rec["s_mean"]["0.0"]["A'"] > rec["s_mean"]["0.0"][arm])
                for w in PRICES
            )
            for arm in ("B", "C")
        }
        rungs[rung] = rec
    out["rungs"] = rungs
    # Best rung per arm by mean S at w = 0, and the paired best-rung ratio.
    if rungs:
        best = {}
        for arm in ("A'", "B", "C"):
            best[arm] = min(rungs, key=lambda rg: rungs[rg]["s_mean"]["0.0"][arm])
        out["best_rung"] = best
        reps = min(rungs[rg]["repeats"] for rg in rungs)
        for arm in ("B", "C"):
            pr = [
                rungs[best["A'"]]["s_per_repeat"]["0.0"]["A'"][k]
                / rungs[best[arm]]["s_per_repeat"]["0.0"][arm][k]
                for k in range(reps)
            ]
            out[f"best_rung_ratio_A'/{arm}"] = OrderedDict(
                per_repeat=pr, geometric_mean=gmean(pr), bootstrap_95=bootstrap_ci(pr)
            )
    # The published-row ratio (accounting): frozen A mean S over this run's B.
    if frozen_inst is not None and n in PUBLISHED_RUNG and PUBLISHED_RUNG[n] in rungs:
        rung = PUBLISHED_RUNG[n]
        fby = rows_by_name(frozen_inst)
        frows = fby.get(RAW + RUNGS[rung], [])
        if frows:
            fmean = sum(v["s"] for v in frows) / len(frows)
            out["published_rung"] = rung
            out["published_s_frozen_mean"] = fmean
            out["published_over_B"] = fmean / rungs[rung]["s_mean"]["0.0"]["B"]
            out["published_over_C"] = fmean / rungs[rung]["s_mean"]["0.0"]["C"]
    # Predictions on the primary rows.
    if n in PRIMARY and census.get("factor_base"):
        f = census["factor_base"]["class_zero_fraction"]
        out["P1"] = OrderedDict(
            measured=f, predicted=CENSUS_P1[n], within_10pct=abs(f - CENSUS_P1[n]) <= 0.1 * CENSUS_P1[n]
        )
    if n in PRIMARY and PUBLISHED_RUNG[n] in rungs:
        rec = rungs[PUBLISHED_RUNG[n]]
        h = inst["cofactor"]
        yb = rec["yield_ratio_B_over_A"]
        out["P2"] = OrderedDict(
            measured=yb, band=[0.7 * h, 1.3 * h], holds=(yb is not None and 0.7 * h <= yb <= 1.3 * h)
        )
        key = "A'/B@0.0"
        out["P3"] = OrderedDict(
            measured=rec["paired_ratios"][key]["geometric_mean"],
            bootstrap_95=rec["paired_ratios"][key]["bootstrap_95"],
            predicted="1.4-1.5" if n in (15, 21) else "about 3",
        )
        if h == 2:
            bc = rec["yield_ratio_B_over_C"]
            out["P4"] = OrderedDict(measured=bc, holds=(bc is not None and abs(bc - 1.0) <= 0.15))
    return out


def verdict(instances, control_problems):
    """The protocol's kill and success rules, read on the published rung."""
    rows = {}
    for inst in instances:
        n = inst["degree"]
        if n in PRIMARY and inst.get("P3"):
            rows[n] = inst
    if not rows:
        return OrderedDict(verdict="no primary row", rows={})
    per = OrderedDict()
    for n, inst in sorted(rows.items()):
        p3 = inst["P3"]
        lo, hi = p3["bootstrap_95"]
        per[n] = OrderedDict(
            ratio=p3["measured"],
            bootstrap_95=[lo, hi],
            success=(p3["measured"] >= 1.10 and lo > 1.0),
            straddles_1_10=(lo < 1.10 < hi),
            P2_holds=inst["P2"]["holds"],
        )
    all_below = all(v["ratio"] < 1.10 for v in per.values())
    p2_fails = any(not v["P2_holds"] for v in per.values())
    control_moved = bool(control_problems)
    success = [n for n, v in per.items() if v["success"]]
    extension = [n for n, v in per.items() if v["straddles_1_10"] and not v["success"]]
    if control_moved:
        word = "kill: the prime-order control moved"
    elif p2_fails:
        word = "kill: P2 failed (the mechanism is misread)"
    elif all_below:
        word = "kill: S(A')/S(B) below 1.10 on every primary row"
    elif success:
        word = "success on rows " + ", ".join(f"n={n}" for n in success)
    else:
        word = "undecided"
    return OrderedDict(
        verdict=word,
        rows=per,
        extension_rows_repeats_20=extension,
        control_problems=control_problems,
    )


def markdown(result):
    lines = ["# LEAD-19 analysis (generated by docs/ic/tools/lead19_projected_base_compare.py)", ""]
    lines.append(f"Verdict: **{result['verdict']['verdict']}**")
    lines.append("")
    lines.append("| row | h | rung | reps | S(A) | S(A') | S(B) | S(C) | A'/B (95 % CI) | A'/C | yield B/A | yield C/A | rho | gap A' / B / C |")
    lines.append("|:--|--:|:--|--:|--:|--:|--:|--:|:--|--:|--:|--:|--:|:--|")
    for inst in result["instances"]:
        for rung, rec in inst["rungs"].items():
            m = rec["s_mean"]["0.0"]
            pb = rec["paired_ratios"]["A'/B@0.0"]
            pc = rec["paired_ratios"]["A'/C@0.0"]
            lo, hi = pb["bootstrap_95"]
            g = rec["gap_to_rho"]["0.0"]
            mark = " (published)" if inst.get("published_rung") == rung else ""
            lines.append(
                f"| {inst['curve']} | {inst['cofactor']} | {rung}{mark} | {rec['repeats']} | {m['A']:.2f} | {m[AP]:.2f} | {m['B']:.2f} | {m['C']:.2f} "
                f"| {pb['geometric_mean']:.2f} ({lo:.2f}, {hi:.2f}) | {pc['geometric_mean']:.2f} "
                f"| {rec['yield_ratio_B_over_A']:.2f} | {rec['yield_ratio_C_over_A']:.2f} | {inst['rho_s_mean']:.2f} "
                f"| {g[AP]:.1f}× / {g['B']:.1f}× / {g['C']:.1f}× |"
            )
    lines.append("")
    lines.append("Pricing band (S at w = 0 / 0.1 / 1 GAE per unpriced native op; only arm C carries any):")
    lines.append("")
    lines.append("| row | rung | A' | B | C | order A'>B holds | order A'>C holds |")
    lines.append("|:--|:--|:--|:--|:--|:--|:--|")
    for inst in result["instances"]:
        for rung, rec in inst["rungs"].items():
            cells = []
            for arm in ("A'", "B", "C"):
                cells.append(" / ".join(f"{rec['s_mean'][str(w)][arm]:.2f}" for w in PRICES))
            lines.append(
                f"| {inst['curve']} | {rung} | {cells[0]} | {cells[1]} | {cells[2]} "
                f"| {rec['order_holds_across_band']['B']} | {rec['order_holds_across_band']['C']} |"
            )
    lines.append("")
    lines.append("Predictions:")
    lines.append("")
    for inst in result["instances"]:
        for key in ("P1", "P2", "P3", "P4"):
            if key in inst:
                lines.append(f"- {inst['curve']} {key}: {json.dumps(inst[key])}")
    if result.get("published"):
        lines.append("")
        lines.append("Published-row ratio (accounting, not the lever): " + json.dumps(result["published"]))
    lines.append("")
    lines.append("Stop checks: " + json.dumps(result["stop_checks"]))
    return "\n".join(lines) + "\n"


def main():
    char2 = json.load(open(sys.argv[1]))
    prime = json.load(open(sys.argv[2]))
    frozen = json.load(open(arg("--frozen"))) if arg("--frozen") else None
    out_path = arg("--out")
    md_path = arg("--md")
    # The frozen ledger predates ICV1 slugs and names its curves by the
    # older random-binary scheme; the regime and the subgroup order
    # identify the instance across both namings.
    frozen_by_key = {}
    if frozen:
        for i in frozen["ledger"]["instances"]:
            frozen_by_key[(i["regime"], i["r"])] = i
    stop = OrderedDict()
    stop["char2_status"] = char2["status"]
    stop["prime_status"] = prime["status"]
    stop["all_verified"] = bool(char2["all_verified"] and prime["all_verified"])
    control_problems = []
    for inst in prime["ledger"]["instances"]:
        control_problems += [f"{inst['curve']['name']}: {p}" for p in check_control(inst)]
    stop["control_bit_identical"] = not control_problems
    stop["control_problems"] = control_problems
    instances = []
    for inst in char2["ledger"]["instances"]:
        instances.append(analyse_instance(inst, frozen_by_key.get((inst["regime"], inst["r"]))))
    stop["frozen_drift"] = {i["curve"]: i["frozen_check"] for i in instances if i["frozen_check"]}
    result = OrderedDict()
    result["what_this_is"] = (
        "LEAD-19 paired comparison of the raw (A), class-test-free (A'), cofactor-projected (B) "
        "and subgroup-only (C) two-summand walk rows, from one ic boundary run per regime; "
        "ratios, means and bootstrap resamples of recorded values only"
    )
    result["inputs"] = {
        "char2": sys.argv[1],
        "prime": sys.argv[2],
        "frozen": arg("--frozen"),
        "char2_commit": char2["host"].get("git_commit"),
        "char2_binary_blake3": char2["software"].get("binary_blake3"),
        "prime_commit": prime["host"].get("git_commit"),
    }
    result["stop_checks"] = stop
    result["instances"] = instances
    result["published"] = {
        i["curve"]: {
            "rung": i["published_rung"],
            "frozen_mean_s": i["published_s_frozen_mean"],
            "over_B": i["published_over_B"],
            "over_C": i["published_over_C"],
        }
        for i in instances
        if "published_over_B" in i
    }
    result["verdict"] = verdict(instances, control_problems)
    if out_path:
        with open(out_path, "w") as f:
            json.dump(result, f, indent=1)
    md = markdown(result)
    if md_path:
        with open(md_path, "w") as f:
            f.write(md)
    print(md)


if __name__ == "__main__":
    main()
