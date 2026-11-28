# Ramification supports of the retained degree-six map

Frozen before implementation or execution. The positive old-base rank goal
is still open at six. This experiment uses the actual retained differential
to select geometric supports; it does not repeat the completed composition
experiment or the earlier pair/six-symbol searches. [pcc:unverified]

Parent composition verification checks f^*omega_E=L(x,y) dx/F_y exactly.
Read L from the existing backmap, recheck the small polynomial identity and
global elliptic equation, and use the canonical section L to describe the
ramification divisor. The retained L has a nonzero y coefficient. Parameterize
its line, restrict the quartic, factor the resulting monic degree-four
polynomial completely with multiplicities, and verify irreducibility and
the full factor product. Check the intersection with the infinity line
separately; no omitted support is accepted.

For every distinct factor, set the existing extension-field modulus to that
factor. Lift a smooth affine local chart at precision 45, as in the retained
map verification. Check the line's local order equals its intersection
multiplicity. Evaluate the actual map by its Laurent numerator/denominator
orders and leading coefficients, including removable/base-locus and infinity
images. Compute the local map index using X-X(P) at finite Y(P)!=0, Y at
finite Y(P)=0, or X/Y at O. Require index=1+line order, the tame
Riemann--Hurwitz identity at each support point, and total weighted
ramification degree four.

Check fresh conjugate lifts against Frobenius where the factor degree exceeds
one. Retain complete extension coefficients, coordinate valuations, orbit
degrees and each rational elliptic trace. Sum every source conjugate,
including repeated image conjugates when the image field is smaller.
The weighted traces must sum to the reference kappa in both rational group
implementations. Also check the local map index at every retained rational
point over O from the old complete pool. A source coordinate chart being
unramified does not by itself establish the map index.

Test an involution only with the specified ramification axis. Restrict the
three homogeneous polar cubics to L, giving a 4x3 necessary linear system
for its center V. Rank three excludes such a center over every extension.
For a one-dimensional kernel, require L(V)!=0, normalize it, and check the
reflection I-2VL/L(V), its square, exact quartic invariance and actual-map
compatibility. If the kernel has larger dimension, report this symmetry
subproblem incomplete and freeze a refinement before claiming completion.
This is not a search for all automorphisms or all elliptic quotients.

Controls include a smooth Fermat quartic with its known axis reflection,
a non-coordinate conjugate of that fixture, and an odd-term perturbation
that passes the linear condition but fails exact reflection. Mutated L,
omitted factors, altered local index and altered finite branch image must
reject. All fixtures are separate from actual points and matrices.

Rebind the complete 685-image, 236-row retained matrix with all 460 old
auxiliaries active: actual expanded-pool/seed geometry, both rational group
checks and direct ranks 236/230/6. Match each rational branch trace to that
union up to sign, adding a fresh auxiliary only when needed. The principal
relation div(L/L_H)=Ram-H supplies one actual row. Resize inherited rows
with zeros and compute full/auxiliary/closed ranks directly; never discard
old private columns. A separate numerical old-unit fixture must detect
closed seven without entering actual data. Positive goal completion still
requires separate elliptic/elimination replay and actual cover/KM validity.

All limits retained: one allowed CPU, 1 GiB virtual address space, strict
C++17 build <=300s, each worker <=600s, dense helpers <=256 rows and
<=1024 columns, source-point pool <=3000 and accepted rows <=4096. Local
precision remains 45; factor degrees are <=4. Native C++ performs every
curve, field, polynomial, matrix and group computation. Python is confined
to file/JSON/hash bookkeeping and rendering. Freeze subjects, argv, exits,
costs and result hashes; preserve failures. Run locally; ignore Conductor
per explicit user instruction.

Sources checked: parent composition REPORT and frozen check.cpp:110--119;
parent projective REPORT and its extension/jet helpers; current full-image
and closed-residual evidence; the primary references already linked in
those reports. Canonical adjunction and tame Riemann--Hurwitz interpret
the exact data. KB tools and executable are unavailable as logged. No
novelty, scalar-recovery or speed claim. [pcc:unverified]
