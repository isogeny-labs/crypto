# Prime31 Rust frontier audit — continuation 5

Date: 2026-11-23 UTC. Owner: `/root/rust_frontier`, parent lane PR
`isogeny-labs/crypto#4928`. Read-only audit; no remote mutation and no new
cryptographic implementation or experiment executed by this agent.

## Scope and recommendation

The current pair oracle and higher-arity controls already cover S3 quadratic
extraction, S4 triple projection, all three binary S3 chain pairings, several
quotient/F4 root gates, signed group dictionaries, and tensor/Holant toy rank
probes. Reimplementing any of those would not establish a new mechanism.

The most concrete newly identified correctness component is the
**rational/twist decomposition gate for a full Dickson fibre at arity three**,
subject to the parent mathematical proof audit and authorization below.
It would check whether an implicit full-fibre Semaev oracle counts rational
signed relations or also counts cancelling nonrational pairs. It is a bounded
correctness component, not a timing panel, a complete relation solver, or an
asymptotic speed claim.

An alternative was a public-coordinate bucket compatibility certificate. That
should not be implemented without a specific bucket rule and a proof of its
restricted-domain compatibility: a global x-only map already faces the
sign-symmetry obstruction stated below.

**Execution status:** proposals sent to parent; implementation and new runs
are awaiting explicit parent go-ahead. Parent owns the existing ACTIVE line,
PR, protocol publication, and all external progress updates.

## Instructions and discovery boundary

Read the retained `crypto-instructions/AGENTS.md` and relevant lane sources.
Compared it to `crypto-instructions/AGENTS-current.md`; the current revision
changes skill links to the workspace trunk and does not change the applicable
execution gates. The parent fetched the current AGENTS blob
`e615bfefcaecd0c0f7cbb143902aec0b37173375`.

Applicable controls: use the parent's lane/PR; Rust only for cryptographic
work; no new experiment before a frozen specification; EKS is the default
for performance/scientific runs; separate fixed development correctness
fixtures from performance measurements; retain original evidence; charge
setup, failed queries, signed extraction, and storage. No PCC-verified claim
is made in this audit.

No research-KB callable tool was exposed by a name search for `kb_status`,
`search_knowledge`, or `research.?kb`. This is a bounded repository fallback,
not an exhaustive prior-art or novelty determination. The sibling literature
agent owns primary-paper checking. Repository search counts/hits alone were
not treated as proof of absence.

## Immutable lane inputs

| Input | Exact identity |
| --- | --- |
| Main continuation PR #4928 | head `65f413e2ca1014a28e5ba6bb456d599f020b8f29` |
| Companion pair-oracle PR #4924 | head `70e04851fae933675dbc4476f22dcea25090551e` |
| Latest implicit-index gate | `research/prime31_summation_compression_20261122/NEXT_IMPLICIT_INDEX.md`, blob `2ece7d19b0600dc08bb2fa8a7e24b2e243a63043` |
| Monic pair oracle | `examples/prime31_monic_s3_pair_oracle.rs`, blob `ec949fa765aee1686ba01a74f420f8f2f0a77df3` |
| Pair-oracle source CI | run `45089472283`, job `137415511884`, checkout `96c9101315d9f0c7ca5fee2af18cdb10d16c645d` |
| Prior Dickson quotient component | blob `750eb4ed1ea511cc70b0e7a4af413a8bf7ebeeab`, SHA256 `b76cd5a1efeff77bc065554d89acc9b53be051e6b17811cca589c2dc93a018b1` |
| Prior original-lock Dickson CI | run `45089569480`, job `137415823110`, source `109fc024e33f122fd678d2c93a18b2c0438a8668` |

The latest pair-oracle source blob is identical at the CI checkout and latest
companion head. The subsequent companion amendment is a protocol note, not a
new source implementation. These are source/receipt bindings, not independent
performance validation.

## Important correction to the pair-oracle comparison

The printed `reference_scan_ms` in the current monic-pair example is **not**
the maintained linear group-complement scan. Its `reference()` loops over
all unordered factor-base x pairs and evaluates S3, hence takes Theta(B^2)
S3 evaluations.

The candidate's `extract()` calls the Newton constructor `construct()`.
`construct_balanced()` is run separately afterward to check exact coefficient
agreement. The balanced tree is not the extractor used for its reported
online time. Its polynomial multiplication, long division, gcd, and root
factorization are also schoolbook routines.

The candidate subsequently filters roots with linear `w.contains()` calls,
returns unordered x pairs, and does not itself produce full signed group
rows. The tests and current timing labels must not be interpreted as an
end-to-end signed relation solver.

