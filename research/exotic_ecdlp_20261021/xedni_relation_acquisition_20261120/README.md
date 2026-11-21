# Rational collision acquisition of the reduced relation

The first exact rational reduction collisions determine `R̄=11Q̄` after
**35** candidate evaluations at `p=1531` and `R̄=1238Q̄` after **1,505**
at `p=2287`. Five alternating same-public-point pairs per cell returned
verified scalars in both the collision arm and rho. Median local online
intervals were 2.054 ms versus 1.288 ms at `p=1531`, and 270.210 ms versus
0.744 ms at `p=2287`. An independent Fraction replay reconstructs every
candidate through each first collision, including its exact coordinate
peak, and replays all ten rho walks.

## Frozen input and measurements

The [protocol](protocol.json) and [runner](run_sage.py) were pushed in
`76487023c5d` before timing. On each reduction, the public point supplied
to **both** arms is `R̄`; the collision arm additionally receives the exact
rational point `R` as a supplied lift. The same rational curve, `Q`, `R`,
128-bit height-pairing score and coefficient radius were fixed by the
preceding [preregistered panel](../xedni_preregistered_prime_panel_20261120/README.md).
The rho arm receives the public point and generator. Repetition seeds vary;
arm order alternates. Each arm has a 60-second cap, and every run and status
is retained.

| p | q | First relation and coefficient difference | Collision evaluations | Peak coordinate bits through collision | Collision median online ms | Rho median online ms |
| ---: | ---: | --- | ---: | ---: | ---: | ---: |
| 1531 | 1553 | `(4,-1)`, `(-7,0)`; `(-11,1)` | 35 | 85 | 2.054 | 1.288 |
| 2287 | 2251 | `(1,10)`, `(0,-10)`; `(-1,-20)` | 1,505 | 3,589 | 270.210 | 0.744 |

The [raw pair rows](pair-rows.jsonl) contain exact nanosecond intervals,
first-relation labels and coefficients, scalar checks, rho walk updates,
degenerate collisions, restarts, and arm order. All 20 arms have `verified`
status. The first collision call at `p=1531` took 40.296 ms; the remaining
four took 1.872–2.102 ms. That first-use effect remains in the raw record
and is not folded away by a setup adjustment. The [result](result.json)
pins source and raw-row digests, host, and setup timing; the accepted
[Sage runtime receipt](sage-runtime-info.json) was saved before execution.

Online timing starts before the first target-dependent rational candidate
or rho walk and stops after the recovered scalar is independently verified
against `R̄`. Failed candidate attempts and rho restarts are included. Global
rational-curve and pairing setup took 55.919 ms; per-cell curve, public-point
and label setup took 28.256 and 10.566 ms, retained in the
[setup rows](setup-rows.jsonl). The exact rational target lift `R` is a
supplied input; its discovery or construction has no measured cost here.
The repeated-target host is ordinary Darwin ARM64 with no Tier 0/1
isolation receipt. These intervals are a component diagnostic under the
declared input law.

## Proof and independent replay

[CollisionRatio.lean](formal/CollisionRatio.lean) proves in `ZMod q` for
prime `q` that a coefficient difference `da*G+db*R=0` with `db≠0`
determines `R=(-da/db)G`. The theorem's `db` condition is checked on both
observed collisions; `Q̄` has order `q` in each finite group. The
elliptic-curve reduction and scalar-coordinate bridge are checked by the
Sage run and the [independent audit](audit.py). The audit rebuilds 35 and
1,505 exact rational points with Python `Fraction`, verifies each finite
reduction against a separate integer group law, matches both first
relations and coordinate peaks, and replays all ten rho walks. Its
[result](audit-result.json) is `PASS`.

The pinned [Lean build](formal/build.sh) succeeds. Its
[#print axioms output](formal/build-output.txt) lists only `propext`,
`Classical.choice`, and `Quot.sound`; `lean4checker --fresh` was unavailable
on this host.

These measurements charge the relation-acquisition stage used by the
subsequent duplicate-filtered target search. The next direct experiment is
to test the same acquisition idea when a rational lift of the supplied
public point is **not** provided, recording the lift-finding work under a
frozen input law and cap.

## Replay

From `/Volumes/SSD990-2/crypto-worktrees/exotic-ecdlp-recovery-20261119`:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_relation_acquisition_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_relation_acquisition_20261120/run_sage.py > research/exotic_ecdlp_20261021/xedni_relation_acquisition_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_relation_acquisition_20261120/run.stderr
python3 research/exotic_ecdlp_20261021/xedni_relation_acquisition_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_relation_acquisition_20261120/audit-result.json 2> research/exotic_ecdlp_20261021/xedni_relation_acquisition_20261120/audit.stderr
sh research/exotic_ecdlp_20261021/xedni_relation_acquisition_20261120/formal/build.sh
```
