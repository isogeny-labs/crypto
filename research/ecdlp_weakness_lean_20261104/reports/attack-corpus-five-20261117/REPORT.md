# Five cryptographic assumption boundaries: adversarial Lean audit

Date: 2026-11-17
Tracking PR: #4597
Formal status: **locally kernel-checked; PCC verdicts unsigned**

## Result at the audited scope

This round turns five common inference gaps into paired formal targets: an
explicit countermodel when a premise is omitted, and a theorem after a
sufficient premise or exact boundary case is stated. The targets concern
the finite transcript-simulation predicate defined below,
Schnorr scalar encodings, Fiat--Shamir transcript serialization, KZG trapdoor
knowledge, and survival tails. None of the countermodels is presented as a
break of the corresponding correctly parameterized protocol.

The editable overview is [`attack-boundaries.dot`](attack-boundaries.dot) and
the rendered diagram is [`attack-boundaries.svg`](attack-boundaries.svg). The
pre-edit theorem and inheritance map is
[`../../NEXT_FIVE_PROOF_MAP.md`](../../NEXT_FIVE_PROOF_MAP.md).

## Requirement-to-evidence table

| target | negative endpoint | positive/boundary endpoint | status in this draft |
|---|---|---|---|
| ZK separation | exact deterministic transcript equality plus completeness can coexist with failed soundness and failed **global accepted-transcript** extraction | global accepted-transcript extraction implies soundness | proof gate PASS [pcc:unverified] |
| Schnorr scalar bridge | `k ↦ k·2` on `ZMod 6` is noninjective | a nonzero multiplier on prime `ZMod q` is injective | proof gate PASS [pcc:unverified] |
| Fiat--Shamir encoding | distinct field tuples can have the same raw concatenation, hence every hash function gives the same output on them | an ordered tagged-length codec has a left inverse and is injective | proof gate PASS [pcc:unverified] |
| KZG toxic waste | known `tau` constructs an accepting scalar-coordinate surrogate opening for any claimed value when `z ≠ tau` | at `z = tau`, scalar verification is exactly `c = y`, independent of the proof | proof gate PASS [pcc:unverified] |
| Survival tail | `S(n)=1/(n+1)` tends to zero while its cumulative tail diverges | a strictly positive uniform lower stopping hazard gives a geometric upper bound and uniformly bounded truncated tails | proof gate PASS [pcc:unverified] |

The five unsigned claim records are `PCC-20261117-a627cf` (ZK),
`PCC-20261117-1ed703` (Schnorr), `PCC-20261117-a0edbe`
(Fiat--Shamir), `PCC-20261117-d4d88d` (KZG), and
`PCC-20261117-2dca91` (survival). Local verification does not create a signed
main-branch verdict, so none is cited as verified [pcc:unverified].

“Global accepted-transcript extraction” is stronger than Bellare--Goldreich
only in its per-accepted-transcript and false-instance coverage. Overall it is
incomparable here because it omits their prover-oracle, success-probability,
knowledge-error, and efficiency obligations. The countermodel is therefore not
labelled “not a proof of knowledge.”

## 1. Zero knowledge is not soundness or extraction

Goldwasser--Micali--Rackoff define completeness and soundness as properties of
an interactive proof system, and zero knowledge as a separate simulation
property. Bellare--Goldreich likewise treat proofs of knowledge through an
extractor interface and explicitly separate soundness; their extractor
obligation is not a universal witness function for false instances.

The finite model uses Boolean statements, a unit witness, and a transcript
equal to the statement. Only `true` is in the relation, while every transcript
is accepted. The honest transcript and simulator are identical functions, so
the transcript distributions agree exactly in this deterministic model. The
false statement is nevertheless accepted, and no global extractor can return
a related witness for it.

This bespoke predicate is **not** GMR zero knowledge: it equates one
deterministic honest transcript with simulator output, rather than simulating
every polynomial-time verifier's full view.

The structural implication is deliberately elementary: if every accepted
transcript for every statement yields a related witness, every accepted
statement lies in the relation's language. That implication does not replace
the computational PoK definition.

## 2. Schnorr needs the exact scalar/group bridge

Schnorr's construction chooses a generator of a subgroup of exact prime order
`q`, performs scalar arithmetic modulo `q`, and extracts from transcripts with
distinct challenges. Pointcheval--Stern retains the exact-prime-order
condition. RFC 8235 likewise fixes prime-order subgroup parameters; its
finite-field check is `1 ≤ A ≤ p-1` and `A^q = 1`, while its EC check is that
`A` is on-curve and `[h]A ≠ O`.

