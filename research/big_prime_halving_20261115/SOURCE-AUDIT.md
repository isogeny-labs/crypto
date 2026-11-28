# Multiprecision setup source audit

Source f5b318b482188979df59463bdab49b9431ce3969 is committed and pushed.
Fresh build002 succeeds with strict C++17 warnings. Build001's missing explicit
FLINT declarations/layout warnings are retained; no numerical qualification
has run. These are source-review observations and local receipts, without
signed PCC standing. [pcc:unverified]

## Actual contracts

The candidate uses GMP integers, proves prime admission through the installed
FLINT entry and calls prime-field polynomial roots/square roots. The separate
reference includes only OpenSSL and the C++ standard library: it derives the
quartic from f'(X)^2-4(2X+xp)f(X), checks the complete root product with its own
BN polynomial gcd, lifts with BN square roots and doubles with EC_GROUP.
It does not include the candidate, its GMP representation or its FLINT roots.
Primality in the reference is BN_check_prime, with its own API trust; this is
not an independent formal primality certificate. [pcc:unverified]

For prime p, X^p-X vanishes on every field element and has derivative-1.
Its gcd with the quartic therefore has exactly the distinct rational roots,
even when the quartic has repetitions. Equality of the monic gcd and the
product over a distinct canonical claimed set proves set completeness at the
mathematical level. Rational x roots may fail to lift; actual doubling still
decides which ordinate lifts are halves of the finite target. The shared
library's additional semantic root-certificate source7364d8 passes the canonical
local gate108theorems/0failures/0trusted native before native qualification;
it does not bind this C++ program. [pcc:unverified]

The old uint64 rejection mapping is retained for fitting domains. Larger
domains mask independently drawn words to the needed bit count and reject
values outside the bound. Every word and rejection is logged and replayed.
These statements describe the ideal uniform-word model; fixed mt19937 seeds
and finite retry caps do not prove that probability model. [pcc:unverified]

The sampler only enumerates the d-element generated subgroup and its coset.
The field-wide census exists in its separate outer control mode. It verifies
exact power-of-two order and 2G outside the subgroup before building every
degree-two step, its codomain and point images. The reference checks a global
cleared curve identity for the advertised codomain before coefficient/point
checks, then recomputes the composed polynomials and fibre factorization.
The planned adverse codomain edit must fail at that identity. [pcc:unverified]

## Proof and cost boundaries

Shared crypto-lean source b38b6b4712017a6d87140f302c04d754a3280dce gives
cleared_doubling_quartic, velu_two_curve_residual, velu_two_curve_identity and
velu_two_affine_curve. Canonical local source/axiom receipts at publication
5e26040252bdf29f3bf4f800a0b44def2eeb74e9 audit106theorems with0failures
and0trusted-native exceptions. PR9 merged as8164aa0; hosted run43728954205
had zero executed steps because of the account billing lock. No hosted pass
is asserted. These are semantic ring/field laws, not group homomorphism,
native execution, parser, completeness or complexity proofs. [pcc:unverified]

The actual naive composition and root-product operations cost O(d^2) field
operations; subgroup/coset construction has polynomial work in d,t. The
parent's abstract expected subgroup bound depends on the classical modular-
curve count and ideal uniform choices. It does not establish polynomial-time
primality admission or remove the software's fixed caps. The relation oracle,
sufficient degree and useful relation yield are separate open dependencies.
This port supplies no new oracle, exponent or ECDLP recovery. [pcc:unverified]

## Qualification remains pending

Exactly one qualification001 is frozen in PROTOCOL.md and qualify.sh. The
copied read-only wrapper is /workspace/scratch/big-prime-halving-runner001.sh;
no raw round directory has been created. Admissions,14754preimages,18setups,
the eight legacy comparisons and four adverse cases are all pending. No
timeout, empty set, numeric success or failure is inferred. [pcc:unverified]

Before launch, scripts/checkpoint.sh gate failed because no AWS profile,
ambient access keys or upload token is available. The tool inventory exposes
no AWS/checkpoint connector. The existing trusted artifact workflow uploads
data through its own secrets; it is not an arbitrary checkpoint-upload or
credential extraction interface. It was not repurposed. The repository rule
at AGENTS.md601 forbids launch after a failed gate, so no wrapper/case ran.
The gate's exact diagnostic is retained under checks/checkpoint001.log.
Checkpoint access was requested without asking for credentials in chat;
source review/proofs continue independently. [pcc:unverified]

