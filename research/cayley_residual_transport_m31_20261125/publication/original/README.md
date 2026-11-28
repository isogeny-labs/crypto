# M31 residual chord labels: exact transport and signed extraction

Completed local development continuation, November 25, 2026, Los Angeles.

## Result

Implemented and qualified a Rust component that transports two trace-zero
chord-function labels, checks their compatibility before square roots, and
recovers every signed rational residual pair for a supplied label. A public
four-point prefix constructor supplies the exact principal-divisor factors.
The candidate was checked against separately authored direct Fp² arithmetic
and the retained symmetric-Semaev reference on two ordinary M31 fixtures.

Both runs passed and produced byte-identical case, intersection, prefix,
seed and summary files. Source and executable manifests were unchanged
before and after each invocation. Full results and identities are in
`evidence/receipt.json`; the pre-execution mathematical review is preserved
as `MATHEMATICS.md`.

| New native development check | Result |
|---|---:|
| Exact transport and complete residual-fiber comparisons | 12,288 / 12,288 |
| Random-slope cases | 8,192 |
| Explicit controlled-slope cases | 4,096 |
| Withheld distinct-pair or doubling controls recovered | 1,024 / 1,024 |
| Incompatible label controls rejected before square roots | 12,288 / 12,288 |
| Exact supplied-set intersection queries | 1,536 / 1,536 |
| Constructed four-point prefix/divisor cases | 256 / 256 |
| Invalid input controls rejected | 52 / 52 |
| Identity/conjugate matrix sanity checks | 16 / 16 |
| Distinct deterministic child streams | 40 |
| Replay and immutable-call manifests | all equal |

The 8,192 random-slope cases had 4,096 empty rational fibers and 8,192
ordered pair occurrences. The controlled stratum had 853 empty fibers and
5,974 ordered pair occurrences. These are component outputs, with repeated
points permitted across cases; they are not independent factor-base relations.
Every full-prefix test is explicitly a constructed positive. Supplied-set
tests use deliberately overlapping sets to exercise the acceptance path.

## The mathematical finding

Fix the residual public target K=(w,v), v nonzero, and two certified
nonrational trace-zero points S_j=(s_j,i*delta_j), where i²=-1 and
delta_j²=-(s_j³+a*s_j+b). Put h_j=s_j-w. These h_j are nonzero because
a rational curve point cannot have the same abscissa as such a support.

The chord through -K has slope m and equation

    ell_m(x,y) = y - m(x-w) + v.

Its normalized evaluation label is

    t_j(m) = ell_m(S_j)/ell_m(-S_j)
           = C(v-m*h_j : delta_j),
    C(X:Z) = (X+iZ)/(X-iZ).

Include the vertical chord with m=infinity and C(1:0)=1. Each label is a
bijection from the p+1 projective slopes to the p+1 norm-one torus values.
If a fixed public prefix supplies eta_j, the raw label is z_j=eta_j*t_j.

Eliminating m gives the exact matrix

    N = [[h2*delta1, (h1-h2)*v],
         [0,          h1*delta2]],
    det(N) = h1*h2*delta1*delta2 != 0.

For eta=C(E:F), torus multiplication is represented by

    M_eta = [[E,-F],[F,E]].

The raw transport is T=M_eta2*N*adj(M_eta1). Its determinant is nonzero,
so the complete compatible label pairs are precisely the graph of T.
In particular, the vertical chord maps raw eta1 to raw eta2; raw identity
does not generally mean vertical slope.

For p=2^31-1, exactly 2^31 raw-label pairs are compatible among 2^62
possible pairs. Conditional on this fixed prefix, independent uniform
labels pass compatibility with probability 2^-31. Choosing the second
label adaptively to force compatibility simply evaluates T on the first
label. This proves that the second label supplies no independent selector
on this same residual pencil. It does not forbid labels controlling
different unresolved branches, or an index shared across changing prefixes.

This is a directly derived algebraic identity with native correctness
evidence and separate review. No Lean/kernel certificate is supplied.
`lean` and `lake` were absent from PATH in this session. The original unseen
one-label note was not recovered; this package proves its own explicit
definitions and does not assume that note's missing details.

## A genuine four-summand divisor label

For P1+P2+P3+P4=R, set A=P1+P2 and K=R-A=P3+P4. The ordinary addition
function f_UV has divisor [U]+[V]-[U+V]-[O]. Therefore

    F = f_P1P2 * f_P3P4 * f_AK
    div(F) = [P1]+[P2]+[P3]+[P4]-[R]-3[O].

