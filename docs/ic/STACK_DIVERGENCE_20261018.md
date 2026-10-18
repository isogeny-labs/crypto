# Stack divergence: `crypto` vs `cryptanalysis/suite` (T13)

Task T13 of [`RESEARCH_IC_LEADS_ASSESSMENT_20261018.md`](../../RESEARCH_IC_LEADS_ASSESSMENT_20261018.md)
(§4, "Tracked follow-ups"). Read-only analysis: no source file in either repository
was changed and nothing was built. Every number below is reproduced by a command in
the [appendix](#appendix-commands-that-reproduce-every-number); `[An]` points at the item.

**Pins.** `crypto` = worktree `wt/t13`, HEAD `f22d55b9c`, whose `src/cryptanalysis`
tree (`acd8b3c10b16`) equals origin/`main` at `b7cafe04`. `suite` =
`aburan28/cryptanalysis` checkout at `0ea8931d`, `suite/src/cryptanalysis` tree
`f6d41e491cb3`. The suite's default branch is `claude/cryptanalysis-repo-setup-p59kek`
(not `main`); its tip is `8fd4429` (#957), one commit newer than the checkout. [A1]

## 0. Summary

1. **The two trees are a fork with two sync events and none since.** The suite was
   copied from crypto on 2026-09-21 (`c00af46`, "Add the attack suite from the crypto
   repository as suite/") and re-merged once, on 2026-09-25 (`613820d`, "merge upstream
   crypto@54e977ab"). Since that merge crypto has made **1,094** commits touching
   `src/cryptanalysis`; the suite has made **39** touching `suite/src/cryptanalysis`,
   **15** of them hand ports from crypto ("(port)" in the subject). [A7]
2. **Entries.** crypto has 260 entries in `src/cryptanalysis`, the suite 177; **154**
   names are shared, **106** are crypto-only, **23** are suite-only. [A3]
3. **Of the 154 shared entries** (6 are directories): **52 identical**, **29 identical\***
   (differ only by edits made when the suite imported them, e.g. the crate path in a doc
   comment), **49 crypto-ahead**, **21 both-changed**, **3 suite-ahead**. Of the
   diff lines, 53,761 are in crypto only and 6,575 in the suite only (8.2 : 1). [A4]
4. **The drift is mostly one-directional.** crypto has moved on 70 of the 102 entries
   that are not byte-identical; the suite has moved on 24, and 15 of its 39 post-sync
   commits are ports of crypto changes. What the suite has that crypto lacks is a
   specific, small set: the rank-indexed `f4_gf2` Boolean F4 kernel, a batched/GPU F4
   path, the m = 4/5 point-decomposition specialisations, the prime-field
   automorphism-orbit pipeline, and non-IC attack breadth (§3).
5. **Highest-value ports, in order** (§5.2): (1) `f4_gf2.rs`, ≈2,000 lines; (2)
   `semaev_decomp/linearised_m4`, ≈1,450 lines; (3) the GPU large-matrix echelon path,
   ≈1,750 lines. About 5,200 lines against 165,927 lines of crypto-only code.
6. **Record per subsystem** (§5.1): crypto for the harness, the counted unit, the
   registry and the solver stack; the suite for the F4/GPU path, the PDP
   specialisations and the non-IC attack library, until each is ported or retired.

All suite-side speedups quoted here are the suite's own stage diagnostics, copied from
its committed receipts. None was re-measured for this note, none moves an end-to-end
`S`, and none is a claim against a matched rho. Under the repository's success rule they
are stage improvements until a ported kernel moves end-to-end `S` in a same-binary
comparison on crypto's workloads (T14).

## 1. Method, and what the labels mean

Git history cannot say directly which side moved, for three reasons found while
doing this task:

- both local checkouts are shallow (crypto's `src/cryptanalysis` history in the
  checkout starts at 2026-10-17), so full-history blobless clones of both remotes were
  taken into a scratch directory [A2];
- the suite import was not byte-for-byte: rustfmt and doc-link edits mean only 119 of
  200 crypto files at the second sync have the same blob in the suite's 207 [A2];
- the suite has taken crypto changes by hand since the sync (15 "(port)" commits), so a
  suite-side change is not always suite work.

The model used is therefore a two-point one. Let `XC` = `54e977ab` (the crypto commit
the suite merged) and `SYNC` = `613820d` (the suite commit that merged it). For each
file: **crypto changed** = crypto HEAD blob differs from the blob at `XC`; **suite
changed** = suite HEAD blob differs from the blob at `SYNC`. Then

| label | rule |
|:--|:--|
| identical | the blobs at the two HEADs are equal |
| identical\* | blobs differ, but neither side changed since the sync: the difference is the suite's import-time edit |
| crypto-ahead | crypto changed, the suite did not; also a suite change whose blob equals an older crypto revision (a port; one file, `koblitz_sparse_la.rs`) |
| suite-ahead | the suite changed, crypto did not |
| both-changed | both changed, and the suite's blob is not an older crypto revision |

A directory takes the union of the labels of its files; a file present on one side only
counts as a change on that side. "Crypto-only / suite-only lines" are the added and
removed counts of `git diff --no-index --numstat crypto suite`, summed over a
directory's files, so a one-line `crypto_lib::` to `cryptanalysis_suite::` path
rewrite counts once on each side. "Commits since sync" are commits touching the path,
crypto from `XC` to `b7cafe04`, suite from `SYNC` to `0ea8931`.

What the model cannot see: a suite-side change that is a *partial* port of crypto is
still labelled both-changed unless the whole blob matches an older crypto revision, so
the 21 both-changed rows overstate suite-original work. The 20 largest diffs below were
read by comparing the public items exclusive to each side and the module headers and
diff hunks of the largest changes, not line by line.

## 2. The 154 shared entries

Columns: lines in crypto, lines in suite, crypto-only / suite-only diff lines,
classification, commits since sync (crypto / suite). The 20 largest diffs (by crypto-only
plus suite-only lines) carry a rank and a one-phrase note. Ties and ordering are by the
command in [A6].

| entry | crypto lines | suite lines | crypto-only / suite-only | class | commits since sync | note (20 largest diffs) |
|:--|--:|--:|--:|:--|:--|:--|
| `aes/` (25 / 24 files) | 8,085 | 7,700 | 402 / 17 | crypto-ahead | 1 / 0 |  |
| `ai_schoof.rs` | 399 | 399 | 0 / 0 | identical | 0 / 0 |  |
| `algebra_cache.rs` | 665 | 666 | 0 / 1 | crypto-ahead | 1 / 0 |  |
| `aut_folded_rho.rs` | 1,710 | 1,710 | 2 / 2 | both-changed | 2 / 1 |  |
| `auto_attack.rs` | 376 | 376 | 1 / 1 | identical\* | 0 / 0 |  |
| `avalanche.rs` | 258 | 258 | 0 / 0 | identical | 0 / 0 |  |
| `b_seed_profile.rs` | 289 | 269 | 40 / 20 | crypto-ahead | 2 / 0 |  |
| `bielliptic_quartic.rs` | 1,183 | 1,183 | 0 / 0 | identical | 2 / 2 |  |
| `binary_isogeny.rs` | 496 | 481 | 28 / 13 | crypto-ahead | 4 / 0 |  |
| `binary_semaev.rs` | 577 | 577 | 1 / 1 | identical\* | 0 / 0 |  |
| `binary_semaev_s4.rs` | 1,933 | 1,919 | 28 / 14 | both-changed | 5 / 1 |  |
| `binary_velu.rs` | 1,920 | 676 | 1,265 / 21 | crypto-ahead | 6 / 0 | **#14** crypto: Vélu point maps and counted kernel polynomials; suite stops at the isogeny |
| `bleichenbacher.rs` | 709 | 709 | 1 / 1 | identical\* | 0 / 0 |  |
| `boolean.rs` | 171 | 171 | 0 / 0 | identical | 0 / 0 |  |
| `boomerang.rs` | 1,058 | 1,058 | 0 / 0 | identical | 0 / 0 |  |
| `bsgs_fast.rs` | 2,495 | 1,331 | 1,269 / 105 | crypto-ahead | 3 / 0 | **#11** crypto: counted field operations (`OpCounts`, `CountingField`, `StepKernel`) |
| `canonical_lift.rs` | 631 | 642 | 72 / 83 | both-changed | 1 / 1 |  |
| `cga_hnc.rs` | 2,207 | 2,207 | 3 / 3 | both-changed | 1 / 1 |  |
| `cheon_attack.rs` | 420 | 420 | 4 / 4 | crypto-ahead | 1 / 0 |  |
| `cipher_registry.rs` | 317 | 317 | 0 / 0 | identical | 0 / 0 |  |
| `cm_canonical_lift.rs` | 478 | 478 | 4 / 4 | identical\* | 0 / 0 |  |
| `coleman_integration.rs` | 699 | 699 | 0 / 0 | identical | 0 / 0 |  |
| `coordinate_descent.rs` | 938 | 938 | 0 / 0 | identical | 0 / 0 |  |
| `coordinate_quotients.rs` | 1,616 | 1,616 | 1 / 1 | identical\* | 0 / 0 |  |
| `coordinate_search.rs` | 2,921 | 2,921 | 0 / 0 | identical | 0 / 0 |  |
| `crossbred.rs` | 1,217 | 1,214 | 6 / 3 | both-changed | 3 / 1 |  |
| `curve_catalog.rs` | 1,017 | 977 | 48 / 8 | crypto-ahead | 3 / 0 |  |
| `degree_reduction.rs` | 3,236 | 3,152 | 113 / 29 | crypto-ahead | 1 / 0 |  |
| `degree_reduction_anf.rs` | 580 | 580 | 1 / 1 | identical\* | 0 / 0 |  |
| `descent_algebraic.rs` | 266 | 266 | 0 / 0 | identical | 0 / 0 |  |
| `descent_expansion.rs` | 900 | 900 | 0 / 0 | identical | 0 / 0 |  |
| `descent_lowgamma.rs` | 747 | 747 | 0 / 0 | identical | 0 / 0 |  |
| `descent_treewidth.rs` | 319 | 319 | 0 / 0 | identical | 0 / 0 |  |
| `diem_descent.rs` | 513 | 513 | 0 / 0 | identical | 0 / 0 |  |
| `ec_index_calculus.rs` | 2,404 | 1,440 | 1,010 / 46 | crypto-ahead | 10 / 0 | **#17** crypto: staged runs, counted relation search, fast `F_p` root finding, mod-n particular solve |
| `ec_index_calculus_j0.rs` | 1,208 | 884 | 337 / 13 | crypto-ahead | 17 / 0 |  |
| `ec_trapdoor.rs` | 957 | 646 | 408 / 97 | crypto-ahead | 4 / 0 |  |
| `ecdlp_variants/` (3 / 3 files) | 1,875 | 1,758 | 173 / 56 | crypto-ahead | 3 / 0 |  |
| `ecdsa_audit.rs` | 529 | 529 | 0 / 0 | identical | 0 / 0 |  |
| `ecm.rs` | 395 | 395 | 1 / 1 | suite-ahead | 0 / 1 |  |
| `eds_mov.rs` | 284 | 284 | 1 / 1 | identical\* | 0 / 0 |  |
| `eds_net.rs` | 534 | 534 | 0 / 0 | identical | 0 / 0 |  |
| `eds_residue.rs` | 1,769 | 1,769 | 0 / 0 | identical | 0 / 0 |  |
| `eds_tate.rs` | 856 | 856 | 0 / 0 | identical | 0 / 0 |  |
| `f4_fp.rs` | 3,724 | 1,493 | 2,373 / 142 | crypto-ahead | 16 / 0 | **#6** crypto: `F4Trace` record and replay, per-operation field counters |
| `f4_fp_tower.rs` | 3,227 | 2,553 | 770 / 96 | crypto-ahead | 7 / 0 | **#19** crypto: dense echelon-row basis elements (`Elem`), full-rank exit, tower term arithmetic |
| `f4_rref_tests.rs` | 887 | 881 | 13 / 7 | crypto-ahead | 1 / 0 |  |
| `fes_gpu.rs` | 462 | 462 | 0 / 0 | identical | 0 / 0 |  |
| `ffd_harness.rs` | 1,371 | 1,211 | 162 / 2 | crypto-ahead | 3 / 0 |  |
| `fght_snfs.rs` | 1,839 | 1,835 | 14 / 10 | crypto-ahead | 1 / 0 |  |
| `fx_hash.rs` | 127 | 124 | 3 / 0 | crypto-ahead | 1 / 0 |  |
| `gaudry_cubic.rs` | 4,679 | 4,431 | 269 / 21 | crypto-ahead | 6 / 0 |  |
| `gaudry_quartic.rs` | 3,109 | 1,878 | 1,356 / 125 | crypto-ahead | 7 / 0 | **#10** crypto: traced solve, `solve_system_n`, eigenvector and charpoly helpers |
| `gf2_elim.rs` | 1,436 | 637 | 852 / 53 | crypto-ahead | 12 / 0 | **#18** crypto: counted, resumable echelon (`echelon_prefix_counted`, `echelon_resume_counted`) |
| `gf3m.rs` | 868 | 865 | 4 / 1 | identical\* | 0 / 0 |  |
| `ghs_descent.rs` | 747 | 703 | 85 / 41 | crypto-ahead | 3 / 0 |  |
| `ghs_full_attack.rs` | 492 | 479 | 30 / 17 | crypto-ahead | 4 / 0 |  |
| `groebner_f4.rs` | 482 | 482 | 0 / 0 | identical | 0 / 0 |  |
| `hash_attacks.rs` | 645 | 645 | 0 / 0 | identical | 0 / 0 |  |
| `hilbert_class_poly.rs` | 300 | 300 | 0 / 0 | identical | 0 / 0 |  |
| `hnp_ecdsa.rs` | 629 | 629 | 0 / 0 | identical | 0 / 0 |  |
| `hyperelliptic_ic_bench.rs` | 1,109 | 968 | 160 / 19 | crypto-ahead | 5 / 0 |  |
| `hyperelliptic_index_calculus.rs` | 2,801 | 1,825 | 1,154 / 178 | crypto-ahead | 10 / 0 | **#13** crypto: counted classification, `factor_base_walk`, `Decomposition` |
| `ic_boundary.rs` | 10,489 | 7,484 | 3,253 / 248 | crypto-ahead | 40 / 0 | **#5** crypto: completion rules, negation-folded prime batch, pinned calibration ratios, strided prime base, Koblitz floor instance |
| `ic_corpus.rs` | 602 | 602 | 1 / 1 | identical\* | 0 / 0 |  |
| `ic_descent_degrees.rs` | 1,068 | 1,067 | 1 / 0 | crypto-ahead | 1 / 0 |  |
| `ic_engine.rs` | 372 | 254 | 118 / 0 | crypto-ahead | 1 / 0 |  |
| `ic_framework/` (12 / 5 files) | 12,736 | 3,935 | 8,910 / 109 | both-changed | 73 / 1 | **#2** crypto adds 7 files (cm_orbit, fiber, large_prime, prime_tower, residual, rootfind, shared_rank); suite keeps 5 and adds 109 lines of its own |
| `ic_oracle_pricing.rs` | 1,198 | 1,198 | 0 / 0 | identical | 0 / 0 |  |
| `ic_progress.rs` | 393 | 393 | 1 / 1 | identical\* | 0 / 0 |  |
| `ic_run.rs` | 759 | 645 | 131 / 17 | crypto-ahead | 7 / 0 |  |
| `inherited_f4.rs` | 3,870 | 2,005 | 2,017 / 152 | crypto-ahead | 11 / 0 | **#7** crypto: lane-parallel engine (`Lane`, `LaneStats`), compact refuted basis, root-elimination counters |
| `invalid_curve_attack.rs` | 423 | 532 | 93 / 202 | suite-ahead | 0 / 1 |  |
| `isogeny_class_search.rs` | 2,127 | 2,127 | 0 / 0 | identical | 1 / 0 |  |
| `isogeny_degree_search/` (5 / 5 files) | 3,037 | 2,943 | 127 / 33 | crypto-ahead | 3 / 0 |  |
| `isogeny_degree_search.rs` | 395 | 395 | 0 / 0 | identical | 0 / 0 |  |
| `j0_twists.rs` | 389 | 389 | 0 / 0 | identical | 0 / 0 |  |
| `koblitz_bench.rs` | 2,952 | 1,737 | 1,293 / 78 | crypto-ahead | 14 / 0 | **#12** crypto: closure-degree and refutation-profile ladder, sparse closure, semi-regular d_reg |
| `koblitz_factor_base_search.rs` | 1,967 | 1,602 | 386 / 21 | crypto-ahead | 15 / 0 |  |
| `koblitz_fast.rs` | 2,021 | 1,637 | 411 / 27 | crypto-ahead | 10 / 0 |  |
| `koblitz_fast_arith.rs` | 1,426 | 967 | 466 / 7 | crypto-ahead | 10 / 0 |  |
| `koblitz_groebner.rs` | 9,826 | 7,443 | 3,093 / 710 | both-changed | 63 / 2 | **#3** crypto: linear-harvest, fall-row gate, lane roots, sparse F5 builders; suite: `F4Kernel` dispatch into `f4_gf2`, `solver_degree_ladder` |
| `koblitz_index_calculus.rs` | 21,760 | 14,332 | 8,728 / 1,300 | both-changed | 127 / 3 | **#1** crypto: PDP outcome and SAT-backend enums, aimed and forced-point collection, fast m=2 paths; suite: only a batched `groebner_decompose_batch` / `SharedDecider` hook |
| `koblitz_isogeny_cost.rs` | 2,294 | 1,701 | 620 / 27 | crypto-ahead | 9 / 0 |  |
| `koblitz_pdp_phase_a.rs` | 1,140 | 1,140 | 0 / 0 | identical | 0 / 0 |  |
| `koblitz_relation_solver.rs` | 1,406 | 955 | 511 / 60 | both-changed | 6 / 1 |  |
| `koblitz_sparse_la.rs` | 5,537 | 2,601 | 3,264 / 328 | crypto-ahead | 10 / 1 | **#4** crypto: block Lanczos, CADO-style filtering, `SparseCoreSolver` / `SpmvBackend`; suite holds an older crypto revision of the mod-n core (division-free products, port 0ab1948) |
| `koblitz_symmetrised.rs` | 3,468 | 3,455 | 16 / 3 | crypto-ahead | 6 / 0 |  |
| `lattice.rs` | 994 | 994 | 1 / 1 | identical\* | 0 / 0 |  |
| `legacy_curve_attacks.rs` | 390 | 390 | 0 / 0 | identical | 0 / 0 |  |
| `matrix_f5_f2.rs` | 1,706 | 697 | 1,056 / 47 | crypto-ahead | 15 / 0 | **#16** crypto: `F5OutputForm`, timed variants, row-space fingerprint |
| `mazur_tate_sigma.rs` | 809 | 809 | 1 / 1 | identical\* | 0 / 0 |  |
| `md5_chosen_prefix.rs` | 1,636 | 1,636 | 16 / 16 | identical\* | 0 / 0 |  |
| `md5_differential.rs` | 243 | 243 | 0 / 0 | identical | 0 / 0 |  |
| `md5_hashclash_ffi.rs` | 269 | 265 | 7 / 3 | identical\* | 0 / 0 |  |
| `ml_dsa_fault.rs` | 434 | 434 | 0 / 0 | identical | 0 / 0 |  |
| `ml_dsa_leakage.rs` | 961 | 961 | 0 / 0 | identical | 0 / 0 |  |
| `ml_kem_pco.rs` | 749 | 749 | 0 / 0 | identical | 0 / 0 |  |
| `ml_rho_walks.rs` | 399 | 397 | 2 / 0 | identical\* | 0 / 0 |  |
| `mlwe/` (8 / 8 files) | 4,548 | 4,546 | 3 / 1 | identical\* | 0 / 0 |  |
| `mod.rs` | 458 | 373 | 112 / 27 | both-changed | 150 / 6 |  |
| `modular_polynomial.rs` | 447 | 447 | 0 / 0 | identical | 0 / 0 |  |
| `mov_attack.rs` | 390 | 365 | 121 / 96 | suite-ahead | 0 / 1 |  |
| `mq_fes.rs` | 1,228 | 1,242 | 9 / 23 | both-changed | 2 / 2 |  |
| `mq_monica.rs` | 539 | 539 | 0 / 0 | identical | 1 / 1 |  |
| `multi_key_hnp.rs` | 385 | 385 | 1 / 1 | identical\* | 0 / 0 |  |
| `nonanom_formal_log.rs` | 887 | 887 | 3 / 3 | identical\* | 0 / 0 |  |
| `orbit_homology.rs` | 389 | 389 | 0 / 0 | identical | 0 / 0 |  |
| `p256_attacks.rs` | 853 | 853 | 1 / 1 | identical\* | 0 / 0 |  |
| `p256_isogeny_cover.rs` | 2,996 | 2,993 | 3 / 0 | crypto-ahead | 1 / 0 |  |
| `p256_speculation.rs` | 407 | 407 | 0 / 0 | identical | 0 / 0 |  |
| `p256_structural.rs` | 1,715 | 1,715 | 0 / 0 | identical | 0 / 0 |  |
| `pc_degree_avg.rs` | 576 | 576 | 1 / 1 | identical\* | 0 / 0 |  |
| `pc_degree_harness.rs` | 790 | 790 | 4 / 4 | identical\* | 0 / 0 |  |
| `petit_quisquater.rs` | 1,596 | 1,593 | 7 / 4 | crypto-ahead | 1 / 0 |  |
| `pkm_criterion.rs` | 738 | 738 | 0 / 0 | identical | 0 / 0 |  |
| `pohlig_hellman.rs` | 651 | 406 | 335 / 90 | both-changed | 3 / 1 |  |
| `pollard_collab/` (8 / 8 files) | 4,601 | 4,277 | 564 / 240 | both-changed | 7 / 2 | **#20** crypto: heartbeat, `publish_artifact`, `reveal_pending`; suite: token-authenticated peers (`start_with_token`, `sync_with_peer_auth`) |
| `pollard_rho.rs` | 2,315 | 2,367 | 110 / 162 | both-changed | 4 / 3 |  |
| `polynomial_reuse.rs` | 733 | 733 | 1 / 1 | both-changed | 2 / 1 |  |
| `pq_descent.rs` | 788 | 788 | 0 / 0 | identical | 0 / 0 |  |
| `pq_descent_symbolic.rs` | 614 | 590 | 147 / 123 | both-changed | 5 / 2 |  |
| `pq_f4_f2.rs` | 2,610 | 1,008 | 1,749 / 147 | crypto-ahead | 25 / 0 | **#8** crypto: Four Russians leading-block tables (`BlockTables`) under the unchanged `word_xors` unit |
| `pq_groebner_f2.rs` | 1,777 | 1,880 | 51 / 154 | both-changed | 7 / 3 |  |
| `pq_sparse_la.rs` | 831 | 831 | 0 / 0 | identical | 4 / 1 |  |
| `pq_wiedemann.rs` | 1,053 | 1,053 | 0 / 0 | identical | 3 / 1 |  |
| `pq_xl.rs` | 419 | 419 | 0 / 0 | identical | 0 / 0 |  |
| `preprocessing_rho.rs` | 472 | 442 | 34 / 4 | crypto-ahead | 1 / 0 |  |
| `quantum_estimator.rs` | 396 | 396 | 0 / 0 | identical | 0 / 0 |  |
| `quasi_subfield.rs` | 829 | 829 | 1 / 1 | identical\* | 0 / 0 |  |
| `research_bench.rs` | 1,556 | 1,474 | 82 / 0 | crypto-ahead | 2 / 0 |  |
| `residual_walk.rs` | 3,852 | 3,885 | 6 / 39 | both-changed | 3 / 3 |  |
| `sat.rs` | 3,250 | 2,518 | 981 / 249 | crypto-ahead | 18 / 0 | **#15** crypto: 32-bit clause arena, packed-key heap, solve deadline; suite: older layout, crate path in docs |
| `sbox.rs` | 803 | 803 | 1 / 1 | identical\* | 0 / 0 |  |
| `semaev_corpus.rs` | 863 | 863 | 1 / 1 | identical\* | 0 / 0 |  |
| `semaev_decomp.rs` | 4,041 | 2,620 | 1,484 / 63 | both-changed | 21 / 2 | **#9** crypto: portable and clmul `GF(2^128)` kernels (`Gf2_128`, `OldGf2`); suite: `mod linearised_m4` and `mod coset` hooks |
| `semaev_higher.rs` | 889 | 889 | 0 / 0 | identical | 0 / 0 |  |
| `semaev_leading_form.rs` | 1,371 | 1,792 | 5 / 426 | both-changed | 1 / 2 |  |
| `semaev_sat.rs` | 1,792 | 1,525 | 272 / 5 | both-changed | 2 / 1 |  |
| `sha1_differential.rs` | 525 | 525 | 0 / 0 | identical | 0 / 0 |  |
| `shor.rs` | 524 | 524 | 0 / 0 | identical | 0 / 0 |  |
| `signal_ratchet.rs` | 608 | 608 | 0 / 0 | identical | 0 / 0 |  |
| `signature_corpus.rs` | 859 | 859 | 2 / 2 | identical\* | 0 / 0 |  |
| `solinas_correlations.rs` | 426 | 426 | 1 / 1 | identical\* | 0 / 0 |  |
| `sparse_macaulay.rs` | 413 | 189 | 224 / 0 | crypto-ahead | 3 / 0 |  |
| `statistical.rs` | 227 | 227 | 0 / 0 | identical | 0 / 0 |  |
| `symmetrized_semaev.rs` | 563 | 563 | 0 / 0 | identical | 0 / 0 |  |
| `tls12_kdf.rs` | 788 | 788 | 1 / 1 | identical\* | 0 / 0 |  |
| `tls13_kdf.rs` | 643 | 643 | 0 / 0 | identical | 0 / 0 |  |
| `visual_demos.rs` | 480 | 480 | 3 / 3 | identical\* | 0 / 0 |  |
| `visualize.rs` | 327 | 327 | 1 / 1 | identical\* | 0 / 0 |  |
| `wdsat_oracle.rs` | 792 | 792 | 0 / 0 | identical | 0 / 0 |  |
| `weil_charts.rs` | 654 | 654 | 1 / 1 | crypto-ahead | 1 / 0 |  |
| `wide_groebner.rs` | 2,124 | 1,721 | 452 / 49 | crypto-ahead | 4 / 0 |  |

Reading the table:

- **Largest, and why they matter.** Ranks 1, 3, 4 and 9 are the Koblitz IC driver, its
  Gröbner engine, its sparse linear algebra and the Semaev oracles: the code the
  assessment's ranked angles (§2) all run on. In every one crypto is ahead by
  thousands of lines (`koblitz_index_calculus.rs` 8,728 crypto-only against 1,300
  suite-only; `koblitz_groebner.rs` 3,093 against 710). The suite-side lines in those
  files include the F4 kernel dispatch (`koblitz_groebner.rs`), a batched decider hook
  (`koblitz_index_calculus.rs`) and two submodule hooks (`semaev_decomp.rs`), all
  tied to suite-only files in §3.
- **`ic_framework/`** (rank 2) is the plug-in pipeline both repositories share by name.
  crypto has 12 files (7 more than the suite: `cm_orbit`, `fiber`, `large_prime`,
  `prime_tower`, `residual`, `rootfind`, `shared_rank`), 12,736 lines against 3,935;
  the suite's copy predates those files.
- **The three suite-ahead rows** are non-IC: `ecm.rs` (a 1-line difference),
  `invalid_curve_attack.rs` (the suite made it a real oracle attack, `53d2892`) and
  `mov_attack.rs` (the suite routed the legacy MOV API to the real Tate-pairing attack,
  `273d1a2`). No IC solver file is suite-ahead.
- **Identical** is 37,414 lines on each side (52 entries), a candidate for the
  shared-crate mechanism of §5.3.
- `groebner_f4.rs` is identical on both sides. The suite's own audit calls it
  Buchberger "despite its filename" with a 5,000-step cap that returns a plain vector
  with no completion status (`experiments/groebner-perf-20260924/README.md`, remaining
  bottleneck 5); that follow-up is open in both repositories. [A10]

## 3. The 23 suite-only entries

`suite/src/cryptanalysis` has 23 entries crypto lacks, 39,327 lines. Seven are directories:
three are test trees (`affine_factor_base/`, `crossbred_blocks/`, `walksat/`, each one
`tests.rs`), which move with their parent file; `koblitz_index_calculus/` and
`semaev_decomp/` are submodule trees of files crypto also has; `factoring/` and
`weak_curves/` are stand-alone modules. Port judgments
use the suite's own numbers, cited to file and line in [A8]; "stage" means a
stage diagnostic, not an end-to-end `S`.

| suite-only entry | lines | port? | reason, with the suite's numbers |
|:--|--:|:--|:--|
| `f4_gf2.rs` | 1,757 | **yes** (port 1) | Rank-indexed Macaulay build (column index from descending DegRevLex rank, XOR rows, no hash or sort), decision from a linear-block RREF, F5 row pruning; answers bit-identical to the reference kernel. Suite: 10.29x [10.26, 10.34] and 9.95x [9.43, 10.25] on the PDP stage inside complete verified DLPs (paired, one core, `icv1-f2m23-tm5197-1f85e9e1`, `icv1-f2m23-t5197-69e76b73`); 2.1 to 12.2x on the frozen stage ladder; 190 to 280x on the degree sweeps (`dreg_sweep` n=11 m=2: 594.3 s to 3.1 s). S and the rho ratio are null in the suite's records. Crypto's `solving_degree_from` runs on `solving_profile_sparse`; the suite's table names a sparse path as its "previous sweep path" and measures it at 158.7 s against 6.11 s on `icv1-f2m13-t181-515ee569` m=2 D=5. Baseline in the suite is the reference kernel, not crypto's inherited engine: the gain over crypto's default is unmeasured, which is T14. |
| `f4_batch.rs` | 573 | no | Lockstep search so a batch of searches hands every round to one backend call. Suite (RTX 5090 pod, `icv1-f2m23-tm5197-1f85e9e1`, 1,024 targets, 13 threads): host lockstep 8.47 s, searches across cores 6.64 s, device decider 5.57 s, the inherited engine (default) **1.25 s**. It pays only with a device and still loses to the default 4.5x. |
| `f4_gpu.rs` | 2,228 | **maybe** (port 3, echelon half only) | `libcuda`/NVRTC through `dlopen`, plus a host emulator. Per-node decider: 127,356 decisions/s on the RTX 5090 against 114,584/s on 13 host threads (1.11x), so no. Large-matrix echelon (`F4_F2_ECHELON=cuda`): `icv1-f2m13-t181-515ee569` m=2 D=6, 168,363 x 188,203, **425.8 s to 5.70 s (74.7x)**; `icv1-f2m11-t67-05f5aa36` D=6 25.5x; `dreg_sweep` 18.8 s to 8.2 s with an identical table. One device, one pod, stage diagnostics. `f4_gf2.rs` calls three hooks here (`offload_expected`, `offload_echelon`, `emulate_echelon`), so port 1 needs stubs until port 3 lands. |
| `suite/cuda/` (5 files, 1,359 lines; outside `src/cryptanalysis`) | 1,359 | echelon half: maybe (port 3) | `f4_gf2_kernel.cu` (82) holds the entry points for both the batched decider (`f4_gf2_decide_batch`) and the large-matrix elimination (`f4e_*`, six kernels); `f4_gf2_echelon.cuh` (465) is the elimination, `f4_gf2_device.cuh` (672) and `f4_gf2_emulate.c` (100) are the decider. Build check: sm_75 to sm_120 compile with 48 to 64 registers, no spills. Take `echelon.cuh`, the `f4e_*` half of the kernel file and `build_kernel.sh`; leave the decider. |
| `semaev_decomp/` (3 files: `linearised_m4.rs` 922, its `tests.rs` 507, `coset.rs` 413) | 1,842 | **yes** for `linearised_m4` (port 2); no for `coset.rs` | m = 4 oracle: loop over triples, octic `R(x) = Res_U(S4, S3)`, `gcd(R, L_V mod R)`. Suite (`0ea8931`): callgrind Ir 2,618,061,549 (generic chain) to 72,249,811 (**36.2x**); paired wall 380.6 ms to 5.97 ms (**61.3x**, 95% CI 38.4 to 79.8, 5 rounds) at n=10, l=4, identical fingerprints. Stage, one cell shape. `coset.rs` is the affine-base oracle (see `affine_factor_base`). It needs `quartic_general_with`, `subspace_poly_for_basis`, `Gf2`, `LvHorner`, `GeneralTargetPowers` and `SubspaceOracle` from `semaev_decomp.rs`, all present in crypto's copy. |
| `semaev_sat_s5.rs` | 2,174 | maybe | Hardcoded 68-term symmetrised S5 as a SAT encoder for m = 4. In-source table: 4.6 to 23.6x fewer instructions and 7.6 to 38.7x paired wall against the generic chain on four kernels (`n8l2`, `n9l3`, `n11l3`). Not uniform: one Tseitin auxiliary per e-monomial (about 5,300 at n=10, l=3) so some targets pay tens of ms where the chain refutes in under 1 ms. Second-best measured m = 4 oracle; worth it only if crypto's m = 4 oracle ladder uses SAT. |
| `s5_weil_template.rs` | 2,157 | no | m = 4 descent from a hardcoded S5, opt-in. The suite's #957 (newer than this checkout) measured the winning regime as l=3, n>=17, base not Frobenius-invariant (n17 1.85x, n23 1.38x) and losses elsewhere (n15 subfield 0.31x, n17 l4 0.54x, n13 l3 about 0.9x); the #954 message adds that the symmetrised variant was not faster on any measured cell (0.91x). Narrow, 1.85x at best. |
| `fes_chain.rs` | 700 | no | Gray-code exhaustive search over summand bits for m >= 3. The suite's #957 (newer than this checkout) had to cap the walk at 2^28 leaves because a cell near l=20 "could run for hours", and documents that the sorted walk can miss degenerate decompositions. No committed speed-up figure beyond perfbench kernel names (`pdp/spec_fes_*`). |
| `fes_chain_tests.rs` | 635 | no | Differential tests of `fes_chain`; moves with it. |
| `koblitz_symmetrised_m5.rs` | 1,841 | **maybe** | m = 5 in the symmetrised Artin-Schreier frame: the `[4,4]` pair shape has 5(l-1)+1+n unknowns against 5l+3n for the x-frame chain, and `meet_roots` solves two-link chains exactly; fits 64 variables at n >= 19 where the chain does not. Differential tests only, no timing against crypto's baseline. The assessment (§2, closing paragraph) names a chain-free m >= 5 formulation as the one place an exponent story could still live; this is a candidate formulation, to be ported as research input rather than as a baseline improvement. |
| `koblitz_index_calculus/mitm_m5.rs` | 541 | no | Pair table times an enumerated triple for m = 5: F^3/6 lookups against F^4/24 group operations (F the factor-base size). Assessment §0.1: every pair-table IC is generic and `S·T^2 >= r/2n` binds it; a fifth summand does not change the class. |
| `pq_descent_symbolic_wide.rs` | 1,633 | maybe | Symbolic Weil descent for m >= 4 (S5 to S7) and a 128-bit monomial mask. Suite: the tensor engine runs at 0.17x to 0.80x of the fast path's time for n' = 3 to 14 at three summands (#954 message). Crypto has its own `pq_descent_symbolic` (crypto-ahead) and `wide_groebner` for the same caps (assessment §3, hard caps); reconcile before porting. |
| `affine_factor_base.rs` | 1,222 | no | Cosets V + c with specialised oracles. Module doc, lines 102 to 107: affine descent 2.7x (m=2) and 6.3x (m=3) faster than the generic hull descent, but "point enumeration remains faster than every algebraic route at these sizes" (0.6 to 0.75x the time of `S3Solve` / `S4PairsAndSolve`), and "no claim about relation-collection cost". |
| `affine_factor_base/` (`tests.rs`) | 1,165 | no | Tests of the above. |
| `crossbred_blocks.rs` | 1,120 | no | `(D, k)` read off the summand blocks. Review notes in the #954 message: m = 2 has "the same k as the generic split, constant-factor gain only"; the k = 16 crossbred was slower than the generic k = 13 and is never dispatched; XL is chosen for m >= 3. Assessment §1 already replaces plain XL degree readings with MutantXL closure. |
| `crossbred_blocks/` (`tests.rs`) | 631 | no | Tests of the above. |
| `walksat.rs` | 1,071 | no | Incomplete local-search backend, never reports UNSAT. Own doc, lines 56 to 69 of the file: the portfolio is about 50% slower than CDCL alone on undecomposable targets (0.043 s to 0.065 s, one noisy probe) and the CNF + XOR engine is slower than CDCL on every measured cell. |
| `walksat/` (`tests.rs`) | 931 | no | Tests of the above. |
| `prime_orbit_index_calculus.rs` | 3,177 | **maybe** (port 4, conditional) | Factor bases of whole `Aut`-orbits (generic w=2, j=0 w=6, j=1728 w=4) with gain-graph relations and large primes on certified scaled-down curves. Suite docs (`suite/docs/ic/README.md`, lines 793 to 801): 682 relations in 6,000 probes against 79 for the {+-1} control on one 24-bit j=0 curve (8.6x, model 8.7x), a relation-yield stage metric on toy curves. Crypto's `cm_orbit.rs` accepts only discriminants -19 and -43 (line 144), so this is the only mu_6 whole-orbit implementation. The assessment (§2.5) says to stop on prime fields apart from invariant-presentation engineering, so port only if prime fields are reopened, and only into ecbench's counted unit. |
| `ec_index_calculus_curves.rs` | 531 | maybe | Prime-field curve typing (generic, j = 0, j = 1728) and scaled certified instances; its tests call `prime_orbit_index_calculus`, so it ports or stays with that file. |
| `index_calculus_budget.rs` | 496 | maybe | Analytic gate: linear algebra `m·D^2` against rho, "a non-positive budget means the cell loses to rho even with a free decomposition oracle". Overlaps in intent with the free-oracle floor in assessment §0.3 (`2^56.46` against rho `2^60.81` at m = 4) and with ecbench bounds; compare before porting. Small. |
| `ec_challenges.rs` | 806 | no | Loader for `challenges/elliptic/corpus.json`, a dataset that lives in the suite repository; crypto's targets are the registry and ecbench public targets. |
| `weak_curves/` (8 files) | 4,640 | no | Smart, MOV (Tate pairing), singular-curve, Pohlig-Hellman and smooth-order attacks behind `crax`. Not index calculus; crypto has its own P-192 singular recovery (`p192_singular_recovery.rs`). |
| `factoring/` (15 files) | 7,456 | no | GNFS, SNFS, SIQS, p-1, rho, RSA attacks. Not a discrete-log lead (suite README, "Integer factorisation"). |

## 4. Crypto-only entries the suite lacks (106 entries, 165,927 lines)

Grouping is by subsystem; membership is the case statement in [A5].

| group | entries / files / lines | what the suite is missing |
|:--|:--|:--|
| G1. ecbench harness and measurement | 9 / 48 / 31,735 | `ecbench/` (33 files, 22,970 lines: counted unit, sealed sessions, replay, claims), `ecbench_large_prime`, `hwcost/` (8 files), `ic_measurement`, `challenge_ladder`, control archives, `groebner_cache` (Redis F4 cache), `matmul_exponent` (omega provenance). The suite has CLI-level `ca-ic bench / boundary / rho`, with no sealed sessions or replay certificates. |
| G2. curve registry and construction | 9 / 29 / 18,510 | `curve_aliases.json`, `curve_id`, `curve_traits/`, `curve_construction/` (13 files), `identity_certificate`, `secg_*`, `nist_seed_provenance`, `data/phi_j_11.txt`. |
| G3. wide arithmetic and field backends | 13 / 13 / 10,663 | `fp_backend` and its IFMA variant, `wide_gf2m`, `binary_field_basis`, `koblitz_wide`, `wide_sixsum`, `f6_wide_geometry`, `gls_fp2`, `ext_curve`, `subfield_fp3`, `biext_k2`, `genus2_fp`, `gpu_rootfind`. |
| G4. Koblitz and binary IC producers and oracles | 21 / 21 / 19,843 | Large-prime collectors (`koblitz_large_prime`, `large_prime_filter`), rotated chains, six-sum extraction, the strong single-target rho (`koblitz_strong_rho`), greedy and cyclotomic bases, symmetrised S4, pair-table orbits, line oracles, `pdp_split`, `delta_method`, `recursive_descent`, `lopsided_thin_product`, `semaev_s3_cost`, binary small-x and torsion walk. |
| G5. prime-field and extension-field IC | 15 / 15 / 23,952 | `prime_fast`, `prime_small_x`, `prime_field_smt`, `smt_oracle`, tower F4s (`f4_tower_generic`, `sig_fp_tower`), the k = 4 Gaudry family (`gaudry_quartic_bin`, `gaudry_quartic_d4`, `glv_gaudry`, `glv_invariant_*`), `d4_weighted_s5`, `quartic_folds`, `fghr_*`. |
| G6. isogeny, cover and Jacobian transfers | 18 / 44 / 38,617 | `isogeny_walk/` (17 files), `isogeny_eval/`, `jacobian_transfer/`, the Joux-Vitse family (`jv_*`, `joux_vitse*`), `ghs_*`, `smith_*`, `q_curve`, `two_sylow_search`. |
| G7. NIST and SEC curve audits, trapdoor probes | 16 / 22 / 16,780 | `ecdlp_nist/` (7 files), `p192_*`, `p224_hypotheses`, `p256_*` (backdoor map, isogeny walk and campaign, trapdoor rounds 2 and 3), `sect113r1_audit`. |
| G8. ECC2K-130 operations | 4 / 4 / 5,809 | `ecc2k130_guard`, `_merge`, `_pyjson`, `_status`. |
| G9. stray artefact | 1 / 1 / 18 | `koblitz_index_calculus.rs.rej.orig`, a `patch` reject file committed in `src/cryptanalysis`; it should be deleted. |

## 5. Recommendations

### 5.1 System of record, per subsystem

The harness of record is crypto's `ecbench`. `AGENTS.md` §12 (lines 1201 to 1206) says
new cross-method measurements "use the native harness `ecbench` ... so every method is
charged in one counted unit". `ml-cryptanalysis/docs/PLAN.md` lines 125 and 126 say
"Cost measurements that matter for a claim go back through ecbench, which is the
counted harness of record." One citation in the task brief did not match: `AGENTS.md`
line 748 sits in §7 ("Keep the scoreboard current") and makes the scoreboard file
canonical; it does not mention ecbench. The statement is at §12 and at line 797 (the
scoreboard-to-ecbench mapping row). [A11]

| subsystem | record | basis |
|:--|:--|:--|
| ecbench, counted unit, sealed sessions, bounds, leaderboard; curve registry and construction (G1, G2) | crypto | §12 and PLAN.md above; the suite has no counterpart |
| Boolean and prime-field Gröbner engines, closure and degree tools, Koblitz IC driver, sparse linear algebra, `ic_framework`, `ic_boundary` | crypto | 49 crypto-ahead and 21 both-changed entries; 53,761 crypto-only diff lines against 6,575; first inbound port is `f4_gf2` |
| m = 4 and 5 point-decomposition specialisations (`semaev_decomp/`, `semaev_sat_s5`, `s5_weil_template`, `fes_chain`, `koblitz_symmetrised_m5`, `pq_descent_symbolic_wide`, `affine_factor_base`, `crossbred_blocks`, `walksat`) | suite, until each is ported or retired | exist only there, with differential tests and perfbench kernels; none is priced in ecbench's unit |
| batched and GPU F4 (`f4_batch`, `f4_gpu`, `suite/cuda/`) | suite, until port 3 | only there; device receipts are under `cryptanalysis/experiments/f4-gpu-20260925` and `f4-gpu-runpod-20261006` |
| prime-field automorphism-orbit IC (`prime_orbit_index_calculus`, `ec_index_calculus_curves`, `ca-ic prime`) | suite, undecided | only there; the assessment says stop on prime fields |
| non-IC attack library (symmetric, hash, mlwe, aes, factoring, weak curves, collaborative rho, `crax`) | suite | its README says the library was "moved here and put under this repository's checks" (fmt, clippy `-D warnings`, tests, cargo-deny); crypto has no factoring or weak-curve arms; crypto-ahead changes in the shared non-IC entries (`aes/` +402 lines, `mlwe/`, `pollard_collab`) should be forward-ported into the suite or declared divergent |
| shared leaf modules, 52 identical entries, 37,414 lines | one record, imported by pin | see 5.3 |

### 5.2 The three highest-value ports

Order is by the suite's own measured gain, then by whether the code sits under crypto's
open measurements (assessment §2 and §3: closure-degree and Gröbner work, the
decomposition-oracle floor), then by coupling. Line counts are `wc -l` of the moved
files plus a judged allowance for glue, marked "est."; they were not built. [A9]

1. **`f4_gf2.rs`, the rank-indexed Boolean F4 kernel (T14). Est. 2,000 lines**:
   1,757 (`f4_gf2.rs`, of which about 320 are tests) + about 200 for the `F4Kernel` dispatch
   in `koblitz_groebner.rs` (`f4_kernel`, the `matrix_f4_f2` match, `f4_profile_add_kernel`,
   `macaulay_profile_fast`, `solving_profile_fast`) + about 30 for stubs of the three
   `f4_gpu` hooks. Dependencies (`fx_hash`, `pq_groebner_f2`, `sparse_macaulay`) exist
   in crypto. Why first: it is the largest measured gain (about 10x on the PDP stage
   against the same reference kernel, 190 to 280x on sweeps), it is a drop-in with an
   environment switch (`F4_F2_RREF=reference`) that keeps the control in the binary, and
   42 example files and 57 source files in crypto reference `solving_degree`,
   `first_fall_degree`, `macaulay_profile` or `matrix_f4_f2`. The comparison T14 owes: a
   same-binary run of `f4_gf2` against crypto's *inherited* F4 and Four Russians paths
   (assessment §3: 7.5x and 6.9x already on `n <= 62`), because the suite's baseline
   was the reference kernel and its own receipts show the inherited engine beating the
   from-scratch lockstep path 4.5x. Open: whether the MutantXL closure matrices use this
   elimination at all.
2. **`semaev_decomp/linearised_m4`, the m = 4 octic oracle. Est. 1,450 lines**:
   922 + 507 tests + the two `mod` hooks in `semaev_decomp.rs`. All names it imports
   from `super::` already exist in crypto's `semaev_decomp.rs`, and the
   `koblitz_groebner` and `polynomial_reuse` items it uses exist too. Gain: 36.2x
   instructions and 61.3x paired wall (CI 38.4 to 79.8) over the generic chain on one
   cell shape. Caveat: one cell shape; the next step is a same-binary sweep over
   `(n, l)` including the shapes ecbench prices at m = 4, since the assessment puts the
   m = 4 oracle budget at `2^5.71` group operations per call.
3. **The GPU large-matrix echelon path. Est. 1,750 lines**: `f4_gf2_echelon.cuh` 465,
   the `f4e_*` kernels (about 40 of the 82-line kernel file), `build_kernel.sh` 40, and
   about 1,150 lines of `f4_gpu.rs` once the batched decider is split out (the file is
   1,621 non-test lines, of which the CUDA driver module and the offload logic are
   shared); upper bound 2,200 if taken whole. Gain: 74.7x at D = 6 and 25.5x on the
   second matrix, on one RTX 5090. Conditions: needs port 1 first (its host fallback and
   hooks), a device to reproduce (none in the analysis container), and it moves matrix
   elimination only; the suite measured that for the matrices the pipeline actually
   builds at degree 23, F4 is not the fastest oracle (0.28 s against 0.05 s for `mq-fes`).

Not ported, with reasons in §3: `f4_batch` (loses to the default engine), the per-node
GPU decider (1.11x over 13 host threads), `s5_weil_template` (1.85x at best, narrow),
`walksat`, `crossbred_blocks`, `affine_factor_base` (each loses to a simpler path on
the suite's own numbers). Held for a decision, not recommended: `prime_orbit_index_calculus`
with `ec_index_calculus_curves` (est. 3,708 lines, port only if prime fields are
reopened), `semaev_sat_s5` (2,174), `koblitz_symmetrised_m5` (1,841, as research input
for a chain-free m >= 5 formulation).

### 5.3 Process

- **Stop re-copying; pin.** On 2026-10-16 the suite's origin/`main` took #891, which
  removed `avalanche.rs` and `statistical.rs` from the suite and imports them from
  crypto's `crates/crypto-analysis-core` by a pinned sparse checkout
  (`tools/bootstrap_crypto_analysis_core.sh`, rev `e4f200cc`). The checkout
  (the default-branch line, at `0ea8931`) does not have it: both files are still
  byte-identical copies there. The 52 identical entries are the next candidates, in
  the same way. [A12]
- **Name the line.** The suite is developed on `claude/cryptanalysis-repo-setup-p59kek`,
  its default branch, 936 commits at `8fd4429`; `main` (957 commits at `3ce4c29`) is
  a different line that lacks #954. A divergence report taken against `main` would
  have missed every suite-only file in §3 that came from #954.
- **Record each port with its delta.** Per the success rule, a ported kernel is a
  stage improvement until it moves end-to-end `S`; report each against the matched
  rho on every row, and keep the reference kernel selectable in the same binary.
- **Delete the stray reject file** (G9) and rename or retire `groebner_f4.rs`
  (identical on both sides, Buchberger by the suite's audit).
- **Re-run this report** after each port: the script in [A4] regenerates the
  table in about 30 seconds from the two clones.

## 6. What this note does not establish

- It re-measured nothing. Every suite speed-up is copied from a committed suite
  receipt or message and is a stage diagnostic on the cell shapes it names. The device
  numbers are one RTX 5090 on one pod.
- The classification is a model (§1). A both-changed row can hide a partial port, and
  an identical\* row is identical only up to the import-time edits.
- The 20 notes in §2 summarise public-item differences and headers; they are not code
  reviews.
- No cross-method comparison is made, so the §5a pricing band (0, 0.1, 1 GAE per
  operation) does not apply; it will apply to the ecbench read-out that follows a port.
- During this task the working tree of `/home/user/cryptanalysis` was changed by
  another process (161 entries instead of 177, the checked-out `HEAD` unchanged). All
  numbers use `git archive 0ea8931` of that repository, not its working tree. [A2]

## Appendix: commands that reproduce every number

Run in a scratch directory `$D` (it needs about 30 MB). `crypto` paths are the worktree
`/home/user/wt/t13`; `suite` files come from `git archive`, not the working tree of
`/home/user/cryptanalysis` (see §6). Nothing here writes into either repository.

### A1. Pins

```sh
git -C /home/user/wt/t13 rev-parse HEAD                              # f22d55b9c...
git -C /home/user/wt/t13 rev-parse HEAD:src/cryptanalysis            # acd8b3c10b16...
git -C /home/user/wt/t13 rev-parse --is-shallow-repository           # true
git -C /home/user/wt/t13 log --format=%ad --date=short -- src/cryptanalysis | tail -1   # 2026-10-17
git -C /home/user/cryptanalysis rev-parse HEAD                       # 0ea8931d...
git -C /home/user/cryptanalysis rev-parse HEAD:suite/src/cryptanalysis   # f6d41e491cb3...
gh api repos/aburan28/cryptanalysis --jq .default_branch             # claude/cryptanalysis-repo-setup-p59kek
gh api repos/aburan28/cryptanalysis/pulls/954 --jq '{merged,merged_at,base:.base.ref}'
```

### A2. Full-history clones, the suite snapshot, and the second-sync agreement

```sh
git clone --bare --filter=blob:none --single-branch --branch main \
    https://github.com/aburan28/crypto.git $D/crypto-full.git         # main tip b7cafe04
git clone --bare --filter=blob:none --single-branch \
    https://github.com/aburan28/cryptanalysis.git $D/cryptanalysis-full.git   # default branch, tip 8fd4429
git -C $D/crypto-full.git rev-parse b7cafe04:src/cryptanalysis       # acd8b3c10b16..., equals the worktree tree
git -C $D/cryptanalysis-full.git merge-base --is-ancestor 0ea8931d HEAD; echo $?     # 0
git -C $D/cryptanalysis-full.git rev-list --count HEAD               # 936
mkdir $D/suite-snap
git -C /home/user/cryptanalysis archive 0ea8931 suite/src/cryptanalysis suite/cuda | tar -x -C $D/suite-snap
ls $D/suite-snap/suite/src/cryptanalysis | wc -l                     # 177
cd $D
git -C crypto-full.git ls-tree -r 54e977ab src/cryptanalysis \
  | awk '{sub("src/cryptanalysis/","",$4); print $3" "$4}' | sort > xc.txt
git -C cryptanalysis-full.git ls-tree -r 613820d suite/src/cryptanalysis \
  | awk '{sub("suite/src/cryptanalysis/","",$4); print $3" "$4}' | sort > sy.txt
wc -l < xc.txt; wc -l < sy.txt; comm -12 xc.txt sy.txt | wc -l      # 200  207  119
```

### A3. Entry sets (260, 177, 154, 106, 23)

```sh
ls /home/user/wt/t13/src/cryptanalysis | sort > crypto.txt
ls $D/suite-snap/suite/src/cryptanalysis | sort > suite.txt
wc -l < crypto.txt; wc -l < suite.txt                  # 260  177
comm -12 crypto.txt suite.txt | wc -l                  # 154 shared
comm -13 suite.txt crypto.txt | wc -l                  # 106 crypto-only
comm -23 suite.txt crypto.txt | wc -l                  # 23 suite-only
for f in $(comm -23 suite.txt crypto.txt); do if [ -d $D/suite-snap/suite/src/cryptanalysis/$f ]; then
  echo "$f/ $(find $D/suite-snap/suite/src/cryptanalysis/$f -type f | wc -l)f $(find $D/suite-snap/suite/src/cryptanalysis/$f -type f | xargs cat | wc -l)";
  else echo "$f $(wc -l < $D/suite-snap/suite/src/cryptanalysis/$f)"; fi; done   # per-entry lines of §3; sums to 39,327
```

### A4. The table (§2) and the counts in §0 and §2

`stack_divergence_table.sh`, run as `D=$D ./stack_divergence_table.sh > table.tsv` (about
30 s). Columns: entry, type, crypto lines, suite lines, crypto-only diff lines,
suite-only diff lines, class, basis, commits since sync (crypto), commits since sync
(suite).

```sh
#!/usr/bin/env bash
# Per-entry divergence table (TSV) for src/cryptanalysis (crypto) vs suite/src/cryptanalysis (cryptanalysis).
# Sync model: the last time the suite took crypto's code was suite commit SYNC (613820d, 2026-09-25,
# "suite: merge upstream crypto@54e977ab into the attack library"); XC is that crypto commit. Blobs at XC
# and SYNC differ for some files (the port's own edits), so the model is:
#   crypto changed = crypto HEAD blob != crypto@XC blob;  suite changed = suite HEAD blob != suite@SYNC blob.
set -u
D=${D:-.}
C=${C:-/home/user/wt/t13}; S=/home/user/cryptanalysis
SNAP=${SNAP:-$D/suite-snap}   # git -C $S archive 0ea8931 suite/src/cryptanalysis suite/cuda | tar -x -C $SNAP
SD=$SNAP/suite/src/cryptanalysis
CF=$D/crypto-full.git; SF=$D/cryptanalysis-full.git   # blobless full-history clones: crypto main (b7cafe04), cryptanalysis default branch (8fd4429)
CP=src/cryptanalysis; SP=suite/src/cryptanalysis
SYNC=613820d; XC=54e977ab   # SYNC: suite merge commit; XC: the crypto commit it merged
CT=b7cafe04   # crypto origin/main tip when the clone was taken
CH=$(git -C $C rev-parse HEAD); SH=0ea8931
blob() { git -C "$1" rev-parse -q --verify "$2:$3" 2>/dev/null; }
classify_file() {  # prints: class basis
  f=$1
  hc=$(blob $C $CH $CP/$f); hs=$(blob $S $SH $SP/$f)
  ax=$(blob $CF $XC $CP/$f); ai=$(blob $SF $SYNC $SP/$f)
  if [ -z "$hc" ]; then [ -n "$ai" ] && echo "crypto-ahead file-removed-in-crypto" || echo "suite-ahead suite-only-file"; return; fi
  if [ -z "$hs" ]; then [ -n "$ai" ] && echo "suite-ahead file-removed-in-suite" || echo "crypto-ahead file-added-in-crypto"; return; fi
  [ "$hc" = "$hs" ] && { echo "identical blob"; return; }
  if [ -z "$ax" ] || [ -z "$ai" ]; then echo "both-changed added-after-fork"; return; fi
  cc=0; sc=0; [ "$hc" != "$ax" ] && cc=1; [ "$hs" != "$ai" ] && sc=1
  # a suite-side change whose blob is an older crypto version is a port from crypto, not suite work
  if [ $sc = 1 ] && git -C $CF log --format=%H $CT -- $CP/$f | while read c; do blob $CF $c $CP/$f; done | grep -qx "$hs"; then echo "crypto-ahead suite-ported-older-crypto"; return; fi
  case "$cc$sc" in 10) echo "crypto-ahead blobs";; 01) echo "suite-ahead blobs";; 11) echo "both-changed blobs";; 00) echo "identical* import-edits-only";; esac
}
# suite commits touching a path from SYNC (exclusive) to the checked-out HEAD, in the full-history clone
suite_commits() { git -C $SF log --format=%H $SYNC..$SH -- "$1"; }
for e in $(comm -12 <(ls $SD | sort) <(ls $C/$CP | sort)); do
  if [ -d $C/$CP/$e ]; then
    type=dir; files=$( (cd $C/$CP; find $e -type f; cd $SD; find $e -type f) | sort -u)
    cl=$(cd $C/$CP; find $e -type f | xargs cat | wc -l); sl=$(cd $SD; find $e -type f | xargs cat | wc -l)
    nfc=$(cd $C/$CP; find $e -type f | wc -l); nfs=$(cd $SD; find $e -type f | wc -l)
    classes=$(for f in $files; do classify_file $f | cut -d' ' -f1; done | sort -u | tr '\n' ' ')
    c=0; s=0
    echo "$classes" | grep -q 'crypto-ahead' && c=1
    echo "$classes" | grep -q 'suite-ahead' && s=1
    echo "$classes" | grep -q 'both-changed' && { c=1; s=1; }
    if [ $c$s = 00 ]; then case "$classes" in "identical ") cls=identical;; *) cls="identical*";; esac
    else case "$c$s" in 10) cls=crypto-ahead;; 01) cls=suite-ahead;; 11) cls=both-changed;; esac; fi
    basis="dir($nfc/$nfs files)"
    nc=$(for f in $files; do git -C $CF log --format=%H $XC..$CT -- $CP/$f; done | sort -u | wc -l)
    ns=$(for f in $files; do suite_commits $SP/$f; done | sort -u | wc -l)
  else
    type=file; cl=$(wc -l < $C/$CP/$e); sl=$(wc -l < $SD/$e)
    set -- $(classify_file $e); cls=$1; basis=$2
    nc=$(git -C $CF log --format=%H $XC..$CT -- $CP/$e | wc -l)
    ns=$(suite_commits $SP/$e | wc -l)
  fi
  set -- $(git diff --no-index --numstat $C/$CP/$e $SD/$e | awk '{a+=$1;d+=$2} END{print a+0,d+0}'); so=$1; co=$2
  printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$e" $type $cl $sl $co $so "$cls" "$basis" $nc $ns
done
```

```sh
cut -f7 table.tsv | sort | uniq -c        # 52 identical, 29 identical*, 49 crypto-ahead, 21 both-changed, 3 suite-ahead
awk -F'\t' '{a+=$3;b+=$4;c+=$5;d+=$6} END{print a,b,c,d,c/d}' table.tsv   # 246737 199551 53761 6575 8.18
awk -F'\t' '{l[$7]+=$3; m[$7]+=$4} END{for(k in l) print k,l[k],m[k]}' table.tsv   # identical 37414 37414
awk -F'\t' '$7=="suite-ahead"{print $1,$5,$6}' table.tsv                # ecm.rs 1 1; invalid_curve_attack.rs 93 202; mov_attack.rs 121 96
awk -F'\t' '$7!="identical"' table.tsv | wc -l                          # 102 not byte-identical; 49+21 = 70 crypto moved, 21+3 = 24 suite moved
git diff --no-index --numstat /home/user/wt/t13/src/cryptanalysis/groebner_f4.rs $D/suite-snap/suite/src/cryptanalysis/groebner_f4.rs | wc -l   # 0: identical
```

### A5. Crypto-only groups (§4)

`crypto_only_groups.sh` (`D` as above); prints entries, files and lines per group. Output:
`G1 9 entries 48 files 31735 lines`, `G2 9 29 18510`, `G3 13 13 10663`, `G4 21 21 19843`,
`G5 15 15 23952`, `G6 18 44 38617`, `G7 16 22 16780`, `G8 4 4 5809`, `G9 1 1 18`; entries sum
to 106, lines to 165,927.

```sh
#!/usr/bin/env bash
# Groups the crypto-only entries of src/cryptanalysis and prints entries, files and lines per group.
set -u
C=${C:-/home/user/wt/t13}/src/cryptanalysis; S=${SNAP:-./suite-snap}/suite/src/cryptanalysis
group() { case "$1" in
 ecbench|ecbench/|ecbench_large_prime.rs|hwcost|ic_measurement.rs|challenge_ladder.rs|prepared_control_archive.rs|prepared_sat_control.rs|groebner_cache.rs|matmul_exponent.rs) echo G1;;
 curve_aliases.json|curve_id.rs|curve_traits|curve_construction|identity_certificate.rs|secg_standards.rs|secg_small_x.rs|nist_seed_provenance.rs|data) echo G2;;
 fp_backend.rs|fp_backend_ifma.rs|wide_gf2m.rs|binary_field_basis.rs|koblitz_wide.rs|wide_sixsum.rs|f6_wide_geometry.rs|gls_fp2.rs|ext_curve.rs|subfield_fp3.rs|biext_k2.rs|genus2_fp.rs|gpu_rootfind.rs) echo G3;;
 koblitz_greedy_base.rs|koblitz_greedy_sampled.rs|koblitz_large_prime.rs|koblitz_rotated_chain.rs|koblitz_sixsum_extract.rs|koblitz_strong_rho.rs|koblitz_sym_s4.rs|large_prime_filter.rs|orbit_pair_table.rs|native_signed_mitm.rs|cyclotomic_factor_base.rs|binary_small_x.rs|binary_semaev_chain_sat.rs|binary_torsion_walk.rs|pdp_split.rs|delta_method.rs|semaev_s3_cost.rs|line_oracle.rs|line_s4_oracle.rs|recursive_descent.rs|lopsided_thin_product.rs) echo G4;;
 prime_fast.rs|prime_small_x.rs|prime_field_smt.rs|smt_oracle.rs|f4_tower_generic.rs|sig_fp_tower.rs|gaudry_quartic_bin.rs|gaudry_quartic_d4.rs|glv_gaudry.rs|glv_invariant_base.rs|glv_invariant_experiments.rs|d4_weighted_s5.rs|quartic_folds.rs|fghr_full.rs|fghr_line.rs) echo G5;;
 isogeny_walk|isogeny_eval|jacobian_transfer|jv_cover.rs|jv_isogeny_walk.rs|jv_quartic.rs|jv_quintic.rs|jv_quintic_edwards.rs|jv_sieve.rs|joux_vitse.rs|joux_vitse_edwards.rs|ghs_class_coverage.rs|ghs_screen.rs|ghs_transport.rs|smith_kernel_coverage.rs|smith_prop24.rs|q_curve.rs|two_sylow_search.rs) echo G6;;
 ecdlp_nist|p192_*|p224_hypotheses.rs|p256_*|sect113r1_audit.rs) echo G7;;
 ecc2k130_*) echo G8;;
 *.rej.orig) echo G9;;
 *) echo UNASSIGNED;; esac; }
for e in $(comm -13 <(ls $S | sort) <(ls $C | sort)); do g=$(group $e); if [ -d $C/$e ]; then n=$(find $C/$e -type f | wc -l); l=$(find $C/$e -type f | xargs cat | wc -l); else n=1; l=$(wc -l < $C/$e); fi; echo "$g $e $n $l"; done |
 awk '{ent[$1]++; f[$1]+=$3; l[$1]+=$4; names[$1]=names[$1]" "$2; if($1=="UNASSIGNED")print "UNASSIGNED",$2} END{for(g in ent) print g, ent[g]" entries", f[g]" files", l[g]" lines"}' | sort
```

### A6. The 20 largest diffs and their notes (§2)

```sh
awk -F'\t' '{print $5+$6"\t"$1}' table.tsv | sort -rn | head -20
# notes: public items exclusive to each side
C=/home/user/wt/t13/src/cryptanalysis; S=$D/suite-snap/suite/src/cryptanalysis
items(){ grep -rhoE '^\s*pub(\([a-z]+\))? (const )?(unsafe )?(fn|struct|enum|trait|mod|type|const|static) +[A-Za-z0-9_]+' "$@" \
  | sed -E 's/^\s*pub(\([a-z]+\))? (const )?(unsafe )?//' | sort -u; }
f=koblitz_groebner.rs; comm -23 <(items $C/$f) <(items $S/$f)    # crypto-only items
                       comm -13 <(items $C/$f) <(items $S/$f)    # suite-only items
git diff --no-index $C/pq_f4_f2.rs $S/pq_f4_f2.rs | grep '^-' | grep -E '^-(//!|\s*pub |\s*fn )'   # headers/items for files with no pub-item difference (sat.rs, pq_f4_f2.rs, f4_fp_tower.rs)
```

### A7. Lineage and commit counts (§0.1)

```sh
git -C $D/cryptanalysis-full.git log -1 --format='%h %ad %s' --date=short c00af46   # 2026-09-21 Add the attack suite from the crypto repository as suite/
git -C $D/cryptanalysis-full.git log -1 --format='%h %ad %s' --date=short 613820d   # 2026-09-25 suite: merge upstream crypto@54e977ab ...
git -C $D/crypto-full.git log --format=%H 54e977ab..b7cafe04 -- src/cryptanalysis | wc -l                  # 1094
git -C $D/cryptanalysis-full.git log --format=%H 613820d..0ea8931 -- suite/src/cryptanalysis | wc -l       # 39
git -C $D/cryptanalysis-full.git log --format=%s 613820d..0ea8931 -- suite/src/cryptanalysis | grep -ci port   # 15
git -C $D/cryptanalysis-full.git log --format=%s 613820d..0ea8931 -- suite/src/cryptanalysis | grep -i port    # the 15 subjects
```

### A8. The suite's own numbers (§3)

All paths are in `/home/user/cryptanalysis` (`$R`); `sed -n a,bp` prints the cited lines.

```sh
R=/home/user/cryptanalysis
sed -n 1,10p $R/experiments/f4-gpu-20260925/RESULT.md           # 10.0-10.3x PDP stage, 2.1-12.2x ladder, 190-280x sweeps
sed -n 62,63p $R/experiments/f4-gpu-20260925/RESULT.md          # 10.29x [10.26,10.34]; 9.95x [9.43,10.25]; S and rho ratio null
sed -n 100,114p $R/experiments/f4-gpu-20260925/RESULT.md        # degree sweeps: 158.7 s sparse vs 6.11 s; 594.3 s to 3.1 s; 13.7 min
sed -n 14p $R/experiments/f4-gpu-runpod-20261006/RESULT.md      # inherited engine 4.4x faster than the device path
sed -n 48,49p $R/experiments/f4-gpu-runpod-20261006/RESULT.md   # 425.8 s to 5.70 s (74.7x); 25.5x
sed -n 76,77p $R/experiments/f4-gpu-runpod-20261006/RESULT.md   # dreg_sweep 18.8 s to 8.2 s
sed -n 100,110p $R/experiments/f4-gpu-runpod-20261006/RESULT.md # 6.64 / 8.47 / 5.57 / 1.25 s; 127,356 vs 114,584 decisions/s
grep -n "0.28 s with the" $R/experiments/f4-gpu-runpod-20261006/RESULT.md   # F4 oracle 0.28 s vs mq-fes 0.05 s at icv1-f2m23-tm5197-1f85e9e1
git -C $R show -s --format=%B 0ea8931 | grep -n "Ir 2,618\|0.17x\|0.91x\|constant-factor\|1.85"   # 36.2x, 61.3x, tensor 0.17-0.80x, symmetrised 0.91x, crossbred
git -C $D/cryptanalysis-full.git show -s --format=%B 8fd4429 | sed -n 1,20p   # #957: fes_chain cap 2^28; s5 winning regime
git -C $R show 0ea8931:suite/src/cryptanalysis/semaev_sat_s5.rs | sed -n 78,92p      # Ir and wall table
git -C $R show 0ea8931:suite/src/cryptanalysis/affine_factor_base.rs | sed -n 100,110p
git -C $R show 0ea8931:suite/src/cryptanalysis/walksat.rs | sed -n 56,69p
git -C $R show 0ea8931:suite/docs/ic/README.md | sed -n 793,801p                        # 682 vs 79 relations; 8.6x
git -C $R show 0ea8931:suite/README.md | sed -n 202,204p                                 # factoring scope
awk 'BEGIN{print 2618061549/72249811, 5.57/1.25, 127356/114584}'        # 36.24 4.456 1.111
```

### A9. Port sizes (§5.2)

```sh
S=$D/suite-snap/suite/src/cryptanalysis
wc -l $S/f4_gf2.rs; grep -n '^mod tests' $S/f4_gf2.rs                    # 1757; tests from line 1435
grep -c 'f4_gf2\|F4Kernel\|f4_kernel\|F4_F2_RREF' $S/koblitz_groebner.rs  # 37 hook sites; est. 200 lines of glue
grep -n 'super::f4_gpu' $S/f4_gf2.rs                                       # 568, 987, 998, 1010: three hooks
wc -l $S/semaev_decomp/linearised_m4.rs $S/semaev_decomp/linearised_m4/tests.rs $S/semaev_decomp/coset.rs   # 922 507 413
grep -c 'linearised_m4\|mod coset' $S/semaev_decomp.rs                    # 2 hooks
wc -l $D/suite-snap/suite/cuda/*                                           # 40 672 465 100 82
grep -n '^#\[cfg(test)\]' $S/f4_gpu.rs                                     # 1622: 1,621 non-test lines
awk 'BEGIN{print (364-289)+(1380-496)+(1572-1380)}'                        # 1151: echelon struct/emulator + CUDA driver + offload
cd /home/user/wt/t13
grep -lE 'solving_degree|first_fall_degree|macaulay_profile|matrix_f4_f2' examples/*.rs | wc -l      # 42
grep -rlE 'solving_degree|first_fall_degree|macaulay_profile|matrix_f4_f2' src --include=*.rs | wc -l  # 57
grep -n 'solving_profile_sparse(polys' src/cryptanalysis/koblitz_groebner.rs                          # 4709
sed -n 144p src/cryptanalysis/ic_framework/cm_orbit.rs                                                # discriminants -19 and -43 only
for s in quartic_general_with subspace_poly_for_basis 'struct Gf2\b' LvHorner GeneralTargetPowers 'struct SubspaceOracle'; do grep -c "$s" src/cryptanalysis/semaev_decomp.rs; done   # all >= 1
```

### A10. Open follow-up in both repositories

```sh
sed -n 186,190p /home/user/cryptanalysis/experiments/groebner-perf-20260924/README.md   # groebner_f4.rs is Buchberger despite its filename
```

### A11. System-of-record citations (§5.1)

```sh
sed -n 1203,1206p /home/user/wt/t13/AGENTS.md          # §12: ecbench as the native harness
sed -n 746,749p /home/user/wt/t13/AGENTS.md            # line 748 is the scoreboard "canonical file" rule, not ecbench
sed -n 797p /home/user/wt/t13/AGENTS.md                # scoreboard-to-ecbench mapping row
sed -n 125,126p /home/user/ml-cryptanalysis/docs/PLAN.md    # "the counted harness of record"
git -C /home/user/cryptanalysis show 0ea8931:suite/README.md | sed -n 5,9p   # "moved here and put under this repository's checks"
```

### A12. The #891 mechanism and the two lines of the suite repository

```sh
git clone --bare --filter=blob:none --single-branch --branch main \
    https://github.com/aburan28/cryptanalysis.git $D/cryptanalysis-main.git
git -C $D/cryptanalysis-main.git rev-list --count main                 # 957 (tip 3ce4c29)
git -C $D/cryptanalysis-main.git cat-file -t 0ea8931d                  # fails: #954 is not on main
git -C $D/cryptanalysis-main.git show main:tools/bootstrap_crypto_analysis_core.sh | grep -n 'rev='   # e4f200cc...
git -C $D/cryptanalysis-main.git show --stat --format=%s bb98c00 | head -12   # #891, 2026-10-16
git -C /home/user/cryptanalysis cat-file -t HEAD:tools/bootstrap_crypto_analysis_core.sh   # fails: not in the checkout
diff -q /home/user/wt/t13/src/cryptanalysis/avalanche.rs $D/suite-snap/suite/src/cryptanalysis/avalanche.rs   # identical
```

### A13. The working-tree change observed during the task

```sh
git -C /home/user/cryptanalysis status --short | wc -l                 # 708 at the time; HEAD still 0ea8931d
ls /home/user/cryptanalysis/suite/src/cryptanalysis | wc -l            # 161 at the time, against 177 in HEAD
```
