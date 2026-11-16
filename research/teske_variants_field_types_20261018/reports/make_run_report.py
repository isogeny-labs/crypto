"""Lay out a run's recorded values as the descriptive run report (Typst).

Usage: python make_run_report.py <run-dir> <out.typ>
Reads env.txt, tool_log.txt, NOTE.md, the CSVs and the check files written
by examples/teske_variants_field_types.rs, whichever exist in the run
directory. Computes nothing: every number is copied from the run's files.
"""

import csv
import sys
from pathlib import Path

run, out = Path(sys.argv[1]), Path(sys.argv[2])
stamp = run.name.replace("run-", "")


def has(name):
    return (run / name).exists()


def rows(name):
    if not has(name):
        return []
    with open(run / name, newline="") as f:
        return list(csv.DictReader(f))


def text(name):
    return (run / name).read_text().strip() if has(name) else ""


def esc(s):
    return (s.replace("\\", "\\\\").replace("#", "\\#").replace("_", "\\_").replace("*", "\\*")
            .replace("@", "\\@").replace("<", "\\<").replace("$", "\\$"))


def table(header, body):
    cells = ", ".join(f"[*{esc(h)}*]" for h in header)
    body_cells = ",\n".join(", ".join(f"[{esc(str(c))}]" for c in r) for r in body)
    return f"#table(columns: {len(header)}, align: left, stroke: 0.4pt + rgb(\"#d9d8d4\"),\n{cells},\n{body_cells}\n)\n"


P = []
P.append(f"""#set page(paper: "a4", margin: (x: 1.5cm, y: 1.4cm), numbering: "1 / 1")
#set text(size: 8.4pt)
#set par(justify: true, leading: 0.5em)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.6pt)
#show table: set text(size: 6.9pt)
#show raw: set text(size: 7.2pt)

= Run report: teske\\_variants\\_field\\_types, run {esc(stamp)}

Descriptive record of one run of `examples/teske_variants_field_types.rs`
(protocol `research/teske_variants_field_types_20261018/PROTOCOL.md`, frozen
at commit `62b4502c7`). It lists what ran, on what host, and the values the
tool recorded, with their files. Interpretation is in `REPORT.md` of the
study. Nothing here is a running time of an attack; the only timings are
the tool's own wall-clock seconds per part, from `tool_log.txt`.

== Command, host and isolation
```
{text("env.txt")}
```
Isolation: none earned (a shared cloud container, 4 threads, no pinning);
the tool is exact, so isolation affects only the wall-clock figures below.
""")
if has("NOTE.md"):
    P.append("== Note on this run\n```\n" + text("NOTE.md") + "\n```\n")
P.append("== Tool log (wall-clock per part)\n```\n" + text("tool_log.txt") + "\n```\n")

bs = rows("binary_summary.csv")
if bs:
    sel = [r for r in bs if int(r["n"]) in (3, 5, 7, 9, 15, 17, 21, 23, 31, 33, 35, 45, 63, 93)]
    P.append("== Part A: binary fields (`binary_types.csv`, `binary_gghs.csv`, `binary_summary.csv`, `binary_bits.csv`)\n\n")
    P.append(f"Rows in `binary_types.csv`: {len(rows('binary_types.csv'))}; in `binary_gghs.csv`: {len(rows('binary_gghs.csv'))}; in `binary_bits.csv`: {len(rows('binary_bits.csv'))}. Every value is an exact count or a formula on the cyclotomic cosets of 2 modulo n; the per-class expectations are the GHS 2002 §5 formula. Selected rows of the summary (smallest genus per variant; δ = n − deg F; margin a = δ − n/2):\n\n")
    P.append(table(["n", "ord", "cosets", "type I (deg,g,δ)", "a_I", "type II (deg,g,δ)", "a_II", "gGHS (r,s,g,δ)", "a_gGHS"],
                   [[r["n"], r["ord_2_mod_n"], r["num_cosets"], f"({r['best_typeI_deg']}, {r['best_typeI_genus']}, {r['best_typeI_delta']})", r["margin_typeI"],
                     f"({r['best_typeII_deg']}, {r['best_typeII_genus']}, {r['best_typeII_delta']})", r["margin_typeII"],
                     f"({r['best_gghs_r']}, {r['best_gghs_s']}, {r['best_gghs_genus']}, {r['best_gghs_delta']})", r["margin_gghs"]] for r in sel]))
    bb = [r for r in rows("binary_bits.csv") if r["N"] in ("161", "155") and r["variant"] != "generic" and int(r["n"]) < int(r["N"])]
    P.append("\nBit table rows for N = 155 and 161 (all variants except `generic`):\n\n")
    P.append(table(["N", "n", "l", "variant", "g", "δ", "rho", "owner", "walk", "margin", "log2 #weak", "per class"],
                   [[r["N"], r["n"], r["l"], r["variant"], r["genus"], r["delta"], r["rho_bits"], r["owner_bits"], r["walk_bits"], r["margin_bits"], r["log2_weak_curves"], r["per_class_expectation"]] for r in bb]))
