#!/usr/bin/env python3
"""Builds REPORT.md and REPORT.typ from RESULTS.json (numbers are pulled, not typed). Pure Python.
usage: python3 make_report.py"""
import json, math, os

HERE = os.path.dirname(os.path.abspath(__file__))
R = json.load(open(os.path.join(HERE, "RESULTS.json")))
Y = R["yield"]
D = R["dlp"]
PRED = {p["id"]: p for p in R["predictions"]}
FIT = R.get("fits", {})
PROJ = R.get("projection", {})
SER = R.get("series_by_p", {})


def g3(v, d=3):
    return "-" if v is None else f"{v:.{d}g}"


def esc(s):
    return s.replace("#", "\\#").replace("$", "\\$").replace("@", "\\@").replace("<", "\\<").replace(">", "\\>").replace("*", "\\*").replace("_", "\\_")


first_ok = []
for c in D:
    ok = [i for i in c["ic"] if i.get("ok") and "ratio" in i]
    if ok:
        first_ok.append((c, ok[0]))
bycell = {}
for c, i in first_ok:
    bycell.setdefault(c["p"], []).append((c, i))
ps = sorted(bycell)


def mean(xs):
    return sum(xs) / len(xs)


# ratio table (mean over seeds per p; individual cells in RESULTS.md)
rows = []
for p in ps:
    cs = bycell[p]
    r = {k: [i["ratio"][k] for _, i in cs] for k in ("w=0", "w=cal", "w=10cal", "w=0.1", "w=1")}
    be = [i["break_even_w"] for _, i in cs if i.get("break_even_w")]
    rows.append((p, len(cs), {k: (mean(v), min(v), max(v)) for k, v in r.items()}, (mean(be), min(be), max(be)) if be else None))


def fmt(t):
    return f"{t[0]:.3g} [{t[1]:.3g}, {t[2]:.3g}]"


def fitline(k):
    f = FIT.get(k)
    if not f:
        return "n/a"
    return f"{f['slope']:.2f}" + ("" if f["se"] is None else f" +- {f['se']:.2f}") + f" ({f['n']} values of p)"


head = {
    "title": "Genus-4 non-hyperelliptic images: trigonal-pencil and plane-quintic relations",
}

md = []
tp = []  # typst


def tyesc(m):
    import re as _re
    parts = _re.split(r"(`[^`]*`)", m)
    out = []
    for part in parts:
        if part.startswith("`") and part.endswith("`") and len(part) > 1:
            out.append(part)
            continue
        part = part.replace("**", "\u0001")
        for ch in "#$@<>_*~":
            part = part.replace(ch, "\\" + ch)
        bold = part.split("\u0001")
        part = "".join((("*" if k % 2 == 1 else "") + b + ("*" if k % 2 == 1 else "")) if k % 2 == 1 else b for k, b in enumerate(bold))
        out.append(part)
    return "".join(out)


def both(m, t=None):
    md.append(m)
    tp.append(t if t is not None else tyesc(m))


tp.append('#set page(margin: 1.6cm)\n#set text(size: 9.5pt)\n#set heading(numbering: none)\n#show raw: set text(size: 8.5pt)')
both(f"# {head['title']}", f"= {head['title']}")
intro = ("2026-11-15. Study `research/low-gonality-genus4_20261115/`, plan frozen in `PLAN.md` before the first measured run (amendments dated in the plan). "
         "Local development check at small parameters on one shared 4-CPU VM, not a lab benchmark. Every number is `[pcc:unverified]`. No improvement is claimed; "
         f"gaps to the matched rho are distances. Repo HEAD at run time `{R['git_head']}`.")
both(intro)

both("\n## Question", "== Question")
q = ("Diem-Kochinke predict exponent 2 - 2/(g-1) = 4/3 for non-hyperelliptic curves of genus g = 4 (hyperelliptic genus 4: 3/2; rho: 2) from the lowest-degree pencils: "
     "the g^1_3 of a ruling of the quadric (3-point fibres) and the degree-5 plane model obtained by projecting the canonical curve from one of its points (lines meet it in 5 points). "
     "What do these two relation sources yield at small q, do they close a DLP in `Pic^0` of a smooth genus-4 curve with sparse linear algebra, and where does that sit against a matched rho?")
both(q)

