# Earlier cutoff completes the degree-32 controls

**2026-11-24 · Native development correctness · [pcc:unverified]**

The refined Dickson/RR5 oracle completed all five fixed degree-32 queries over
\(\mathbb F_{8191}\), including the planted one-relation target. The unsplit
oracle timed out on all five under the same three-second query deadline.
The refinement used eight occupied charts, each with seven variables, instead
of one system with eleven variables. A separately frozen recovery extension
returned scalar \(4356\) after 50 queries. These are producer-local correctness
and completion observations; admitted cost measurements remain outstanding.

![Fixed-panel construction and completion counts.](refinement.pdf){width=88%}

## Mechanism and scope

Let \(T_j\) be \(j\) iterations of \(X^2-2\). The original regular chart
requires the four values \(T_{d-2}(x_i)\) to be distinct roots of
\(T_2(X)-c\). Each such coarse image has two possible preimages one level
earlier. Selecting one preimage over each coarse image gives at most sixteen
monic quartics \(P_b(Z)=\prod_{i=1}^{4}(Z-b_i)\).

The candidate pushes the RR5 quartic through \(T_{d-3}\) and equates its
coefficients to those of \(P_b\). Every original rational relation determines
exactly one selection. Conversely, each refined relation maps to the four
original coarse images. This gives a disjoint partition on the regular rational
point set; it makes no assertion about unsaturated-ideal multiplicities.
Unoccupied fine images are removed using the factor base alone.
The [protocol and argument](REFINEMENT.md) record the assumptions and prior art.

For the F8191 degree-32 planted query, the eight systems contain 64 equations
in total, versus 12 in the single system. The largest degree remains four.
The variable reduction therefore trades a smaller elimination problem for
more problems. All their construction, serialization, solver calls, extraction,
signed replay and union work share one deadline. A partial union is undetermined.

\newpage

## Exact query and recovery records

| Fixture / selector | Unsplit exact queries | Refined exact queries | Refined charts per query |
|---|---:|---:|---:|
| F127 / degree 16 | 4 / 4 | 4 / 4 | 4 |
| F8191 / degree 16 | 4 / 4 | 4 / 4 | 0 |
| F8191 / degree 32 | 0 / 5 | 5 / 5 | 8 |

Exact means the full signed set matched the group-law reference on the same
regular chart. At F127 the relation counts are \(1,0,3,3\). The F8191
degree-16 base has no occupied regular chart, so refinement decides those four
empty answers during its base-only partition step. The degree-32 panel has
four empty answers and one positive answer. The emitted planted row is
\([1,3,5,7]\); it is replayed against target \((4375,6159)\).

[F127 panel](evidence/refinement_v1/f127_d4.jsonl) ·
[F8191 degree-16 panel](evidence/refinement_v1/f8191_d4.jsonl) ·
[F8191 degree-32 comparison](evidence/refinement_v1/f8191_d5.jsonl) ·
[Signed row output](evidence/refinement_v2/f8191_signed_rows.jsonl)

| Recovery check, seed 781815 | Calls | Selected rows | Base rank | Scalar |
|---|---:|---:|---:|---:|
| F127, Direct | 9 | 9 | 8 | 69 |
| F127, Unsplit | 10 | 9 | 8 | 69 |
| F127, Refined | 10 | 9 | 8 | 69 |
| F8191, Direct | 13 | 12 | 11 | 4356 |
| F8191, Refined, 30-draw limit | 30 | 9 | 9 | none |
| F8191, Refined, 128-draw limit | 50 | 15 | 14 | 4356 |

The factor base has eight unsigned points at F127 and fourteen at F8191
degree 32. A dependency with a nonzero public-target coefficient can recover
the scalar before full base rank, as in the Direct control. The candidate
extractor receives public target coordinates and base data, with no pair index.
Direct uses generic decomposition and covers additional collision charts;
its query counts are retained, not treated as equal-coverage algebraic timings.

The failed 30-draw candidate attempt is preserved. The 128-draw run is a
separately frozen extension with the same seed, not a success at the old limit.
Its first thirty attempt records match the original prefix byte for byte after
JSONL projection. Every returned scalar is checked by multiplication on the
fixture curve.

[F127 recovery](evidence/refinement_v1/f127_recovery.jsonl) ·
[Original F8191 recovery](evidence/refinement_v1/f8191_recovery.jsonl) ·
[Extension protocol](RECOVERY_EXTENSION.md) ·
[Extended recovery](evidence/refinement_v2/f8191_recovery_128.jsonl) ·
[Prefix check](evidence/refinement_v2/prefix_check.txt)

\newpage

## Evidence, accounting and remaining work

The two copied native qualifiers were hashed before their calls. Version 1
source is pinned by commit \texttt{7407a75c}; version 2 by
\texttt{45aa9d5b}. Each version has its own build record and pre-call manifest.
The older \texttt{NATIVE\_SHA256SUMS} describes the original source revision;
its historical hashes are not changed to match the new implementation.

Six standalone Rust tests passed. They include exhaustive partition checks for
all four-element unsigned tuples on five fixture/depth pairs, zero-budget
rejection, and failure after a completed subchart. All five development
invocations in version 1 and both invocations in version 2 exited zero.
The root-library validation exhausted both its 180-second and subsequent
600-second build allowances before reaching tests; both exited 124. Its logs
are preserved in the [development-check record](DEVELOPMENT_CHECKS.md).
The benchmark-record gate also stops on an unchanged historical archive record.

The query deadline is cooperative between native phases. The msolve subprocess
is killed at expiry; a construction or extraction phase that overruns the
deadline makes the whole query undetermined. This is not a guarantee that the
wall duration of a failed query is at most three seconds. No stage or full
pipeline timing is admitted from this shared host. Base construction, mask
generation, linear algebra and final verification are executed by the recovery
driver but have not been priced in a qualified cold comparison.

| Original requirement | Current evidence / dependency |
|---|---|
| Low-/high-degree mechanism | Finite-partition argument and native implementation; independent proof standing open |
| Reproducible relation extraction | Exact fixed panels and signed output on the regular chart |
| Complete recovery | Local F127 and F8191 public-fixture scalar checks |
| Closest published comparison | PKM map/isogeny construction and existing first-collision controls reviewed |
| Strongest production IC comparison | Compact quotient-product and oriented-S3 controls still need matched integration |
| Setup, collection and linear algebra gain | Qualified cold measurements and independent replay remain open |

The useful next comparison integrates Refined into the production workload
alongside its algebraic controls, preserving the complete query interval,
regular-chart coverage and failed calls. The standalone workload has no
registered EKS image/build route yet, so no EKS job was submitted. No leaderboard
or benchmark row is promoted by these development results.

Claim records: \texttt{PCC-20261124-a0c535} (partition argument),
\texttt{PCC-20261124-583d94} (degree-32 completion),
\texttt{PCC-20261124-5ddbcd} (30-draw recovery), and
\texttt{PCC-20261124-6f5ee8} (extended recovery). All have unverified standing.

## Reproduce

Build the isolated crate with Rust 1.90 or compatible and msolve 0.7.5. The
following commands run the new encoding and preserve the explicit recovery
allowance. The code checks exact query sets and returned scalars internally.

    cd native
    cargo test --release --locked
    cargo build --release --locked
    ./target/release/dickson-four-summand 8191 5 "$MSOLVE" distinct refined
    ./target/release/dickson-four-summand 8191 5 "$MSOLVE" recover=781815 draws=128 refined

To reconstruct the original 30-draw extension prefix, omit \texttt{draws=128}.
The report and figure rebuild with Graphviz and Pandoc/XeLaTeX from the
committed sources.
