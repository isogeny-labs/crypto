# Actual cofactor equations force a full nonzero norm

2026-11-13. Source5e7f9bb5c6be163be26606f340f24ace757f7945.
Unsigned producer proof evidence [pcc:unverified]. The tracked-source gate
audits 217 statements with 0 additional trusted axioms; import-inclusive fresh
replay exits 0. Exact source bindings are recorded separately.

Central unsigned bindings: full nonzero norm PCC-20261113-3f7f58, even-chart
normalization PCC-20261113-ddc7c0, monic uniqueness PCC-20261113-01ad5c and
native control data PCC-20261113-1aa548. All 16 statement paths are in
[claim-pins.json](../recovery-evidence/claim-pins.json).

The conditional certificate now recovers the full polynomial identity

    (C alpha*F)^2 - f*(C beta*H)^2 = C c*G,  c != 0,

from the actual reduced coefficient cofactor equations. No norm identity,
nonzero normalization oracle, numerical rank or coordinate oracle is an input
to the final theorem. The earlier actual branch/corank/cofactor construction
provides its structural guards. Suitable F,H,V and the cofactor equations
remain supplied; their efficient public-input finder is not established.

In the even chart, deg F=deg H+2, deg f<=3 and deg G=2 deg F, the normalization
is exactly c=alpha^2, which is nonzero. This c is the polynomial normalization;
it is not the secret discrete logarithm.

## Exact hypotheses and conclusion

Over any field, assume F,H,G monic, deg H>0, deg H+1<deg F,
UF+V(fH)=1, F(q) nonzero, alpha nonzero and
alpha F(q)-y beta H(q)=0. Public degree bounds are

    2 deg F <= deg G,       deg f + 2 deg H <= deg G.

For every j<deg G, take the same actual reduced coefficient matrix as the
parent component, built from

    mu_j(P)=coeff_j(P)-coeff_degG(P) coeff_j(G).

Require its pairing with the adjugate of the actual reduced quotient-moment
matrix to vanish. actualNonzeroNorm then supplies c nonzero and the full norm
identity above. actualNormalizedNorm specializes the normalization in the
even chart. No norm identity is assumed indirectly through the structural
guards: the prior exact radical construction uses F,H,V and the branch,
independently of G [pcc:unverified].

## Why the proof is not circular

A nonzero diagonal cofactor gives an actual nonzero adjugate column. The
matrix-adjugate identity places it in the actual kernel. Symmetry and the
proved radical line make it a nonzero scalar multiple of the reduced
candidate polynomial direction. The rank-one adjugate factorization transfers
every cofactor equation to the corresponding quadratic equation; nonzero
field scaling can then be cancelled. Exact public branch insertion connects
these quadrics to all residual coefficients of the original polynomial norm
[pcc:unverified].

The public bounds control the norm degree, so coefficient extensionality
reconstructs the whole polynomial, including its top coefficient. Independently,
the branch guards force beta nonzero. If the norm were zero, then fH^2 would
be a constant multiple of F^2. Multiplying the supplied Bezout identity by H
would force F to divide H, contradicting the explicit degree gap. Therefore
the reconstructed scalar cannot be zero. In the even chart, the lower norm
term has degree strictly below2 deg F; monicity makes the top coefficient
alpha^2. The old normOfWitnessLine theorem is not used to assume the very
identity being recovered [pcc:unverified].

## A control for rank-directed proposals

The actual reduced moment matrix for fixed F,H,V,q,y does not depend on G.
Thus its rank can remain unchanged when the target changes. The new monic
target uniqueness lemma says a fixed nonzero norm has at most one monic target.
Together with actualNonzeroNorm, at most one monic G can satisfy all the
actual cofactor equations under the stated degree bounds [pcc:unverified].

A correct experimental gate must measure these equations and valid resulting
relations as well as rank. Include a changed monic target of the same degree:
rank alone must not count that case as a recovered relation. This supplies a
specific falsification control for the annotated nonscalar-origin/jet-completion
proposal. It does not disprove a construction that genuinely uses the target
to find suitable moments; reachability through W_base+C(T)H(T) remains unproved.
The H in that jet expression is not identified with the norm polynomial H.
The finite native regression below checks this distinction.

## Native control on the existing F17 instance

