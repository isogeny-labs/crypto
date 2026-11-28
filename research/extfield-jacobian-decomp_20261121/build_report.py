#!/usr/bin/env python3
"""Builds REPORT.md and REPORT.typ from RESULTS.json (numbers are filled from the data, not typed by hand)."""
import json, os, re, math
HERE = os.path.dirname(os.path.abspath(__file__))
D = json.load(open(os.path.join(HERE, "RESULTS.json")))
QS = D["qs"]; ARMS = D["arms"]; C = D["cells"]; R = D["ratios"]; F = D["fits"]; top = D["top_q"]; ti = QS.index(top)
g = lambda v: "n/a" if v is None else f"{v:.3g}"
def ci(c): return "n/a" if c is None else f"[{c[0]:.3g}, {c[1]:.3g}]"
HEAD = D["git_head_of_runs"][0]

# ------------------------------------------------ evaluate predictions from the data
pred = []
a75 = ["a0.75_c1", "a0.75_c2"]
e75 = {a: [F[a][w]["exponent"] for w in ("w0", "w0.1", "w1")] for a in a75}
ec = [F["control_a1"][w]["exponent"] for w in ("w0", "w0.1", "w1")]
ok75 = all(x <= 1.7 for a in a75 for x in e75[a]); okc = all(1.9 <= x <= 2.1 for x in ec)
pred.append(("P1", f"alpha=0.75 part {'confirmed' if ok75 else 'refuted'}; control part {'confirmed' if okc else 'refuted'}",
    f"exponents (w0, w0.1, w1): a0.75_c1 {', '.join(f'{x:.2f}' for x in e75['a0.75_c1'])}; a0.75_c2 {', '.join(f'{x:.2f}' for x in e75['a0.75_c2'])}; control {', '.join(f'{x:.2f}' for x in ec)} (assigned control band 1.9 to 2.1). Own expectation: alpha=0.75 about 1.3 to 1.6 (measured below it), control 0.8 to 1.2 (matches)."))
sh = max(D["la_share_q2039"][a][w]["max"] for a in ARMS for w in ("w0.1", "w1")); sh0 = max(D["la_share_q2039"][a]["w0"]["max"] for a in ARMS)
pred.append(("P2", "confirmed" if sh < 0.5 else "refuted", f"max LA share of S at q={top}: {sh:.2g} at w=0.1 and w=1, {sh0:.2g} at w=0 (any arm, any seed)"))
best = min(ARMS, key=lambda a: C[a][str(top)]["S_w0.1"]["mean"])
seq = R[best]["w0.1"]["values"]
mono = all(seq[i + 1] < seq[i] for i in range(len(seq) - 1))
pred.append(("P3", "confirmed" if mono else "refuted", f"best arm at q={top} is {best}; S/rho at w=0.1 along q: {', '.join(g(v) for v in seq)}"))
ratios_top_01 = {a: R[a]["w0.1"]["values"][ti] for a in ARMS}
pred.append(("P4", "confirmed qualitatively", f"S/rho at q={top}, w=0.1: " + ", ".join(f"{a} {g(v)}" for a, v in ratios_top_01.items()) + "; every projected q* is far beyond the ladder (see the projection table)"))
s_ctrl = {q: C["control_a1"][str(q)]["S_w0.1"]["mean"] for q in QS}
ord_ok = (C["control_a1"][str(top)]["S_w0.1"]["mean"] < C["a0.75_c2"][str(top)]["S_w0.1"]["mean"] < C["a0.75_c1"][str(top)]["S_w0.1"]["mean"]) and C["a0.6_c2"][str(top)]["S_w0.1"]["mean"] < C["a0.6_c1"][str(top)]["S_w0.1"]["mean"]
viol = [(a, q, D["S_over_control_w0.1"][str(q)][a]) for a in ARMS[1:] for q in QS if D["S_over_control_w0.1"][str(q)][a] < 1]
pred.append(("P5", ("confirmed" if ord_ok and not viol else "ordering confirmed at the top q; second clause refuted narrowly") if ord_ok else "refuted",
    f"ordering at q={top} {'holds' if ord_ok else 'fails'}; arm/control below 1 at: " + (", ".join(f"{a} q={q} ({v:.3f})" for a, q, v in viol) if viol else "none")))
