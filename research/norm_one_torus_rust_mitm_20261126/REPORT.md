# Batched signed-quotient MITM on the p+1 Dickson factor base

## Result and scope

The implemented component performs five-term factor-base decomposition on the
Prime31 fixture `icv1-fp31-tm3989-ae9db972`:

\[
  Q = P_{i_1}+P_{i_2}+P_{i_3}+P_{i_4}+P_{i_5}.
\]

The factor base is the production Dickson/T2 tower at `t = 6`, `lo = 0`, and
`c_top = 0`. Its 34 rational abscissae lift to all 68 signed elliptic points.
This is the actual p+1 selector, not the earlier secp256k1 engineering
prototype, which is excluded from this result.

On the publicly precommitted 40-target input, the local producer medians were:

| interval | individual normalization | batch + signed quotient | ratio |
| --- | ---: | ---: | ---: |
| setup | 20.552 ms | 3.907 ms | 5.260302x |
| query | 25.998 ms | 6.470 ms | 4.018238x |
| setup + query | 48.356 ms | 10.355 ms | 4.669821x |

These are non-isolated wall-clock diagnostics on an Intel Xeon Platinum 8573C.
They are not an admitted benchmark or independently verified speed record.
The corresponding data claim is `PCC-20261126-7a50b7`, whose standing is
`unverified` because its replay and recomputation artifacts have no keyring
signatures. [pcc:unverified]

The contention-independent structural changes are:

| counted object | control | candidate | change |
| --- | ---: | ---: | ---: |
| setup inversions | 57,052 | 2 | 28,526x fewer |
| query inversions, 40 targets | 83,060 | 179 | 464.02x fewer |
| three-sum projective states | 54,740 | 27,370 | exactly halved |
| setup group additions | 59,432 | 32,062 | 1.8537x fewer |

The candidate pays more query multiplications because batch inversion replaces
inversions with prefix/backward products. All arms emit the same hit/miss mask
and result digest. Exactly the eight planted targets decomposed; none of the 32
public random targets did.

![Control and candidate component medians, with deterministic structural counters. [pcc:unverified]](speedup.svg)

## Algorithm

The control builds every unordered signed two-sum and three-sum with
repetition. Each Jacobian state is independently converted to affine form,
then a target query scans the two-sums and looks up `Q - L` in the three-sum
table.

The optimized arm makes three changes while preserving that search space:

1. It batch-normalizes each table. For nonzero projective denominators
   `z_1,...,z_n`, prefix products and one inverse recover every `z_i^-1`.
2. The signed factor base supplies an involution `i -> neg_index[i]`. A sorted
   triple witness `w` and its sorted termwise negation `-w` sum to opposite
   curve points. The builder retains only `min(w,-w)`, exactly halving the
   54,740 triple witnesses.
3. The quotient table is keyed by affine `x`. If a lookup has the opposite
   `y` from the retained representative, the solver maps its three indices
   through `neg_index`. The resulting five indices are replayed with the
   repository's separate affine group law outside the timed interval.

The Mersenne reduction for `p = 2^31 - 1` is shared by both arms, so it does
not change the comparison's mathematical work. The asymptotics remain
`O(B^3)` setup and `O(B^2)` query; this is a component-level constant-factor
and symmetry improvement, not a subexponential breakthrough. [pcc:unverified]

Two Rust tests check the projective formulas against the production affine
law and exhaustively confirm that the signed quotient covers the complete
three-sum table on this fixture.

## Seed and replay custody

The final 256-bit master seed is public in
[`public-precommit/manifest.json`](public-precommit/manifest.json). That file,
its SHA-256 `f63b7b534c62037c6f8834c2129367c167e4351f1c9b0afb9a2c52e7f6613c71`,
the solver source hashes, and all tuning disclosures were pushed in remote
commit `df88d266047559526a0337db8df01b839d847651` before execution. Results
appear only in a later commit. Anyone can verify the order with `git show`.

Targets are derived with domain-separated SHA-256. The 32 random targets use
digest-to-scalar reduction; the eight planted targets use a separate stream
for five distinct signed factor-base indices. The manifest also records both
development seeds and every tested query block size. Block size 512 was fixed
before the public seed was used.

The native verifier reconstructs the same production factor base and targets,
builds all affine three-sums without calling the optimized Jacobian code,
scans every affine two-sum for every target, compares the complete hit/miss
mask, and replays every disclosed witness. Its retained report says
`witness_replay = pass` and `exhaustive_mask = pass` for all 40 targets.

This custody records public precommit ordering and deterministic replay; it does
not establish seed entropy, unbiased seed selection, or execution on the claimed
host.

## Proof status: deliberately not called zero knowledge

There is no zero-knowledge proof of this performance run. The retained
five-index witnesses are transparent certificates: they reveal the
decompositions. A PCC `zk-certificate` is a Schnorr proof of knowledge for an
actual discrete logarithm `Q = [k]P`; this component neither recovers such a
`k` nor claims a full ECDLP solve. Labelling these witnesses as zero knowledge
would therefore be false.

There is also no trusted execution receipt. The producer-written replay and
validator reports pass PCC's local structural checks, but correctly remain
`not_covered` until an independent keyring holder signs them. A future trusted
execution run would require a keyring-pinned verifier and a receipt that binds
the raw result bytes. [pcc:unverified]

## Relation to Amadori and PKM

Amadori, Pintore, and Sala dynamically construct a random factor base and then
solve one `m`-term Semaev dependency, with experiments at `m = 3,4,5`. Their
main unresolved cost is the polynomial solver. This work reaches `m = 5`, but
uses exhaustive generic 2+3 decomposition on a small structured base; it does
not accelerate their algebraic solver or establish their asymptotics.

Petit, Kosters, and Messeng define prime-field index calculus around factor
bases selected by compositions of low-degree maps. Their concrete p-1 and
auxiliary-isogeny constructions do not themselves give a p+1 elliptic transfer.
The Dickson tower here is a valid norm-one-torus selector in that interface,
while elliptic relation extraction remains separate. The new signed quotient
is therefore classified as **generic decomposition engineering**, not a new
PKM algebraic algorithm.

The symmetry idea is also adjacent to established negation-orbit acceleration.
No novelty claim is made without a broader literature audit.

Primary sources:

- Amadori, Pintore, Sala, [*On the discrete logarithm problem for prime-field
  elliptic curves*](https://iris.unitn.it/retrieve/handle/11572/202653/186850/Amadori%2C%20Pintore%2C%20Sala%20-%20On%20DLP%20for%20prime-field%20EC%20%28rev%29.pdf).
- Petit, Kosters, Messeng, [*Algebraic Approaches for the Elliptic Curve
  Discrete Logarithm Problem over Prime Fields*](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf), PKC 2016.
- The repository's broader p+1 assessment:
  [`research/pplus_primefield_ic_20261123/REPORT.md`](../pplus_primefield_ic_20261123/REPORT.md).

## What remains

- Run the frozen public input under the repository's isolated benchmark or EKS
  protocol; the current wall clock is explicitly non-isolated.
- Obtain independent keyring signatures for replay and recomputation, or a
  trusted execution receipt.
- Measure larger Dickson bases and memory, including `m = 7` splits.
- Integrate the kernel into a full relation stream, linear algebra, and final
  scalar recovery before comparing with single-target rho.
- Keep the current result out of the ECDLP leaderboard: no full logarithm or
  matched rho reference exists here.
