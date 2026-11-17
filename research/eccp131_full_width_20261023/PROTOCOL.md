# ECCp-131 full-width auxiliary isogeny-tower protocol

Frozen before the recovery run on 2026-10-23. This is a bounded construction and relation-support study on the exact published field. The natural-relation, independent-rank, target-descent, and scalar criteria remain open until separately verified.

* Prime `p=0x48e1d43f293469e33194c43186b3abc0b`; published curve, generator, subgroup order, and target from `src/ecc/curve.rs` and `src/bin/ic/params.rs`.
* Candidate auxiliary curves `E'_b: y²=x³+x+b`, `b=1..512`, skipping singular curves. PARI/GP computes each group order. For each order divisible by 256, scan `x=0..255` for a point `P` with `Q=(#E'/256)P` of exact order 256. Then scan `x=256..511` for `R` with `256R≠O`. Stop at the first accepted curve, otherwise candidate 512.
* Save every computed order and all 256 abscissae `x(R+kQ)`. Independently replay point order, coset, all eight rational Vélu 2-isogeny maps, their codomains, and the single terminal x-image using Python integer EC arithmetic. Lift the abscissae on the published curve.
* Seed `random.Random(131)`: 256 setup points `aG` and 256 target points `aG+Q_pub`, tested for signed three-point decompositions using a complete signed pair table; eight planted identities are a separate correctness control. Natural rows and rank exclude planted identities.
* GP wall limit 120 s; replay/screen wall limit 60 s; sampled process RSS limit 2048 MiB for both. Preserve stdout, stderr, run receipts, failures and source. A watchdog stop or invalid certificate closes only this attempt.

Success for the construction is a replayed order-256 point and eight valid maps. The first natural-relation milestone requires an unplanted row independently replayed from published-curve points and coefficients. Rank and target descent require enough independent rows, a determined target equation, and a direct scalar check. All setup, misses, verification, memory, and wall cost remain charged.