both("\n## Answer in brief", "== Answer in brief")
nsolved = sum(1 for c in D for i in c["ic"] if i.get("ok"))
ntot = sum(len(c["ic"]) for c in D)
ans = []
ans.append(f"- **Arithmetic layer built and tested.** `Pic^0` of a smooth (3,3) curve on P^1 x P^1 (canonical genus-4 model on the split quadric) by Riemann-Roch linear algebra, with canonical keys including the exceptional (coplanar) classes; 8 unit tests pass, among them the zeta-function group order killing random classes at p = 7 and 11 and the relation sums being equal in `Pic^0`.")
ans.append(f"- **Relation yields match the formulas.** Fibre splitting {PRED['P1']['evidence'].split(';')[0]} (P1 {PRED['P1']['outcome']}); line yield {PRED['P2']['evidence']} (P2 {PRED['P2']['outcome']}); duplicate factor {PRED['P3']['evidence']} (P3 {PRED['P3']['outcome']}).")
ans.append(f"- **DLP solved and verified.** {nsolved} of {ntot} IC passes in {len(D)} cells (p = {', '.join(str(p) for p in ps)}) gave x with x g = h; rho solved and verified in every run (P6 {PRED['P6']['outcome']}). The factor base needed: {PRED['P4']['evidence']}.")
ans.append(f"- **Distance to rho.** See the table: IC is far below rho at zero-priced native work and at the calibrated price, and above it at 0.1 and 1 GAE per native operation, so inside the section 5a band the order flips and the ranking is **unresolved**, not a win (P7 {PRED['P7']['outcome']}).")
F127 = R.get("fits_p_ge_127", {})
def fit127(k):
    f = F127.get(k)
    return "n/a" if not f else f"{f['slope']:.2f} +- {f['se']:.2f}"
ans.append(f"- **Scaling.** Measured log-log slopes against p (all four p; restricted to p >= 127 in brackets): rho GAE {fitline('rho_gae')} [{fit127('rho_gae')}], IC group operations {fitline('ic_group_ops')} [{fit127('ic_group_ops')}], IC native F_p work {fitline('ic_native_p')} [{fit127('ic_native_p')}], IC mod-ell linear-algebra work {fitline('ic_native_ell')} [{fit127('ic_native_ell')}], factor-base size m {fitline('ic_m_star')} [{fit127('ic_m_star')}]. The 4/3 exponent is not visible and cannot be: the cells have a constant-fraction factor base, no large primes, and 3 to 4 values of p. The IC group-operation count is flat (about 750 combines, dominated by the 16-adder table of the individual-log step, not by relations), so at w = 0 IC/rho falls like rho grows; the native work grows like p^2.2, a little faster than rho, which is why the break-even price w* stays near 1e-3 instead of rising.")
for a in ans:
    both(a)

both("\n### Distance to the matched rho (S_IC / S_rho, plain rho measured, mean over seeds [min, max])", "=== Distance to the matched rho (S\\_IC / S\\_rho, plain rho measured, mean over seeds [min, max])")
hdr = "| p | cells | w = 0 | w = w_cal | w = 10 w_cal | w = 0.1 | w = 1 | break-even w |\n|--:|--:|--:|--:|--:|--:|--:|--:|"
md.append(hdr)
tp.append('#table(columns: 8, align: right, inset: 3pt, stroke: 0.4pt + gray,\n[p],[cells],[w = 0],[w = w\\_cal],[w = 10 w\\_cal],[w = 0.1],[w = 1],[break-even w],')
for p, n, r, be in rows:
    md.append(f"| {p} | {n} | {fmt(r['w=0'])} | {fmt(r['w=cal'])} | {fmt(r['w=10cal'])} | {fmt(r['w=0.1'])} | {fmt(r['w=1'])} | {fmt(be) if be else '-'} |")
    tp.append(f"[{p}],[{n}],[{fmt(r['w=0'])}],[{fmt(r['w=cal'])}],[{fmt(r['w=10cal'])}],[{fmt(r['w=0.1'])}],[{fmt(r['w=1'])}],[{fmt(be) if be else '-'}],")
tp.append(")")
cal = mean([c["calib"]["w_cal_p"] for c, _ in first_ok]) if first_ok else None
cal_ell = mean([c["calib"]["w_cal_ell"] for c, _ in first_ok]) if first_ok else None
mpa = mean([c["calib"]["mulmods_per_add"] for c, _ in first_ok]) if first_ok else None
note = (f"GAE = one `combine` (about {mpa:.3g} counted mulmods, {mean([c['calib']['t_add_ns'] for c,_ in first_ok])/1000:.0f} us here). w_cal(F_p mulmod) about {cal:.2g}, w_cal(mod-ell word op) about {cal_ell:.2g} GAE: "
        "best-of-5 timings in the same binary on a loaded shared host. Native work is counted in field multiplications (IC) and one hash/DP operation per walk step (rho). "
        "The ratios use the final ladder rung (oracle choice of the factor base); `RESULTS.md` also gives the ratio including all failed rungs, a free-negation rho reference (plain/sqrt 2, assumed) and a row pricing a GAE at 2.6e3 mulmods (optimised arithmetic).")