At each support, vertical factors cancel between S_j and -S_j, giving

    F(S_j)/F(-S_j) = eta_j * t_j(m),
    eta_j = (f_P1P2(S_j)/f_P1P2(-S_j))
            * (f_AK(S_j)/f_AK(-S_j)).

`Frame::from_prefix` computes these eta values from E,R,P1,P2 and the
certified anchors. No target scalar or residual witness is accepted by
the interface. Identity prefixes, opposite prefix points and tangencies
have their ordinary divisor definitions. Prefix line ratios use projective
line coefficients to avoid dividing for the line slope.

This is explicitly a chord-function/principal-divisor label. It is a
different definition from the earlier product of x-coordinate Cayley
values. The old extractor is used only where the mathematical conversion
is explicit: its independent symmetric reference finds a complete
x-Cayley superset, and direct chord evaluation selects the exact fiber.

## Implemented API and behavior

`transport.rs` provides:

- `Frame::new`: certify curve, affine residual, supports and supplied factors;
  compute immutable cached projective maps.
- `Frame::from_prefix`: construct the residual and actual divisor factors
  from public point coordinates.
- `label_from_slope` and `slope_from_label`: exact forward/inverse maps,
  with explicit projective infinity.
- `Matrix::compatible`: compare projective images by cross multiplication,
  with no field inversion or square-root call.
- `extract_compatible`: return `None` for incompatible labels, or
  `Some(pairs)` for a compatible fiber. `Some(empty)` means the remaining
  two points are not rational over Fp; these outcomes remain distinct.
- `LabelSet` and `intersect`: enumerate precisely the compatible pairs
  from two supplied sets, scanning the smaller deduplicated set.

For a finite slope the remaining abscissae solve

    X² - (m²-w)X + (w*m²+w²+a+2*v*m) = 0.

The ordinates are y=m*(x-w)-v, so no ordinate square roots are needed.
A finite compatible query requires one coordinate square-root call;
incompatible labels are rejected first. A repeated root is emitted once.
Infinite slope yields both ordered pairs (O,K) and (K,O).

Frames expose certified state only through copy getters. Invalid field
residues, singular curves, off-curve points, zero-ordinate residuals,
identity residuals, invalid supports and invalid projective data are
rejected. A caller cannot mutate cached-map invariants after certification.

## Exact cost model and remaining dependency

Write a=|A1|, b=|A2|, m=min(a,b), and k for the number of compatible
output labels. The actual implementation uses deterministic `BTreeSet`:

| Stage | Accounted complexity |
|---|---|
| Build and deduplicate supplied sets | O(a log a + b log b) comparisons, O(a+b) storage |
| Scan/query | m projective maps and O(m log max(a,b)) membership comparisons |
| Normalize mapped label | At most one field inversion plus field arithmetic per visited entry |
| Sort matched outputs | O(k log k) comparisons, O(k) output storage |
| Extract finite residual fiber | One coordinate square-root call per compatible finite label |
| Prefix setup | Curve/support certification, public group additions, line evaluations and torus products; charge every new prefix |

The development panel recorded 8,192 visited set entries and 125,440
reference Cartesian checks. These are structural test counts for designed
sets, not runtime or complete-cost measurements. They do not price set
construction, inversion, output sorting, prefix discovery, membership in
the elliptic factor base, duplicate removal or independent relation rank.
No speed ratio is inferred from them.

The remaining mathematical target is an index that shares work across the
changing transforms T induced by many prefixes, or a construction in which
labels constrain genuinely different unresolved branches. A per-prefix
linear scan is a useful exact baseline but does not meet a sublinear
discovery objective. Any successor must specify how useful labels and
prefixes are obtained, then charge their generation and reject costs.

## Reproduction and evidence

Run on Linux with a compatible Rust compiler, `taskset`, `timeout` and
`sha256sum` available:

```bash
RUSTC=/absolute/path/to/rustc bash replay.sh replay-output
```

The script refuses to overwrite an existing output directory. It creates
an immutable source copy, binds compilation and execution to one allowed
CPU, records source/compiler/executable identities, executes the same
binary twice, and compares every output byte. Resource caps are 120 seconds
and 1 GiB virtual memory for compilation, then 60 seconds and 512 MiB per
execution. There are no Cargo dependencies. The retained successful run
used rustc 1.100.0, edition 2021, optimization, denied warnings, one codegen
unit and one linker thread. Both compiler and execution stderr files are
empty. The first native build and both executions succeeded.

