# Source-point incidence and effective rank of the 40 witness records

**The 40 verified wide-period rational relations contribute two independent reduced relation rows after known point-negation folding.** Every positive record uses a distinct exact rational curve, and each such curve contains exactly its two designated points among the frozen centered lifts of all 42 nonidentity `E(F₄₇)` points. Thus these 40 records do not form multiple relation rows on one rational curve within this source-point set. The two sign-mirror input-pair families give row patterns `A`, `−A`, `B`, and `−B`, with counts 8, 8, 12, and 12.

| Finite incidence and rank result | Exact count |
| --- | ---: |
| Positive rational relation records | 40 |
| Distinct exact rational curve coefficient tuples | 40 |
| Centered source points tested per curve | 42 |
| Total rational point-incidence tests | 1,680 |
| Curves with exactly the two designated source points | 40 |
| Extra incident centered source points | 0 |
| Positive records per exact curve | 1 on every curve |
| Sign-folded reduced columns | 21 |
| Rank of all 40 signed rows over `F₄₃` | 2 |

The [protocol](protocol.json) and [runner](run.py) were pushed before measurement in `b8af686636c`. They read the sharded 13,440-row rational-check stream by its original decompressed SHA-256 digest, take all 40 positive records, evaluate each exact rational curve equation at every centered source lift, and construct signed relation rows over the 21 known negation orbits. The complete [curve rows](curve-rows.jsonl) retain the incidence points, designated pair, curve coefficients, and 21-column vector; the [result](result.json) hashes every row and reports per-curve and global ranks.

An independent [checked-Sage audit](sage_audit.py) re-evaluates all 1,680 incidence tests using Sage elliptic-curve equations, reconstructs the sign orbits through the actual `E(F₄₇)` group inverse, checks every signed row, and computes the matrix rank with Sage linear algebra. Its [result](sage-audit-result.json) passes, including the exact `A/−A/B/−B` row classification. The accepted [Sage runtime receipt](sage-runtime-info.json) and empty stderr are retained.

## Proof map and boundary

In the frozen 21-column order, row `A` has nonzero entries `1` and `2` at columns 2 and 19; row `B` has entries `1` and `2` at columns 5 and 8. Their supports are disjoint. The pinned [Lean theorem](formal/RelationRows.lean) `XedniWitnessRows.independent_coefficients` proves that `a • A + b • B = 0` over `ZMod 43` forces `a=b=0`. Its [build](formal/build-output.txt) reports only the standard `propext` and `Quot.sound` axioms under Lean `v4.35.0-rc4` and Mathlib revision `021c46bdd042f67b538b9f9871d556a93713a9a9`. The full 40-row classification and 42-point incidence are independently audited finite computations; their formalization is partial because those data and elliptic-curve incidence checks are not encoded as Lean objects. `lean4checker --fresh` is unavailable on this host.

This rank statement applies to the 40 recorded coefficient-2 relations and the fixed centered source-point set. A rational curve may contain other rational points outside that set. Relations from other coefficients, lift choices, or curve-selection policies could add rows. For the present records, 40 certificates reduce to two row directions, so the record count cannot be substituted for relation-matrix rank in an index-calculus cost case. The next construction experiment should seek multiple usable source points and independent relation rows on a fixed curve or a justified transport between curves, then charge that search before any target recovery comparison.

Replay:

```sh
python3 research/exotic_ecdlp_20261021/xedni_witness_incidence_20261120/run.py > research/exotic_ecdlp_20261021/xedni_witness_incidence_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_witness_incidence_20261120/run.stderr
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_witness_incidence_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_witness_incidence_20261120/sage_audit.py > research/exotic_ecdlp_20261021/xedni_witness_incidence_20261120/sage-audit-result.json 2> research/exotic_ecdlp_20261021/xedni_witness_incidence_20261120/sage-audit.stderr
sh research/exotic_ecdlp_20261021/xedni_witness_incidence_20261120/formal/build.sh > research/exotic_ecdlp_20261021/xedni_witness_incidence_20261120/formal/build-output.txt
```
