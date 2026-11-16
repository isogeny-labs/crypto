# Quartic representation of a shorter-prefix collapsed target

2026-11-05. Exact finite construction and bounded native correctness checks.

For an odd prime r and 1<=q<r, partition the public coefficient field by s_q(x)=(chi(x),...,chi(x+q-1)). Define F_q(x) to be the field mean of the signature fiber containing x. Every fiber has size below r because chi is nonconstant, so its size is invertible. F_q equals x on singleton fibers and is constant on every ambiguous fiber. This is a projection onto signature-constant functions, not a recovery map on the ambiguous fibers. [pcc:unverified]

Reflection tau(x)=-x-(q-1) permutes fibers: the signature is reversed and multiplied by chi(-1). An affine map commutes with taking a finite-field mean, so F_q(tau(x))=tau(F_q(x)). With c=(q-1)/2, H(Z)=F_q(Z-c)+c is odd. Its canonical polynomial can therefore enter the existing reflection-odd coefficient solve. The implementation also checks every even coefficient is zero. [pcc:unverified]

The compiler enumerates the bounded public coefficient field to find complete fibers. It constructs F_q=X+sum_x (mean(fiber(x))-x) delta_x, where delta_x=1-(X-x)^(r-1), then solves in shifted-character features of degree at most four. Enumeration is setup and consumes field-sized storage; it is not a compressed synthesis bound. No elliptic target or unknown scalar enters compilation. The output polynomial is evaluated through the existing bitplane decoder; fibers.tsv is a certificate of the target, not a runtime scalar-answer table. [pcc:unverified]

## Quartic nonsingular columns

The two-ended Miller construction extends from degree three to four. After removing a zero root, let D=d(r-1)/2. The forward recurrence generates indices0..min(D,r-1). Reversing the base polynomial generates original indicesD..r from reciprocal indices0..D-r. For d<=4, D<=2r-2 and D-r<=r-2: both recurrence windows stay below the singular index r and cover all coefficients. Restore the zero-root power and reduce modulo X^r-X after ordinary polynomial construction. The API rejects degrees other than3 or4. [pcc:unverified]

## Fixed F479 / width14 result

The recorded cubic control rejects this collapsed target with observed rank231. The quartic solve returns465 terms, exactly four of degree four, with observed odd rank233 and721 orbit columns. The separate checker confirms all479 values of the pair-mean target from a square bitmap, without importing the constructor, recurrence or decoder. A coefficient-sign corruption is rejected by a field-value assertion. Ranks are backend outputs; the separate check certifies the target values, not rank. [pcc:unverified]

The eight ambiguous fibers are (85,381), (26,174), (47,419), (217,249), (25,173), (293,441), (121,345), (292,440). Thus463 inputs are singleton fibers and16 require further information. Increasing degree has made the collapsed target representable; it has not distinguished equal signatures. [pcc:unverified]

The native suite reports12 passing tests. It includes all479 width14 target values, the retained width15 cubic collapsed target, five small-prime fixtures, domain/degree/size limits, and inherited decoder tests. Five source/data hashes bind the run; three compiler warnings are retained. The checker independently rebuilds the signature partition and means using elementary square-set characters and direct feature products. It is a separate implementation in this producing session, with no signed independent-review standing. [pcc:unverified]

No new public-point recovery layer is implemented here. The previous adaptive source and its original4563-case cohort remain unchanged. No cloud job, timing panel or comparative runtime claim is included. The immediate next construction is a conditional separating-character correction for these eight pairs, using the existing lazy source extension. Its necessary extra offsets and complete costs must be checked before claiming fewer source computations.

## Prior work

Read the saved signature-selection and fiber-repair exclusions, adaptive pair-mean construction, reflection solver and bidirectional recurrence derivation. The sign convention and reflection law are standard [NIST DLMF27.9](https://dlmf.nist.gov/27.9). Fiber averaging and two-ended recurrence coverage are derived above; no novelty-priority claim is made. The primitive is generalized in degree, while finite evidence is limited to the stated fixtures.
