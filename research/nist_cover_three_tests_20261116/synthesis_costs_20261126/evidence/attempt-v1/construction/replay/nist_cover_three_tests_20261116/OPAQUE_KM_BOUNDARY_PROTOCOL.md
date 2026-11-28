# Opaque decoder at special and repeated effective divisors

Frozen before data,2026-11-17. Separate qualifier;opaque_km.cpp and
OPAQUE_KM.md/PDF stay byte-identical under their existing PCC pins.
Sources:the same Khuri-Makdisi RR/division statements and elementary RR for
degree3 on a genus3 plane quartic. Native local development only,no paid
campaign or timing. One600second/1GiB call,fresh source/input/binary manifests.

The completed panel has RR dimension1 in all cases. A valid degree3 divisor
can instead have dimension2. RR gives h0(E)=1+h0(K-E),with deg(K-E)=1;
a degree1 bundle on a positive-genus curve has at most one section,otherwise
it would give a degree1 map to the projective line. Thus the only possible
dimensions are1 and2. A plane-quartic hyperplane is canonical.

Fixed additional opaque inputs,constructed as actual target classes E-Eref:

1. E is the first three points of the first complete split vertical fiber.
   It equals a canonical hyperplane minus its fourth point,so h0(E)=2.
   Require the extracted RR space dimension2. The extractor may choose a
   different effective representative;require class re-encoding and the dual
   of the original three-point divisor,not equality with the original support.
2. E=3Q,where Q is the first fixed reference point. Require its tangent to
   have exact intersection order2,establishing Q is not a flex and h0(3Q)=1.
   Require the decoded cycle to retain multiplicity3 at that exact point.
3. E=2Q+Q',with Q' the second fixed reference point. Require the line Q,Q'
   is not tangent at Q,establishing h0(E)=1. Require the exact decoded
   multiplicities2 and1. Do not replace either reference if a premise fails.

Construct each degree3 input cycle directly from the specified rational points
and multiplicities. Choose z=x+mu*y,mu=0..8,checking separation and transverse
derivative. Interpolate y on the distinct support and Hensel-lift to the full
support,checking the original quartic. Encode with the completed CM adapter,
then pass only the resulting KM class to the unchanged opaque decoder.

Compare every decoded/re-encoded class. Evaluate its dual by the existing
line-moving closed transpose. The expected source answer is the sum,with
multiplicities,of the direct transpose source classes at the original rational
points,minus the three reference classes. Require degree9 raw fibers for the
pole-fiber points;the reference degree8 classes have prior direct/moving
agreement. Retain the explicit cycles,input spaces,RR dimensions,full
intermediate extraction,decoded multiplicity checks and source answers.

Reject changed y and a changed source answer. Failure of an expected premise,
RR dimension,multiplicity or equality remains a failure with its original
source and input;no selected-point search or relaxed criterion.
