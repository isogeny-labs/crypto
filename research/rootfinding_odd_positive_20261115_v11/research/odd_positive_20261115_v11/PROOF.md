# General-cubic, arbitrary-domain odd-extension positive controls

Authoring only; no scientific execution. The hand-derived GF9/GF25 fixtures
and independent review are pinned in field_split_novelty_20261115_v10 and
field_split_review_20261115_v10. The executable is fresh and must receive its
own independent code review and preregistration. Exposed constructive controls
do not establish novelty, hardness or any speed ratio. [pcc:unverified]

The runtime arithmetic is self-contained, copied from the reviewed v4 helper
bodies into this new worker. GF9=GF3[t]/(t²+1), GF25=GF5[t]/(t²+2), encoded
a0+p*a1. Inverses and defining-polynomial irreducibility are checked; direct
two-coordinate multiplication tables are compared with independent coefficient
convolution on every pair, and all triples check associativity/distributivity.
Integer constants and formal derivative factors embed modulo p, including
3=0 in GF9. Squaring/cubing is never assumed Fp-linear in odd characteristic.

The public inputs retain the full ascending monic cubic C, including C2=10
in GF25, and an arbitrary two-point source domain D. A Euclidean gcd with
C' proves squarefreeness rather than testing only rational roots. Verify the
target/source curve points, nonzero selected ordinate, a!=z and target
abscissa outside D. Odd h=0 is admitted; no binary h-inversion guard is used.

For P=X³+rX²+sX+t,G=hX+k, target T=(z,yT) fixes
t=-z³-rz²-sz-(hz+k)yT. Thus P(z)+G(z)yT=0 and N=P²-CG² is monic degree6
with exact target quotient H=N/(X-z). On G(a)!=0, P(a)=-G(a)ya, ya!=0,
the double-root condition is equivalent to
P'(a)=-C'(a)G(a)/(2ya)-hya. The two r/s equations have determinant
-(a-z)². Their exact affine solution in the SAME k is implemented from the
prior proof; it divides neither by h nor by3. Both the formal coefficient
norms N0,N1,N2 and every derived k receive literal polynomial division.
No Delta branch is dropped: Delta is recorded but not used to restrict k.

The independent dense oracle traverses every r,s,k in both fixed fields:
9³+25³=16354 states. It does not reuse affine r/s. It directly evaluates
the selected point condition and the literal expression
N'(a)=2P(a)P'(a)-C'(a)G(a)²-2C(a)G(a)h. Every regular survivor then receives
literal convolution and exact target/double-factor divisions. These two
conditions are necessary and sufficient by the preceding proof, so the
prefilter preserves the complete selected regular parameter set. Compare
full canonical(k,r,s,t) sets, not merely counts; retain each dense survivor Q.
Counts distinguish raw states from actual literal norm evaluations.

All q possible k values are visited by the derived chart except the unique
G(a)=0 value. Those common-factor raw assignments are counted but their
branch remains UNKNOWN. Ramified and target-collision inputs are outside
these admitted fixtures and are explicitly unexercised/UNKNOWN. No general
original auxiliary or nonrational-point closure is inferred.

For each monic cubic Q, form A_D=product(X-u) over the two distinct source
elements. Exact support, including repetitions, is equivalent to
(A_D modQ)³=0: primary factors of Q must divide the squarefree split A_D,
and exponent3 covers every cubic multiplicity. Independently multiply ALL
four possible cubic multisets over D and compare complete support decisions.
For supported Q append two copies ofa and emit all canonical five-abscissa
orders. A separate oracle multiplies the factors for all32 ordered elements
of D^5 and compares directly with H, so every positive source ordering and
unsupported empty ordering set is checked independently of bag recovery.
Different k fibers are retained even when they produce equal H/Q.

PUBLIC_INPUTS is the only input to solving. EXPECTED_CONTROLS is loaded
AFTER solving both fields; its known parameters and ten-order source bags
are postrun witnesses only. Both fields must produce a positive supported
parameter. Source labels refer to x coordinates: ten canonical x orders is
not a count of rational ordinate assignments or original S3 auxiliary roots.
The norm point T is the negative of the source-sum point, as proved in the
fixture derivation; no opposite sign is silently substituted.

Every regular candidate is archived with full P,G,N,H,Q, domain polynomial,
support remainder/cube and all source orders, including all rejections.
Independent postrun replay must inspect those recorded certificates without
rerunning candidate search. Raw bytes<=128KiB, gzip<=16KiB, stdout<=16KiB;
overflow returns inconclusive with an omission hash and no complete flag.
The stdout contains compact counts only. The exclusive artifact is raw.json.gz.

Internal limits are500000 checked predicates,20000 raw states,80s work,
84s post-serialization completion demotion and88s final print. The enclosing
runner charges the entire90s process lifetime,4GiB per-process and one core.
Late results clear exact conditional counts/completion; raw provisional
observations never override an inconclusive supervising status. Individual
peak RSS is not aggregate descendant memory. No matched timing comparison
or global all-roots result is claimed.
