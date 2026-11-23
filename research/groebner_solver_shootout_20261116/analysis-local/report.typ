#set page(paper: "a4", margin: 1.6cm)
#set text(size: 9pt)
#set heading(numbering: none)

= GB-SOLVER-SHOOTOUT-20261116, amendment-1: local diagnostic results

*LOCAL DIAGNOSTIC, non-admitted, [pcc:unverified]; diagnostic tier (local, non-admitted).* Not an IC leaderboard or benchmarks-README row. No pcc claim filed.

Protocol `protocol.amend1.json` canonical sha256 `c521e04c…fe858`. Run `run-local/` 2026-11-17T07:23Z–08:19:51Z, `run_local.py` sha256 `ed27d2e7…28120ac`, exit 0, 56:17.76 wall. Cell `K17-m3-plain-l5-a1` (K1 / GF(2^17), r = 65587, m = 3 plain, l = 5, 32 vars, 14 fb abscissae). Host Intel Xeon 2.30 GHz, 4 vCPU shared VM, no PMU, isolation L0 (contended). 996 rows (6 masked arms × (1 warm-up + 5 planted + 80 SAT + 80 UNSAT)), warm-up excluded. Analysis `analysis-local/analyze_amend1.rs` (sha256 `d62848b4…514223a`), written *after* the data (see deviations). Raw: `rows.jsonl` `7c5c98f9…2aedf1`, `cells.json` `87d4b7a8…b71c3f0f`.

== Answer

*inherited-f4* is fastest per verified relation: 0.786 rel/CPU-s [95% CI 0.752, 0.827] vs msolve-f4 0.00585 [0.00564, 0.00612] vs cms-xor 0.00125 [0.00120, 0.00131]. Paired rate ratios: inherited-f4/msolve *134×* [131, 138]; inherited-f4/cms *629×* [606, 651]; msolve/cms *4.68×* [4.57, 4.79]. All registered comparisons: Wilcoxon p = 1.4e-48 (n_eff 160, 160/160 targets same sign), |median log2| ≥ 2.26 vs A/A floor ≤ 0.024.

*§5a band: the ordering does not hold across the band.* At w = 0 no solver does priced work, so every arm's C(0) = 0 (tie, by construction). At w = 0.1 and 1 (native units) the order is inherited-f4 < cms-xor < msolve-f4; on the clock inherited-f4 < msolve-f4 < cms-xor. The CPU ranking is a same-host clock result only.

#figure(image("figures/fig1_rel_per_cpu.svg", width: 85%), caption: [Primary endpoint per arm, stratified, mean of passes, 95% stratified paired bootstrap CI (20000). run-local, n = 160 natural targets/arm.])

== Per arm × cell

#table(columns: 8, stroke: 0.4pt,
  [*Arm (A / A/A)*], [*Rel/CPU-s [95% CI]*], [*CPU-s/target SAT / UNSAT*], [*Max deg*], [*Largest matrix*], [*Max RSS*], [*Timeouts*], [*Planted*],
  [inherited-f4 (7a/e2)], [0.7864 [0.7523, 0.8272]], [0.0353 / 0.0354], [3], [n/e], [5.58 MB], [0], [5/5, 5/5],
  [msolve-f4 (71/6f)], [0.005853 [0.005645, 0.006115]], [4.94 / 4.74], [4], [40561×41092], [235.4 MB], [0], [5/5, 5/5],
  [cms-xor (3f/88)], [0.001250 [0.001199, 0.001313]], [24.94 / 22.16], [n/a], [57×202], [47.5 MB], [0], [5/5, 5/5],
)
All rows match the fb-enum oracle; 0 spurious roots, 0 M1 violations, 0 false relations on 80 oracle-zero targets; 6/6 tamper checks rejected or oracle-consistent. Max charged CPU A/B: inherited 0.057/0.053 s, msolve 5.76/5.53 s, cms 32.5/32.3 s (cap 60 s).

#figure(image("figures/fig2_per_target_cpu.svg", width: 80%), caption: [Per-target charged CPU (mean of passes) by stratum. run-local.])

== Registered comparisons
#table(columns: 6, stroke: 0.4pt,
  [*Id*], [*Median log2 CPU A/B [95% CI]*], [*p*], [*A/A floor*], [*Rate ratio A/B [95% CI]*], [*Verdict*],
  [C-msolve-cms], [−2.255 [−2.300, −2.238]], [1.4e-48], [0.0230], [4.68 [4.57, 4.79]], [msolve-f4 uses less CPU per target],
  [C-inh-msolve], [−7.139 [−7.164, −7.121]], [1.4e-48], [0.0239], [134.4 [130.9, 137.7]], [inherited-f4 uses less CPU per target],
  [C-inh-cms], [−9.418 [−9.449, −9.362]], [1.4e-48], [0.0239], [629.1 [606.5, 651.4]], [inherited-f4 uses less CPU per target],
)
Each verdict carries "diagnostic tier (local, non-admitted)". M8: msolve < CMS (expectation not falsified); no solver beat incumbent inherited-f4.

== A/A noise floor
inherited-f4 median log2 A/B +0.0132 [+0.0051, +0.0239] (p 0.0095: pass B ≈1% cheaper, an order/pass effect); msolve +0.0028 [−0.0104, +0.0230]; cms −0.0020 [−0.0076, +0.0023]. Native ops identical across passes on 160/160 targets for every arm.

#figure(image("figures/fig4_aa_noise.svg", width: 85%), caption: [A/A per-target log2(CPU pass A / pass B), dashed = floor. run-local.])

== Matched rho (§5a, C3)
Rho: signed-Frobenius on `KoblitzCurve::new(1,17)`, 1000/1000 verified, A_rho 55.197 (E 55.05), U_rho 111.296, 3.995e-5 CPU-s/DLP (cross-check from ns: 3.72e-5 s). Projected phase 24 relations; LA etc. uncharged (IC lower bound). log2 distance: inherited-f4 w=0: 0, w=0.1: 22.33, w=1: 24.33, clock 19.54 [19.47, 19.61]; msolve 31.28 / 33.27 / 26.61; cms 24.93 / 26.92 / 28.84. Verdict for all: unresolved in band. M8 abandon (clock > 2^20) not triggered, narrowly (2^19.54).

#figure(image("figures/fig3_rho_band.svg", width: 85%), caption: [Distance remaining to matched rho per reading. run-local.])

== What can and cannot be concluded
Can: on this cell and host, at diagnostic tier, charged CPU per target orders inherited-f4 < msolve-f4 < cms-xor, with gaps 4.7×–629× far outside A/A floors; all arms complete against the oracle. Cannot: admitted measurement, leaderboard/benchmarks row, band-robust ranking, IC-vs-rho beyond "unresolved in band", scaling, other encodings or hosts, Magma/Julia/msolve 0.11 (UNAVAILABLE), any IC baseline improvement.

== Deviations
(1) No amendment-2-pinned analysis script; this one is post-data; analyze.py v1 not used (inadmissible). (2) The analyst printed `blinding_key.json` before phase 1; blinding was partial anyway (native_unit per row); verdicts are not blind. (3) RNG splitmix64/FNV-1a. (4) Censored branch fails closed (none present). (5) C-dreg not computed. (6) Amendment-2 pins absent; binaries only in scratchpad. (7) benchrec record OPEN; no S3 (AWS does not work); refusals in `run-local.launch/REFUSALS.md`.
