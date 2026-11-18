# Logarithmic conic descent amendment

The original gate333061c10519/checkpoint222341 ran once and passed with7of20
pairs, all independent group identities. Preserve RUN.txt and RUN.stderr.
This separate gate tests logarithm recovery, not timing. It reuses the same
registered quartic, two recovered divisors and ten anchor pairs each.

Point-log kernel is exported verbatim from the earlier mappedic RUN.txt's
LOG_KERNEL line only: no target scalar/class log or full earlier output.
Reconstruct the public point-list ordering by its existing deterministic
rational_points procedure as setup; this is not descent root finding and is
charged in any later cold run. For each conic passing EXACTLY the original
conditions, use point-log lookups; every accepted pair must yield the same
source-verified scalar. No pair selection, extra anchors or retuned residuals.
Public signs (+,-) are the already verified RECOVER.txt orientations; this
is a joined correctness gate reusing sign/setup, not an independent inverse
replay or a cold solve. Known G calibration remains setup.

Let L(P)=log(P-O), K0=log(K-4O), D0=log(Dinf-3O).
The checked centering gives beta=D0-K0/2 in the odd subgroup. Thus the log of
X=(E-Dinf)+beta is log(E-3O)-K0/2. Since E+A+B+R~2K,
log X=3K0/2-L(A)-L(B)-sum_R L(P).
There is no beta or Dinf scalar multiplication in this logarithmic formula.
For the already oriented sign s, log B=s*log X-log G.
G=1005372*2*((43,125)-(101,43)), the existing public calibration.
Recover k=log B/log G and verify [k]2*(0,44)=2*(3,21) on the source curve.
No expected k is supplied. Require nonzero logG and identical k on all seven
accepted pairs; retain all exclusions. First scientific execution only after
separate commit, binary hash, S3 checkpoint and gate. No10xclaim.
