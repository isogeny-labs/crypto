# Half-GCD and rational-tower continuation

Deliverables authorized by the user's “Do it”: keep PR4410 gated, add opt-in recursive polynomial half-GCD; implement general homogeneous rational-map tower membership/evaluation with certified full finite fibers and charged construction; audit the proposed last-fall-degree bound; compare the new implementation to the unchanged references under a new prospective protocol.

Acceptance: literal matrix identity plus degree reduction and independent classical gcd tests (odd/unbalanced/zero/sparse/common-factor cases, word+BigUint); full curve relation-set equality against scalar lookup for rational towers, including identity, diagonals and 2-torsion; reject poles, noncoprime maps, noncanonical data, repeated/partial fibers and field mismatch. Preserve budgets and historical default. Audit every algebra-to-last-fall inference explicitly, preserve earlier pinned arguments with sibling corrections if needed. Producer checks cannot supply independent signed standing.

Sources: FLINTv3.3.1 src/nmod_poly/hgcd.c and src/gr_poly/hgcd.c (forward versus inverse matrix conventions), repository src/cryptanalysis/isogeny_eval/poly.rs, ic_framework/prime_tower.rs and continuation-20261116/SPLIT_TOWER_BOUND.md. KB MCP absent; wrapper status and two searches exit127. Guessed gcd_hgcd.c404, no absence claim. Conductor absent127. No novelty assertion.

Arithmetic design: forward reduction matrices act on column(a,b); Euclidean step [[0,1],[1,-q]]. Recursive high-part half reduction, a full correction division and second high-part half reduction. Threshold64; Karatsuba multiplication and Newton only for long quotients. No unsupported quasi-linear GCD claim (Karatsuba M(n) and counted budgets apply).

Tower design: each N/D has degN=q>=2, degD<=q-1, gcd1; total degree<=16384. Construction checks each finite backward fiber has exactlyq distinct field roots, charges every step and retains leaves for separately charged factor-base lifting. Query homogeneous evaluation never divides by W, checks terminal membership/poles, handles diagonal and identity branches separately. Auxiliary isogeny search is not included unless implemented and priced explicitly.

Measurements: none yet. Development tests are correctness checks and do not select benchmark claims. Before comparisons: new frozen experiment.protocol/v1 and native analyzer, precise source/binary hash, calibration and independent populations/controls. Local isolated/Git-only, no cloud/S3 actions. Original completed protocols/data untouched; no old benchmark rerun.
