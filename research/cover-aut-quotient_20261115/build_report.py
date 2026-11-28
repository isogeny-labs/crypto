#!/usr/bin/env python3
"""REPORT.md and REPORT.typ from RESULTS.json (run analyze.py first)."""
import json, os
D = os.path.dirname(os.path.abspath(__file__))
R = json.load(open(D + "/RESULTS.json"))
bp = R["by_prime"]
pr = R["paired"]
narm = R["n_arm_rank_analysis"]
NM = R["n_arm_rank_multi"]
RM = R["rho_matched"]
KJ = R["kummer_j_check"]
pp = RM["by_prime_pooled_ref"]
def prow(p):
    d = pp[str(p)]
    return f"| {p} | {d['n_instances']} | " + " | ".join(f"{d['geo_w'+w]:.3f} [{d['ci_w'+w][0]:.3f}, {d['ci_w'+w][1]:.3f}]" for w in ("0", "0.1", "1")) + " |"
prows = "\n".join(prow(p) for p in sorted(map(int, pp)))
def irow(c):
    return f"| {c['p']} | {c['seed']} | {c['rho_steps_per_sqrt_l']:.3f} | " + " | ".join(f"{c['S_over_rho_w'+w]:.3f} [{c['ci_w'+w][0]:.3f}, {c['ci_w'+w][1]:.3f}]" for w in ("0", "1")) + " |"
irows = "\n".join(irow(c) for c in RM["cells"])
nmrows = "\n".join(f"| {r['seed']} | {r['B']} | {r['rank_and_columns_at_420']['plain'][0]} of {r['unknowns_N']} | {r['first_k_d_determined']['plain']} | {r['first_k_d_determined']['iota']} | {r['first_k_d_determined']['quot']} |" for r in NM)
rnd = {(r["kummer"]): r for r in R["census_random"]}
ctl = R["controls"][0]
cells = ", ".join(f'p={x["p"]} s{x["seed"]}' for x in pr)
def row(p):
    b = bp[str(p)]
    return f'| {p} | {b["n"]} | {b["geo_S"]:.4f} | {b["geo_S_over_negrho_w0"]:.3f} (range {b["range_w0"][0]:.3f} to {b["range_w0"][1]:.3f}) | {b["geo_S_over_negrho_w0.1"]:.3f} | {b["geo_S_over_negrho_w1"]:.3f} |'