The protocol and seed schedule were registered before compilation and
fixture generation. The initial registration is preserved; separate review
then tightened the frame API and reporting before execution, with an
explicit amendment and second registration. The 40 child streams are
deterministic and distinct. Seed/corpus replays establish repeatability,
not independent statistical replication.

The two field/group fixtures are kept distinct:

| Parameter | Primary fixture | Secondary fixture |
|---|---:|---:|
| p | 2147483647 | 2147483647 |
| a | 2147483644 | 847514846 |
| b | 21 | 2087444724 |
| Prime group order r | 2147487637 | 2147446831 |
| Gx | 1391443431 | 625037715 |
| Gy | 177979044 | 2100627251 |
| Targets | 64 | 64 |
| Transport/extraction rows | 6,144 | 6,144 |

Field/group primality, nonsingularity, generator membership and unreduced
[r]G=O / [r-1]G=-G were checked; the Hasse interval certifies the group
orders. All computational research and correctness checks are native Rust;
Python was used only for copying, hashing, metadata and packaging files.

Important files:

- `PROTOCOL.md`, `SEEDS.json`: frozen scope, panel and seed specification.
- `MATHEMATICS.md`: independent pre-execution derivation and scope review.
- `REVIEW.md`: review findings and their resolution.
- `transport.rs`: production candidate component.
- `independent_reference.rs`: independently authored Fp²/group reference.
- `main.rs`: evaluator; target scalars and positive witnesses stay here.
- `upstream/`: byte-preserved native arithmetic and prior comparator sources.
- `evidence/receipt.json`: exact results, hashes and evidence limits.
- `evidence/validation-attempt-1/`: immutable executed sources, executable,
  before/after manifests, raw output from both runs and empty error streams.
- `HANDOFF.json`: exact remaining task and publication/concurrency state.

## Prior work and integration

The original package pins [crypto commit eb55e931](https://github.com/isogeny-labs/crypto/tree/eb55e93110676c91213d91d6c8d00489311a4bd8).
Its arithmetic source is `research/prime31_ic_20261115/native/lib.rs` and
its primary fixture is in `src/cryptanalysis/ic_boundary.rs`. Their source
and adaptation hashes are retained unchanged in `upstream/SOURCE.json`.

[PR #5232](https://github.com/isogeny-labs/crypto/pull/5232) tracks packaging
of the original x-Cayley extractor. Its checkout was actively being changed
by another session and was left untouched. The final read found the original
implementation published at head `73c7e387f70c9f0a5935a4b6e92248fcf1c8b8bf`:
open, non-draft, and unmerged. This new residual-transport component is
separate and is not included in that ready-for-review PR. [PR #5229](https://github.com/isogeny-labs/crypto/pull/5229)
owns the current five-term MITM engineering; this work does not repeat it.
[PR #5220](https://github.com/isogeny-labs/crypto/pull/5220), pinned at
`720723bd6c5bd7369bdf638ddd670d8326365417`, implements materialized
pair-sum support and polynomial-norm intersection; compressed intersection
remains an open dependency there. Its non-draft research scope was preserved.

The standard T2 parameter and compressed multiplication are from
Rubin–Silverberg, [Torus-Based Cryptography, section 5.2](https://www.math.uci.edu/~asilverb/bibliography/ceilidh.pdf).
The indexed primary-source section was checked; direct PDF opens failed.
Semaev's [2004 summation-polynomial paper](https://eprint.iacr.org/2004/031)
could not be reopened in this turn; its retained source-bound symmetric
implementation and elementary chord identities were checked directly.
The knowledge-base MCP/CLI service was unavailable; exact repository
sources and the separate derivations supplied the fallback review.
No novelty is asserted for the underlying torus parametrization, addition
functions, matrix elimination or ordinary set-intersection algorithm.

Publication of this successor should use a coordinated draft lane or a
distinct follow-up branch; it must preserve the live original package and
its frozen evidence. This local package has not changed GitHub or enabled
merging. No PCC ID or signed validation status has been created.

**Impact score: 5/10.** A working, checked public-coordinate extraction
component and an exact restriction on same-pencil two-label searches;
shared useful-relation discovery remains the next technical dependency.
