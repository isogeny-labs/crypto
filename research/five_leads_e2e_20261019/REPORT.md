# Five ECDLP follow-ups: executed evidence and remaining work

This is a follow-up to the five leads in
[the lead register](../new_leads_20261019/README.md). Each result below names
the operation that was actually run. The requested end-to-end criterion
remains open for four leads; the two-choice genus-2 collector has a separate
full-cohort execution and replay below. All tested curves, points and scalars
are public synthetic or published fixtures.

## Cubic odd-torsion transfer

For [Tian's published cubic example](https://arxiv.org/html/2012.07173),
the native [kernel checker](native/tian_cubic_kernel.rs) constructs
\(\mathbf F_{523^3}\) and its quadratic tower, checks irreducibility, verifies
all nine listed point components have order three on their conjugate
elliptic curves, and enumerates all 27 kernel elements. The three generators
are stable under Frobenius with rows \((0,1,0),(0,0,1),(2,0,0)\). Product
Weil pairings evaluated with two auxiliary choices equal one for every
generator pair; the subgroup is maximal isotropic. A point of prime order
143,069,629 and the Hasse interval certify the source group order, so the
prime source subgroup
has trivial intersection with the 3-kernel. The
[machine receipt](correctness/tian-cubic-kernel-v1.json) includes the
witness, component pairings and explicit false flags for quotient and
transfer execution.

The [quartic checker](../../src/bin/tian_quartic_smoothness.rs) derives a unit
gradient ideal in each affine chart for the copied published \(\mathbf F_{523}\)
quartic; a deliberately singular quartic fails the same check. This is
geometric smoothness of the written equation, not proof that it is the
quotient of the checked kernel. No source point was mapped, and no DLP was
solved through this transfer. The executable quotient, its actual kernel,
rational descent and complete-cost solver remain
[an open problem](../../problems/odd-isogeny-quartic-transfer-execution.md).

## Genus-2 target-useful relations

The [frozen protocol](protocols/genus2-choice2-v1.md) adds one seeded
two-choice collector method to the original exact-order cohort: 21 curves,
60 targets each, sizes 16 through 40. Each continuation evaluates and charges
both prospective jumps, inserts both valid relations, and then follows the
candidate favored by a component/cycle heuristic. The signed elimination or
a charged group identity is still required to recover the target scalar.
This is one heuristic candidate, not a proof of optimal rank targeting.

The seven generated [specifications](specs/genus2-choice2-v1/) retain the
original curve/target identities and include the incumbent v2 collector,
source plain/negation rho and destination Jacobian rho. Analysis and replay
receipts cover all 6,300 arm records and 1,260 paired one-target workloads.
Every arm recovered and source-verified its scalar, and all 6,300 independent
replays reproduced the original records with zero audit problems
([audit summary](results/replay-audit-summary.json)). The
[price grid](results/genus2-choice2-price-grid.csv) retains all 12 arithmetic
prices and three native-operation charges, with per-curve as well as
per-size ratios. At the representative
\(M+0.5S+10I\) price and 0.1 \(M\) per declared native logical operation:

| Source order bits | Two-choice / incumbent | Two-choice / source negation rho |
| ---: | ---: | ---: |
| 16 | 0.943697 | 3.812897 |
| 20 | 0.972590 | 3.824903 |
| 24 | 0.981876 | 4.229300 |
| 28 | 0.988278 | 4.373698 |
| 32 | 1.018553 | 4.366459 |
| 36 | 1.027152 | 4.694925 |
| 40 | 0.993748 | 4.444705 |
| **All 1,260** | **1.000857** | **4.483247** |

The ratios divide sums of complete, cold, one-target operation costs for
paired targets. The cohort's total is 4,881,369,517 source-addition
equivalents for two-choice, 4,877,191,974 for incumbent and 1,088,802,370
for source negation rho at that price. Across the 21 individual curves, the
two-choice/incumbent ratio spans 0.933237 to 1.102391. Without any native
logical-operation charge, the two-choice/incumbent ratio is 0.990528 at the same arithmetic
price; over the entire frozen price/native grid it ranges from 0.989481 to
1.006720. The candidate processed 221,270,948 divisors against 223,343,235
for incumbent. It solved at the first graph cycle on 23 targets, against
16 for incumbent; 1,237 and 1,244 respectively required later work.
The [cohort summary](results/genus2-choice2-summary.json) includes all
verification flags, session digests, cycle counts and logical memory peaks.
These mixed, price-sensitive results do not establish a robust full-cohort
improvement or a rho crossover. Operation counts are L0 evidence; they do
not qualify a wall-speed claim.

The next collector test is an explicit target-coefficient rank score from
the current signed linear system before branch selection. It must charge
both provisional eliminations and every rejected relation, preserve the
same target identities and caps, and be compared with this two-choice arm
as well as incumbent and source rho. The component score tested here is
only a proxy for that algebraic dependency.

## Projective genus-2 arithmetic

The [model gate](native/genus2_model_gate.rs) applies
\(x=L u-a_4/(5L),\;y=L^3 v\) to all 42 recorded Scholten quintics
from the field-size and exact-order fixtures. Each transformed equation
is monic and has no \(u^4\) term, the model required by the published
[complete projective group law](https://link.springer.com/article/10.1007/s13389-024-00351-7).
The gate validates 100 transferred divisor samples and 100 generator-addition
commutation checks per model: 4,200 of each.
The [receipt](correctness/genus2-model-gate-v1.jsonl) records coefficients
and samples. This establishes applicability of the equation form only.
Projective addition, inversion/normalization charging, equally optimized
source rho, and the requested 1,260-target replay are
[still missing](../../problems/genus2-projective-transfer-backend.md).

## Odd-kernel exits from Smith-negative inputs

The published \(\mathbf F_{257}\) genus-3 positive control from
[Tian's odd-isogeny construction](https://arxiv.org/html/2007.03172)
was partially replayed: a rational divisor \(T_1\) validates and
\(3T_1=0\) in the repo's hyperelliptic arithmetic
([receipt](correctness/tian-genus3-f257-t1-result.json)).
The copied rational quartic passes the gradient-ideal smoothness check.
Its extension-field generators, quotient and maps have not been checked.

A [20-input holdout](fixtures/smith-negative-holdout-v1.json) was frozen
before odd-kernel testing from the prior \(p=2^{31}-1\) Smith census:
ten with no tractable two-torsion kernel and ten with a trigonal candidate
but no isotropic two-kernel. The
[materialization replay](correctness/smith-negative-materialized-v1.jsonl)
regenerates every degree-eight polynomial and checks the archived verdicts.
No odd torsion or odd-kernel exit was tested on these inputs. The held-out
coverage question remains open with the quotient/map work above.

## Mixed-fiber PKM systems

The [small oracle protocol](protocols/mixed-fiber-oracle-v1.md) freezes three
prime fields, three curves per field and 20 targets per curve. For each
target, the native pair-sum oracle enumerates the complete three-summand
solution set for homogeneous, mixed and matched random fibers, then compares
it with an independent exhaustive triple enumeration. All 540 oracle records
agree ([receipt](correctness/mixed-fiber-census-v1.jsonl)).
The [cost summary](results/mixed-fiber-summary.json) reports cold pair-table
work, including fiber selection. At the declared
\(M+0.75S+10I\) price, mixed/homogeneous cost is 1.0191, 1.0052 and
1.0108 for \(p=97,193,257\), respectively; mixed/random cost is
0.8650, 0.8876 and 0.8952. These are **pair-table oracle ratios**.
They are not costs for heterogeneous polynomial solving or a DLP.
The complete polynomial all-roots/F4 comparison and source-to-answer run
remain [open](../../problems/mixed-fiber-pkm-full-solver.md).

## Reproduction and qualification

The exact source and protocol are under this directory. Native fixture tests
pass with:

```sh
cargo test --release --offline --locked --manifest-path research/five_leads_e2e_20261019/native/Cargo.toml
cargo test --release --offline --locked --lib
```

The first command passed 10 tests; the second passed 4,416 tests with 118
ignored and no failures. The cohort's ecbench audit and independent replay
receipts are under `replay/`; full raw sessions are preserved in an archive.
No L2/A/A isolated timing or fastest external implementation comparison was
run. This report therefore claims no hardware-speed record.