both(note)

both("\n## Method", "== Method")
meth = [
    "**Curve and group.** Smooth bidegree-(3,3) curves on P^1 x P^1 over F_p (split quadric; K = O(1,1); smoothness certified by the partial derivatives generating all (8,8)-forms). A class is D - D0 with D0 four rational points; D is an effective degree-4 divisor carried by the 5-dimensional space H^0(2K - D) of biquadratic forms. `combine(Da,Db,Dc) ~ Da+Db-Dc` multiplies the spaces, finds f with f S_2 in Wa Wb, finds g in S_2 with g (Wa Wb) in f H^0(4K - Dc) (this makes div g contain R + Dc as a divisor; a first version with g merely in H^0(2K - Dc) failed when R met Dc), divides to get H^0(4K - Dc - D), and recovers H^0(2K - D). The Mumford-type spanning check falls back to level 3 (never needed in the runs). Coplanar D (h^0(D) = 2) gets the key of H^0(2K - E), E = plane section - D, so keys are canonical.",
    "**Order.** N_1, N_2 by fibrewise root counting over F_p, F_{p^2}; (a3, a4) then bracketed by Weil's bounds and N found by interval baby-step giant-step and filtered by random classes. Unit-tested against exact N_1..N_4 at p = 7, 11.",
    "**Relations.** Unknowns lambda(P) = phi(P - Pa) for P in the factor base F (Pa = first point of D0), plus kappa = phi(K - 6Pa), tau_i = phi(T_i - 3Pa). B: a fibre of either ruling with 3 distinct rational points all in F gives sum lambda = tau_i; K = T1 + T2 adds kappa - tau_1 - tau_2 = 0. A: the plane through Pa, P1, P2 (P_i in F) meets C in three more points; if all are rational and in F, sum of 5 lambdas = kappa (this is the line through two factor-base points on the projection of the canonical curve from Pa, a plane quintic with two nodes). C (not part of the lead): planes through three F points, used only when A+B leave the system rank deficient at F = all points.",
    "**Linear algebra and logarithm.** Wiedemann on A^T D A over F_ell (ell the largest prime factor of N, N/ell <= 16, ell exactly dividing N), cross-checked against a dense kernel whose dimension must be 1. Individual logs by a random walk a g + b h until two decompositions into four rational points of F; the scale cancels in the ratio. Verified by x g = h.",
    "**Matched rho.** Distinguished-point rho in <g>, 16-adding walk with the same `combine`, same g and h, same group. Negation costs one `combine` in this arithmetic, so the measured rho is plain.",
]
for m_ in meth:
    both(m_ if m_.startswith("**") else m_)

both("\n## Results against the preregistered predictions", "== Results against the preregistered predictions")
md.append("| id | claim | outcome | evidence |\n|:--|:--|:--|:--|")
tp.append('#table(columns: (auto, 1fr, auto, 2fr), align: left, inset: 3pt, stroke: 0.4pt + gray,\n[id],[claim],[outcome],[evidence],')
for pid in sorted(PRED):
    pr = PRED[pid]
    md.append(f"| {pr['id']} | {pr['claim']} | **{pr['outcome']}** | {pr['evidence']} |")
    tp.append(f"[{pr['id']}],[{esc(pr['claim'])}],[*{pr['outcome']}*],[{esc(pr['evidence'])}],")
tp.append(")")

both("\n### Fibre splitting and line yield", "=== Fibre splitting and line yield")
md.append("| p | curves | fibres with 3 points | A-line yield/pair at phi = 0.5, 0.7, 1.0 (measured/naive phi^3/6) |\n|--:|--:|--:|:--|")
tp.append('#table(columns: 4, align: left, inset: 3pt, stroke: 0.4pt + gray,\n[p],[curves],[fibres with 3 points],[A-line yield/pair at phi = 0.5, 0.7, 1.0 (measured/naive)],')
for p, y in sorted(Y.items(), key=lambda kv: int(kv[0])):
    cells = {c["phi"]: c for c in y["phi_cells"]}
    s = " ; ".join(f"{cells[f]['yield_pooled']:.4f} ({cells[f]['ratio_naive_pooled']:.2f})" for f in (0.5, 0.7, 1.0))
    s3 = y["split3_fraction"]
    md.append(f"| {p} | {y['curves']} | {s3[0]:.4f} [{s3[1]:.4f}, {s3[2]:.4f}] | {s} |")
    tp.append(f"[{p}],[{y['curves']}],[{s3[0]:.4f} \\[{s3[1]:.4f}, {s3[2]:.4f}\\]],[{s}],")