Consequently, the reported B677 loss (about 28.9–32.0 ms candidate versus
4.8–5.5 ms printed reference in the retained source receipt) is a loss even
against a quadratic algebraic reference. It is not evidence of performance
against the stronger O(B) group baseline. No new timing was run here.

### Baselines required for later candidate work

* Pair target R: retain both signs of the base points and test R-P against
  exact point membership, with all additions/inversions/hash lookups charged.
  This is O(B+output) online after O(B) base storage/setup.
* Triple target R: complete signed pair table costs O(B^2) setup/storage and
  O(B+output) online by scanning R-P. The low-memory comparison scans pairs
  at O(B^2+output) online with O(B) base storage.
* A full signed m-tuple dictionary is a correctness oracle with charged
  Theta((2B)^m) setup/storage; its O(1+output) expected hash query is not an
  implicit-index breakthrough.
* Compare exact rational signed witness sets, including duplicates and
  infinity conventions, rather than only projected x-root counts.

## Existing higher-arity components: do not duplicate

### Prime31 native controls in crypto

`S4_TRIPLE_QUADRATIC_ELIMINATION_20261115.md`, blob
`f0d7e89d4b600350ed059a6dc6fcc8aee870cb85`, and
`examples/prime31_s4_triple_projection.rs`, blob
`926f23c9aae6fe86abb4d0de2384396d209aaeb8`:

* Uses distinct base indices i<j<k, exact quadratic S3 branches, then a
  second exact quadratic S3 elimination.
* The original B31 census has 4,495 triples, 9,455 quadratic calls and
  17,980 finite projected S4 x-values, all distinct on that fixture.
* Batched inversions and scalar controls already exist. The retained local
  clocks are not qualified performance evidence.
* Explicit projection degree/census does not bound compact circuit cost.

`S4_S3_CHAIN_TARGET_DEPENDENT_GATE_20261116.md`, blob
`9eb9e9f876df2b14ca26f1180a5dbe09ca3b243d`, and
`examples/prime31_m8_balanced_f4_input.rs`, blob
`3f2430ed9af4e7bbe91d19be9f9bb30e330bf0a6`:

* Already implements all three S4-to-two-S3 pairings, the split-S4 gate and
  the m8 S3-chain gate.
* Original degree/nonzero/budget limits lead to censored partial-basis work;
  partial bases are not exact negative certificates.
* Repeating S3 chain expansion is not a new relation-solving strategy.

Other already-existing native examples discovered: `prime31_s3_root_index`,
`prime31_s3_quotient_gcd`, `prime31_m8_quotient_bridge`,
`prime31_m8_symmetric_root_gate`, `prime31_m6_formal_quotient`,
`prime31_rr9_seven_quotient_split`, and `prime31_m8_oriented_f4`.
They were not all audited line by line; their presence prevents an unsupported
claim that a new similarly named control is first-of-kind.

The M6 any-witness result already compares a complete generic-equivalent
IC-shaped solver with its own parity baseline. It does not supply a
nonenumerative prime-field summation-polynomial oracle.

### Other repository precedents

`crypto/research/ic_mega_pass_20261031/experiments/L5-s4-subquadratic-membership/`:

* README blob `3f1ea28247638d97b7a452e01456e6369e90c4f0`;
  AUDIT blob `fc60f30a07b82168dab96a5e197e7928a33cf9dd`.
* Existing pair tables, low-memory scans, resultant/gcd arm, and 2-torsion
  quotient comparisons. The audit corrects several overclaims: hard-coded
  online lookup counts, a non-adaptive model only, and naive polynomial
  product code whose apparent small-range exponent is not its asymptotic
  arithmetic cost.
* Adaptive indexing and algebraic algorithms remain outside its scoped
  equality-lookup bound.

`crypto/research/lead_sweep_20261110/LEAD-EXO-gag-holant-semaev-chain/PROTOCOL.md`,
blob `5360c764b7e63093677e7bcc7127db64a93607e2`:

* Already proposes a one-step decomposition tensor x-bit cut-rank probe
  with random and DL-label controls. A generic toy tensor-rank experiment
  would overlap it.
* It is not a lower bound for every contraction scheme or a full S3 chain.

`crypto/research/prime_norm_rank_20261112/SIGNED_HANKEL_PROTOCOL.md`, blob
`46a0689858ad537d3ea28003f8a91a16f35a02a2`, and report blob
`4cb7d8a4d63707bddbea45f4e6c1b79781b939b8`:

* Exact public signed Hankel reconstruction and rank-one branch updates
  already exist. Sign search complexity remains separate from updates.

