#!/usr/bin/env python3
"""Validator: every number RESULTS.md and reports/run-20261128T0345Z/report.typ display,
against the validator's recompute (recompute.json) and the frozen outputs.

Usage:
  python3 -I docs_check.py SNAP RECOMPUTE.json OUT.json

Each displayed number is compared with its full-precision source rounded ONCE at the
displayed precision (format(x, '.3f') etc.; lesson L-6cac0c58a699): a value whose next
digit is 5 after a first rounding exposes double rounding.  The script first checks that
the displayed token occurs on the cited line, so the file:line references are verified
too.  Counts and fractions are compared exactly.  Statements that are not numbers (for
example "grows with p") are listed with the counts that decide them.
"""
import json
import math
import sys

STUDY = "research/quartic_diem_p0lines_20261128"
NOFB, P0, P0I3 = "Q_diem_2lp_sfb_sge_nofb", "Q_diem_2lp_sfb_sge_p0", "Q_diem_2lp_sfb_sge_p0i3"


def main():
    snap, recp, outp = sys.argv[1:4]
    st = f"{snap}/{STUDY}"
    R = json.load(open(recp))
    files = {"RESULTS.md": open(f"{st}/RESULTS.md").read().splitlines(),
             "report.typ": open(f"{st}/reports/run-20261128T0345Z/report.typ").read().splitlines()}
    pr = R["paired"]
    ic = R["ic_over_rho"]
    att = R["attempts"]
    ex = R["explore"]

    def pooled(c, pn, w):
        x = pr[c][pn][f"gae_{w}"]
        return [x["pooled"]["gm"]] + x["mc_producer_stream"]["pooled"]

    def prime(c, pn, w, p):
        x = pr[c][pn][f"gae_{w}"]
        return [x["per_prime"][p]["gm"]] + x["mc_producer_stream"]["per_prime"][p]

    def icv(a, w, p, ci=False):
        x = ic[a][f"all_{w}"]
        v = [x["per_prime"][p]["gm"]]
        return v + x["mc_producer_stream"]["per_prime"][p] if ci else v
    P5, PA = "primary_seeds_5_6", "all_seeds_1_6"
    checks = []  # (file, line, [displayed], [values], fmt)

    def add(f, ln, disp, vals, fmt):
        checks.append((f, ln, disp, vals, fmt))
    # decision table (RESULTS 29-31, report 26-28)
    for (ln_r, ln_t, w) in ((29, 26, "1.0"), (30, 27, "0.1"), (31, 28, "0.0")):
        a, b = pooled("p0_over_nofb", P5, w), pooled("p0_over_nofb", PA, w)
        c, d = pooled("p0i3_over_nofb", P5, w), pooled("p0i3_over_nofb", PA, w)
        row = [format(v, ".3f") for v in a + b + c + d]
        add("RESULTS.md", ln_r, row, a + b + c + d, ".3f")
        add("report.typ", ln_t, row[:6], a + b, ".3f")
    add("RESULTS.md", 33, ["0.934"], [pooled("p0_over_nofb", P5, "1.0")[1]], ".3f")
    # per-prime p0/nofb, w = 1 (RESULTS 40-43)
    for ln, p in ((40, "4099"), (41, "16411"), (42, "65537"), (43, "131101")):
        v = prime("p0_over_nofb", P5, "1.0", p) + prime("p0_over_nofb", PA, "1.0", p)
        add("RESULTS.md", ln, [format(x, ".3f") for x in v], v, ".3f")
    # P0 line counts (RESULTS 18-21): means over all seeds, rounded to integers
    pm = R["integrity"]["p0_means_p0_arm_all_seeds"]
    for ln, p in ((18, "4099"), (19, "16411"), (20, "65537"), (21, "131101")):
        v = [pm[p]["p0_lines_mean"], pm[p]["p0_line_relations_mean"]]
        add("RESULTS.md", ln, [format(x, ".0f") for x in v], v, ".0f")
    # IC / rho, w = 1 (RESULTS 86-89, report 48-49)
    for ln, p in ((86, "4099"), (87, "16411"), (88, "65537")):
        v = icv(NOFB, "1.0", p) + icv(P0, "1.0", p) + icv(P0I3, "1.0", p)
        add("RESULTS.md", ln, [format(x, ".2f") for x in v], v, ".2f")
    v131 = icv(NOFB, "1.0", "131101", True) + icv(P0, "1.0", "131101", True) + icv(P0I3, "1.0", "131101", True)
    add("RESULTS.md", 89, ["2.02", "1.85", "2.20", "1.91", "1.82", "2.00", "2.04", "1.93", "2.14"], v131, ".2f")
    add("report.typ", 48, ["8.76", "4.96", "2.84", "2.02", "1.85", "2.20"],
        [icv(NOFB, "1.0", p)[0] for p in ("4099", "16411", "65537")] + icv(NOFB, "1.0", "131101", True), ".2f")
    add("report.typ", 49, ["8.88", "4.45", "2.70", "1.91", "1.82", "2.00"],
        [icv(P0, "1.0", p)[0] for p in ("4099", "16411", "65537")] + icv(P0, "1.0", "131101", True), ".2f")
    # RESULTS 91: w = 0.1 range at 131101 over the arms, w = 0 order of magnitude
    w01 = [icv(a, "0.1", "131101")[0] for a in (NOFB, P0, P0I3)]
    add("RESULTS.md", 91, ["0.19", "0.20"], [min(w01), max(w01)], ".2f")
    w0 = [icv(a, "0.0", "131101")[0] for a in (NOFB, P0, P0I3)]
    add("RESULTS.md", 91, ["2·10⁻⁴", "2·10⁻⁴"], [min(w0), max(w0)], ".0e")
    w1 = [icv(a, "1.0", "131101")[0] for a in (NOFB, P0, P0I3)]
    add("RESULTS.md", 92, ["1.9", "2.0"], [min(w1), max(w1)], ".1f")
    add("report.typ", 51, ["1.9", "2.0"], [min(w1), max(w1)], ".1f")
    # explore table (RESULTS 69-72): explore.py rounds to 3 decimals
    cells = {69: ("4099", ["nofb1_p01", None, None, "nofb1_p02"]), 70: ("16411", ["nofb1_p01", "nofb2_p01", "nofb2_p02", None]),
             71: ("65537", ["nofb1_p01", "nofb2_p01", "nofb2_p02", None]), 72: ("131101", ["nofb1_p01", "nofb2_p01", "nofb2_p02", "nofb1_p02"])}
    for ln, (p, ks) in cells.items():
        disp, vals = [], []
        for k in ks:
            if k is None:
                assert k not in ex[p]
                continue
            disp += [format(ex[p][k]["gm"], ".3f"), f"({ex[p][k]['n']})"]
            vals += [ex[p][k]["gm"], ex[p][k]["n"]]
        add("RESULTS.md", ln, disp, vals, "explore")
    both1 = [ex[p]["nofb1_p01"]["gm"] for p in ex]
    add("RESULTS.md", 74, ["3%"], [max(abs(x - 1) for x in both1) * 100], "pct_le3")
    add("report.typ", 38, ["0.99", "1.03"], [min(both1), max(both1)], ".2f")
    # "about 0.62": the geometric mean over all 15 rescued pairs (per prime 0.620, 0.628, 0.616)
    rp = [(ex[p]["nofb2_p01"]["gm"], ex[p]["nofb2_p01"]["n"]) for p in ex if "nofb2_p01" in ex[p]]
    resc = [math.exp(sum(n * math.log(g) for g, n in rp) / sum(n for _, n in rp))]
    add("RESULTS.md", 75, ["0.62"], resc, ".2f")
    add("report.typ", 39, ["0.62"], resc, ".2f")
    newf = [ex[p]["nofb1_p02"]["gm"] for p in ex if "nofb1_p02" in ex[p]]
    add("report.typ", 41, ["1.56"], [math.exp(sum(math.log(x) for x in newf) / len(newf))], ".2f")
    out = {"schema": "validator-docs-check/v1", "numbers": [], "errors": [], "statements": []}
    for f, ln, disp, vals, fmt in checks:
        text = files[f][ln - 1]
        for d in disp:
            if d not in text:
                out["errors"].append({"file": f, "line": ln, "token_missing": d, "text": text})
        if fmt in (".3f", ".2f", ".1f", ".0f"):
            if f == "RESULTS.md" and ln in (75,):
                mine = sorted({format(v, fmt) for v in vals})
                ok = len(mine) == 1 and mine[0] == disp[0]
            elif ln in (91, 92) or (f == "report.typ" and ln in (38, 51)):
                mine = [format(v, fmt) for v in vals]
                ok = mine == disp
            elif f == "report.typ" and ln == 39:
                mine = sorted({format(v, fmt) for v in vals})
                ok = len(mine) == 1 and mine[0] == disp[0]
            else:
                mine = [format(v, fmt) for v in vals]
                ok = mine == disp
        elif fmt == ".0e":
            mine = [f"{v:.0e}" for v in vals]
            ok = all(m.startswith("2e-04") for m in mine)
        elif fmt == "explore":
            mine = []
            for i in range(0, len(vals), 2):
                mine += [format(vals[i], ".3f"), f"({vals[i + 1]})"]
            ok = mine == disp
        elif fmt == "pct_le3":
            mine = [round(vals[0], 2)]
            ok = vals[0] <= 3.0
        for i, d in enumerate(disp):
            out["numbers"].append({"file": f, "line": ln, "displayed": d,
                                   "recomputed_display": mine[i] if i < len(mine) else mine, "ok": ok})
        if not ok:
            out["errors"].append({"file": f, "line": ln, "displayed": disp, "recomputed_display": mine,
                                  "full_precision": vals})
    # counts and fractions
    def a_(pn, a):
        return att[pn][a]
    counts = [
        ("RESULTS.md", 51, "10 / 40 | 25 / 120 | 0 / 9 / 7 / 9", f"{a_(P5, NOFB)['multi']} / {a_(P5, NOFB)['runs']} | {a_(PA, NOFB)['multi']} / {a_(PA, NOFB)['runs']} | " + " / ".join(str(a_(PA, NOFB)['per_prime'][p][0]) for p in ("4099", "16411", "65537", "131101"))),
        ("RESULTS.md", 52, "8 / 40 | 12 / 120 | 1 / 1 / 4 / 6", f"{a_(P5, P0)['multi']} / {a_(P5, P0)['runs']} | {a_(PA, P0)['multi']} / {a_(PA, P0)['runs']} | " + " / ".join(str(a_(PA, P0)['per_prime'][p][0]) for p in ("4099", "16411", "65537", "131101"))),
        ("RESULTS.md", 53, "10 / 40 | 19 / 120 | 1 / 2 / 5 / 11", f"{a_(P5, P0I3)['multi']} / {a_(P5, P0I3)['runs']} | {a_(PA, P0I3)['multi']} / {a_(PA, P0I3)['runs']} | " + " / ".join(str(a_(PA, P0I3)['per_prime'][p][0]) for p in ("4099", "16411", "65537", "131101"))),
        ("RESULTS.md", 55, "1 of 10", f"{a_(P5, NOFB)['per_prime']['16411'][0]} of {a_(P5, NOFB)['per_prime']['16411'][1]}"),
        ("RESULTS.md", 55, "8 of 20", f"{att['seeds_1_4'][NOFB]['per_prime']['16411'][0]} of {att['seeds_1_4'][NOFB]['per_prime']['16411'][1]}"),
        ("report.typ", 31, "1/10 against 8/20", f"{a_(P5, NOFB)['per_prime']['16411'][0]}/{a_(P5, NOFB)['per_prime']['16411'][1]} against {att['seeds_1_4'][NOFB]['per_prime']['16411'][0]}/{att['seeds_1_4'][NOFB]['per_prime']['16411'][1]}"),
        ("RESULTS.md", 59, "8/40 against 10/40", f"{a_(P5, P0)['multi']}/40 against {a_(P5, NOFB)['multi']}/40"),
        ("RESULTS.md", 59, "12/120 against 25/120", f"{a_(PA, P0)['multi']}/120 against {a_(PA, NOFB)['multi']}/120"),
        ("report.typ", 30, "10/40 → `p0` 8/40", f"{a_(P5, NOFB)['multi']}/40 → `p0` {a_(P5, P0)['multi']}/40"),
        ("report.typ", 30, "25/120 → 12/120", f"{a_(PA, NOFB)['multi']}/120 → {a_(PA, P0)['multi']}/120"),
        ("RESULTS.md", 61, "10 against 8 on seeds 5–6, 19 against 12", f"{a_(P5, P0I3)['multi']} against {a_(P5, P0)['multi']} on seeds 5–6, {a_(PA, P0I3)['multi']} against {a_(PA, P0)['multi']}"),
        ("RESULTS.md", 11, "360 runs (120 per arm)", f"{R['integrity']['certified_ok_recovered_eq_t']} runs ({R['completeness']['runs'] // 3} per arm)"),
        ("RESULTS.md", 13, "80 of 80", f"{R['integrity']['C0']['identical_24']} of {R['integrity']['C0']['compared']}"),
        ("report.typ", 21, "80 of 80", f"{R['integrity']['C0']['identical_24']} of {R['integrity']['C0']['compared']}"),
        ("RESULTS.md", 75, "15 of the 25", f"{sum(ex[p]['nofb2_p01']['n'] for p in ex if 'nofb2_p01' in ex[p])} of the {a_(PA, NOFB)['multi']}"),
        ("report.typ", 39, "15 of 25", f"{sum(ex[p]['nofb2_p01']['n'] for p in ex if 'nofb2_p01' in ex[p])} of {a_(PA, NOFB)['multi']}"),
        ("report.typ", 40, "(10)", f"({sum(ex[p]['nofb2_p02']['n'] for p in ex if 'nofb2_p02' in ex[p])})"),
        ("RESULTS.md", 76, "5 of 9 at 131101", f"{ex['131101']['nofb2_p02']['n']} of {a_(PA, NOFB)['per_prime']['131101'][0]} at 131101"),
        ("report.typ", 43, "5 of 9 at 131101", f"{ex['131101']['nofb2_p02']['n']} of {a_(PA, NOFB)['per_prime']['131101'][0]} at 131101"),
        ("RESULTS.md", 77, "Two runs", "Two runs" if sum(ex[p]["nofb1_p02"]["n"] for p in ex if "nofb1_p02" in ex[p]) == 2 else "?"),
    ]
    for f, ln, disp, mine in counts:
        text = files[f][ln - 1]
        ok = disp in text and disp == mine
        out["numbers"].append({"file": f, "line": ln, "displayed": disp, "recomputed_display": mine, "ok": ok})
        if not ok:
            out["errors"].append({"file": f, "line": ln, "displayed": disp, "recomputed_display": mine, "in_text": disp in text})
    # statements decided by counts
    share = {p: [ex[p].get("nofb2_p02", {"n": 0})["n"], att[PA][NOFB]["per_prime"][p][0]] for p in ("4099", "16411", "65537", "131101")}
    shares = [share[p][0] / share[p][1] for p in ("16411", "65537", "131101")]
    out["statements"].append({"where": ["RESULTS.md:76", "report.typ:43"], "text": "the share of unrescued failures grows with p",
                              "unrescued_of_nofb_multi_attempt": share,
                              "monotone_increasing_16411_65537_131101": shares[0] < shares[1] < shares[2]})
    unres = [(ex[p]["nofb2_p02"]["gm"], ex[p]["nofb2_p02"]["n"]) for p in ex if "nofb2_p02" in ex[p]]
    gm_un = math.exp(sum(n * math.log(g) for g, n in unres) / sum(n for _, n in unres))
    out["statements"].append({"where": ["report.typ:40"], "text": "unrescued failures (10): about 1.0",
                              "per_prime_gm_n": {p: [ex[p]["nofb2_p02"]["gm"], ex[p]["nofb2_p02"]["n"]] for p in ex if "nofb2_p02" in ex[p]},
                              "pooled_gm_of_the_10": gm_un,
                              "individual_range": [min(ex[p]["nofb2_p02"]["min"] for p in ex if "nofb2_p02" in ex[p]),
                                                   max(ex[p]["nofb2_p02"]["max"] for p in ex if "nofb2_p02" in ex[p])]})
    ver = R["ic_over_rho_verdict"]
    out["statements"].append({"where": ["RESULTS.md:91", "report.typ:51"],
                              "text": "every arm UNRESOLVED at every prime: below rho at w = 0 and 0.1, above at w = 1",
                              "holds_exact_and_stream": all(ver[a][p][s]["below_at"] == [0.0, 0.1] and ver[a][p][s]["above_at"] == [1.0]
                                                            for a in ver for p in ver[a] for s in ("exact95", "mc_producer_stream"))})
    out["summary"] = {"numbers_checked": len(out["numbers"]), "errors": len(out["errors"])}
    with open(outp, "w") as fh:
        json.dump(out, fh, indent=1, ensure_ascii=False)
        fh.write("\n")
    print(json.dumps(out["summary"]))
    for e in out["errors"]:
        print(json.dumps(e, ensure_ascii=False))
    for s in out["statements"]:
        print(json.dumps(s, ensure_ascii=False))


if __name__ == "__main__":
    main()
