#set page(paper: "a4", margin: (x: 1.8cm, y: 1.8cm), numbering: "1")
#set text(size: 9.5pt)
#set par(justify: false)
#show raw: set text(size: 8pt)
#set table(stroke: 0.4pt + gray, inset: 3pt)

#align(center)[#text(size: 15pt, weight: "bold")[Smith route H to a plane quartic, round 2: Diem relations, charged group order, measured negation rho on J\_H]]

Status: `[pcc:unverified]`, local development check at small p, no isolab, no benchrec, no improvement claimed. \
Plan: `PLAN.md` (frozen 2026-11-21 at git HEAD 473c4064294aea5131fb869b8aaa30810f9ed228, amendment A1 before any run of record). Numbers: `RESULTS.json`, tables `RESULTS.md`, graph `graph.svg` (from `plot.py`), raw rows `results/run_p*.jsonl`. Binary: `src/bin/smith_walk_dlp_ic.rs` (new); the first-round bin is untouched.

== Question
After the verifier's major issue (S for the index-calculus arm left out the group-order phase of the quartic Jacobian) and two minor ones (rho analytic, BSGS instead of Diem), how far is the full Smith route from a measured negation-map rho on the same J\_H, with every phase charged? Distance remaining is reported at w = 0, 0.1 and 1 GAE per native operation. \[pcc:unverified\]

== What changed since the first round
+ Solver on the quartic: BSGS replaced by the sparse Diem arm of `research/quartic-diem-sparse_20261115` (factor base about 2 sqrt(p) points, double large primes, row mode `light`, Wiedemann), run on the quartic that the Smith transfer produces (`src/qds_ext.rs`: a copy of `qds_core.rs` with multi-target descent and a driver appended; group law and sparse arm unchanged; `quartic_n4_la_20261021/ic.rs` was not read separately, its code reaches this study through `qds_core.rs`).
+ Group order charged inside S, two ways: route Z3, point counts N1, N2, N3 of H over F\_p, F\_p^2, F\_p^3 by exhaustive enumeration (F\_p^3 by forward differences and the norm character), then the genus-3 L-polynomial; route Z2B, N1 and N2 only, then a BSGS with Cantor additions over the window left for the last coefficient (|s3| \<= 6 p^1.5, 4 p^1.5 candidates). The two orders agree on all 12 curves; N G = 0 holds on H and ell G' = 0 on C for every curve.
+ Measured negation-map rho on J\_H with the same Cantor arithmetic (`src/hgroup.rs`: r = 32 walk, distinguished points 2^-5, two-cycle escape, restart on long walks), same G', Q', same ell, 6 runs per target (two seed sets of 3 for A/A).
+ Instances now live in a prime-order subgroup: ell is the largest prime with ell || N and N/ell \<= 64, G' = (N/ell) \[P1 - P2\], Q' = (N/ell) \[P3 - P4\]; ell is odd so the pullback has no 8-torsion ambiguity (first round: 19 of 160 logs differed by the kernel).
+ End-to-end check \[5\]: k from C is checked as k G' = Q' on the ORIGINAL H with Cantor arithmetic for every target. \[pcc:unverified\]
+ Panic in `plane_quartic_pic::complete`: partly addressed, see "Limitations and items not closed".

== Method (what is charged)
Unit GAE\_H = F\_p mults + 3 inversions of one Cantor addition on H (measured per curve over 300 additions, about 400 field operations; one addition on the quartic costs about 7000, `gae_c_field_ops` in RESULTS.json). S(w) = (M + 3 I) / GAE\_H + w \* native. Native operations: hash probes and table lookups, union-find, the mod-ell multiply-adds of elimination, Wiedemann and Berlekamp-Massey, table lookups of the point counts, negation and compare in rho. Field adds are not counted on either side. Phases of S for one target: transfer (20 canonical points via the counted copy of `smith_trigonal`, plane fit), quartic setup (smoothness certificate, group-law context), push-forward classes with cofactor clearing on C, group order (Z3 or Z2B or none), factor base and relations, linear algebra, descent, pullback on H, and every failed attempt of the escalation c = 2, 4, 6 (all charged). Single-target S is the shared cost plus the mean descent and pullback over the found targets; no amortisation over targets. Library helpers inside the transfer (`sqrt_mod`, Legendre, inversion) are modelled, not counted (stated in `src/st.rs`); `Trigonal::new` internals and curve selection (3 of 80 candidates accepted at p = 101, see below) are not charged. Ladder: p = 101, 211, 521, 1021, 3 curves each (first accepted in generator order, seed 31), 3 targets per curve, IC run twice per curve (A/A), rho 6 runs per target.