`crypto-autoresearcher/notes/g31-004-wp2-evalcost-20261120/RESULTS.md`, blob
`4fc33086886b3b92c814f5951f709866d8eafc6d`:

* Existing S3/S4 evaluator audits universal polynomial specialization,
  independent quadratic-resultant evaluation, diagonal handling,
  coefficient reuse, and the distinction between evaluator and relation
  discovery cost. It is a legacy Python result, not code to extend here.

`cryptanalysis/suite/src/cryptanalysis/semaev_higher.rs`, blob
`e3eadf70693caaa2ce8b569addd9b4ffdcb69b4a`:

* Existing S3–S6 univariate resultant construction; factor-base tuples are
  still swept. Its header's stronger asymptotic wording is not adopted.

`cryptanalysis/suite/src/cryptanalysis/semaev_sat.rs`, blob
`07bbedc32f3252b1d042296a564833a1ad81c68c`:

* Binary-extension SAT/Weil-descent implementation, not a prime31 oracle.
* Its documented small instances already show conflict-per-triple behavior.

`cryptanalysis/experiments/prime-field-m3-cdclt-20261031/README.md`, blob
`02c91606af2e4d639e81463bfcf9fa1d5c989c5e`:

* Documents rational-lift signs, disjoint windows, chained S3 and group
  replay, exact-UNSAT restrictions and resource accounting.
* Its historical missing-implementation list must not be treated as current:
  current main already has Rust `cdclt.rs`, `proof.rs`, `selector.rs`, and
  `curve.rs` under `crates/prime-field-m3-cdclt/src`.

## Proposed fixed correctness component: rational/twist gate

The mathematical sibling identified a concrete gap in carrying the pair
lifting lemma to higher arity. If w is a nonliftable fibre abscissa and R is
rational with x(R) in the fibre, geometric points P,-P,R project to an S4
solution, although P and -P are not rational factor-base points.

Under the exact hypotheses being checked by the mathematical sibling,
E(Fp²) has odd order and each full-fibre signed tuple summing to rational R
decomposes into a rational subtotal R and anti-rational subtotal O.
For ordered signed tuple counts,

    C_m(R) = sum_{h=0}^m binom(m,h) A_{m-h}(R) b_h,

where A_j counts rational j-tuples, b_h counts anti-rational h-tuples summing
to O, and C_m counts the complete fibre. For nonzero R and no nontrivial
2-torsion, the proposed first checks are

    C_3(R) = A_3(R) + 3 L_minus A_1(R),
    C_4(R) = A_4(R) + 6 L_minus A_2(R) + 4 b_3 A_1(R).

These formulas are parent/sibling derivations awaiting proof review here,
not asserted as a new verified theorem by this audit. Counting correction
alone does not extract rational witnesses and has no claimed speedup.

### Minimal implementation, only after parent approval

1. Reuse original Rust Fp/curve/factor-base code and exact complete group
   operations; do not introduce a new field or curve implementation.
2. Bind one tiny fixed public fibre with both rational and anti-rational
   points, ordinary curve parameters, order checks, and sign convention.
3. Build a direct full-fibre count through rational/twist split states,
   and an independent rational-only complete signed enumeration.
4. Produce an explicit cancelling anti-rational pair plus rational target
   witness. Check projected S4 through existing chained S3 identities and
   separately reject it as a rational factor-base relation.
5. Compare exact counts and witness sets; include a wrong-sign or wrong
   correction mutation that must fail. Include identities, repeated points,
   and nonzero-target conventions explicitly.
6. Emit no wall-time, scaling, random-yield or exponent claim. A later
   performance protocol requires a separate frozen proposal.

Exact baseline: complete signed rational group enumeration, charged as a
correctness reference. The purpose is to test whether a future implicit
higher-arity index has the right rational output semantics before comparing
it with the O(B²)-setup/O(B)-online triple meet-in-the-middle baseline.

## Deferred bucket and linear-state gates

For a global x-only map h on an odd-order group, h(P)=h(-P). Suppose a fixed
carry set C satisfies h(P+Q)-h(P)-h(Q) in C for every pair. Comparing P+Q
with P-Q, and using unique halving, gives h(U)-h(V) in C-C for all U,V.
Therefore |h(G)| <= |C-C| <= |C|². Growing balanced bucket counts cannot
coexist with constant global carry sets. This says nothing by itself about
restricted factor-base domains or bucket-pair-dependent carry families.

A fixed candidate public rule could be falsified with a branch-complete
Rust carry certificate, but inventing arbitrary buckets solely to reject them
would not advance the user's goal. Await the literature/math mechanism.