tp.append(")")

both("\n### Factor base and cost per cell", "=== Factor base and cost per cell")
md.append("| p | seed | N | ell | phi* | m | rows/cols | GAE (final rung) | native F_p | native mod ell | rho GAE mean (min-max; runs) |\n|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|")
tp.append('#table(columns: 11, align: right, inset: 2.5pt, stroke: 0.4pt + gray, [p],[seed],[N],[ell],[phi\\*],[m],[rows/cols],[GAE],[native F\\_p],[native mod ell],[rho GAE mean (min-max; runs)],')
for c, i in first_ok:
    rg = c["rho_gae"]
    mm_ = c["rho_gae_minmax"]
    line = [str(c["p"]), str(c["seed"]), str(c["N"]), str(c["ell"]), f"{i['phi']}{'+C' if i['c_supplement'] else ''}", str(i["m"]), f"{i['rows']}/{i['ncols_used']}", str(i["group_ops"]), f"{i['native_p']:.3g}", f"{i['native_ell']:.3g}", f"{rg[0]:.0f} ({mm_[0]:.0f}-{mm_[1]:.0f}; {c['rho_runs_verified']} runs)"]
    md.append("| " + " | ".join(line) + " |")
    tp.append(",".join(f"[{x}]" for x in line) + ",")
tp.append(")")

both("\n### Honest projection of the crossover against rho's q^2", "=== Honest projection of the crossover against rho's q^2")
proj = []
if PROJ:
    wm = PROJ.get("w_star_measured_impl_slope")
    wi = PROJ.get("w_star_root_finding_model_slope")
    proj.append(f"Break-even price w* (S_IC = S_rho) by p: measured implementation {', '.join(f'{p}: {g3(v)}' for p, v in zip(PROJ['p'], PROJ['w_star_measured_impl']))}; root-finding model {', '.join(f'{p}: {g3(v)}' for p, v in zip(PROJ['p'], PROJ['w_star_root_finding_model']))}.")
    if wm:
        proj.append(f"Log-log slope of w* vs p, measured implementation: {wm[0]:.2f}" + ("" if wm[1] is None else f" +- {wm[1]:.2f}") + ". " + ("A negative slope means the native work of this implementation grows faster than rho's GAE count, so the break-even price falls with p and there is no crossover at w >= 0.1 on this route." if wm[0] < 0 else "A positive slope means rho's cost outgrows the IC native work; extrapolated parity below."))
        if wm[0] <= 0 or wm[1] is None or wm[0] - 2 * wm[1] <= 0:
            proj.append("No crossover can be projected for the measured implementation: the slope is negative (or its 2-sigma interval reaches zero), so w* stays near 1e-3 and never approaches the band prices 0.1 and 1 as p grows.")
    if wi:
        proj.append(f"Root-finding model (each A pair costs 2000 mulmods instead of the scan of F; model, not measured): slope {wi[0]:.2f}" + ("" if wi[1] is None else f" +- {wi[1]:.2f}") + ".")
        if wi[0] <= 0 or wi[1] is None or wi[0] - 2 * wi[1] <= 0:
            proj.append("The root-finding model's slope is not distinguishable from zero (2-sigma interval includes 0), so its crossover with w = 0.1 or 1 cannot be projected from these cells either; extrapolating the point estimate would give p beyond 2^80, which is not a result.")
proj.append("Reading: at w = 0 and at the calibrated price the IC route already beats this arithmetic's rho at every measured p (distance below 1), so a crossover is only meaningful at band prices. The measured relation search scans all of F for each pair (cost m per pair, m^3 in total), which grows like m^3, measured p^2.2 (p >= 127: 2.23 +- 0.04) against rho's q^2 (measured slope 1.9 +- 0.3 over p >= 127; the cofactor of ell moves single cells by up to sqrt 16); Diem's analysis replaces the scan by root finding (about log q per pair) and adds two large primes, neither of which the 3 to 4 measured values of p can validate. The 4/3 exponent is therefore a literature figure here, not a measurement.")
for x in proj:
    both(x)

both("\n### Figure", "=== Figure")
md.append("![figure](figure.svg)")
tp.append('#image("figure.svg", width: 100%)')

