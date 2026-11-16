#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set par(justify: true)
#show table: set text(size: 8.5pt)

= Inherited-F4 pricing jump between `2245b75` and `main`
2026-10-18 · `research/f4_pricing_jump_20261018/` · class: *accounting* (PR \#1774) + *algorithm* (PR \#1862) · no bug found · no leaderboard row moves

== Question
The committed spec of `ECBS1hfd5bf1885227` (`claim-koblitz-rootfind-n13.json`) gave very different inherited-F4 `total_gae` when built from `2245b75` and from `main` on x86-64. Reproduced exactly on this host: `ic-f4-m2` target 0 warm-up 982.628 → 1,949.912 GAE; `ic-f4-m3` 671.404 → 25,818.785 GAE.

== Method
x86-64 (AMD EPYC 9V74, 4 vCPU), rustc 1.97.0, isolation L0, counts only. `cargo build --release --bin ecbench` at opt-level 1 in one sparse worktree with one shared target directory. Reduced spec `bisect/f4only.json` (arms `rho-strong`, `ic-f4-m3`, `ic-f4-m2`; F4 signature byte-identical to the full spec at both ends). Signature = SHA-256 of the 64 F4 rows' `total_gae`, `lower_bound`, `unpriced`, per-phase `gae`, `solver_ops`, calls, trials, relations, adds, doubles. Recursive bisection over the 303 first-parent commits of `2245b75..origin/main`: 16 points evaluated, two change points, each then split to a single commit.

#table(columns: 4, align: (right, left, left, left),
  [*index*], [*commit*], [*signature*], [*note*],
  [0], [`2245b7596`], [`69a5576f9885`], [archived],
  [38 → 39], [`6ea104b4b` → `cd226d0ce`], [`69a5…` → `6bdf…`], [merge of \#1774],
  [inner], [`3f4a87aae` → `c8b1594f6`], [`69a5…` → `6bdf…`], [single commit],
  [95 → 96], [`e15e29dbd` → `f2c9202ca`], [`6bdf…` → `becf…`], [merge of \#1862],
  [inner], [`df1fbcf67` → `d1d034d94`], [`6bdf…` → `becf…`], [single commit],
  [303], [`abee07ee7`], [`becf95c66869`], [main],
)
Other evaluated points (37, 41, 46, 56, 75, 94, 98, 103, 113, 151) agree with their neighbours; the full log is `bisect/drive.log`.

== Culprit 1: `c8b1594f6` (PR \#1774), accounting
`run_pipeline` priced the solver at the pinned `ns_per_word_xor` only for `op_unit == "word XORs"`. Inherited F4 reports `word XORs (elimination, specialisation and linear elimination only)`, so the term was wall-priced, and `ecbench` then removed it as `…_uncharged` (`lower_bound: true`). The commit matches by prefix. Pinned ratio for `K_0 / GF(2^13)` in `docs/ic/calibration.json`: 0.002304 GAE per word XOR, unchanged since before `2245b75`. Across the merge, every F4 row keeps the same `solver_ops`, calls, trials, relations, adds, doubles and non-relations phase GAE. The relations increase divided by `solver_ops` is 0.0023040 on all 64 rows. A correction, not a change of work.

== Culprit 2: `d1d034d94` (PR \#1862), algorithm
"Harvest root linear rows by default." Where the root gate fires, the multi-variable linear rows join the system, which changes the tree. `ic-f4-m3` target 0 warm-up: reductions 1,886 → 1,917; propagations 2,800 → 2,619; eliminated 1,940 → 1,790; splits 808 = 808; word XORs 10,305,435 → 10,914,662. Over the 32 m3 rows, after/before word XORs has median 1.0715, range [1.0485, 1.1109]. `ic-f4-m2` is unchanged (gate does not fire). Roots unchanged (all records verified); the counter charges real work. On this workload it is a *regression* of about 7% in m3 relations work, against the PR's "no regression" on its own held-out cells.

#table(columns: 5, align: (left, right, right, right, left),
  [*state*], [*m2 mean total_gae*], [*m3 mean total_gae*], [*m3 mean word XORs*], [*lower bound*],
  [`2245b75`], [744.8], [712.9], [9,399,300], [yes],
  [after \#1774], [1,600.3], [22,368.9], [9,399,300], [no],
  [after \#1862 = main], [1,600.3], [23,937.6], [10,080,165], [no],
)
Non-warm-up rows, n = 24 per arm; arithmetic means of recorded values.

#figure(image("pricing_jump.svg", width: 100%), caption: [(a) Mean and min–max `total_gae` per arm at the archived commit, after each culprit, and at main. (b) Solver word XORs per F4 row before vs after \#1862; m3 sits above y = x. Data `bisect/sigs/`, source `plot.py`.])

== Affected rows
`docs/ic/leaderboard.json` is built from recorded study files, so no row moves. Tables A–C use no word-XOR-priced arm. Inherited-F4 `S` appears only in the lower-bound diagnostics `f6_ecbench_ladder_diagnostic`, `f4_f5_f6_cross_family_diagnostic` and the frontier replay block, all recorded before \#1774. Their specs were re-run with `main` (mean `S`, non-warm-up rows):

#table(columns: 6, align: (left, left, right, right, right, left),
  [*session*], [*arm*], [*n*], [*archived S (LB)*], [*main S*], [*main LB?*],
  [ladder n13], [ic-f4], [24], [16.27], [*488.89*], [no],
  [ladder n13], [ic-f6], [24], [861.0], [*1,182.3*], [no],
  [ladder n19], [ic-f4], [24], [4.916], [4.916], [yes],
  [ladder n23], [ic-f4], [24], [2.134], [*9,479.7*], [no],
  [cross K13], [inherited-f4], [8], [16.49], [*634.12*], [no],
  [cross K13], [matrix-f4], [8], [16.44], [*113,830*], [no],
  [cross K13], [matrix-f5], [8], [16.44], [*73,606*], [no],
  [cross R13 / K19 / R19], [inherited-f4], [8 each], [18.45 / 4.957 / 3.368], [unchanged], [yes],
)
Hazards (accounting): (1) F4-family `S` from before `cd226d0ce` is not comparable with `S` from after it on a curve with a pinned word-XOR ratio; (2) on today's binary the same engine is a complete count on pinned curves (K_0/2^13, K_0/2^23) and a lower bound elsewhere, so a ladder across them mixes the two.

== Landed and proposed
Landed: a documentation comment on `is_word_xor_op_unit` (`src/cryptanalysis/ic_framework/mod.rs`); no counted quantity changes. Proposed: version word-XOR-priced IC method identities with the pricing rule; re-measure the \#1862 default on the m3 pdp3-Koblitz spec with the harvest as a method parameter (ecbench scrubs `KIC_*` from the child environment).
