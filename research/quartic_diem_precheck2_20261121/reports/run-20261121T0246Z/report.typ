#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
= Pre-check 2, lead \#1: Diem 2LP line IC on prime-order plane quartics (fixed instrument)

Run report, `research/quartic_diem_precheck2_20261121`. Every number is [pcc:unverified]: one session on a shared host, with no replay certificate and no VALIDATION.json.

*Run.* The binary is `genus_ladder.qdiem2` (sha256 57e45b7c…860f). It was built from base `339dfb86af` plus the flag-gated changes in `src/cryptanalysis/quartic_ladder.rs` and `src/bin/genus_ladder.rs` (hashes in `source.sha256`). The protocol was frozen before data (`frozen.sha256`). The main session ran on 2 threads at 2026-11-21T02:46Z (156 s wall). Two processes at p = 65537 ran on 1 thread each (1551 s and 623 s). There were four arms on each (curve, seed):
- OLD: the pre-check-1 instrument;
- ALG: algebraic descent;
- KICK: ALG with a T step after every row;
- NEW (primary): ALG with descent last, linked rows and the kick.

The grid had 7 primes from 1031 to 65537, 4 prime-order curves per prime and 2 seeds, for 56 runs per arm. All 224 runs were certified.

*Checks.*
- C0: the OLD arm reproduces pre-check 1 exactly (48 of 48 pairs).
- C1: ALG has the same relations as OLD (56 of 56) with 0 fallbacks.

*Verdict (frozen K1/K2 on NEW).* The `la_mul` exponent is 0.971 [0.919, 1.024], so K1 does not fire. The FPM mid exponent is 0.897 [0.858, 0.934], so K2 does not fire. The decision is *PROCEED-TO-LADDER*, which is not a success claim. OLD on the same data: FPM mid 1.329 [1.236, 1.419], which would stop again.

*Paired NEW/OLD (pooled).*
#table(columns: 6, [metric], [FPM mid], [la_mul], [rel_mul], [GAE w=1], [attempts],
  [ratio], [0.391 [0.349, 0.437]], [0.560 [0.486, 0.644]], [0.221 [0.201, 0.243]], [0.239 [0.217, 0.263]], [0.552 [0.487, 0.627]])
By prime, FPM mid falls from 1.002 at p = 1031 to 0.144 [0.107, 0.215] at p = 65537.

#figure(image("../../fig_scaling.svg", width: 100%), caption: [Left: per-curve FPM mid (OLD, NEW) and `la_mul` (NEW) against p, with exponents and boot95. Right: paired NEW/OLD ratio per prime with cluster-bootstrap 95% intervals. Counted ops only; 56 runs per arm.])

*Limits.* The `O(p log p)` point enumeration is unchanged and makes up 78% of NEW's `rel_mul` at 65537. Residual retries remain in 11 of 56 runs; they are charged. The fix was developed on non-grid curves (4093, 8191). No rho was run and no validation gate applies. Full text: `RESULTS.md`; raw data: `raw/`, hashed in `data.sha256`.