rows = "\n".join(row(p) for p in sorted(map(int, bp)))
inst = R["census_instances"]
n_inst = len(inst)
all_trivial = all(c["census"]["linear_nontrivial"] == 0 and c["census"]["semilinear_total"] == 0 for c in inst)
eq = all(x["rows_Q_equal_rows_A"] for x in pr)
big = max(map(int, bp))
md = f"""# Automorphism quotient of the factor base on Joux-Vitse cover images: REPORT

All numbers are [pcc:unverified]; this is a local development check at small parameters, not a lab benchmark.
No improvement is claimed. Repo HEAD 3c24520f7cdc; bin `src/bin/cover_aut_quotient.rs`, helpers in `src/`.

## Question

The lead proposed quotienting the Nagao factor base of the genus-3 cover H/F_(p^2) of E(F_(p^6)) by an
automorphism group G of order n, to divide unknowns by |G|, relations by |G| and linear algebra by |G|^2,
assuming (besides the hyperelliptic involution iota) an automorphism of order 3 or 6 coming from
Gal(F_(p^6)/F_p). Is there such a group on the instances the Nagao arm solves, and what does quotienting give?

## Method

1. Census. For each instance `generate_spec(p, seed)` (p in 503, 1009, 1301, 1451, 1601, seeds 1 to 3) take
   h = F*N (monic septic, so infinity is a branch point) and search exhaustively every Moebius map over F_q
   (q = p^2) that sends the branch set of y^2 = h onto that of y^2 = h (linear case) or of y^2 = h^tau
   (semilinear case, tau the p-Frobenius). Affine candidates are all q-1 values of s (the translation is forced by the x^6
   coefficient); pole candidates x -> u + k/(x-v) are enumerated for the rational roots u, v and all k.
   Completeness argument: a map usable on the factor base must be defined over F_q, so it permutes the F_q-rational
   branch points, so it is affine or of pole type (PLAN.md). kappa (the scalar in H(mu x)(cx+d)^8 = kappa h(x)) is recorded
   with its squareness. iota acts on Phi(G), Phi(Q) as lambda = l-1: verified by neg(Phi G) = [l-1] Phi G.
2. Quotient layer. `orbit.rs` builds orbits of the signed factor base under generators (permutation, scalar lambda),
   with log P_s = coef[s] log rep, and rewrites a relation through it. Unit-tested on a synthetic group of order 6
   (iota and a cube root of unity): unknowns divide by exactly 6 and the hidden logs are recovered.
3. Paired Nagao arm. `paired` is the library's relation collection (same residual stream, F4 with degree 24, factor-base
   line search with 65536 lines, JV_FB_LINE config of research/jv_cover_fbline_20261102) with two row builders
   fed by the same decompositions: A, the library's rows (iota already folded in: one unknown per class, coefficient 1 or l-1);
   Q, rows built through the orbit layer from G = {{1, iota}}. LA is Wiedemann on the square core, as in the library.
   S is total F_p multiplications / (c_e sqrt l) (group-addition units). S/negation-rho divides by the rescore
   protocol's pooled reference 0.9322 (U1), and priced at w = 0, 0.1, 1 per native operation:
   (S + w*native/sqrt l) / (0.9322 + w*15.81).

## Results against the preregistered predictions

**P0 (structure): confirmed.** {n_inst} instances, all with the identity as the only linear map and no semilinear map
(`all_trivial = {all_trivial}`), deg h = 7 on all. iota acts as lambda = l-1 on Phi(G) and Phi(Q) on all. Controls:
planted h = x^7 + 5x at p = 1301 returned {ctl["planted_x7_plus_5x"]["linear_total"]} maps, not the 6 preregistered:
the reduced automorphism group of y^2 = x(x^6 + c) is dihedral of order 12 (6 affine plus 6 pole-type x -> c'/x), so
the detector found the whole group and my "exactly 6" expectation was wrong; {ctl["planted_x7_plus_5x"]["fp_line_preserving"]} of the 12 preserve P^1(F_p).
A random septic: identity only. Random alpha at p = 101: {rnd[False]["n_alpha"]} alpha, {rnd[False]["with_nontrivial_linear_aut"]} with a nontrivial linear
automorphism (Wilson 95% upper bound about 0.1%); {rnd[False]["with_semilinear"]} had a semilinear map (small-field coincidences at q = 10201, none at the 15 instances).
Kummer control (alpha^3 in F_q, p = 103 = 1 mod 3, {rnd[True]["n_alpha"]} alpha): all {rnd[True]["with_nontrivial_linear_aut"]} have a cyclic group of order 3 (x -> omega x), square kappa,
preserving the F_p line: the detector sees the effect when it exists, and a G of order 6 does exist on that subclass.
That subclass is not an instance class: alpha^3 in F_q makes E the base change of a curve over F_q, so #E(F_(q^3)) is
divisible by #E'(F_q) > 4 for q large and is never 4 times a prime (numerically checked: a2^2/a4 = (1+omega)^2/omega, so j(E) lies in F_p, on {KJ["kummer_j_in_fp"]} of {KJ["n_alpha"]} Kummer alpha at p = 103; a j in F_p curve over F_(p^6) is a twist of a base change of a curve over F_p, hence its order is divisible by an F_p curve order of size about p, not 4 times a prime).
So the premise "an automorphism of order n from the field extension" fails for the weak class used by the Nagao arm:
sigma sends E to E^sigma, a different curve. G = {{1, iota}}, |G| = 2.

**P1 (unknowns): confirmed, trivially.** Q rows equal A rows on every cell (`{eq}`), identical relation digest, same unknowns
(B+1 classes, core 715 at p = 1301 s1). The orbit layer maps 2B signed ids to B orbits (reduction 2.000), which is exactly what the
repo already does. The lead's "unknowns shrink by |G| for |G| >= 6" is untestable on this class.

**P2/P3 (relation phase, overall S): the quotient arm equals the baseline; the arm-N prediction is refuted.**
S(Q)/S(A) = 1.000 on every cell, relation-phase ratio 1.000 (identical rows). The preregistered unquotiented arm N (signed
unknowns, no iota information) could not be measured as planned: at p = 503 it ran 45 min with no result and was stopped
(wall-clock cut, see Limitations); the p = 251 re-run never determined d, through 922 relations. Dumping the first 420 relations
at p = 251 s1 and doing exact linear algebra over F_l shows why: the system has rank {narm["prefix"][-1]["rank_plain"]} of {narm["unknowns_N"]}
with d the free column, and the kernel has constant symmetric part and an antisymmetric part tied to the class logs. With the
B free identification rows x_(i,+) + x_(i,-) = 0 added, d is determined at {narm["first_relations_d_determined"]["with_iota_rows"]} relations
(grid step 10) against 154 for arm A. So the iota quotient is not a 2x saving of relations; it is a necessary relation,
free to add, and an unquotiented system with it needs the same B+1 decomposition relations and only a larger LA dimension
(2B+1 against B+1, a factor 4 in a dense or Wiedemann n^2 cost, but LA is under 0.1 percent of S here: S_la {pr[0]["S_la"]:.5f} against S {pr[0]["S"]:.3f}).
This also corrects the lead's accounting: relations needed scale with the number of orbit unknowns, so an extra automorphism group
of order n would cut the relations needed by about n relative to the iota-only baseline, and nothing else; it is a model for a class that has no instances.

**P2 arm-N rank analysis on three instances.** The p = 251 analysis was extended from seed 1 to seeds 1, 2, 3 (first 420 relations of each, exact linear algebra over F_l, grid step 10; `rank_n_multi.py`):

| seed | B | rank of plain N at 420 | first k, plain N | first k, N + B iota rows | first k, orbit-quotient system (arm A rows) |
|---|---|---|---|---|---|
{nmrows}

On all three instances plain N has rank 2B (one less than its 2B+1 unknowns, d free, never determined), and N plus the free iota rows determines d at
exactly the same prefix as the orbit-quotient system. So the earlier n = 1 finding holds on n = 3: iota is a necessary free relation, not a 2x saving. (Arm N as a timed run was still not measured.)

**P4 (distance to rho).** Cells completed: {cells} (seed 3 added in this revision). Geometric mean over seeds of S/negation-rho, reference = the rescore's pooled 0.9322 / 15.81:

| p | cells | geo-mean S | S/negrho, w=0 | w=0.1 | w=1 |
|---|---|---|---|---|---|
{rows}

At the largest prime measured ({big}) the point estimate is below 1 at all three prices ({bp[str(big)]["geo_S_over_negrho_w0"]:.3f}, {bp[str(big)]["geo_S_over_negrho_w0.1"]:.3f}, {bp[str(big)]["geo_S_over_negrho_w1"]:.3f}): P4 holds on the point estimate.
My prediction that 1301 and 1451 would be above 1 is refuted: I took the rescore's cover records (1.405, 1.160, 0.870) to be this
configuration; they are the plain model (results/inputs/cover/dlp_p1301_s1.json has no fb_line, base 634, S 1.494), and the factor-base-line config used here (base 728 at the same instance, S 0.729) is the 1.71x one of jv_cover_fbline_20261102, so
S/rho is lower. Cover native operations are `f4_native_ops`, a lower bound (F_p additions and memory not emitted), so an "below" verdict could flip under
complete counts. These are measured distances, not an improvement claim of this lead: the quotient changed nothing.

Matched rho on the same instances (added in this revision, `rho_matched.py`, `rho_matched.json`). The rescore's 12 negation-map rho runs per instance (`research/jv_cover_negrho_rescore_20261101/results/new/rhostrong_p*_s*_run*.json`, same E(F_p) instances: l and c_add_e asserted equal to this study's records in all 9 cells) give per-instance and pooled references, priced at w = 0, 0.1, 1 with the rho native count (canon_cmp + hash + dp_check + scalar_ops + table_ops + cycle_cmp) per sqrt l. Pooled reference: {RM["pooled_rho_n"]} runs, {RM["pooled_rho_steps_per_sqrt_l"]:.4f} steps/sqrt l, native {RM["pooled_rho_native_per_sqrt_l"]:.2f}. Bootstrap (20000) over rho runs and over the 3 instances; with 3 instances the cover-side spread is under-covered:

| p | instances | S/rho w=0 [95%] | w=0.1 [95%] | w=1 [95%] |
|---|---|---|---|---|
{prows}

Per instance against that instance's own 12 rho runs (interval over those 12 runs only; w = 0 and w = 1 shown):

| p | seed | rho steps/sqrt l | S/rho w=0 [95%] | w=1 [95%] |
|---|---|---|---|---|
{irows}

The per-instance intervals are wide (12 rho runs, rho run-to-run sd about half its mean) and three of nine include 1 at w = 0 or w = 1; the pooled reference uses 108 runs and is the primary because rho expected cost does not depend on the instance (homogeneity across p was not rejected in the rescore, permutation p = 0.314) and the per-instance reference means span 0.80 to 1.14 purely by sampling noise. A/A check: the odd/even split of each instance's rho runs gives ratios 0.64 to 1.20 (stored per cell), which is the noise floor of a 6-run half and is why per-instance references are not used as the headline. Distance remaining to the matched rho: none at the point estimate at p = 1301, 1451, 1601 under the pooled reference, with upper interval ends below 1 at all three prices (0.94 at p = 1301, w = 1); under the per-instance references the interval at p = 1301 s1 and s2 reaches above 1. This is a property of the factor-base-line configuration already in the repo, not of the quotient. It is not validated: the cover native count is a lower bound (F_p additions, pair selection and memory traffic are not emitted by the library, so complete pricing at 0, 0.1, 1 is not possible without changing it), there is no replay certificate or validator report, and the pooled reference mixes the 9 instances.

## Verifier disagreement

The skeptic's major issue says the rho reference "was not re-run on these fbline-model instances". Partly wrong: the rho runs of the rescore are on exactly these E(F_p) instances (generate_spec(p, seed), l and c_add_e equal in all 9 cells, asserted in `rho_matched.py`); the cover configuration (plain model versus factor-base line) changes only the cover arm, never the group or rho's cost, so re-running rho would only add more samples of the same distribution. What was right and is fixed: there was no per-instance matching, no interval and no seed 3. Those are added above, and the headline numbers moved only because seed 3 was added (e.g. p = 1301 w = 0: 0.809 with 2 seeds to 0.791 with 3), not because of a disagreement. What is not fixable here: complete native-operation pricing (the library does not emit cover F_p additions) and a replay certificate or validator report (forbidden here: no benchrec, isolab or ecbench). The summary phrase "remaining distance is none at the largest prime" is kept only as a measured point estimate of the baseline config.

**P5 (replay): confirmed.** Arm A at p = 1301 s1 reproduces the recorded line_p1301_s1 record of research/jv_cover_fbline_20261102:
relation digest 4198168094706900228, 729 relations, 275600 residuals, 715 unknowns, 13297571 LA ops, S 0.72877507 against 0.72877516.

## Limitations

* All three seeds ran at p = 1301, 1451, 1601 ({cells}). The arm-N timing runs at p = 503 and 251 were stopped; arm N was not measured as a timed run, only by exact rank analysis on 3 instances (p = 251, seeds 1 to 3, 420 relations each). The cover side of every ratio is one deterministic run per instance, so intervals over only 3 instances under-cover.
* G was only searched for automorphisms of the genus-3 curve over F_q usable on an F_p line; automorphisms over extensions of F_q are irrelevant to the F_p-line factor base.
  The census is exhaustive over the candidate maps in the PLAN argument, whose completeness rests on F_q-rationality of a map preserving P^1(F_p) and commuting with the q-Frobenius.
* No LA-phase model was fitted; LA is under 0.1 percent of S in this config.
* Nothing here enters the leaderboard, benchmarks or a claim; no replay certificate or validator report was produced for these cells; a matched-rho reference with a bootstrap band is given above (section 5a band is the w = 0, 0.1, 1 columns), with the cover native count a lower bound.
* GHS binary covers (secondary target) were not touched.

## Repro

`cd /home/user/crypto && CARGO_TARGET_DIR=/home/user/crypto/target cargo build --release --bin cover_aut_quotient -j2 && research/cover-aut-quotient_20261115/run.sh && python3 research/cover-aut-quotient_20261115/analyze.py && python3 research/cover-aut-quotient_20261115/build_report.py`
(census commands: `cover_aut_quotient census-inst --sizes 503,1009,1301,1451,1601 --seeds 3`, `census-random --p 101 --n 4000`, `census-random --p 103 --n 300 --kummer`, `control`;
rank analysis: `python3 rank_n_arm.py`, `DUMP_N_ROWS=results/naive_p251_s<k>_first420_rows.json target/release/cover_aut_quotient paired --p 251 --seed <k> --naive --out /tmp/x.json` then `python3 rank_n_multi.py`; matched rho: `python3 rho_matched.py`; seed 3 cells: `cover_aut_quotient paired --p <p> --seed 3 --out results/paired_p<p>_s3.json`). Unit tests: `cargo test --release --bin cover_aut_quotient`.

## Sources read

src/cryptanalysis/jv_cover.rs; src/cryptanalysis/gaudry_cubic.rs (square_core, wiedemann_u64); research/jv_cover_negrho_rescore_20261101/{{README.md,fit.json}};
research/jv_cover_fbline_20261102/results/line_p1301_s1.json; research/jv_cover_negrho_rescore_20261101/results/new/rhostrong_p*_s*_run*.json; research/jv_nagao_hilbert_20261112/{{README.md,results/walls.txt}}; research/jv_sieve_prospective_20261023/PROTOCOL.md;
examples/jv_endo_check.rs; papers/ecdlp-index-calculus-treatise/survey/transfer.md (section 17 heading only).
"""
open(D + "/REPORT.md", "w").write(md)
# Typst
svg = "graph.svg"
def esc(t):
    return t.replace("\\", "\\\\").replace("#", "\\#").replace("$", "\\$").replace("*", "\\*").replace("_", "\\_").replace("<", "\\<").replace("@", "\\@")
