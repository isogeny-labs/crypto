# Full-law CRT selection finds eight rational relations without a reduced-event filter

A deterministic coefficient-selection policy applied to **all 1,722 ordered point pairs** on the fixed `E(F₄₇)` input law finds **eight rational relation records** on two ordered sign-mirror pairs. It never reads the reduced-relation flag when selecting local curves or CRT coefficients. Across the complete finite census it examines 2,249,856 local coefficient residues, constructs 32,256 centered rational CRT curves, and checks each rational point sum. The eight records have four distinct (j)-invariants; one is the [Lean-checked compact witness](../xedni_crt_lifts_20261120/formal/Witness.lean).

| Stage or status | Exact count |
| --- | ---: |
| Ordered input pairs | 1,722 |
| Same rational x-coordinate | 42 |
| Local x collision at 7 | 224 |
| Local x collision at 11 after the 7 check | 112 |
| Eligible pairs constructed | 1,344 |
| Local coefficient residues at 7 / 11 | 460,992 / 1,788,864 |
| Local order-gate witnesses at 7 / 11 | 4,032 / 10,752 |
| Centered rational CRT curves | 32,256 |
| Rational relation records | 8 |
| Ordered input pairs with a rational relation | 2 |

Every eligible pair has exactly three order-4 local witnesses at 7 and eight order-6 witnesses at 11, yielding 24 centered CRT curves per pair. The exact finite rates are `8/32,256 = 1/4,032` per constructed curve and `2/1,722 = 1/861` per ordered input pair under this policy. The two positive pairs are source indices 425 and 467, sign mirrors of one unordered pair. All eight satisfy the reduced relation at 47, the prescribed local curve orders and point relations at 7 and 11, and `P₁+2P₂=O` on their rational curves.

The [protocol](protocol.json) fixes the full point-pair law and never conditions construction on the reduced event. It skips pairs whose centered x-coordinates coincide over `ℚ`, 7, or 11, then enumerates all `a₁,a₂,a₃` residues at both auxiliary primes and retains every nonsingular curve with the specified order and two-row incidence rank. For each ordered pair of local witnesses, it solves `aᵢ=47kᵢ` by CRT, chooses the unique centered `kᵢ∈[−38,38]`, and reconstructs `a₄,a₆` from the two lifted-point equations. All source statuses and local work counts are in [compressed per-pair rows](pair-rows.jsonl.gz); [positive rows](positive-rows.jsonl.gz) retain each rational certificate. The [result](result.json) hashes the complete ordered candidate stream and exact row bytes.

The [independent audit](audit.py) re-enumerates the local coefficient spaces by a different four-free-coefficient method, checks smoothness by partial derivatives, counts points by quadratic characters, uses a separate CRT search over `[-38,38]`, and verifies rational relations by direct doubling. It matches all 1,722 rows, 32,256 candidate-stream entries, and eight positive records. A separate [checked Sage replay](sage_replay.py) verifies all eight positive rational point sums and the 43-, 4-, and 6-point reductions; it finds four distinct (j)-invariants. Both [audit](audit-result.json) and [Sage replay](sage-replay-result.json) pass, with empty stderr and the [Sage runtime receipt](sage-runtime-info.json) retained. The recorded 5.15-second Python process duration is an ordinary-host execution receipt; the operation counts are the comparison boundary for this selection study.

The compact positive curve `y²+1081xy=x³+1034x²−6816x−3997` with `P=(14,7)`, `Q=(17,10)` is among these eight records. The adjacent pinned Lean theorem proves its actual rational group relation `P+2•Q=0`. The full-law candidate enumeration and local order counts remain independently replayed arithmetic rather than Lean theorems.

The next experiment should address the 336 local-x-collision pairs with a higher-precision CRT rule and test additional representatives in each residue class under a frozen cost budget. To evaluate a discrete-log algorithm, a later workload must specify an unknown target, relation extraction and linear algebra, charge all stages, and compare with same-target rho under the repository's measurement contract.

Replay from this checkout:

```sh
python3 research/exotic_ecdlp_20261021/xedni_full_law_crt_20261120/run.py > research/exotic_ecdlp_20261021/xedni_full_law_crt_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_full_law_crt_20261120/run.stderr
python3 research/exotic_ecdlp_20261021/xedni_full_law_crt_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_full_law_crt_20261120/audit-result.json
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_full_law_crt_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_full_law_crt_20261120/sage_replay.py > research/exotic_ecdlp_20261021/xedni_full_law_crt_20261120/sage-replay-result.json 2> research/exotic_ecdlp_20261021/xedni_full_law_crt_20261120/sage-replay.stderr
```
