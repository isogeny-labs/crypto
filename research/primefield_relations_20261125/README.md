# Prime-field relation discovery: shared selectors and public tags

**Tracking PR:** [#5206](https://github.com/isogeny-labs/crypto/pull/5206) merged
only the task card. The implementation follow-up is on
[`lane/m31-shared-selector-integration-20261125`](https://github.com/isogeny-labs/crypto/tree/lane/m31-shared-selector-integration-20261125).
**Standing:** unsigned producer evidence and reviewed hand proofs; no signed
independent PCC verdict or Lean proof. [pcc:unverified]

No new PCC ledger IDs are minted: the archived replay does not satisfy the
repository's exhaustive coordinate-input and matched-cost admission rule for
new claims. The complete unsigned records remain available for review.

## Repository replay

This directory is a dependency-free, study-local Cargo package. On Linux with
Rust, Bash, GNU `timeout`/`sha256sum`, and `jq`:

```sh
cargo test --manifest-path research/primefield_relations_20261125/Cargo.toml \
  --offline --locked --release --bins -- --test-threads=1
bash research/primefield_relations_20261125/check.sh /tmp/m31-new-replay attempt-002
```

Use an unused output directory and attempt number. With rustup, setting
`RUSTC="$(rustup which rustc)"` identifies the actual compiler for hashing.
The publication replay passed all six distinct tests and matched the complete
capacity table, rational certificates, and M31 gate JSON byte-for-byte. The
source, build/pre-call metadata, and raw outputs are in
[`publication/replay/`](publication/replay/). These are local producer checks,
not benchmark results. [pcc:unverified]

The [two-page evidence brief](report/REPORT.pdf), [editable admission diagram](report/membership-replay.svg),
and [capacity figure](report/capacity-bounds.svg) summarize the fixed results.
The typesetting script only renders previously recorded values; all research
implementations and exact calculations are Rust.

The root library gate remains incomplete in this isolated checkout. Its bounded
attempts and environment limitations are retained in `publication/`; the local
study checks do not imply a passing whole-repository gate. [pcc:unverified]

## Preservation and publication inventory

Frozen execution sources, qualifiers, raw records, and historical hashes are
retained. Four compiled ELF binaries are omitted from Git and can be rebuilt;
their original hashes remain metadata. `HISTORICAL_SHA256SUMS` preserves the
original local-package inventory, including those binaries and pre-publication
versions of editable documentation. It is not a current all-files verifier.

`PUBLICATION_SHA256SUMS` records the retained publication files. The original
attempt's `frozen/source-manifest.sha256` and `retained-output-hashes.sha256`
can also be checked from their respective directories. Updated wrappers,
theorem-domain wording, and PCC standing labels are outside the frozen records.

No canonical performance chart or scoreboard row changes: this study has no
priced comparison or new speed result. The new figures describe its own
conditional bound and correctness fixture. [pcc:unverified]

Research cycle, 25 November 2026. Source problem remains the native M31 elliptic
curve discrete logarithm problem. This study contains a deterministic Rust
capacity checker, a deterministic M31 selector/tag/relation replay gate, and
mathematical notes defining the next construction problem.

## Outcome

The cycle made two pieces of the prime-field relation program concrete:

1. A shared chain of quadratic maps can select several fibers, but for a
   uniformly sampled public-tag pool there are rigorous limits on how much
   compactness it can obtain, even if the chain is fitted after observing the
   pool. The exact Rust checker evaluates the bound without floating-point
   arithmetic.
2. A selector can admit more field elements than curve abscissae, and more
   curve points than original publicly labeled columns. The M31 gate keeps
   those sets separate, checks all original signs and labels, and replays an
   archived useful APS relation. It rejects an algebraic sibling merely being
   present in the selector as evidence that the sibling has a usable label.

The constructive control is the translated elliptic quotient
`h_A(R)=x(R-[1/2]A)`. For a tagged nonzero anchor `A` and an eligible nonfixed
point `R`, the complete two-point fiber `{R,A-R}` has cheap public tags. The
known pair identity has zero aggregate target coefficient. That distinguishes
a real, tag-preserving construction from a useful new discrete-log relation.

No new relation search or performance comparison was run in this cycle.
The historical relation is a replay, not a discovery. No exponent improvement
or practical ECDLP speedup is claimed. [pcc:unverified]

## Canonical public source problem

```
p = 2147483647 = 2^31 - 1
E: y^2 = x^3 + 847514846*x + 2087444724 over F_p
r = #E(F_p) = 2147446831
G = (625037715, 2100627251)
Q = (1477799919, 824532935)
```

The gate checks primality of `p,r`, nonsingularity, nonzero `G,Q`, their prime
subgroup order, and the total group order using the integer Hasse interval.
Source arithmetic and factor-base code are copied from immutable Git blobs,
with source pins, hashes, and exact copied-body comparisons retained. The
existing collectors and their working trees were not modified.

## Capacity result

Let `M=(r-1)/2`, and let `N` distinct nonidentity negation classes be sampled
uniformly without replacement. Consider all chains of at most `s` reduced
degree-two rational maps over `F_p`, permitting degree-one coordinate changes,
with at most `t` terminal fibers chosen adaptively. The number of chain
partitions is at most `A_s = sum_{j=0}^s p^(2j)` and each fiber has at most
`D=2^s` eligible classes.

The reviewed bound for capturing at least `h` sampled classes is

```
A_s * C(N,h) * sum_{j=1}^t S(h,j) * ((D-1)/(M-h+1))^(h-j),
```

where `S(h,j)` is a Stirling number of the second kind. The expression is
clamped at one. Chain fitting and the choice of terminal fibers are both
included; there is no free adaptive coefficient search hidden in the model.

If the union of selected fibers has at most `b` eligible classes in total,
an alternative bound is

```
A_s * C(N,h) * C(h,t) * (b-t)_(h-t) / (M-t)_(h-t).
```

Here `(u)_k` denotes a falling factorial. `b` is the **total support cap**.
When more than `h` labels are captured, `b-h` must not be described as the
actual number of unlabelled admissions. The proof note treats that distinction.

For `s=10, D=1024, N=32768, h=128` on the canonical M31 fixture:

| Selected fibers | Additional support restriction | Certified probability upper bound |
|---:|---|---:|
| 4 | none beyond degree | 2^-405 |
| 8 | none beyond degree | 2^-207 |
| 12 | none beyond degree | 2^-66 |
| 14 | none beyond degree | 2^-5 |
| 16 | at most 128 total classes | 2^-864 |
| 32 | at most 128 total classes | 2^-458 |
| 32 | at most 1024 total classes | 2^-7 |

These are conditional mathematical probability bounds, not empirical failure
rates of an implemented extractor. The formulas becoming inconclusive for
other parameters does not prove that a successful construction exists.
Correlated public-tag sources and maps on the elliptic curve itself are
separate hypotheses requiring separate analysis. [pcc:unverified]

## Executable M31 bridge

The shared-terminal gate uses the existing Dickson chain `z -> z^2-2` at depth
three and six selected terminal values. It certifies the complete rational
support by comparing `gcd(H(D_8(X)), X^p-X)` with the product of roots obtained
by independently recovering each fiber, rather than enumerating `F_p`.

The selected fixture has:

| Stage | Count |
|---|---:|
| Distinct selector roots in F_p | 29 |
| Abscissae that lift to E(F_p) | 17 |
| Signed curve lifts | 34 |
| Original labeled columns in this replay roster | 3 |

The three public columns were reconstructed from an archived 4,608-point
source and their archived indices, without executing the old relation search.
Their signed identity yields

```
alpha = 1144570523
beta  = 2011873483 != 0 mod r
d     = -alpha / beta mod r = 1038579171
[d]G  = Q
```

Expected archived values are verifier data. This is not a blinded new-target
run. The reflection control also uses archived selected points; it validates
an identity that holds generically, without claiming discovery of another
useful relation.

The gate rejects an unlabeled sibling, changed public coefficient, wrong
sign, wrong source column, noncanonical point, wrong curve, wrong target,
invalid sign, and a zero aggregate `Q` coefficient. It checks exceptional and
duplicate terminals and deliberately incomplete root certificates. All 47
recorded assertion entries passed; these comprise 31 distinct check names,
including 17 separate curve-lift checks. Two Rust unit tests also passed.

## Reproduce and inspect

- `capacity-checker/README.md` gives the standard-library Rust replay command.
  Four tests passed, and 62 complete exact rational certificates are retained.
- `shared-terminal-gate/README.md` gives the pinned arithmetic/source
  provenance, immutable qualifier, execution commands, and trust boundary.
- `notes/adaptive-chain-proof.md` contains the chain-family count, capture and
  support proofs, and the formalization map.
- `notes/selector-tag-bridge.md` treats actual rational fibers, complete tagged
  fibers, translated elliptic quotients, and the limits of two-anchor closure.
- `handoff.json` gives the next concrete task for the hourly research process.

The mathematical statements have readable proofs and independent agent
review. They have **not** been checked by Lean. The exact-integer program
certifies evaluation of the formulas, not their general mathematical proof.
The gate is producer-run correctness evidence, not an independently signed
research admission or a benchmark result.

## Next construction problem

Co-design the public-tag source, the selector, and the extractor. Use the
translated-quotient pair as a positive control, then seek a richer family that
preserves labels while permitting the extractor to discard whole candidate
classes. A generated group identity must have a nonzero aggregate target
coefficient; merely concatenating known pair identities is not useful.

One precise algebraic primitive is involution synthesis. For
`sigma(x)=(a*x+b)/(c*x-a)`, a pair `x != y` is folded together exactly when
`c*x*y-a*(x+y)-b=0`, with `a^2+b*c != 0`. This turns a layer of adaptive folding
into a finite-field incidence problem. A proposed implementation must account
for constructing the pairs; a quadratic pair table is not an unpriced shortcut.
This primitive has not yet been implemented or shown to accelerate relations.

Compact public-tag progression circuits are another useful control, but
fixed-slope interval ideas and enumerated joins already have prior studies and
active owners. Re-encoding them with bit variables is not itself a new method.
The research claim must concern a genuinely cheaper extraction step and pass
the existing original-column, sign, label, and final-scalar gates.

## Prior art and source boundary

- Petit, Kosters, Messeng, *Algebraic approaches for the Elliptic Curve
  Discrete Logarithm Problem over prime fields*, PKC 2016:
  <https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf>.
  Composed small-degree rational maps and low-degree membership constraints
  are prior art.
- Amadori, Pintore, Sala, *On the discrete logarithm problem for prime-field
  elliptic curves*, 2018:
  <https://iris.unitn.it/retrieve/e3835194-60e8-72ef-e053-3705fe0ad821/Amadori,%20Pintore,%20Sala%20-%20On%20DLP%20for%20prime-field%20EC%20(rev).pdf>.
  Publicly labeled factor-base points and useful single-relation recovery
  are prior art; partitioning and split summation equations are not new claims.
- Native source snapshot: `isogeny-labs/crypto` at
  `b60f801660eb423901ddea0c03c629089a1b5bfc`.
- Historical public APS source: commit
  `90a0314f5be63d5ca56465a884091f4be0f114fd`.

The bounds here are derived for this study. A broad literature-novelty claim
has not been established.