lines = ['#set page(margin: 1.8cm)', '#set text(size: 9.5pt)', '= Automorphism quotient of the cover factor base', 'All numbers are \\[pcc:unverified\\]; local development check.', '#image("graph.svg", width: 100%)']
for ln in md.split("\n")[1:]:
    if ln.startswith("## "):
        lines.append("== " + esc(ln[3:]))
    elif ln.startswith("|"):
        continue
    else:
        lines.append(esc(ln).replace("\\*\\*", "*") if False else esc(ln))
tab = ['#table(columns: 6, inset: 4pt, [p], [cells], [geo-mean S], [S/negrho w=0], [w=0.1], [w=1],']
for p in sorted(map(int, bp)):
    b = bp[str(p)]
    tab.append(f'[{p}], [{b["n"]}], [{b["geo_S"]:.4f}], [{b["geo_S_over_negrho_w0"]:.3f}], [{b["geo_S_over_negrho_w0.1"]:.3f}], [{b["geo_S_over_negrho_w1"]:.3f}],')
tab.append(')')
tab.append('#table(columns: 4, inset: 4pt, [p], [S/rho w=0 \\[95%\\]], [w=0.1], [w=1],')
for p in sorted(map(int, pp)):
    d = pp[str(p)]
    tab.append(f'[{p}], ' + ', '.join(f'[{d["geo_w"+w]:.3f} \\[{d["ci_w"+w][0]:.3f}, {d["ci_w"+w][1]:.3f}\\]]' for w in ('0', '0.1', '1')) + ',')
tab.append(')')
lines.insert(6, "\n".join(tab))
open(D + "/REPORT.typ", "w").write("\n\n".join(lines) + "\n")
