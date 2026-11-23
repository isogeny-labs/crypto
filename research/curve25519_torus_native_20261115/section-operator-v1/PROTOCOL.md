# Six-section operator: pre-code development contract

17 November 2026. Fixed native mathematical controls only. No root-discovery,
timing, sampling, paid job, solver campaign or full-base enumeration. This
component makes the preceding norm/section lead explicit; it does not admit
an experiment or supply a decomposition-cost exponent.

Question: can a six-dimensional elliptic section evaluation predicate agree
with the original group sum, including point aliases and target poles?
Classical source: Cheng cs/0507026 section2/Theorem1 and Li--Wan--Zhang
1501.01138 section4. No novelty claimed for the divisor/subset-sum equivalence.
Do not transfer the variable-dimension worst-case hardness statement to a
fixed-arity distribution or the real-RAM degeneracy algorithms to Fp.

## Public input and exact scope

Library receives a public Montgomery curve y^2=x^3+A*x^2+x, six indexed,
already oriented candidate points and a public target R. It checks a fixed
candidate tuple, NOT whether any tuple exists in a base. Raw column IDs must
be distinct; equal geometric points are allowed and use confluent jets.
No scalar/label/witness search input or hidden coefficient tag is consumed.
Membership, raw-index/sign decoding and cofactor provenance remain outside
this predicate and must still be checked by the original pipeline.

Native v1 admits only p101 or p=2^255-19; canonical A/coordinates, nonsingular
A^2!=4, six finite members with y!=0, and target O or finite y!=0. Identity
members and finite 2-torsion are explicit UNSUPPORTED errors, never a negative
sum decision. A generic geometric statement is not wider native coverage.

Use D=5[O]+[R], or 6[O] for R=O. For finite R=(r,s), a basis is
1,x,y,x^2,xy,h_R with h_R=(y+s)/(x-r). At a finite non-2-torsion member P,
uniformizer t=x-x(P), obtain Hasse coefficients of y from y(t)^2=f(x(P)+t).
Only inversion of2y(P) is used in that recurrence. Jet order is multiplicity-1,
at most5; order6 is retained for the removable-pole h_R chart.

- P=R: evaluate local sections after multiplication by t; last component y+s.
- P=-R: h_R's coefficients are shifted y-series coefficients y_(n+1).
- Other P: divide series y+s by the unit x(P)-r+t.
- R=O: regular basis 1,x,y,x^2,xy,x^3 at admitted finite members.

Group equal geometric members and impose all jets up to their multiplicity.
The resulting six-by-six matrix is singular iff the effective multiset of
six points has elliptic sum R, by the classical degree-zero divisor argument.
Retain a nonzero normalized null-section coefficient vector and check it
against recomputed original input evaluation rows. Never silently use six
identical value evaluations for a repeated point. Never clear x-r without
local trivializations at R/-R.

## Frozen fixed controls before implementation

Use public G=[8](2,17) on p101,A95; validate13G=O and all twelve nonidentity
multiples. Actual field: G=[8](9,sqrt(9^3+486662*9^2+9)), choosing the existing
native square-root orientation. These actual points are public mathematical
controls, NOT a claimed subset of the original torus base.

1. All28 multiplicity patterns of six members drawn from G,2G,3G. Toy: each
against all13 subgroup targets (364 decisions). Actual: each against fixed
targets O,G,-G,2G,3G,its known sum,its known sum+G (196 decisions).
2. Distinct sets {G,2G,3G,4G,5G,6G} and {G,2G,3G,4G,5G,-G}. Toy13 targets
each (26), actual the same seven target rule (14). Total600 fixed decisions.
Compare with independently implemented existing native curve addition;
check every positive null-section and retain all decisions, no exclusions.
3. In BOTH fields, preserve two concrete naive false positives: six raw
columns equal to G against7G (ordinary evaluations); distinct G..6G against
-G (unguarded denominator clearing). Correct operator must refuse both as
non-sums. These synthetic indexed inputs demonstrate the predicate defect;
no existence of those aliases in the huge original base is inferred.
4. Mutation controls: noncanonical coordinates/coefficient/section entries,
malformed or repeated raw IDs, wrong arity, off-curve point/target, singular
curve, unsupported modulus, member identity/2-torsion, target2-torsion;
zero/truncated/altered section certificates and a changed-target certificate.
Retain every outcome. Normalized first nonzero coefficient is one.

## Identity, admission and outputs

New isolated Rust crate; reuse existing immutable torus-s4 Field/Curve only
for field primitives and independent group replay. Snapshot every compiled
local source/config/lock input. Unit tests/fmt/strictClippy/build are development
checks; freeze source, separately copied executable, compiled-in hashes and
pre-call qualifier before the one final fixed-control call. Distinct BUILD,
PRECALL and POSTCALL names, no pre-call replacement. Preserve first failures
and any source revision; no source edits after qualification without a new
version/qualification. No root solver call or benchmark is permitted here.

Fixed matrix6, jet storage7coefficients, <=6members and modulus<=255bits
bound allocations per predicate; output cap16MiB and600 fixed comparison
cases. CLI output uses create_new and is separate from captured stdout.
These are development admission bounds, NOT complete work/RSS/GAE accounting.
No resource refusal is a group-sum or route absence certificate.

Success:600/600 decisions match, all positive certificates pass and declared
malformed/mutation controls reject; naive false positives reproduced. Failure:
any mismatch invalidates native v1 until corrected and requalified. A completed
predicate still performs no factor-base discovery. Next exponent obligation:
find its singular six-column configurations in less than cubic total relation
work under the prior full setup/yield/extraction/common-column contract.
