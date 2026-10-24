"""Headline list for the two validator reports.

Usage: python3 -I headlines.py <repo-root> <recompute_out.json> <out.json>

`claimed` is parsed from the committed text (file and line cited), `recomputed`
is the validator's full-precision value from recompute.py; rel_tol is half a
unit in the last printed digit divided by |claimed| (0 for integer counts).
"""
import json
import re
import sys

R = "research/weil_descent_angles_20261018"
RES = f"{R}/results"
OJ_SUM = f"{RES}/itemOJ-run-20261018T2132Z/summary.txt"
F_SUM = f"{RES}/itemF-control-run-20261018T2128Z/control_summary.txt"
H1_SUM = f"{RES}/H1-hsweep-run-20261018T2240Z/hsweep_summary.txt"
H2_SUM = f"{RES}/H2-glue-run-20261018T2240Z/glue_summary.txt"
H4_SUM = f"{RES}/H4-memsweep-run-20261018T2209Z/memsweep_summary.txt"
H6_SUM = f"{RES}/H6-large-run-20261018T2210Z/summary.txt"
G_OJ = f"{R}/validation/gae-band/itemOJ-run-20261018T2132Z.txt"
G_H2 = f"{R}/validation/gae-band/H2-glue-run-20261018T2240Z.txt"
G_H6 = f"{R}/validation/gae-band/H6-large-run-20261018T2210Z.txt"
REP3 = f"{R}/REPORT.md"
REP4 = f"{R}/REPORT_ROUND4.md"

NUM = r"([0-9]+(?:\.[0-9]+)?(?:e-?[0-9]+)?)"


def half_ulp(s):
    if "e" in s:
        m, e = s.split("e")
        d = len(m.split(".")[1]) if "." in m else 0
        return 0.5 * 10.0 ** (int(e) - d)
    d = len(s.split(".")[1]) if "." in s else 0
    return 0.5 * 10.0 ** (-d)


class Src:
    def __init__(self, repo, rel):
        self.rel = rel
        self.lines = open(f"{repo}/{rel}").read().splitlines()

    def find(self, pattern):
        rx = re.compile(pattern)
        hits = [(i + 1, rx.search(l)) for i, l in enumerate(self.lines) if rx.search(l)]
        assert len(hits) == 1, (self.rel, pattern, len(hits))
        return hits[0]


