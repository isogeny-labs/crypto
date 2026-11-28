# M31 residual chord labels: exact transport and signed extraction

Publication follow-up to [merged PR #5232](https://github.com/isogeny-labs/crypto/pull/5232),
prepared November 26, 2026 (America/Los_Angeles), in
[PR #5266](https://github.com/isogeny-labs/crypto/pull/5266).

## Result and scope

This standalone Rust research component transports two trace-zero chord-evaluation
labels on a fixed residual pencil, rejects incompatible pairs before square roots,
and extracts every signed rational residual pair for a compatible label. It also
constructs the public factors of a four-point principal-divisor label and intersects
two supplied label sets. The central restriction is that, for a fixed public prefix,
the two labels lie on the graph of one invertible projective map. The second allowed
set can still filter that graph; it introduces no additional residual degree of
freedom. [pcc:unverified]

The source and correctness records are the completed November 25 development
result. Publication preserves all ten files in the
[final prebuild registration](evidence/registration-before-build-v2.json).
No scientific source was changed and no new native run was performed for this PR.
The original [receipt](evidence/receipt.json) records the successful Rust 1.100.0
build and two byte-identical executions. These are producer correctness records,
with a separately authored reference and review, not a signed PCC verdict or a
Lean/kernel certificate. [pcc:unverified]

Read the [mathematical derivation](MATHEMATICS.md),
[review and resolutions](REVIEW.md), and
[three-page report](report/REPORT.pdf). The report includes an
[editable explanatory diagram](report/residual_transport.svg) and its
[build source](report/build_report.py).

## Fixed-prefix transport

Work on `E: y² = x³ + a*x + b` over `Fp`, with `p = 2147483647` and `i² = -1`.
Let the certified residual be `K=(w,v)`, `v != 0`, and let the two certified
nonrational trace-zero supports be `S_j=(s_j,i*delta_j)`, where
`delta_j² = -(s_j³+a*s_j+b)` and `delta_j != 0`.
Write `h_j=s_j-w`; certification implies `h_j != 0`. [pcc:unverified]

The chord through `-K` with slope `m` has equation

```text
ell_m(x,y) = y - m*(x-w) + v
C(X:Z)    = (X+i*Z)/(X-i*Z)
t_j(m)    = ell_m(S_j)/ell_m(-S_j) = C(v-m*h_j : delta_j).
```

Projective infinity includes the vertical chord, with `C(1:0)=1`. Each normalized
label parametrizes the `p+1` projective slopes. If a fixed public prefix contributes
`eta_j`, the raw label is `z_j=eta_j*t_j`. Eliminating the slope gives

```text
N = [[h2*delta1, (h1-h2)*v],
     [0,          h1*delta2]]
det(N) = h1*h2*delta1*delta2 != 0

M_eta = [[E,-F],[F,E]]       for eta=C(E:F)
T     = M_eta2 * N * adj(M_eta1).
```

Thus the compatible raw label pairs are precisely the graph of `T`. There are
`p+1 = 2^31` pairs on that graph among `(p+1)^2 = 2^62` possible pairs. Independent
uniform raw labels pass this fixed-prefix compatibility condition with probability
`2^-31`; selecting the second label by evaluating `T` changes that sampling model.
The vertical chord maps raw `eta1` to raw `eta2`; raw identity need not indicate a
vertical chord. These statements concern the fixed prefix and leave open structures
shared across changing transforms or labels on different unresolved pencils.
[pcc:unverified]

## Four-summand divisor connection

For `P1+P2+P3+P4=R`, set `A=P1+P2` and `K=R-A=P3+P4`. The usual addition
function satisfies `div(f_UV)=[U]+[V]-[U+V]-[O]`. Consequently

```text
F      = f_P1P2 * f_P3P4 * f_AK
div(F) = [P1]+[P2]+[P3]+[P4]-[R]-3[O]

F(S_j)/F(-S_j) = eta_j * t_j(m)
eta_j = (f_P1P2(S_j)/f_P1P2(-S_j)) * (f_AK(S_j)/f_AK(-S_j)).
```

Vertical factors cancel between each support and its conjugate. `Frame::from_prefix`
computes the residual and prefix factors from `E,R,P1,P2` and certified supports;
its input contains no target scalar or residual witness. Identity prefixes,
opposite prefix points, and tangencies retain their usual divisor definitions.
[pcc:unverified]

This chord-function/principal-divisor label differs from the x-coordinate
Cayley-product label in #5232. The independent symmetric-Semaev comparator supplies
an x-Cayley candidate superset only where the conversion is explicit; direct
chord evaluation selects the exact fiber. No equivalence of the two label
definitions is assumed. [pcc:unverified]

## API and extraction outcomes

| Interface | Behavior |
|---|---|
| `Frame::new` | Certifies curve, affine nonzero-ordinate residual, supports, and factors; caches immutable projective maps |
| `Frame::from_prefix` | Constructs residual and divisor factors using public coordinates |
| `label_from_slope`, `slope_from_label` | Forward/inverse maps, including projective infinity |
| `Matrix::compatible` | Projective cross-product check with no field inversion or square-root call |
| `extract_compatible` | `None` for incompatible labels; `Some(empty)` for a compatible nonsplit fiber; otherwise all signed ordered pairs |
| `LabelSet`, `intersect` | Deduplicates the supplied sets and scans the smaller set for exact transported membership |

For a finite slope, residual abscissae solve

```text
X² - (m²-w)*X + (w*m²+w²+a+2*v*m) = 0.
```

Their ordinates are `y=m*(x-w)-v`, so extraction needs one coordinate square-root
call and no ordinate square roots. A repeated root is emitted once. Infinite slope
yields `(O,K)` and `(K,O)`. Frames expose certified state only through copy getters,
and invalid field, curve, point, support, and projective inputs are rejected.
[pcc:unverified]

## Frozen correctness evidence

The following are recorded development-panel counts from
[the unchanged receipt](evidence/receipt.json), not a new publication experiment.
The candidate was checked against separately authored direct `Fp²` arithmetic and
the retained symmetric reference on two M31 fixtures. [pcc:unverified]

| Check | Recorded result |
|---|---:|
| Exact paired transport/extraction cases | 12,288 |
| Random-slope cases | 8,192 |
| Controlled-slope cases | 4,096 |
| Withheld pair/doubling controls recovered | 1,024 |
| Incompatible controls rejected before square roots | 12,288 |
| Exact supplied-set intersection queries | 1,536 |
| Constructed four-point prefix cases | 256 |
| Invalid-input controls rejected | 52 |
| Matrix sanity cases | 16 |
| Distinct deterministic child streams | 40 |
| Source/binary before-and-after manifests; replay outputs | Equal |

The random-slope stratum contains 4,096 empty rational fibers and 8,192 ordered
pair occurrences. The controlled stratum contains 853 empty fibers and 5,974
ordered pair occurrences. Repeated points across cases are allowed. All 256
full-prefix cases are constructed positives, and supplied-set queries use designed
overlap. These counts do not measure naturally discovered useful relations or
independent relation rank. [pcc:unverified]

| Fixture parameter | Primary | Secondary |
|---|---:|---:|
| `p` | 2147483647 | 2147483647 |
| `a` | 2147483644 | 847514846 |
| `b` | 21 | 2087444724 |
| Group order `r` | 2147487637 | 2147446831 |
| `Gx` | 1391443431 | 625037715 |
| `Gy` | 177979044 | 2100627251 |
| Targets | 64 | 64 |
| Paired transport/extraction cases | 6,144 | 6,144 |

The original evaluator checked primality, nonsingularity, generator membership,
and unreduced `[r]G=O` / `[r-1]G=-G`; the Hasse interval supplies the order argument.
All mathematical computation in this component is Rust. Publication Python scripts
perform document rendering, copying, hashing, and metadata checks. [pcc:unverified]

## Cost model and open dependency

Let `a0,b0` be the raw supplied input lengths, `a,b` the distinct set sizes,
`m=min(a,b)`, and `k` the number of matching labels. The implementation uses
deterministic `BTreeSet` storage. [pcc:unverified]

| Stage | Accounted work |
|---|---|
| Build and deduplicate | `O(a0*log(a0+1) + b0*log(b0+1))` comparisons; `O(a+b)` stored entries |
| Scan/query | `m` projective maps and `O(m*log(max(a,b)+1))` membership comparisons |
| Normalize a mapped label | At most one field inversion per visited entry, plus field arithmetic |
| Sort outputs | `O(k*log(k+1))` comparisons and `O(k)` output storage |
| Extract a finite compatible fiber | One coordinate square-root call |
| Set up each new prefix | Curve/support certification, public additions, line evaluations, and torus products |

The panel recorded 8,192 visited set entries and 125,440 reference Cartesian pair
checks. Those structural counts exclude construction, inversions, sorting, prefix
discovery, factor-base membership, duplicate processing, and relation rank. They
support no runtime or complete-cost ratio. A useful successor must state an actual
index shared across varying transforms or constrain different unresolved branches,
then account for generating labels/prefixes, updates, rejects, and useful output.
No sublinear discovery algorithm or complete ECDLP solve is supplied.
[pcc:unverified]

## Verify a fresh checkout

From the repository root, recover the original large records and verify the
publication mappings, registered sources, receipt, and original hashes:

```bash
bash research/cayley_residual_transport_m31_20261125/publication/replay_published.sh
```

This default performs custody checks only. It uses the existing artifact reader
with `--source git`; three deterministic gzip payloads recover all six logical
large-file paths from both historical runs. Manifest sizes and SHA-256 digests are
checked before installation. The 50-entry original manifest is preserved at
[publication/original-manifest.sha256](publication/original-manifest.sha256), and
[publication/packaging.json](publication/packaging.json) maps each original path.

Forty-nine manifest entries are recoverable from the repository; nine duplicate
source entries map to identical canonical sources. The historical executable is
retained only in the original archive, with its original digest explicitly recorded.
Repository verification does not claim to have verified that absent executable.
Original README and HANDOFF snapshots are preserved in `publication/original/`.
[pcc:unverified]

For an explicitly requested new correctness replay, provide a compatible Rust
compiler and a new output directory:

```bash
RUSTC=/absolute/path/to/rustc bash \
  research/cayley_residual_transport_m31_20261125/publication/replay_published.sh \
  --run /absolute/path/to/new-replay-output
```

The wrapper first verifies the historical records, invokes the unchanged
source-registered `replay.sh`, and compares the new outputs with the archived
JSONL and summaries. The original script on its own compares two new executions;
it does not perform the archive comparison. A newly compiled binary need not have
the historical executable digest. [pcc:unverified]

The frozen script requires Linux `/proc`, `taskset`, `timeout`, `sha256sum`, Rust,
and a linker accepting `--threads=1`. Its caps remain 120 seconds and 1 GiB virtual
memory for compilation, then 60 seconds and 512 MiB for each one-CPU execution.
Publication did not rerun this passing panel: Rust tooling was absent in the
restored environment. [pcc:unverified]

## Provenance and publication standing

The source provenance remains pinned to
[crypto commit eb55e931](https://github.com/isogeny-labs/crypto/tree/eb55e93110676c91213d91d6c8d00489311a4bd8),
with exact upstream hashes in [upstream/SOURCE.json](upstream/SOURCE.json).
The registered protocol and dated session records preserve their original
observations of then-open PRs. #5232 subsequently merged; this distinct follow-up
does not amend its frozen evidence.

The complete original archive is identified by SHA-256
`2a00cc7dfd3449f2acbc66ef6ed296051dc619fd94754ae2be6bbca7fff7e322`.
[publication/validation.json](publication/validation.json) records publication-only
checks separately from inherited native evidence. No PCC identifier, signed
validation status, performance leaderboard row, or certified algebraic identity
was created. [pcc:unverified]

Canonical dashboard applicability was reviewed under AGENTS section 7d. The IC
scoreboard, timeline, current-state page, leaderboards, browser corpus, and
performance-gains views remain unchanged: this publication changes no priced
IC/rho row, reference, registered curve, factor-base identity, ecbench session,
tournament/candidate identity, or measured component performance. The exact path
list is recorded in `publication/validation.json`. Full browser regeneration and
coverage validation were not run. [pcc:unverified]

**Impact score: 5/10.** An exact public-coordinate extraction component and a
restriction on same-pencil two-label searches; useful shared relation discovery
remains the technical dependency. [pcc:unverified]