== Results

Verification (all `[pcc:unverified]`): 36 targets (9 per p), 36 logs found, 36 verified on H, 0 found-not-verified. The same Diem arm fails its first attempt at c = 2.0 for p = 101, 211, 521 (all curves) and passes at c = 4.0; at p = 1021 it passes at c = 2.0. Candidates tried and accepted: 80/3, 26/3, 54/3, 30/3 at p = 101, 211, 521, 1021; almost all rejections are "no prime ell with N/ell \<= 64" (61, 22, 47, 26); the rest are driver failures to form the group instance (cofactor clearing, pool negation, class difference, ell G' != 0) and one fit with no unique quartic.

Measured negation rho (54 runs per p, every run found the log) \[pcc:unverified\]: steps / sqrt(ell) = 3.69, 1.58, 1.35, 1.42 at p = 101, 211, 521, 1021 (ideal for the negation walk 0.886; the large value at p = 101 is distinguished-point overhead at ell about 3e4); one-off setup (32 multipliers) is 73, 61, 39, 14 percent of the rho cost at w = 0. Measured over the first-round analytic reference sqrt(pi ell/4) x field ops per add: 12.7, 4.5, 2.35, 1.87.

Distance to matched rho, single target, ratio of means with 95% bootstrap over 3 curves (wide):

#table(columns: 5, align: left, fill: (x, y) => if y == 0 { luma(235) },
  [p], [price w], [S (Z3 charged) / rho], [S (Z2B) / rho], [S (no order phase) / rho],
  [521], [0], [1.11e3 \[971, 1.47e3\]], [6.65 \[5.7, 9.05\]], [0.96 \[0.73, 1.54\]],
  [521], [0.1], [3.6e3 \[3.1e3, 4.9e3\]], [27.1 \[23.3, 36.8\]], [17.2 \[14.7, 23.5\]],
  [521], [1], [1.11e4 \[9.3e3, 1.62e4\]], [88.2 \[73.9, 129\]], [65.9 \[55.2, 96.5\]],
  [1021], [0], [2.28e3 \[1.32e3, 3.91e3\]], [6.5 \[3.9, 11\]], [0.295 \[0.249, 0.312\]],
  [1021], [0.1], [7.24e3 \[4.2e3, 1.26e4\]], [14.6 \[8.6, 25.1\]], [4.33 \[2.71, 7.25\]],
  [1021], [1], [1.94e4 \[1.1e4, 3.5e4\]], [34.3 \[19.7, 61.6\]], [14.2 \[8.4, 25.2\]],
)

The mean rho cost is 4.4e3 / 5.3e3 / 1.3e4 GAE\_H at p = 521 and 1.55e4 / 2.0e4 / 5.7e4 at p = 1021 for w = 0 / 0.1 / 1. Full tables for all four p, the phase shares, the A/A ratios (main over A/A of S, 1.00 to 1.08) and the rho rescaled to ell = p^3 are in `RESULTS.md`. Native work moves the no-order ratio from 0.30 to 14 at p = 1021: the mod-ell linear algebra is 97 to 99 percent of the no-order S at w = 0.1 and 1 (for example 8.5e4 of 9.2e4 GAE\_H at p = 521, w = 0.1). Under the band rule (rank only where the order holds at every weight) the no-order route against rho is unresolved: it is below rho at w = 0 and above rho at w = 0.1 and 1; nothing here is a win.

