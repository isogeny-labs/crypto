# Target encoding control, after the first native output

The original arbitrary-lift control has produced a kernel witness. This
amendment does not change its selection, source or frozen raw output.

The informed successor is selective extraction of the component label of a
torsion branch, rather than recovery of all global coordinates. Before trying
that selector, test the proposed target encoding itself.

Use the classical Tate period q=2^r with r in {19,83,127}. Its points with
Tate parameters u=2^k, 1≤k<r, are nonzero r-torsion points with component
label k. These are target-side auditor controls, not images computed from
unknown source scalars. This Tate curve is not asserted to be the exact
global CRT curve from the first protocol.

For absolute precision N in {3,5,8}, q is zero modulo 2^N. Truncating the
convergent Tate series gives, with v=2^(r−k),

    X=u/(1−u)^2 + v/(1−v)^2,
    Y=u^2/(1−u)^3 − v/(1−v)^3              modulo 2^N.

Every omitted series term has valuation at least r>N. Each denominator is
odd. The Tate coefficients a4,a6 are also zero at this precision. Check each
recorded pair against Y²+XY=X³ modulo 2^N. Compare all coordinate encodings,
retaining collisions, and count labels with N≤k≤r−N.

Prediction: those r−2N+1 distinct component labels all produce the pair
(0,0). If observed, fixed low absolute precision is not a faithful target
encoding. This does not exclude an encoding carrying valuations separately,
a targeted label extractor, or any other computational reduction.

Use a separate small native source and output file; never replace native-01.
No timings, cloud jobs, speed claim or new criterion for goal completion.