Native source freeze582e21bfb; code snapshot e2f372001; prospective fixed-case
contract ad3005105. One write-once artifact invocation contains two controls,
using the same actual five-dimensional reduced matrix. Both use q=0,y=6,
the same curve, monic degree6 and six distinct rational nonzero-y abscissas.
The changed set replaces13 by7; it is rejected for this fixed certificate,
not asserted globally unsatisfiable [pcc:unverified].

| Support abscissas | Rank | Actual cofactor pairings modulo17 |
|---|---:|---|
| 0,3,5,6,10,13 | 4 | 0,0,0,0,0,0 |
| 0,3,5,6,7,10 | 4 | 0,12,4,12,16,5 |

The actual pairings equal2 times the norm coefficient residuals in both cases.
Checks validate Bezout and branch identities, monicity/degrees, distinct
support and rational lifts, literal congruence against the explicit entry
formula for every matrix, Gaussian rank against independent Leibniz
determinants, both matrix-adjugate products and scaled-column factorization.
A nonsymmetric2-by2 control catches adjugate-transposition errors. Three native
tests, strict clippy and formatting pass. These are deterministic development
checks, not independent A/A, relation-yield or timing evidence [pcc:unverified].

[Original native artifact](../recovery-evidence/native-control.json),
[checks](../recovery-evidence/checks.json),
[native source](../native-recovery-control/src/main.rs) and
[frozen control contract](../NATIVE_RECOVERY_CONTROL.md). Artifact digest:
81e7b4b55b6db1efba8f886b0d638fb236940cdc370cbdab638a6cfcb31d6f21.
The unsigned data attestation reports not_covered. Its solver_executed=false
field means no attack solver; Gaussian elimination is explicitly used for
finite rank checking. No F4/GB or discrete-log search runs.

## Checked scope and remaining work

There are 16 new theorem statements in NormCertificateRecovery. Builds001–005
all pass; all are retained with explicit exits. The whole-study tracked-source
gate audits 217 statements with 0 additional trusted axioms. A separate fresh
replay receipt covers imports. Producer checks are not independent signed PCC
standing; exact source bindings have their own recorded status.

This closes the polynomial norm and its nonzero normalization for this chart.
It does not by itself establish a prime subgroup, rational factor-base support,
a full divisor/group relation, the secret scalar, a relation finder or a
uniform sub-exponential bit bound. A sub-exponential algorithm would still need
to find the supplied F,H,V/cofactor certificate from public input, charge
setup/rejections/linear algebra/recovery and prove exp(o(log r)) total expected
work. A faster certificate checker is not that finder [pcc:unverified].

All 16 maintained source-only checks report not_covered because their default
PATH lacks Lean. Explicit-toolchain source audit and fresh replay have separate
successful kernel receipts; independent signed standing is not claimed.
Native strict lint initially lacked its development component; installed it
and reran successfully. The global Cargo.lock ignore initially omitted the
lock from the code snapshot; tracked the unchanged tested lock before artifact
execution. No proof or native arithmetic check failed.

Sources checked (KB unavailable): the prior actual branch/rank/cofactor source,
RankOneExtraction pivot factor, NormDualPencil pairing/minor lemmas, actual
NormMomentMatrix coefficient matrices, NormLift.norm and pinned polynomial
monic/degree/coefficient/divisibility APIs. These are standard algebraic
arguments; no novelty or exhaustive literature claim. Pure local proof work;
no solver, timing, paid run, benchmark, leaderboard or ecdb result.

## Reproduce

```sh
lake -d research/prime_norm_rank_20261112/lean build
bash tools/lean/proof-gate.sh research/prime_norm_rank_20261112/lean
cd research/prime_norm_rank_20261112/lean
lake env leanchecker --fresh --verbose NormCertificateRecovery
```

For the native regression, from the repository root:

```sh
cargo test --locked --release --manifest-path research/prime_norm_rank_20261112/native-recovery-control/Cargo.toml
cargo run --locked --release --manifest-path research/prime_norm_rank_20261112/native-recovery-control/Cargo.toml -- /tmp/norm-control.json
```

The output file must not already exist. The recorded compiler is Rust1.99.1.
Proof and native input hashes, original logs and available process exits are
retained in recovery-evidence. No performance conclusion follows from compiler
or test-harness elapsed fields.

[PDF](report.pdf), [TeX](report.tex), [DOT](recovery.dot), [SVG](recovery.svg),
[source gate](../recovery-evidence/gate-001/status),
[fresh replay](../recovery-evidence/fresh-status.json).