else:
    P.append("== Part A: not part of this run\n")

ot = rows("oddchar_types.csv")
if ot:
    P.append("\n== Part B: odd characteristic (`oddchar_types.csv`, `oddchar_classes.csv`, `oddchar_checks.txt`)\n\n")
    P.append(f"Rows (one per (q, n, branch type)): {len(ot)}. instances\\_B is the number of weak branch sets (pairs (E, x)); weak\\_j the number of distinct j-invariants; ratio = weak\\_j / q^(r−3).\n\n")
    P.append(table(["q", "n", "L", "orbits", "r", "m", "g", "instances B", "weak j", "log_q", "pred.", "ratio", "all-j"],
                   [[r["q"], r["n"], r["L"], r["orbit_sizes"], r["r"], r["m"], r["genus"], r["instances_B"], r["weak_j"], r["log_q_weak_j"], r["predicted_exponent"], r["ratio_weak_j_over_q_pow"], r["all_j_table"]] for r in ot]))
    ocl = rows("oddchar_classes.csv")
    P.append(f"\nClass statistics (rows: {len(ocl)}; only where the all-j trace table was computed): classes is the number of traces realised over F\\_(q^n); covered are those with a weak member of the type; share is the fraction of all curves (both twists of every j) in covered classes; walk = curves in covered classes / weak curves in them.\n\n")
    P.append(table(["q", "n", "orbits", "r", "m", "g", "classes", "covered", "share", "mean weak", "max weak", "walk"],
                   [[r["q"], r["n"], r["orbit_sizes"], r["r"], r["m"], r["genus"], r["total_classes"], r["covered_classes"], r["share_curves_in_covered"], r["mean_weak_per_covered_class"], r["max_weak_per_class"], r["mean_walk_length"]] for r in ocl]))
    P.append("\nChecks (`oddchar_checks.txt`):\n```\n" + text("oddchar_checks.txt") + "\n```\n")
else:
    P.append("\n== Part B: not part of this run\n")

cs = rows("census_summary.csv")
if cs:
    P.append("\n== Part C: census (`census.csv`, `census_summary.csv`, `census_patterns.csv`, `census_checks.txt`)\n\n")
    P.append(table(["q", "models", "squarefree", "Res-shaped models", "classes", "signature", "no signature", "roots outside F_q^6", "#E prime", "prime, no sig.", "t subfield-shaped", "of which sig.", "traces", "traces with sig.", "s"],
                   [[r["q"], r["models_enumerated"], r["squarefree"], r["res_shaped_models"], r["res_shaped_classes"], r["classes_signature"], r["classes_no_signature"], r["classes_roots_outside_Fq6"], r["classes_numE_prime"], r["classes_numE_prime_no_signature"], r["classes_t_subfield_shape"], r["classes_t_subfield_shape_with_signature"], r["distinct_traces"], r["traces_with_signature"], r["seconds"]] for r in cs]))
    cp = rows("census_patterns.csv")
    P.append("\nClasses by Weierstrass cycle type and signature:\n\n")
    P.append(table(["q", "cycle type", "signature", "classes"], [[r["q"], r["cycle_type"], r["ghs_signature"], r["classes"]] for r in cp]))
    P.append("\nChecks (`census_checks.txt`):\n```\n" + text("census_checks.txt") + "\n```\n")
else:
    P.append("\n== Part C: not part of this run\n")

P.append("""
== Graphs
Recorded values only, drawn by `figures/plot.py` from the CSVs of the two recorded runs (Parts A and B from run 20261018T0630Z, Part C from run 20261018T0845Z).

#figure(image("../../figures/oddchar_counts.svg", width: 100%), caption: [Weak j-invariants per branch type divided by q^(r−3); exact counts, no uncertainty.])
#figure(image("../../figures/oddchar_classes.svg", width: 100%), caption: [Share of curves in isogeny classes with a weak member, and weak members per covered class; exact, from the all-j trace tables.])
#figure(image("../../figures/census.svg", width: 70%), caption: [Res-shaped genus-3 hyperelliptic classes over F_q by GHS signature; exact census.])
#figure(image("../../figures/margins.svg", width: 100%), caption: [Margins a = δ − n/2 by n: binary (exact cosets) and odd characteristic (smallest-genus type).])
""")
out.write_text("".join(P))
print("wrote", out)
