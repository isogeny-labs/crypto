# Twist7 kernels, rational isogenies and relation transport — v8

This is a new bounded correctness experiment, not completion of the original
full campaigns or a performance experiment. Fixed public source:
`icv1-fp10-t5-192cb216`, F827, y²=x³+x+15, order823, G=(2,5).
Standard twist d=2: Y²=X³+4X+120, order833. Q=37G is a public fixture;
the relation solver receives only Q, G, relation coefficients and offsets.

Enumerate all rational twist points in lexicographic (x,y) order. Select the
first nonzero7-torsion point and the first outside its span. Enumerate the
eight lines T1+c*T2 (c=0..6), then T2. Embed into E(F827²) by
(X,Y) -> (X/2, Y*omega/4), omega²=2. Check all49 torsion points, anti-Frobenius,
basis independence and all49² sums. Do not replace missing lines or outcomes.

Each line supplies three distinct paired x-roots. Construct the normalized
degree7 Vélu map (N/H², y*M/H³), M=N'H-2NH'. Check gcd(N,H)=1,
degrees7/3, squarefree H, the exact cleared curve identity, every kernel
pole and infinity. Evaluate every source rational point; recount every image
curve and verify all823 images are distinct and cover the destination.
Check generator-addition homomorphism on all points.

Map all49 source7-torsion points: the resulting seven-point group defines
the normalized reverse Vélu map. Its codomain must be
y²=x³+7⁴*x+7⁶*15. Rescale by (x/7²,y/7³). Verify both dual compositions
equal [7] as exact rational functions, and on every rational point of both
curves. An independent rational-function group law computes [7]. Recovery
uses588*dual, since7*588=1+5*823. Record exact model and ordered map identities.

Only after map certificates pass, build two target-blind bases of eight
oriented source points. Entry0 is G. The ordinary base fills from ascending
source x with least positive y. The twist base finds the lexicographically
first globally maximal-order twist point, scans its nonidentity progression,
shifts X/2 by1 and keeps source-valid distinct x until filled; output is G
then ascending other coordinates. No scalar logs construct either base.
Each destination base consists of actual mapped points, preserving y signs.

For each of nine models (source plus all8 quotients), two bases, arities2,3,
and offsets j=0..15, collect **all** signed witnesses of
sum(s_i*P_i)=Q+jG. First indices i<=j; arity3's third index is free;
all2^arity sign masks are included, including repeated-index witnesses.
There are64 source and512 destination query cases. Record negative and empty
cases. Require exact index/mask witness equality with the same source base.
Evaluate F3 for arity2 and chained F3 for arity3; explicitly count identity
first-pair exceptions. Build the mod823 relation system
sum(c_i*u_i)-k=j, with anchor u_0=1. Gaussian elimination gets no point logs
or37 input. Record rank, consistency and recovery or rank deficiency for all
18 systems; verify any recovered scalar against its public target. These
16 offsets are relation-generation queries for one Q, not independent DLPs
or the original E28 heldout target labels.

Independent PARI verifies the group cardinalities and twist structure,
torsion basis, all8 `ellisogeny` maps and reverse maps, generic dual
composition, all rational source/destination images and scalar recovery,
and independently enumerates every signed relation set and matrix rank.
Strict admission requires process exit0, no unexpected stderr, exact final
sentinel, and every generated assertion. Corrupt-coefficient and flipped-sign
controls must be detected. Native audit recomputes the complete output.

Bounds: one native producer, one independent GP execution, each <=300s and
4GiB virtual memory, one pinned CPU, fresh output paths. Development tests
use different F11/F101 fixtures. Use the previously documented local fallback:
CodeBuild monitor and ECR readiness were denied and a custom verified EKS
image is unavailable. No paid provisioning or permission change. L0 only;
wall times are operational receipts, not admitted algorithm costs. Record
failures without changing this panel. Freeze source/binary/protocol, update
resume and push Setec checkpoint gate before main data. Preserve prior waves.

Success establishes an executable twist-derived relation-preserving route.
It does not establish a novel method, more same-base relations, a cheaper
solver, a prime-field asymptotic improvement or a NIST result. Charged
representation selection remains a separate future experiment.