def main():
    repo, recp, outp = sys.argv[1:4]
    V = json.load(open(recp))["values"]
    S = {k: Src(repo, k) for k in (OJ_SUM, F_SUM, H1_SUM, H2_SUM, H4_SUM, H6_SUM, G_OJ, G_H2, G_H6, REP3, REP4)}
    heads = {"r3": [], "r4": []}

    def add(study, name, src, pattern, group, recomputed, integer=False):
        ln, m = S[src].find(pattern)
        s = m.group(group)
        claimed = float(s.replace(",", ""))
        if integer:
            claimed = int(claimed)
            tol = 0.0
        else:
            tol = half_ulp(s) / abs(claimed)
        heads[study].append(dict(name=name, claimed=claimed, recomputed=recomputed, rel_tol=tol,
                                 claimed_text=s, claimed_at=f"{src}:{ln}"))

    oj, f, h1, h2, h4, h6 = V["oj"], V["f"], V["h1"], V["h2"], V["h4"], V["h6"]
    goj, gh2, gh6 = V["gae_oj"], V["gae_h2"], V["gae_h6"]

    # ---------------- round 3
    add("r3", "itemOJ routes.csv rows with verified=1 (of 1260)", REP3, r"\*\*All ([0-9,]+) answers verified", 1, oj["verified_rows"], True)
    # control: 108 = sum of the 'runs' counts printed in control_summary.txt; also the report's Jind/Jwalk table
    runs = sum(int(re.search(r"runs ([0-9]+) ", l).group(1)) for l in S[F_SUM].lines if l.startswith("F bits "))
    heads["r3"].append(dict(name="itemF control.csv rows with verified=1 (of 108)", claimed=runs, recomputed=f["verified_rows"],
                            rel_tol=0.0, claimed_text=f"sum of 'runs' over the 12 'F bits' lines = {runs}; 'F all answers verified: true'",
                            claimed_at=f"{F_SUM}:1-12,17"))
    k6 = oj["K6/40"]
    add("r3", "K6 rho steps/p mean, 40 bits", OJ_SUM, r"^K6 bits 40: rho steps/p mean " + NUM, 1, k6["mean"])
    add("r3", "K6 rho steps/p s.e., 40 bits", OJ_SUM, r"^K6 bits 40: .*\(s\.e\. " + NUM, 1, k6["se"])
    add("r3", "K6 theory 0.886*sqrt(ell)/p mean, 40 bits", OJ_SUM, r"^K6 bits 40: .*/p mean " + NUM + "$", 1, k6["theory"])
    for arm in ("J", "D"):
        for c in ("1", "2", "4"):
            b = oj[f"band_frozen/40/{arm}{c}"]
            pat = rf"^O/J bits 40 arm {arm} c {c}: IC/rho ratio in \[{NUM}, {NUM}\]"
            add("r3", f"Arm {arm} c={c} frozen band (M-native) min, 40 bits", OJ_SUM, pat, 1, b["lo"])
            add("r3", f"Arm {arm} c={c} frozen band (M-native) max, 40 bits", OJ_SUM, pat, 2, b["hi"])
            add("r3", f"Arm {arm} c={c} steps/p, 40 bits", OJ_SUM, rf"^O/J bits 40 arm {arm} c {c}: .*steps/p {NUM} ", 1, b["steps_over_p"])
            g = goj[f"band_gae/40/0/{arm}{c}"]
            pat = rf"^GAE band bits 40 h 0 {arm}{c}: {arm}{c}/rho in \[{NUM}, {NUM}\]"
            add("r3", f"Arm {arm} c={c} GAE band (Erratum V) min, 40 bits", G_OJ, pat, 1, g["lo"])
            add("r3", f"Arm {arm} c={c} GAE band (Erratum V) max, 40 bits", G_OJ, pat, 2, g["hi"])
    for c in ("1", "2", "4"):
        for w in ("0", "0.1", "1"):
            g = goj[f"gate/40/0/J{c}/w{w}"]
            pat = rf"^GAE point bits 40 h 0 J{c} w {re.escape(w)}: ic {NUM} GAE, rho {NUM} GAE, ratio {NUM}$"
            add("r3", f"gate point J c={c} w={w} GAE: IC mean cost per solve (GAE), 40 bits", G_OJ, pat, 1, g["ic"])
            add("r3", f"gate point J c={c} w={w} GAE: rho mean cost per solve (GAE), 40 bits", G_OJ, pat, 2, g["rho"])
            add("r3", f"gate point J c={c} w={w} GAE: IC/rho ratio, 40 bits", G_OJ, pat, 3, g["ratio"])

    # ---------------- round 4
    pat4 = r"H1 ([0-9,]+); H2 ([0-9,]+); H4 ([0-9,]+); H6 ([0-9,]+) \(([0-9,]+) in this round"
    add("r4", "H1 hsweep.csv rows verified (of 2800)", REP4, pat4, 1, h1["verified_rows"], True)
    add("r4", "H2 glue.csv rows verified (of 640)", REP4, pat4, 2, h2["verified_rows"], True)
    add("r4", "H4 memsweep.csv rows verified (of 240)", REP4, pat4, 3, h4["verified_rows"], True)
    add("r4", "H6 routes.csv rows verified (of 180)", REP4, pat4, 4, h6["verified_rows"], True)
    add("r4", "round-4 verified total", REP4, pat4, 5,
        h1["verified_rows"] + h2["verified_rows"] + h4["verified_rows"] + h6["verified_rows"], True)
    for arm in ("J", "R", "Jind"):
        a = h1[f"alpha/{arm}"]
        pat = rf"^H1 alpha fit {arm}: alpha {NUM} \(bootstrap 95% \[{NUM}, {NUM}\]"
        add("r4", f"H1 alpha point estimate, arm {arm}", H1_SUM, pat, 1, a["alpha"])
        add("r4", f"H1 alpha bootstrap 2.5% (RNG replicated), arm {arm}", H1_SUM, pat, 2, a["lo"])
        add("r4", f"H1 alpha bootstrap 97.5% (RNG replicated), arm {arm}", H1_SUM, pat, 3, a["hi"])
    for b in (28, 32, 36, 40):
        for h in (1, 4):
            for arm in ("Jglue", "R"):
                s = h2[f"steps/{b}/{h}/{arm}"]
                add("r4", f"H2 steps/p {arm}, {b} bits, h={h}", H2_SUM, rf"^H2 bits {b} h {h} {arm}: steps/p {NUM} ", 1, s["mean"])
    add("r4", "H2 Jglue measured per Jacobian op, M", H2_SUM, rf"^H2 Jglue: measured per Jacobian op {NUM}M \+ {NUM}I$", 1, h2["per_op"]["M"])
    add("r4", "H2 Jglue measured per Jacobian op, I", H2_SUM, rf"^H2 Jglue: measured per Jacobian op {NUM}M \+ {NUM}I$", 2, h2["per_op"]["I"])
    for b in (28, 32, 36, 40):
        for h in (1, 4):
            fb = h2[f"band_frozen/{b}/{h}"]
            pat = rf"^H2 band bits {b} h {h}: Jglue/rho in \[{NUM}, {NUM}\]"
            add("r4", f"H2 frozen band min, {b} bits, h={h}", H2_SUM, pat, 1, fb["lo"])
            add("r4", f"H2 frozen band max, {b} bits, h={h}", H2_SUM, pat, 2, fb["hi"])
            g = gh2[f"band_gae/{b}/{h}/Jglue"]
            pat = rf"^GAE band bits {b} h {h} Jglue: Jglue/rho in \[{NUM}, {NUM}\]"
            add("r4", f"H2 GAE band (Erratum V) min, {b} bits, h={h}", G_H2, pat, 1, g["lo"])
            add("r4", f"H2 GAE band (Erratum V) max, {b} bits, h={h}", G_H2, pat, 2, g["hi"])
    for w in ("0", "0.1", "1"):
        g = gh2[f"gate/40/1/Jglue/w{w}"]
        pat = rf"^GAE point bits 40 h 1 Jglue w {re.escape(w)}: ic {NUM} GAE, rho {NUM} GAE, ratio {NUM}$"
        add("r4", f"gate point H2 Jglue h=1 w={w} GAE: IC mean cost per solve (GAE), 40 bits", G_H2, pat, 1, g["ic"])
        add("r4", f"gate point H2 h=1 w={w} GAE: rho mean cost per solve (GAE), 40 bits", G_H2, pat, 2, g["rho"])
        add("r4", f"gate point H2 h=1 w={w} GAE: IC/rho ratio, 40 bits", G_H2, pat, 3, g["ratio"])
    for b in (44, 48):
        for arm in ("J", "D"):
            fb = h6[f"band_frozen/{b}/{arm}2"]
            pat = rf"^O/J bits {b} arm {arm} c 2: IC/rho ratio in \[{NUM}, {NUM}\]"
            add("r4", f"H6 Arm {arm} c=2 frozen band min, {b} bits", H6_SUM, pat, 1, fb["lo"])
            add("r4", f"H6 Arm {arm} c=2 frozen band max, {b} bits", H6_SUM, pat, 2, fb["hi"])
            g = gh6[f"band_gae/{b}/0/{arm}2"]
            pat = rf"^GAE band bits {b} h 0 {arm}2: {arm}2/rho in \[{NUM}, {NUM}\]"
            add("r4", f"H6 Arm {arm} c=2 GAE band (Erratum V) min, {b} bits", G_H6, pat, 1, g["lo"])
            add("r4", f"H6 Arm {arm} c=2 GAE band (Erratum V) max, {b} bits", G_H6, pat, 2, g["hi"])
        k6 = h6[f"K6/{b}"]
        add("r4", f"H6 K6 rho steps/p mean, {b} bits", H6_SUM, rf"^K6 bits {b}: rho steps/p mean {NUM}", 1, k6["mean"])
        add("r4", f"H6 K6 theory mean, {b} bits", H6_SUM, rf"^K6 bits {b}: .*/p mean {NUM}$", 1, k6["theory"])
    for b in (32, 36):
        for th in ("0.125", "0.25", "0.5"):
            x = h4[f"H4/{b}/{th}"]
            pat = rf"^H4 bits {b} theta {re.escape(th)}: trials/p {NUM} stored/p {NUM} "
            add("r4", f"H4 trials/p, {b} bits, theta={th}", H4_SUM, pat, 1, x["trials"])
            add("r4", f"H4 stored/p, {b} bits, theta={th}", H4_SUM, pat, 2, x["stored"])

    # self-check against the gate's own rule
    bad = []
    for st, hs in heads.items():
        for hd in hs:
            scale = max(abs(hd["claimed"]), abs(hd["recomputed"]), 1e-300)
            if abs(hd["claimed"] - hd["recomputed"]) > hd["rel_tol"] * scale:
                bad.append((st, hd["name"], hd["claimed"], hd["recomputed"]))
    json.dump(dict(headlines=heads, failures=bad), open(outp, "w"), indent=1)
    print({k: len(v) for k, v in heads.items()}, "failures", bad)


if __name__ == "__main__":
    main()