The negative model removes that hypothesis. In additive `ZMod 6`, using the
element represented by `2` gives `3·2 = 0·2` although `3 ≠ 0`, so a scalar
equality cannot be recovered from a group equality. The normalized-coordinate
positive endpoint is the exact missing algebra: over prime `ZMod q`,
multiplication by a nonzero element is injective by field cancellation. The
pre-existing theorem `JKSSTCyclicECDLP.orderScalarMap_injective` supplies the
separate abstract exact-order group bridge; concrete implementations must
still validate their actual points and parameters.

This is an attack on an underspecified “group plus scalars” abstraction. It is
not a refutation of Schnorr's prime-order setup.

## 3. Fiat--Shamir hashing needs canonical transcript bytes

Raw concatenation aliases typed data: the distinct pairs
`([1],[2,3])` and `([1,2],[3])` both serialize as `[1,2,3]`. Consequently
every hash function receives the same input. This is an **encoding alias**,
not a hash collision.

The positive codec frames a typed transcript in a fixed order with a protocol-domain
prefix and a tag and length for every field. A decoder accepts only that order
and requires full consumption. The Lean endpoint proves [pcc:unverified]
`decode (encode t) = some t`, then obtains encoder injectivity from the existing
`Function.LeftInverse.injective` theorem.

The logical codec uses natural-number tokens for lengths. A byte-level
implementation additionally needs a fixed-width or uniquely decodable length
format, a length cap, canonical integer parsing, and rejection of trailing
bytes. Even an encoding alias becomes a protocol exploit only when both typed
interpretations are reachable and admissible.

CFRG draft-04 asks for prefix-free encoding maps with efficient left inverses.
Its scope is narrower: injectivity and a left inverse for one whole-transcript
logical-token codec, not prefix-freeness or draft conformance.

## 4. KZG toxic waste

For the scalar-coordinate surrogate of the KZG verification equation

\[
  c = q(\tau-z)+y,
\]

knowledge of `tau` and `tau ≠ z` gives

\[
  q = \frac{c-y}{\tau-z}.
\]

Substitution accepts any chosen claimed value `y`. A pairing-free linear group
core admits the analogous constructor
`pi = (C - y·G)/(tau-z)` (or multiplicatively
`(C·G^{-y})^(1/(tau-z))`). At the exceptional point `z = tau`, the proof term
vanishes from verification and acceptance is equivalent to `c = y`; knowing
the trapdoor does not forge an arbitrary wrong value there.

The evaluated identity certificate
[`kzg-additive-core.identity.certificate.json`](kzg-additive-core.identity.certificate.json)
records only the polynomial simplification `(c-y)+y=c` and has id
`IDC1ha9b3d803847b97b6`. It does **not** certify inversion, nonzeroness of
`tau-z`, a pairing equation, polynomial degree, a ceremony, or a computational
security reduction. Those boundaries stay in the Lean hypotheses and prose.
No equivalence between either surrogate and the actual pairing verification
equation is asserted; that bridge needs explicit cyclic-subgroup and pairing
hypotheses.

KZG's published binding theorem assumes the trapdoor remains hidden and is
based on the stated strong-Diffie--Hellman-type assumptions. The constructor
therefore demonstrates why setup secrecy is load-bearing; it is not a break of
hidden-trapdoor KZG.

## 5. Vanishing survival does not imply a bounded cumulative tail

The analytic sequence `S(n)=1/(n+1)` tends to zero, but its partial sums tend
to infinity. Thus eventual termination or a vanishing survival tail alone
does not bound the cumulative tail. The formal endpoint pairs two existing
Mathlib theorems rather than re-proving harmonic convergence/divergence.

The opposite-direction sufficient premise assumes a lower stopping hazard
`delta`: in
survival form,

\[
  S(n+1) \le (1-\delta)S(n), \qquad S(0)=1,
\]

with `0 ≤ delta ≤ 1`. It yields `S(n) ≤ (1-delta)^n` and the division-free
finite inequality

\[
  \delta\sum_{n<N}S(n) \le 1-(1-\delta)^N.
\]

For the actual unscaled bound, `delta` must be strictly positive; then the
formal corollary gives `sum_{n<N} S(n) ≤ 1/delta` uniformly in `N`.

Identifying this abstract sequence with a waiting-time tail needs a
probabilistic realization and corresponding tail-sum theorem.
`JKSSTExpectedTrials.truncated_expectation_eq_survival_sum` supplies this for
finite truncated waits, while an infinite/nontermination bridge remains open.

Under the assumptions of Theorem 4.1, JKSST give the one-run upper bound
`Pr[success] < C0/p`, then say the xedni algorithm “must be repeated at least
O(p) times.” A lower bound is `Omega(p)`; a formal adaptive runtime theorem
additionally needs the marginal-survival recurrence or history-uniform
conditional bound exposed here. An upper success hazard supports a runtime
lower bound; it is not the lower stopping hazard used for the finite-tail
upper endpoint here.

## Primary-source correspondence