Likewise, a universal exact linear-state sum recognizer with rows A^h and
columns (target R, suffix tuple) has rank |hA|: rows depend only on prefix
sum, and fixing one suffix yields an identity minor after selecting targets.
This scopes linear-state dimension, not arbitrary nonlinear implicit
representations or target-specific contractions. Existing Holant probes make
another generic rank census low priority.

## Parent decision requested

Approve only the fixed rational/twist correctness component if the exact
hypotheses and independent baseline are useful to the current algebraic
gate. Otherwise retain this audit and proceed with the paper gate. No claim
that an asymptotic speedup has been achieved, and no promise of one, follows
from this audit.

## Parent refinement: exact auxiliary domain, before implementation

The parent narrowed the desired fixed control to compare projected S4 ghosts
with a fully affine S3 chain whose auxiliary abscissae are actually in Fp,
and to retain all-rational positives and identity-intermediate exceptions.
The mathematical sibling is checking the corresponding Frobenius purity
statement. This domain distinction must precede a deconvolution fixture.

Read source evidence:

* `src/cryptanalysis/f4_fp_tower.rs` is a polynomial quotient over Fp.
  Its own header says free auxiliary abscissae remain ordinary polynomial
  variables. It does not automatically add u^p-u. Coefficients in Fp do
  not constrain algebraic roots to Fp.
* `DICKSON_FIRST_COLLISION_S3_20261115.md`, blob
  `23efdee96d0372fb8cebb2f643e80c56e04b822e`, explicitly scopes branch
  disjointness to rational point assignments and says P+(-P)=O is not
  represented by a finite auxiliary w. Its 961 finite-root census does
  not include that identity exception.
* The original pair adapter's `Extract::visit` obtains actual Fp roots via
  scalar square-root checks, then `leaf_relations` tests membership in
  `base.points` and verifies signs in the original group.
* The current `--m8-s3chain-f4` mode in
  `prime31_m8_balanced_f4_input.rs` does not make the entire graph a pure
  S3 tree. It contains four S3 constraints for the two leaf triples and
  retains one terminal S4(u,v,w,xR), with four free auxiliary variables.
  Its emitted result explicitly says root extraction is not implemented.
  Therefore a purity lemma for a connected finite-Fp S3 tree does not
  automatically characterize this mixed algebraic F4 ideal.

### Original extension arithmetic available

Preferred existing backend: `src/cryptanalysis/ext_curve.rs`, blob
`5ba07735ae238e6ebf4b414692a717d47081513e`. It exports `Fp2`, `ExtField`,
`ExtCurve`, `ExtPoint`; Fp2 accepts odd primes strictly below 2^31 and thus
accepts p=2^31-1. It has Frobenius, square-root, field and curve operations.
Linking through `crypto_lib` reuses the original implementation directly.
Its module dependencies are `glv_invariant_base`, `ic_boundary`,
`residual_walk`, and serde; a native standalone import must respect those
dependencies instead of copying the arithmetic.

Alternative field-only backend: `src/cryptanalysis/biext_k2.rs`, blob
`a613abd128b63801a833ef8a49ad1b5a2f393661`. Public counted `Fp` exposes
Fp2 addition, subtraction, multiplication, inversion and exponentiation
with i²=-1, valid for p=3 mod4 and p<2^63. Its EC routines themselves
specialize a=1,b=0 and must not be used for PRIME31. Module tests import
`eds_mov`; a direct source adapter would have to retain those test
dependencies or use the complete library.

`src/prime_hyperelliptic/fp2.rs`, blob
`aa38cd2908c1ab9e82785e3596ff2a5c89c0888e`, has an explicit supported
maximum modulus of 4096. Its bound must not be bypassed for this fixture.

### Duplicate-control audit

`research/twist_obstruction_20261026/src/main.rs` already checks rational
and anti-rational subtotals in characteristic two. Likewise
`research/frobenius_split_20261116_v73/small.py` checks binary mixed leaves
with Fq auxiliary abscissae; its O/T exception relies on characteristic-two
2-torsion. These are relevant precedents, not prime31 ordinary odd-order
controls. `research/orientation_yx_validation_20261115/validation.rs`
checks binary signed orientation and rational lifts, also not this domain.

`research/twist_index_calculus_20261020/s4_twist_elimination/REPORT.md`
concerns quartic-extension j=0 mixed-line bases and exact S4 identities.
It is not a Prime31 Dickson-fibre ghost-versus-affine-chain fixture.

No same-scope fixture was found in this bounded audit. This is not a global
novelty claim. No new experiment has yet been implemented or run.