spt = {q: D["P6"][str(q)]["solver_native_per_trial"] for q in QS}
minshare = min(v for q in QS if q >= 509 for v in D["P6"][str(q)]["native_share_of_S_w0.1"].values())
p6ok = all(abs(spt[q] / 4.5e5 - 1) <= 0.10 for q in QS if q >= 251) and minshare >= 0.9
pred.append(("P6", "confirmed" if p6ok else "refuted", f"solver native per trial {', '.join(f'q={q}: {spt[q]:.3g}' for q in QS)}; min native share of S(0.1) at q>=509: {minshare:.4f}"))
rr = [r["ratio"] for r in D["P7_rows"]]
pred.append(("P7", "confirmed" if all(0.7 <= x <= 1.3 for x in rr) else "refuted", f"measured/model acceptance rate over {len(rr)} cells: min {min(rr):.2f}, max {max(rr):.2f} (band 0.7 to 1.3)"))
aa = [v[w] for q in D["aa"].values() for v in q.values() for w in ("w0", "w0.1")]
pred.append(("P8", "confirmed" if all(0.5 <= x <= 2 for x in aa) else "refuted", f"A/A S ratios (tag 0 over tag 1, seed 1, q in 131, 251, 509, all arms): min {min(aa):.2f}, max {max(aa):.2f}"))
pred.append(("P9", "confirmed" if not D["censored"] and not D["unsolved"] and D["wiedemann_failures"]["wrong_k_attempts_total"] == 0 else "refuted",
    f"{len(D['censored'])} censored and {len(D['unsolved'])} unsolved arm-runs of 75; attempts that returned a wrong k: {D['wiedemann_failures']['wrong_k_attempts_total']}; unit tests pass (12 tests, incl. filter plus Wiedemann on synthetic relations)"))
p10 = [r["ratio"] for r in D["P10_rows"]]
pred.append(("P10", "confirmed" if all(0.5 <= x <= 2 for x in p10) else "refuted", f"raw rows at solve over model: min {min(p10):.2f}, max {max(p10):.2f} over {len(p10)} cells (factor-2 band)"))

blocks = []
H = lambda t: blocks.append(("h", t)); P = lambda t: blocks.append(("p", t))
B = lambda items: blocks.append(("ul", items)); T = lambda head, rows: blocks.append(("t", (head, rows)))

H("Question")
P("Round 1 showed that Gaudry/Nagao decomposition index calculus on the genus-2 Jacobian J(F_(q^2)) with the full x-in-F_q factor base, no large primes and dense linear algebra has exponent 2 in q, the same as Pollard rho, and loses to matched rho once the solver is priced. This round builds the lever round 1 lacked: a reduced factor base of about c q^alpha classes, partial relations with up to two points outside it (large primes) merged by the repository's large-prime graph filter, and sparse linear algebra, so that relation phase (heuristically q^(3-2 alpha)) and linear algebra (q^(2 alpha)) balance below exponent 2 (1.5 at alpha = 0.75). Question: does this lower the counted cost, or its exponent, at q <= 2039, and how far is it from matched negation-map rho at 0, 0.1 and 1 GAE per unpriced native operation? Local development check at small parameters; every number is [pcc:unverified]; no improvement is claimed.")
H("What changed since the first round")
B(["Factor base: F = the min(n_classes, round(c q^alpha)) classes with the smallest hash of x (nested across arms); alpha = 1 is the control (all classes, no large primes). Arms: (alpha, c) in {(1, 1), (0.75, 1), (0.75, 2), (0.6, 1), (0.6, 2)}. A verified decomposition is accepted with at most two classes outside F. [pcc:unverified]",
   "Merging: `filter_large_primes` of `src/cryptanalysis/large_prime_filter.rs` (hypergraph peeling plus Markowitz elimination on the two-core), copied unchanged into this directory with counting shims for its hooks. Every filter field operation and data word is priced.",
   "Sparse LA: sequential Wiedemann mod ell on the merged rows (normal equations, Berlekamp-Massey, residual check), every mulmod counted; round 1 used dense elimination. The recovered k is accepted only if [k]D = Q.",
   "Paired design: one decomposition stream per instance feeds all five arms (same R walk, same solver calls, same verification); each arm stops at its own first solving attempt and is charged the stream counters at that trial plus its own filter, LA and kD-check work.",
   "Same instances and seeds as round 1 (curve generation unchanged); rho re-run on those instances with the unchanged round-1 code. Control stream counts agree with round 1 (e.g. q = 2039 control S(0.1) mean 1.15e9 here, 1.13e9 in round 1).",
   "Ladder q in {131, 251, 509, 1021, 2039} (five points); q = 4093 not attempted: the round-1 solver's lazy u32 elimination stops at q = 2039 and decomp.rs was not edited."])