- S. Goldwasser, S. Micali, and C. Rackoff, “The Knowledge Complexity of
  Interactive Proof Systems,” *SIAM J. Comput.* 18(1), 1989,
  DOI [10.1137/0218012](https://doi.org/10.1137/0218012). Author-PDF SHA-256
  `17b24f25b180ba64559a089efb443337c61c916078f73be4c496bf8d27410222`.
- M. Bellare and O. Goldreich, “On Defining Proofs of Knowledge,” CRYPTO 1992,
  DOI [10.1007/3-540-48071-4_28](https://doi.org/10.1007/3-540-48071-4_28).
  Author-PDF SHA-256
  `42c8ead7376db2f818cd975c6ab5c3026ff98c39d24e447faebc3626165848cb`.
- C. P. Schnorr, CRYPTO 1989, DOI
  [10.1007/0-387-34805-0_22](https://doi.org/10.1007/0-387-34805-0_22).
  PDF SHA-256
  `0adf1be26ae4b14dd21b433c2f05f7d64cc47428de9670b82c600fe34aea01df`;
  D. Pointcheval and J. Stern, “Security Arguments for Digital Signatures and
  Blind Signatures,” *J. Cryptology* 13(3):361--396 (2000), DOI
  [10.1007/s001450010003](https://doi.org/10.1007/s001450010003); F. Hao,
  “Schnorr Non-interactive Zero-Knowledge Proof,” RFC 8235 (2017), DOI
  [10.17487/RFC8235](https://doi.org/10.17487/RFC8235).
- CFRG, *The Fiat--Shamir Transformation*, Internet-Draft
  draft-irtf-cfrg-fiat-shamir-04, work in progress, 2026-11-02. Plain-text
  artifact SHA-256
  `d0764022bb92b717ab7ecbf985235e88c9ed088363a77bacd44b8a6bf6fdc193`.
  FIPS 204, FIPS 186-5, and RFC 9497 are concrete encoding precedents, not
  proofs of the generic codec theorem.
- A. Kate, G. Zaverucha, and I. Goldberg, “Constant-Size Commitments to
  Polynomials and Their Applications,” ASIACRYPT 2010,
  DOI [10.1007/978-3-642-17373-8_11](https://doi.org/10.1007/978-3-642-17373-8_11).
  Publisher-PDF SHA-256
  `0cca25e00b56e7ebe731e9b0ec489315e469e4df8000aced3ead55f0165f72e9`.
- M. J. Jacobson, N. Koblitz, J. H. Silverman, A. Stein, and E. Teske,
  “Analysis of the Xedni Calculus Attack,” *Designs, Codes and Cryptography*
  20 (2000), 41--64, DOI
  [10.1023/A:1008312401197](https://doi.org/10.1023/A:1008312401197).
  Author-PDF SHA-256
  `981bf955f7aecc5df4f271837f6653bd7666e5026f59df44196609d4a1dd6bc7`.

Repository predecessors read directly were
`ProtocolCountermodels.always_accept_complete_unsound`,
`JKSSTExpectedTrials.truncated_expectation_eq_survival_sum`,
`JKSSTOmegaRuntime.inverse_order_hazard_isOmega`, and the JKSST proof map.
The repository KB wrapper was unavailable: status, four targeted searches,
and the theorem search all returned exit 127 (“no executable found”).

## Formal trust and open bridges

At source commit `4ce098bdb0`, `lake build` completed all 8,941 jobs. The
repository proof gate reported `PASS 158 thm 0 trusted` for the whole project;
its retained `audit.log` SHA-256 was
`d50ac7b8770665626bb5cc42ca311ccc2703c10c6137647d46d526bc6fa73d68`.
The audit found only `propext`, `Classical.choice`, and `Quot.sound`, with
several endpoints depending on none of them. Independent Lean, source,
statement-fidelity, report, and certificate reviews found no blocking issue
after the corrections incorporated here. Exact commands and receipt hashes
are recorded in [`VERIFICATION.md`](VERIFICATION.md).

The PCC records remain unsigned [pcc:unverified]. Moreover, the pre-existing
`origin/main` ledger currently fails its own modern schema check because
legacy records omit `claim_type`; this branch's five records do set
`claim_type = interpretive` and do not repair that unrelated repository-wide
migration debt.

Even after those checks, the following remain outside the formal scope:

- a computational/oracle proof-of-knowledge framework;
- a concrete Schnorr implementation and subgroup-validation refinement;
- a byte-level Fiat--Shamir codec and reachability analysis;
- pairings, degree bounds, ceremony behavior, and KZG security reductions;
- the measure-theoretic expectation bridge and the adaptive xedni law.

No performance result, curve identity, leaderboard row, or canonical
index-calculus graph changes in this round. The canonical scoreboard and
progress timeline were checked conceptually and are unaffected because these
are logical boundary statements, not measurements.
