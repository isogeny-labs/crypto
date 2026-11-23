# Original-coset low-degree interpolation rank — prospective v1

Continuation of PR5071. Own only tools/torus-folding-rank and this study;
existing ACTIVE line tracks work. Task-first9d10a249af published as5b24ae3d
before literature/source work. SAME CI-only checker paused. No edit to frozen
folding-gate or prior source/claims/raw evidence; old TWO pending raw archives
remain byte-identical. Conductor check unavailable (binary absent); tracked
CONDUCTOR guide explicitly describes warning-only behavior when uninstalled.
No other session ownership is inferred. New paths only.

## Question and exact domain

Test whether the defined low-degree linear interpolation spaces have a
nonzero solution on a finite prefix of ORIGINAL actual-field torus-coset rows.
This is an exact finite algebra check, not sampling, a decoder or a cost
benchmark. It targets the rank-dependent escape left by the previous generic
dimension-count audit. No novelty for classical interpolation or elimination.

Public Spec is unchanged actual p=2^255-19, A486662, a60824497,
q213156431, K20633. Use j=1 and t=0..107, z_t=h^(1+a*t), gamma=h^a.
Verify gamma^q=1/gamma!=1 and all prefix z distinct. Use original rational
raw-point branch selection: inspect BOTH x=-A/2 +/- A/(2*z.a), require exactly
one admitted nonzero-v rational branch, then take min(v,p-v) as canonical
orientation. An exception is an ERROR/refusal, never silently skipped. Both
global sign orientations are retained. Original Circuit witness/check,
IndexTable decode and independent raw-point/eightfold-image replay must agree
with j1/t and the canonical label; retain full original member witnesses.
No q1 substitution, planted section/target coefficients or large-base scan.

For each1<=s<=m<=6, D in{0,1}, each sign, set

 Q(X,Y)=A0(X)+sum(i=1..s) Ai(X)*Yi,
 deg A0<=D+4, deg Ai<=D.

Unknown count c=(D+1)*(s+1)+4<=18. Received values are r_t=-x_t*y_t,
the negative normalized phi4 coefficient used by the earlier section model.
The matrix row for start t is [z_t^d,d0..D+4] followed by
[z_t^d*r_(t+i),i0..s-1,d0..D]. Coefficients live in ORIGINAL Fp2;
r embeds from Fp. Rows follow natural aligned blocks t=b*m+offset,
offset0..m-s, complete blocks only within108 positions. Do not concatenate
good positions across gaps, reorder after seeing rank or replace z by integers.
There are84 fixed cases. Use all declared rows until full column rank, retaining
their original start indices; a short prefix rank deficit is not universal
absence. The positive/negative streams are related by known column signs;
check this relation rather than treating them as independent evidence.

## Acceptance and falsification

Classical row reduction selects original independent row indices. Full-rank
case retains its square minor, nonzero determinant and determinant inverse.
A separate Bareiss-style pivot recurrence recomputes determinant from ORIGINAL
minor entries and must agree with the first Gaussian determinant. Retain the
complete original rows, full minor and determinant-inverse evidence in captured
native output; no digest-only substitute. Corrupted determinant/inverse,
row labels, dimensions and noncanonical input must refuse. If not full rank,
retain rank and a nonzero null vector, verify every declared-row residual;
classify the result as this finite prefix only. No extrapolation to other
cosets, orderings, degrees, charts, decoders or the whole actual group.

Controls before qualification: small exact determinant cases including row
swap/singular/rank-zero, polynomial received stream with an explicit kernel,
noncanonical/malformed/range refusals. Original membership/index/cofactor
checks and two separately implemented determinant recurrences are native
producer checks, not independently signed/formal validation.

Source committed/pushed before immutable final qualification/call. Separate
write-once BUILD/PRECALL/POSTCALL, compiled input hashes, exact composite source
archive round-trip, binary pre/post, actual shell positive/collision transport.
Final fixed native call <=600seconds/2GiB; zero paid/backend jobs/retries.
Focused release unit/fmt/strictClippy only. No source edit/test after sealing.
Full raw tree in ONE lossless archive; actual artifact policy applies without
exemption, splitting/renaming or invented receipt. Missing upload access is
explicit, not publication success. No Python field/group/LA driver.

## Sources before source edits

Original native membership/index/torus source, prior folding-gate protocol,
Lean guards and original section phi4 normalization read. Guruswami
https://arxiv.org/pdf/1106.0436 section2.2 equations4--8 gives this classical
linear interpolation template; section2.3/Theorem7 gives separate candidate
retrieval assumptions. The elliptic section code is NOT established as that
polynomial-message code. Source locator correction is recorded separately;
prior frozen reports mistakenly cite sections3.1--3.2.

Official FLINT https://flintlib.org/doc/fmpz_mat/index.html documentation for
fmpz_mat_fflu/fmpz_mat_det_bareiss describes classical fraction-free minor/
determinant checks. We implement a separate finite-field recurrence, not call
FLINT or claim fastest/novel linear algebra. Bareiss original AMS PDF returned
403 and is not claimed read. KB unavailable: no loaded connector; wrapper
status/search exit127, intended query "folded interpolation rank dependent low
degree torus". Existing pinned curve/source/criteria remain unchanged.
[pcc:unverified]
