# Wave 7: shared brief for the five untried leads (2026-11-22)

Origin: owner question 2026-11-22 ("point hashing, auxiliary curves and twists, point representation,
coordinate types, curve equations"); a repository sweep found five items never measured. Owner: "Work on
the leads above in parallel".

## Rules for every lead agent

- Repository /home/user/crypto, branch claude/curve25519-index-calculus-i674j4. Write ONLY inside your own
  lead directory under research/curve25519_wave7_untried_20261122/. Do not git commit, push, stash, reset or
  checkout; the main session commits. Do not edit any other file.
- Disk is tight (about 8 GB free): no workspace cargo builds, no worktrees, no large outputs (keep every
  file under 1 MB; PNG figures under 60 KB). Prefer Python stdlib + numpy (both installed; sympy is not).
  Existing engine binary: /home/user/bin-archive/curve25519_family-5abc70ba (read src/bin/curve25519_family.rs
  for its subcommands such as f4sub). Host: 4 cores shared by five agents; use ONE core.
- Curves (Montgomery B y^2 = x^3 + A x^2 + x): family miniatures k = 20: p = 1048573, A = 926, #E = 8r,
  r = 131203; k = 24 faithful member p = 16777141, A = 7842. Other k (16..24) and the family rule:
  research/curve25519_frame_search_20261103/README.md (erratum E1 there: k = 18 A = 78, k = 22 A = 43186).
  Real Curve25519: p = 2^255 - 19, A = 486662, #E = 8r, r = 2^252 + 27742317777372353535851937790883648493,
  twist order 4r'. Verify every curve fact you use (point counts, orders) by computation and record it.
- Build on, do not repeat: research/curve25519_frame_search_20261103 (frame theorem: every degree-2
  coordinate is a Moebius frame of x; coset selection gain), prime_field_ic_closure_20261112/closure-ledger.md,
  curve25519_autolab_20261104/ideas/{STRUCTURE-R4.md,NONCOLLISION-20261108.md,STRUCTURAL-20261110.md},
  curve25519_algebraic_ladder_20261102, curve25519_t0_symmetry_20261102, curve25519_torsion_leads_20261103,
  curve25519_descent_skip_20261114, curve25519_isogeny_seed_20261103, curve25519_isogeny_seeds_20261114,
  lead_sweep_20261110/{LEAD-EXO-coordinate-revealing-oracles,LEAD-EXO-curve-model-representation,
  LEAD-EXO-hashed-factor-base-tensor-ecdl}, hashed_ecdl_counts_20261031, curve25519_ic_baseline_20261102.
- The research knowledge base and the lead atlas are unreachable in this container; "Sources checked" lists
  repository files you actually read.
- Discipline: PROTOCOL.md frozen before any data: question, mechanism, instrument (exact commands), cells,
  seeds by a derived rule, counted unit(s), estimator with intervals, numbered predictions with numeric
  thresholds written before data, a positive control (shows the detector can see an effect of the size that
  would matter), a negative/random control, and what counts as failure. A failed prediction stays failed.
  Every result sentence carries [pcc:unverified]. Never relabel rho or BSGS as index calculus. Distance to
  rho is imported from cited studies, never invented.

## Phase 1 (now): design only
Deliver in your directory: PROTOCOL.md, the run script(s), the analysis script, and a smoke test of at most
60 s CPU on the smallest case, written under smoke/ and labelled "smoke, not data". Estimate the full run's
CPU time and memory; it must fit in 3 hours on one core and 4 GB. Then STOP and report: the question in one
line, the predictions with thresholds, the controls, the estimated runtime, and the files. Do not take data.