Q: Does a complete x-root list already give all point halves? A: No; enumerate
all ordinate lifts and check their actual double against the target.

Q: Do these software caps support a growing-family theorem? A: No; the
abstract family and finite implementation have separate admission boundaries.

Q: What is the next concrete numerical action? A: Restore the existing upload
access, pass the same checkpoint gate, then invoke the frozen wrapper once.
Do not relax caps or replace inputs in response to a failed control.

Primary links actually checked: [MIT2023 lectures](https://math.mit.edu/classes/18.783/2023/lectures.html),
[Lecture2](https://math.mit.edu/classes/18.783/2023/LectureNotes2.pdf),
[Lecture3](https://math.mit.edu/classes/18.783/2023/LectureNotes3.pdf),
[Lecture4](https://math.mit.edu/classes/18.783/2023/LectureNotes4.pdf),
[Lecture5](https://math.mit.edu/classes/18.783/2023/LectureNotes5.pdf);
FLINTv3.1.3 [is_prime.c](https://raw.githubusercontent.com/flintlib/flint/v3.1.3/src/fmpz/is_prime.c),
[roots.c](https://raw.githubusercontent.com/flintlib/flint/v3.1.3/src/fmpz_mod_poly_factor/roots.c),
[sqrtmod.c](https://raw.githubusercontent.com/flintlib/flint/v3.1.3/src/fmpz/sqrtmod.c).
Shared formal sources: [EllipticMaps](https://github.com/aburan28/crypto-lean/blob/b38b6b4712017a6d87140f302c04d754a3280dce/CryptoLean/Polynomial/EllipticMaps.lean)
and [RootCertificate](https://github.com/aburan28/crypto-lean/blob/7364d8dab7014a8bc610ed6911e7fb0839f6d47d/CryptoLean/Polynomial/RootCertificate.lean).

Shared PR10 merged as e0e86e131728089a12759b8860db3834517f5f89. Its
hosted run43729911221 also has zero executed steps; the check annotation says
the job was not started because the account is locked due to billing. This
does not change either local proof receipt or supply a hosted pass.
[pcc:unverified]

## Relation-solver checkpoint, 15:50Z

KB discovery still returns no tools and the repository wrapper still has no
executable. Re-opened the precise initial-elimination, selective-oracle and
code/guessing reviews, plus the November7 elliptic-code RESULTS. Downloaded
and read [PKM2016](https://christophe.petit.web.ulb.be/files/16PKC_primeECDLP.pdf)
sections3.1--3.5/5 from the author's site; the separate
checks/oracle-review-sources.sha256 binds that PDF. Its auxiliary-kernel
factor-base framework is prior work, and its generalized-root-finding cost
remains an explicit dependency. The present setup is not a new relation
solver or an asymptotic improvement claim. [pcc:unverified]

The existing selective reduction needs a label-free positive oracle with a
known per-call cap T and known success lower bound theta on a constant fraction
of sampled bases. For Bmax bounded globally, collect B+1 answers in a block
of ceil(4(Bmax+1)/theta) attempts, choose a nonzero left-kernel vector from the
relation matrix alone and recover from the random labels if its denominator
is nonzero. Every failed call and base block is charged. Both T and theta^-1,
setup, base size and final linear algebra need effective subexponential bounds.
The existing review supplies this conditional reduction, not the oracle.
[pcc:unverified]

The two bounded critical-pair panels do not establish a sufficient degree or
positive yield; their capped outcomes are not degree lower bounds. The coding
review's exact m-column singularity predicate still needs a column-selection
algorithm. Its almost-complete support guess reduces to group completion and
membership; November7's toy decoder verdict is UNRESOLVED, not a general
impossibility result. The explicit cover review scopes full materialization,
without excluding smaller implicit bases. No one of these documents supplies
the missing complete classical algorithm. No new solver/model computation or
scientific measurement was run during this source checkpoint. [pcc:unverified]

Next discriminating mathematical work is an explicit positive-only algorithm
with a provable cost/yield contract, or a sufficient degree bound for the
actual source-provenanced norm system. The frozen multiprecision controls
would qualify software only. They remain pending the unchanged upload gate;
additional setup lemmas or larger implementation caps do not resolve this
oracle obligation. [pcc:unverified]

Source checkpoint claim PCC-20261115-b6065a remains unverified. The four exact
repository blob identities are retained in checks/oracle-review-source-blobs.txt.