Share of the group-order phase in S at p = 521: Z3 99.9, 99.5, 99.4 percent at w = 0, 0.1, 1; Z2B 86, 36, 25 percent. Z3 counting grows with exponent 2.75 to 2.93 (expected 3) and Z2B about 0.9 to 1.0 (the p^2 count of N2 dominates; expected 2 asymptotically, small p). Other phases of the no-order S at p = 521, w = 0: transfer 232, quartic setup 110, push-forward classes 515, relations 369, failed attempts 811, descent 2200, pullback on H 28 GAE\_H.

Exponent fits (4 points, p \<= 1021, bootstrap over curves, \[pcc:unverified\]): S without order phase 0.545 \[0.47, 0.63\] at w = 0.1 and 0.714 \[0.70, 0.735\] at w = 1 (-0.44 \[-0.77, -0.07\] at w = 0, not meaningful); measured rho 0.83 \[0.62, 1.01\] to 1.09 \[0.86, 1.29\]; rho rescaled to ell = p^3 0.69 to 0.95; rho without its setup rescaled 1.11 \[1.01, 1.26\]. The expected rho exponent is 1.5; the measured values are below it because of the fixed setup and distinguished-point overhead at these tiny sizes, so the trend of S/rho in p is confounded by rho's own small-size inflation and is not an asymptotic statement.

#figure(image("graph.svg", width: 100%), caption: [S and measured rho on J_H against p, three prices (GAE_H, single target); log-log. \[pcc:unverified\]])

== Predictions against outcomes
#table(columns: 3, align: left, fill: (x, y) => if y == 0 { luma(235) },
  [], [prediction (PLAN.md)], [outcome],
  [Q1], [Diem arm \>= 3 times cheaper than the first-round BSGS arm (1.83e7 mul+inv per target) at p = 521, judged at w = 0], [confirmed at w = 0: relations plus descent cost 9.9e5 (M + I) per target, a factor 18.4; refuted once the linear algebra is priced: in field-operation equivalents the Diem arm is 3.7e7 at w = 0.1 (BSGS over Diem 0.49) and 3.6e8 at w = 1 (0.05). The BSGS figure has no native work priced (about 4e3 probes), so it is a lower bound.],
  [Q2], [group-order phase below 10 percent of S at p = 521], [refuted for Z3 (99.4 to 99.9 percent) and for Z2B (86, 36, 25 percent)],
  [Q3], [measured negation rho within a factor 2 of the first-round analytic reference], [refuted at p = 101, 211, 521 (12.7, 4.5, 2.35); within the factor at p = 1021 (1.87)],
  [Q4], [S/rho at w = 0 below 3 at p = 521 for the Z3 headline (expected refuted), for Z2B and for no order phase (expected confirmed)], [Z3: refuted (1.1e3). Z2B: refuted (6.65 \[5.7, 9.05\]; my expectation was wrong: the p^2 count of N2 already costs more than rho at w = 0). No order phase: confirmed (0.96 \[0.73, 1.54\]) at w = 0 only; at w = 0.1 and 1 the ratio is 17 and 66],
  [Q5], [pulled-back logs verify on H; P7 has no untested pairs], [verification confirmed (36 of 36). P7 not closed: see below],
)