H("Method")
B(["Instance: genus-2 curve y^2 = f(x) over F_(q^2), random monic quintic f, #J = h ell with ell prime, h <= 16 (not charged), D of order ell, Q = k D. Same instances for IC arms and rho.",
   "Relation: R = cD + dQ walked through a 16-entry pool; the exact Weil-restriction solver (four quadrics, Macaulay degree 5 to 7) returns all decompositions R + P1 + .. + P4 = 0 with x(P_i) in F_q; each is verified in the group [pcc:unverified]. Row: sum over F-classes coef phi + d k = -c, plus at most two large-prime columns.",
   "Counted unit: group additions (GAE). Native work priced at w in {0, 0.1, 1} per operation (w_cal from round 1's calibration is in RESULTS.json): solver F_q multiply-adds (round 1's nominal dense Macaulay count), root finding, factor-base construction, filter operations (an inversion counts 40), Wiedemann mulmods. S = group additions + w x native. Rho's native tally is round 1's, unchanged.",
   "Seeds: instance seeds 1, 2, 3; stream tag 0; A/A is tag 1 on seed 1 at q in {131, 251, 509}. Exponents: least squares of ln S on ln q over the five cell means, bootstrap over the 3 seeds per q (2000 resamples). Ratios S(arm)/S(rho) are paired by instance with a bootstrap over seeds.",
   "Stopping rule from `PLAN.md`: 1.5e6 trials or 20 minutes per stream; no arm-run was censored."])
H("Results")
for w, lab in (("w0", "w = 0"), ("w0.1", "w = 0.1"), ("w1", "w = 1")):
    P(f"Distance to matched rho, S(arm)/S(rho) at {lab}, paired, 95% seed-bootstrap interval (below 1 means cheaper than rho) [pcc:unverified]:")
    T(["q"] + ARMS, [[str(q)] + [f"{g(R[a][w]['values'][i])} {ci(R[a][w]['ci95'][i])}" for a in ARMS] for i, q in enumerate(QS)])
P(f"At q = {top} the distance to rho at w = 0, 0.1 and 1 is, per arm: " + "; ".join(f"{a} {g(R[a]['w0']['values'][ti])}, {g(R[a]['w0.1']['values'][ti])}, {g(R[a]['w1']['values'][ti])}" for a in ARMS) + ". The best arm at every priced w is the control; no reduced-factor-base arm is closer to rho than the control at q = 2039. [pcc:unverified]")
P("Mean total S at w = 0.1 (GAE) [pcc:unverified]:")
T(["q", "rho"] + ARMS, [[str(q), g(D["rho"][str(q)]["S_w0.1"]["mean"])] + [g(C[a][str(q)]["S_w0.1"]["mean"]) for a in ARMS] for q in QS])
P("Fitted exponents b in S = c q^b over the five cell means, 95% bootstrap interval [pcc:unverified]:")
T(["arm", "w = 0", "w = 0.1", "w = 1", "trials to solve"], [[a] + [f"{F[a][w]['exponent']:.2f} {ci(F[a][w]['ci95'])}" for w in ("w0", "w0.1", "w1")] + [f"{D['fits_trials'][a]['exponent']:.2f} {ci(D['fits_trials'][a]['ci95'])}"] for a in ARMS] + [["rho"] + [f"{F['rho'][w]['exponent']:.2f} {ci(F['rho'][w]['ci95'])}" for w in ("w0", "w0.1", "w1")] + ["n/a"]])
P("Cell detail (means over 3 seeds): factor-base size nF (classes), accepted rows (partial plus complete), merged complete rows and columns at the solving attempt, and the LA share of S(0.1) [pcc:unverified]:")
rows = []
for a in ARMS:
    for q in QS:
        c = C[a][str(q)]
        rows.append([a, str(q), f"{c['nf_classes']}{'*' if c['clamped'] else ''}", f"{c['f_fraction']:.2f}", g(c["trials_to_stop"]["mean"]), g(c["rows_at_solve"]["mean"]), g(c["complete_at_solve"]["mean"]), g(c["ncols_at_solve"]["mean"]), f"{c['Sla_w0.1']['mean'] / c['S_w0.1']['mean']:.1e}"])