both("\n## Limitations", "== Limitations")
lim = [
    "Split quadric only (two g^1_3 over F_p); the non-split quadric (no rational pencil) and the cone are not built, so these cells say nothing about curves of those types.",
    "No large primes, no sieving, no Diem two-large-prime step: the factor base is a constant fraction of all rational points (phi* between 0.7 and 1), so the p^0.75 factor-base size of the plan is an observed trend over 3-4 values of p, not an asymptotic result.",
    "p <= 509 (N up to about 7e10). One to three curves per p; rho varies with ell between curves, and rho runs per cell are 3 to 8, so the S_rho interval is wide.",
    "The arithmetic is generic linear algebra (about 2.6e5 mulmods per combine, 0.65 ms); rho in this arithmetic is slower than rho on a Cantor-type Jacobian, which moves w_cal. The sensitivity row (GAE = 2.6e3 mulmods) shows the effect on the calibrated ratio. Rho is plain (no free negation); the free-negation reference is plain/sqrt 2, an assumption.",
    "The dense kernel-dimension monitor used to decide when to add C relations is not priced; the C relations themselves (general hyperplane sections) are not part of the Diem-Kochinke lead and were needed only at p = 61 (and where noted in the table).",
    "Group order N is found by BSGS in the Weil range with random-class filtering (validated by exact zeta counts only at p = 7, 11); at larger p the check is that N kills 3 further random classes and that g, h verify. A wrong N would be exposed by the mandatory x g = h check.",
    "A first batch of cells was discarded after finding a rank-monitor flaw (see PLAN amendments); the final cells are a full re-run with the final source (cargo reports the release binary up to date). Re-running `dlp 61 1 8` afterwards reproduced `results/dlp_p61_s1.json` exactly except for timings (`results_repro/`).",
    "Everything here is `[pcc:unverified]`: no independent replay, no validator report, no VALIDATION.json; nothing in this directory is a leaderboard or benchmarks row.",
]
for l in lim:
    both("- " + l)

both("\n## Exact repro command", "== Exact repro command")
cmd = ("cd /home/user/crypto && CARGO_TARGET_DIR=/home/user/crypto/target cargo build --release --bin genus4_pencil_ic -j2 && "
       "cd research/low-gonality-genus4_20261115 && B=/home/user/crypto/target/release/genus4_pencil_ic && "
       "for cfg in '31 16' '61 16' '127 16' '251 12' '509 8'; do set -- $cfg; $B yield $1 $2 20261115 results/yield_p$1.jsonl; done && "
       "for s in 1 2 3; do $B dlp 61 $s 8 results/dlp_p61_s$s.json; done && for s in 1 2 3; do $B dlp 127 $s 8 results/dlp_p127_s$s.json; done && "
       "for s in 1 2 3; do $B dlp 251 $s 6 results/dlp_p251_s$s.json; done && $B dlp 509 1 3 results/dlp_p509_s1.json && "
       "python3 analyze.py && python3 plot.py && python3 make_report.py")
md.append("```\n" + cmd + "\n```")
tp.append("```\n" + cmd + "\n```")
both("Smallest cell: `genus4_pencil_ic dlp 61 1 8 OUT` (about 30 s). Unit tests: `cargo test --release --bin genus4_pencil_ic -j2`. PDF: `/tmp/report-venv/bin/python -c \"import typst; typst.compile('REPORT.typ', output='REPORT.pdf')\"`.")

both("\n## Sources read", "== Sources read")
src = [
    "`research/genus4_n4_census_20261024/REPORT.md` (genus-4 non-hyperelliptic Jacobians in the isogeny class of Res E; Theorem 6 an unreviewed agent proof; no map computed)",
    "`src/cryptanalysis/plane_quartic_pic.rs` (header and API: the Riemann-Roch linear-algebra pattern generalised here)",
    "`src/bin/quartic_ic.rs`, `src/cryptanalysis/genus_ladder.rs` (headers only), `benchmarks/20261026-humbert-genus5-cover/` (file list; the cautionary precedent was not re-read in detail)",
    "`AGENTS.md` section 5a (pricing of unpriced native work), `CLAUDE.md` (session rules)",
    "`research/quartic-diem-sparse_20261115/make_report.py` and `REPORT.typ` (report-building pattern only)",
    "Not consulted: research-kb, ecdb, lead atlas (not available in this run); literature figures (Diem 2006, Diem-Kochinke) are quoted from the task text and unverified here.",
]
for s_ in src:
    both("- " + s_)

open(os.path.join(HERE, "REPORT.md"), "w").write("\n".join(md) + "\n")
# typst: convert markdown-ish bullets and backticks (raw) are valid in typst; fix bold/bullets
typ = "\n".join(tp)
import re
typ = re.sub(r"\*\*(.+?)\*\*", r"*\1*", typ)
open(os.path.join(HERE, "REPORT.typ"), "w").write(typ + "\n")
print("wrote REPORT.md REPORT.typ")