== Limitations and items not closed
- Item (4), the `plane_quartic_pic` panic, is NOT closed. The module copy (`src/pqp.rs`, completion bound raised from 7 to 12) still panics on the same 254 pairs with the message "generating space did not complete by degree 12", so the bound was not the cause: the generating space of those degree-3 divisors never completes (results/pushfwd2\_bound12\_only.jsonl). A retry with up to six other base points (`smith_walk_dlp_p7`, results/pushfwd2.jsonl) leaves 167 of 1196 pairs untested under all bases (175 panics and 35 missing Phi over 49 base runs); every base run that tested pairs was single valued (58 of 58 instances), and the negative control separated in 48 of 48 base runs that had control pairs. The Diem pipeline here does not use this module: its push-forward goes through the quartic of `qds_core`, and the 36 verified end-to-end pullbacks are a stronger check of the divisor map than P7.
- The transfer only pushes divisors made of rational points of H (differences of points with an F\_p or conjugate F\_p^2 image pair); a general reduced divisor with points in extension fields is not pushable by this code. Q' is built as a multiple of such a difference, so the instance is structured.
- The group-order route Z3 is brute force; its counted cost scales with p^3 by construction. It is charged because the verifier asked for it; if ell is public (the usual DLP statement) the no-order column applies, and that column is the one a cheaper order method would approach. Z2B is not a published algorithm choice but a sensitivity (N1, N2 and a BSGS over the window).
- Curve selection, the Smith-positive generator, `Trigonal::new` internals, modular square roots inside the transfer (modelled) and the Hasse-style candidate scan of the orders are not or only partly charged; the instance filter (cof \<= 64) keeps 4 percent of the candidates at p = 101 and 6 to 12 percent at the larger p.
- Three curves per p; intervals are wide. rho at ell about 1e4 to 1e5 is dominated by setup and distinguished-point overhead, which lowers its measured exponent. p \<= 1021 only. The first-round analytic reference counted field additions on H; here adds are not counted in S (both conventions are in RESULTS.json).
- `verified_on_C` is inconclusive for 12 of the 36 found targets (a scalar multiple on the quartic returned `None`); the check on H is authoritative.
- The ladder was run twice with identical seeds; the second run only added the rho setup fields; the measured values are identical (checked key by key, first run kept in `results_run1/`).
- Code checks: `rustfmt --check` passes for both new bins and their modules; `cargo clippy --release --bin smith_walk_dlp_ic --bin smith_walk_dlp_p7` reports no warning in the new files, but `-- -D warnings` cannot be run on this toolchain because the library itself fails with `unknown lint: clippy::chunks_exact_to_as_chunks` (13 errors, not from these files). Six unit tests pass (point counts against the elliptic trace, norm multiplicativity, L-polynomial window, order window and negation rho on H, `pick_ell`, and an end-to-end p = 101 route that checks every found log on H). A rebuilt binary after the lint edits reproduces `results/run_p101.jsonl` exactly (wall times aside); `results/binary_sha256_final.txt`.
- W \> 1 walk not touched. One session, no replay certificate, no validator, no matched-rho band beyond the three prices: nothing here may enter a leaderboard row. All values `[pcc:unverified]`.

== Repro
`cd /home/user/crypto && CARGO_TARGET_DIR=$PWD/target cargo build --release -j2 --bin smith_walk_dlp_ic --bin smith_walk_dlp_p7 && bash research/isogeny-walk-quartic_20261121/run_ladder.sh && for p in 37 101; do for s in 1 2 3; do target/release/smith_walk_dlp_p7 --p $p --seed $s --want 10 2>/dev/null; done; done > research/isogeny-walk-quartic_20261121/results/pushfwd2.jsonl && python3 research/isogeny-walk-quartic_20261121/summarize.py && python3 research/isogeny-walk-quartic_20261121/plot.py` \
Smallest cell: `target/release/smith_walk_dlp_ic run --p 101 --seed 31 --curves 1 --z3 1 --rho-runs 2` (about one second).

== Sources read (repo paths)
research/isogeny-walk-quartic\_20261115/{PLAN.md, REPORT.md, RESULTS.json, repro.sh, src/planefit.rs}; src/bin/smith\_walk\_dlp.rs; research/quartic-diem-sparse\_20261115/{src/qds\_core.rs, summarize.py, RESULTS.md, run\_ladder.sh}; src/bin/quartic\_diem\_sparse.rs; src/cryptanalysis/{smith\_trigonal.rs, plane\_quartic\_pic.rs, smith\_prop24.rs (helpers), diem\_quartic.rs (included)}; src/prime\_hyperelliptic/{curve.rs, ops.rs, fp\_poly.rs}; AGENTS.md section 5a; CLAUDE.md.