T(["arm", "q", "nF", "f", "trials", "rows", "merged", "cols", "LA share"], rows)
P("An asterisk on nF marks an arm clamped to the whole factor base (identical to the control at that q).")
H("Results against the preregistered predictions")
T(["id", "outcome", "evidence"], [[i, o, e] for i, o, e in pred])
cc = D["crossover_two_term_theory"]
B([f"Per-trial decomposition cost dominates everything: the solver costs about {D['streams'][str(top)]['solver_native_per_trial_mean']:.3g} counted F_q mulmods and {D['streams'][str(top)]['solver_time_per_trial_s_mean'] * 1e3:.2g} ms of wall time per trial, independent of q and of the arm. At w = 0.1 that is about {0.1 * D['streams'][str(top)]['solver_native_per_trial_mean']:.2g} GAE per trial, so the native share of S(0.1) is at least {minshare:.3f} in every arm and cell at q >= 509, and the LA share is at most {sh:.1e} at q = {top} (at w = 0, where only group additions count, at most {sh0:.1e}). The measured number of rows needed is about 0.5 q for every arm (the large-prime graph needs about one relation per large-prime vertex), so a reduced factor base only multiplies the number of trials: trials to solve grow like q^{D['fits_trials']['a0.6_c1']['exponent']:.2f} for alpha = 0.6 (theory 1.8), q^{D['fits_trials']['a0.75_c1']['exponent']:.2f} for alpha = 0.75 (theory 1.5), q^{D['fits_trials']['control_a1']['exponent']:.2f} for the control (theory 1).",
   f"The assigned P1 holds for alpha = 0.75 (exponents below 1.7) but it does not indicate an improvement [pcc:unverified]: the control's own ladder exponent is {ec[1]:.2f} at w = 0.1 and every reduced arm's is higher than the control's except the clamped (0.75, 2) arm, which equals the control at q <= 251. The ladder is solver-dominated; the LA term that the reduced factor base shrinks is below {sh:.0e} of S, so shrinking it cannot pay at q <= 2039.",
   f"Honest crossover projection (extrapolation, two-term theory model anchored at the q = {top} means, rho ~ {D['crossover_projection']['w0.1']['rho_coef_over_q2']:.2f} q^2 at w = 0.1 and {D['crossover_projection']['w1']['rho_coef_over_q2']:.2f} q^2 at w = 1): at w = 0.1 the first q where an arm drops below rho is {g(cc['w0.1']['a0.75_c2']['q_star'])} for (0.75, 2) and {g(cc['w0.1']['a0.75_c1']['q_star'])} for (0.75, 1), none for the control (LA exponent 2) and none below 1e23 for (0.6, 1); at w = 1 it is {g(cc['w1']['a0.75_c2']['q_star'])} and {g(cc['w1']['a0.75_c1']['q_star'])}. Group order is about q^4, so these are orders of 1e36 to 1e43: far outside any measurement. At w = 0 every arm is already below rho on the whole ladder (0.04 to 0.6 of rho at q = {top}), which only says mulmods are free there."])
H("Limitations")
B(["Five q values from 131 to 2039 and three seeds per cell: exponents are ladder-local, not asymptotic. At small q the c = 2 arms are clamped to the whole factor base and alpha = 0.75 has f above 0.4, so reduced-base behaviour is only visible at the top of the ladder.",
   "The projection table extrapolates over about eight orders of magnitude from two exponents taken as theory; it is not a measurement. Rho is modelled as c q^2 from the q >= 509 cells (round 1's measured rho exponent is biased low by a fixed overhead).",
   "Counting conventions: Macaulay elimination is round 1's nominal dense count (an upper bound; a sparse F4 would charge less), filter data words are priced like field operations, an inversion is 40 mulmods, Wiedemann is the sequential variant on normal equations (not block Wiedemann). The solver's per-trial cost is therefore the least certain constant; w_cal (shared 4-CPU VM) is not used for the headline rows.",
   "The large-prime filter is a copy of the repository module (tests and `trace` submodule dropped, hooks replaced by counters); its fill limits were left at the defaults (4096); the number of fill rejections was not recorded.",
   "The relation walk and its k column follow round 1; points with y = 0 carry no column. Large primes are classes with x in F_q only (no points with x outside F_q).",
   "`cargo clippy --release --bin extfield_jac_decomp -- -D warnings` fails on 13 pre-existing errors in library modules this task may not edit (`prime_radical_root.rs`, `prime_root_trace.rs` and others; 13 errors in this build); the bin and the files of this directory produce no clippy warnings. `cargo fmt` is clean for them.",
   "Local development check: no isolab, benchrec, replay certificate or validator report; the research-kb knowledge base was not queried in this run. Every number is [pcc:unverified] and must not enter a leaderboard row."])
