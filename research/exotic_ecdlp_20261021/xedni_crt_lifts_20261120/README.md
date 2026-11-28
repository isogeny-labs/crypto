# CRT coefficient choice yields 36 rational xedni relation curves

An explicit CRT lift of the strict auxiliary-prime survivors constructs **72 rational-curve relation records**, representing **36 distinct (j)-invariants**, from 1,512 frozen coefficient choices. The records occur on two ordered sign-mirror input pairs (source indices 425 and 467), with 36 choices for each. Every constructed curve reduces to `y²=x³−x−2` over `F₄₇`, has `#E(F₄₇)=43`, passes `#E(F₇)=4` and `#E(F₁₁)=6`, and has the required local point relation at both auxiliary primes. The 72 records also satisfy `P₁+2P₂=O` over `ℚ`, checked independently by exact rational doubling, Sage's elliptic-curve group law, and a Lean theorem for one explicit witness.

The smallest-bit-height witness under the frozen tie rule is

`E/ℚ: y² + 1081xy = x³ + 1034x² − 6816x − 3997`,

with `P=(14,7)` and `Q=(17,10)`. The tangent slope at `Q` is `1`, its intercept is `−7`, and the exact group law gives `2Q=(14,−15141)=−P`. Its five coefficient bit heights have maximum 13. This curve has good reduction and the stated orders at 47, 7 and 11 in the checked Sage replay. The [Lean module](formal/Witness.lean) constructs this exact rational Weierstrass curve and its points, proves the curve is elliptic, and proves `XedniCRTWitness.rational_relation : P + 2 • Q = 0` in Mathlib's affine point group.

| Frozen construction | Count |
| --- | ---: |
| Reduced-event pairs with relation-compatible residues at both strict auxiliary gates | 14 |
| Cartesian pairs of local residue witnesses | 56 |
| Centered CRT representatives with shifts in `{−1,0,1}³` | 1,512 |
| Rational relation records | 72 |
| Ordered source pairs represented in those records | 2 |
| Distinct rational-relation coefficient tuples | 72 |
| Distinct (j)-invariants among those tuples | 36 |

The [protocol](protocol.json) fixes every source hash, residue choice, CRT rule, centered representative, and 27-shift budget before construction. For each of the 14 pairs, exactly two relation-compatible coefficient residues at 7 and two at 11 pass the strict gates. Because their lifted x-coordinates differ at both primes, CRT on `a₁,a₂,a₃=47(k₁,k₂,k₃)` also fixes the reductions of rationally reconstructed `a₄,a₆`. Each CRT class is tested at the centered representative and 26 specified shifts by multiples of 77 in the three `k` coordinates. The [result](result.json) and [compressed per-curve rows](curve-rows.jsonl.gz) retain all 1,512 statuses, coefficient heights, and exact byte hashes.

The [independent audit](audit.py) re-enumerates local survivors using a different finite-field order and doubling algorithm, independently finds the centered CRT multipliers, and verifies every rational curve equation and group relation by direct rational duplication. It reproduces all 1,512 rows, the 72 relation records, and the compact witness. The [checked Sage replay](sage_replay.py) independently reconstructs all 1,512 curves, verifies their rational point sums and all three finite-field group orders and relations, and finds 36 distinct (j)-invariants among the 72 records. Both [audit](audit-result.json) and [Sage replay](sage-replay-result.json) pass, with empty stderr. The [Sage runtime receipt](sage-runtime-info.json) was recorded before the checked jobs.

## Lean proof map

| Result | Declaration | Exact scope | Status |
| --- | --- | --- | --- |
| Compact witness points lie on the displayed curve | `p_on`, `q_on` | `E/ℚ`, `P=(14,7)`, `Q=(17,10)` | Checked |
| Compact witness is nonsingular | `E.IsElliptic` instance | Nonzero discriminant in `ℚ` | Checked |
| Rational relation | `XedniCRTWitness.rational_relation` | `P + 2 • Q = 0` in Mathlib's affine elliptic-curve point group | Checked |
| Orders 43, 4, 6 and CRT residue enumeration | — | Exact indicated reductions and full finite candidate set | Independently replayed by Sage and Python; Lean formalization open |

The [build](formal/build.sh) pins Lean `v4.35.0-rc4` and Mathlib revision `021c46bdd042f67b538b9f9871d556a93713a9a9`, checks the shared dependency manifest byte-for-byte, compiles the actual theorem module, and retains [`#print axioms` output](formal/build-output.txt). The theorem and point-membership lemmas use only standard Lean foundations (`propext`, `Classical.choice`, `Quot.sound`); there are no admitted claims or custom axioms. `lean4checker --fresh` is unavailable on this host. Formalizing the finite group-order census and reduction map for the three concrete primes is the remaining bridge to a completely Lean-checked CRT certificate.

## Rank diagnostic and next experiment

The compact witness's proof-enabled PARI rank call reported lower 0, upper 3 and exited with an error. Simon two-descent with the known rational point completed with bounds **1 to 3**; increased PARI effort reported lower 1, upper 3 and exited with an error. All three [rank](rank-result.json), [Simon](rank-simon-result.json), and [increased-effort](rank-pari-effort-result.json) rows are retained under their separate frozen protocols. This selected-curve diagnostic leaves the `d<r` premise for `r=2` unresolved.

The construction starts from the 14 pairs already known to satisfy the reduced relation. The next algorithm experiment should freeze a target-independent policy over the full 1,722-pair law, charge local residue enumeration and CRT search, and record how often a rational relation is obtained without using that event to choose the input. A higher-field, higher-point policy is still needed to test the uniform rank and height premises of the general xedni obstruction.

Replay from this checkout:

```sh
python3 research/exotic_ecdlp_20261021/xedni_crt_lifts_20261120/run.py > research/exotic_ecdlp_20261021/xedni_crt_lifts_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_crt_lifts_20261120/run.stderr
python3 research/exotic_ecdlp_20261021/xedni_crt_lifts_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_crt_lifts_20261120/audit-result.json
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_crt_lifts_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_crt_lifts_20261120/sage_replay.py > research/exotic_ecdlp_20261021/xedni_crt_lifts_20261120/sage-replay-result.json 2> research/exotic_ecdlp_20261021/xedni_crt_lifts_20261120/sage-replay.stderr
sh research/exotic_ecdlp_20261021/xedni_crt_lifts_20261120/formal/build.sh > research/exotic_ecdlp_20261021/xedni_crt_lifts_20261120/formal/build-output.txt
```
