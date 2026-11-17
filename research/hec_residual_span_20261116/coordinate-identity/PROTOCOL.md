# Exact standard Kummer coordinate identity

Prospective assertions [pcc:unverified]. Fixed candidate coefficients
[209,127,378,47,539,495] and recorded node/conic directions unchanged.
Use Corte-RealSantos/Flynn section2 Eq2.2-2.4 generalKummer forms, read
directly at https://people.maths.ox.ac.uk/flynn/arts/art43.pdf . Implement
full quintic-curve standard quartic; no point-count identification alone.

Recover conic parameterization matrix M from the recorded quadratic map;
standard tangent-conic coordinates are (v²,2uv,u²). Require M invertible,
old conic=alpha*(k2²-4k1k3), alpha nonzero. Candidate polynomial equals
7times polar cubic on the conic, so standard cubic equals -28times old
cubic modulo tangentconic. Solve h=lambda*t+L(k1,k2,k3), lambda=-28alpha,
L=(-28Bold-Bstandard)/(2Astandard). Require exact linear quotient and
full quartic equality, then derive nonsingular4x4matrix GL mapping old
image coordinates directly to recovered curve standardKummer coordinates.
Check full polynomial identity again in old coordinates, output GL and
four transformed quintic vectors. No numerical fitting or point filtering.

This identifies the quartic model algebraically, not globalbasepoint
freeness, degree25, Jacobianlift or targetscalarrecovery. Producer selfcheck,
not independent verification/speed measurement. Freeze all imports/native/
previous map/curve certificates/source/protocol/binary and push Git before
one120CPU-second invocation. Retain failures, no reruns/S3/cloud. Next
explicit Mumford lift and original rational-subgroup homomorphism checks.