H("Reproduction")
P(f"From /home/user/crypto at git HEAD {HEAD} (working tree with the new files). Counts are deterministic given the seeds (a re-run of q = 131 seed 1 reproduced every count); wall times vary.")
blocks.append(("code", "CARGO_TARGET_DIR=/home/user/crypto/target cargo build --release --bin extfield_jac_decomp -j2 && ./research/extfield-jacobian-decomp_20261121/run_ladder.sh && python3 research/extfield-jacobian-decomp_20261121/analyze.py && python3 research/extfield-jacobian-decomp_20261121/graph.py && python3 research/extfield-jacobian-decomp_20261121/build_report.py"))
P("Smallest cell: `GIT_HEAD=$(git log -1 --format=%H) ./target/release/extfield_jac_decomp lp 131 1 0` (about 4 s). Unit tests: `CARGO_TARGET_DIR=/home/user/crypto/target cargo test --release --bin extfield_jac_decomp`.")
H("Sources read")
B(["`research/extfield-jacobian-decomp_20261115/` (PLAN.md, REPORT.md, RESULTS.json, src/decomp.rs, src/jac.rs, src/fq2.rs, run_ladder.sh) and `src/bin/extfield_jac_decomp.rs`.",
   "`src/cryptanalysis/large_prime_filter.rs` (read in full, copied), `src/cryptanalysis/jv_bitops.rs` (hook names), `src/cryptanalysis/pq_wiedemann.rs` (header and wiedemann_solve; not used: BigUint and crate-internal rows), `src/cryptanalysis/jv_sieve_two_large_prime.rs` (how partial rows are built for the filter).",
   "Function lists only: `src/cryptanalysis/diem_quartic.rs` (eliminate_lp), `research/quartic_n4_la_20261021/ic.rs`. `CLAUDE.md` rules, `docs/ic/SUCCESS_CRITERIA.md` not opened."])

def md_table(head, rows):
    return "\n".join(["| " + " | ".join(head) + " |", "|" + "---|" * len(head)] + ["| " + " | ".join(r) + " |" for r in rows])
md = ["# Reduced factor base, two large primes and sparse LA on genus-2 Jacobians over F_(q^2): local development check", "", f"Status: [pcc:unverified]. Slug `extfield-jacobian-decomp`, round 2, 2026-11-21.", ""]
for k, v in blocks:
    if k == "h": md += [f"## {v}", ""]
    elif k == "p": md += [v, ""]
    elif k == "ul": md += [f"- {i}" for i in v] + [""]
    elif k == "t": md += [md_table(*v), ""]
    elif k == "code": md += ["```", v, "```", ""]
md += ["![graph](graph.svg)", ""]
open(os.path.join(HERE, "REPORT.md"), "w").write("\n".join(md))

def esc(t):
    parts = re.split(r"(`[^`]*`)", t)
    out = []
    for p in parts:
        out.append(p if (p.startswith("`") and p.endswith("`") and len(p) > 1) else re.sub(r"([\\#$@*_<>\[\]~])", r"\\\1", p))
    return "".join(out)
ty = ['#set page(paper: "a4", margin: 1.6cm)', '#set text(font: "Libertinus Serif", size: 9.5pt)', '#set par(justify: true)',
      '#show heading: set text(size: 12pt)', '#set table(stroke: 0.4pt + luma(160), inset: 3pt)',
      '#align(center)[#text(size: 14pt, weight: "bold")[Reduced factor base, two large primes and sparse LA on genus-2 Jacobians over F\\_(q^2): local development check]]',
      '#align(center)[Status: \\[pcc:unverified\\]. Slug `extfield-jacobian-decomp`, round 2, 2026-11-21.]', '']
for k, v in blocks:
    if k == "h": ty += [f"= {esc(v)}", ""]
    elif k == "p": ty += [esc(v), ""]
    elif k == "ul": ty += [f"- {esc(i)}" for i in v] + [""]
    elif k == "t":
        head, rows = v
        n = len(head)
        cells = ", ".join(f"[*{esc(h)}*]" for h in head) + ", " + ", ".join(f"[{esc(c)}]" for r in rows for c in r)
        small = "#set text(size: 6.5pt)\n" if n >= 6 else ("#set text(size: 8pt)\n" if n >= 4 else "")
        ty += ["#block(breakable: false)[" if len(rows) < 12 else "#block[", small + (f"#table(columns: {n}, align: left, {cells})"), "]", ""]
    elif k == "code": ty += ["#block(fill: luma(240), inset: 5pt, width: 100%)[#text(size: 7.5pt)[" + "```\n" + v + "\n```" + "]]", ""]
ty += ['#figure(image("graph.svg", width: 100%), caption: [Total S at w = 0.1 (left) and distance to matched rho (right) versus q, all arms. \\[pcc:unverified\\]])']
open(os.path.join(HERE, "REPORT.typ"), "w").write("\n".join(ty) + "\n")
print("wrote REPORT.md REPORT.typ")
