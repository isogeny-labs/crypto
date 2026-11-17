# Rational signed relations: algebra and anchor-rank results

Classification: implementation and finite algebra/rank evidence for an
established rational interpolation construction. No elapsed-time comparison,
asymptotic improvement or mathematical novelty is claimed. Producer evidence;
the independent algorithms below share the same field/Jacobian implementation.

Protocol commits: 10ff4d7727 (algebra), c6903c64ea (rank screen), ced6386c0e
(independent rank verification amendment). The latter was frozen before
implementing the second rank calculation. Historical raw outputs are retained.

## What passed

The subset formula and Walsh evaluation agree on all9408signed curve cases.
An independent homogeneous Gaussian calculation agrees projectively whenever
the interpolation kernel is one-dimensional. Full norm division and degree
bounds pass. All1108primitive split residuals give the zero Jacobian place
sum, including branch-root and repeated-root handling. There are694rejected
nonprimitive pairs and81lower-degree residuals in the curve panel. Separately,
2401exhaustive arbitrary F7 ordinate assignments, zero-pair/higher-nullity,
branch-anchor and duplicate-abscissa controls pass. RATIONAL-GATE.txt contains
the five-test output. The passing duplicate-input panic is intentional.

The36arm rank screen visits145268sign classes and verifies5360unique
admitted principal relations. There are23rank attainments,12family exhaustions
and one tuple-cap result. No arm is replaced. An independent full Gaussian
rank calculation verifies every forest rank using

    anchor-intersection dimension = rank(R) - rank(R_off).

RATIONAL-RANK-VERIFIED.jsonl agrees with every original field in
RATIONAL-RANK.jsonl; it adds full_rank/off_rank only. All36equalities pass.
The5algebra tests are included in the existing heldout Cargo example test
target; all17tests in that target pass (RATIONAL-CI-TESTS.txt).

## What changed in the cost case

These are exact deterministic counts on one selected curve per genus, not
held-out population estimates or timing ratios. d0 is the original polynomial
family with M=g+1 anchors; d>=1 uses M=g+2d rational interpolation anchors.
The target rank is H-1. Constructed outputs include unused batch tails.

| Genus, prime | Family | H | Visited | Constructed | Rank | Terminal status |
|---|---|---:|---:|---:|---:|---|
| 2,1009 | d0 | 8 | 224 | 224 | 0/7 | exhausted |
| 2,1009 | d2 | 8 | 705 | 736 | 7/7 | rank reached |
| 2,1009 | d0 | 12 | 714 | 716 | 11/11 | rank reached |
| 3,503 | d0 | 14 | 8008 | 8008 | 0/13 | exhausted |
| 3,503 | d2 | 14 | 10611 | 10624 | 13/13 | rank reached |
| 3,503 | d0 | 24 | 6049 | 6056 | 23/23 | rank reached |
| 4,127 | d0 | 16 | 16384 | 16384 | 13/15 | tuple cap |
| 4,127 | d2 | 16 | 6924 | 7040 | 15/15 | rank reached |
| 4,127 | d0 | 28 | 2372 | 2384 | 27/27 | rank reached |

All four families exhaust at rank0 at g3H9 and g4H10. The higher-pole
construction enlarges the available function family and permits a smaller
anchor space in these instances. The increase in generation work at g3/g4
must be charged. This does not justify a10x claim or a full timing campaign
without a complete cost model. Do not multiply these count ratios by earlier
held-out timing ratios: they use different curves and workloads.

## Structural constraint on the next mechanism

On an odd-degree hyperelliptic model, for n>2g-2, Riemann-Roch gives
dim L(n*infinity)=n-g+1. To force all but two of a function's n zeros into
specified distinct anchor places imposes n-2 homogeneous evaluation equations.
The matrix has n-g+1 columns. A nonzero kernel therefore requires at least
g-2 fewer independent equations than the number of rows when g>=3.
Equivalently, its rank must fall below the full-column-rank value by at least
one. This is why simply increasing the pole degree does not remove the
genus-dependent constraint. It also does not prove the evaluations are
generic or predict a probability: their algebraic structure matters.

The current family instead imposes n-g anchors, leaving a residual of degree
g automatically. For squarefree residuals of full degree g, its <=2off-anchor
filter at g3/g4 still needs one/two residual roots in the anchor set,
respectively. Repeated roots, coefficient cancellation and degree drops are
exceptions retained by the screen. A successor must exploit structure in that
condition or alter recombination, rather than infer yield from sign capacity.
For g2 the residual is already degree2, so the remaining bottlenecks are cold
construction, normalization, target handling and recombination cost.

## Reproduction and next gate

Use native Rust plus jq; no Python result generation. With a suitable Rust
toolchain on PATH and an explicit CARGO_TARGET_DIR:

    sh research/hec_anchor_relations_20261114/rational_gate.sh /tmp/hec-rational-replay
    cargo test --release --manifest-path crates/hec-native-ic/Cargo.toml --example heldout

The replay creates source/library/binary SHA256 receipts and complete stdout
in the chosen output directory. Compare rank.jsonl byte-for-byte with
RATIONAL-RANK-VERIFIED.jsonl. The checked source uses Poly CAP24; d<=3,g<=4
keeps the norm below that cap. It is a bounded research implementation.

Next discriminating work: charge subset construction, Walsh transforms,
normalization, orientations, rank accumulation, all cold setup and target
handling on an identical input. The present evidence supports the algebra and
finite smaller-anchor rank premise, while leaving its performance unmeasured.
Primary interpolation source and KB receipts are in RATIONAL-PROTOCOL.md.
Nine evaluated identity.certificate/v1 records are in RATIONAL-IDENTITIES.json,
with IDs listed in RATIONAL-IDENTITY-IDS.tsv. For example, the g2,d2 cleared
nodewise identity is IDC1h0a44102490e13efe. The native repository issuer checks
32derived points and16additional producer-selected-seed points per statement;
proof pointers are null. This is finite-field evaluation evidence, not a formal
proof or independent authorship. The polynomial statement treats weights as
arbitrary and substitutes yi=wi*A'(xi); signs are absorbed into the weights.
The Riemann-Roch convention was checked in the official Magma documentation:
https://magma.maths.usyd.edu.au/magma/handbook/text/1473 . The dimension/rank
argument above is a derivation from that convention, not a probability bound.
