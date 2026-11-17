# Completing the P-192 divisor-map interfaces

Frozen 2026-11-17 before arithmetic. User-selected scope: the existing P-192
genus3 construction, its maps and exceptional inputs. Scalar recovery is not
this unit's acceptance condition. Previous IC development was cancelled before
arithmetic and its snapshots are archived separately.

## Sources and standing

Read generic_bridge.cpp/GENERIC_BRIDGE_PROTOCOL.md, actual_kernel.cpp,
closed_fibers.cpp, cover_replay.cpp, quartic_km_v3.cpp and
POLARIZED_QUOTIENT.md. Primary sources are Tian section4.3
<https://arxiv.org/abs/2007.03172> and Milne's Abelian Varieties, II6.1/6.3,
<https://www.jmilne.org/math/CourseNotes/AV.pdf>. Standard moving-divisor,
finite-algebra norm/trace, Newton identities and Hensel arguments are written
in the new report, not claimed as new algorithms. KB unavailable: MCP absent
and scripts/research-kb has no executable. Lean/Cargo/Conductor unavailable;
no compiled formalization supplied. All new outcomes remain unsigned producer
assertions until an independent signed verifier covers the actual claim.

Local native functional qualifiers are development work under isolab-eks.
No paid/performance campaign: remaining budget unknown. Freeze input/hash
manifests; preserve unsuccessful attempts and fresh output directories.

## Forward map, including source poles and nonseparating target coordinates

Input is a checked reduced rational Mumford divisor (u,v) of degree d<=3 on
the fixed odd-degree model. Zero maps to zero explicitly. Form Dden as the
polynomial lcm of every retained U,V coefficient denominator.

If needed, use g=v+a*u and e=(h-g^2)/u, monic, with y=-g modulo e.
For d>0, deg(e)=7-d. The source function (Y+g)/u has divisor
E-D-(7-2d)*infinity; hence E-(7-d)*infinity represents the original class.
Try integer a=0..4*deg(Dden), retaining every rejection, until gcd(e,Dden)=1.
At a root of Dden, e's value is a nonzero polynomial of degree<=2 in a;
at a branch root of u it is a nonzero constant by smoothness. Thus this finite
schedule contains an admissible a. This is an exact representative change,
not omission of exceptional support. A forced-moving mode qualifies the
fallback even for ordinary inputs.

Evaluate the cubic correspondence in A=Fp[S]/e, producing the rank3 free
A-algebra B=A[T]/U. For regular direct input use e=u. Retain finite-algebra
dimension n=3*deg(e). The target cycle is reconstructed from multiplication
traces, with coordinate z=x+l*y, l=0..n*(n-1)/2+n. Compute its characteristic
polynomial f by Newton identities and trace numerator b for y. Let g=gcd(f,f'),
s=f/g. Recover y0=(b/g)/(f'/g) modulo s. Require
(target_y-y0(target_z))^n=0 in B, the quartic equation modulo s, and a unit
transverse derivative f_y-l*f_x. Rejected l values are retained. Hensel-lift
y0 to f, then x=z-l*y. Check the full quartic equation and characteristic
identity in B. This constructs the effective pushforward cycle, including
repeated support; it does not require B to have a full target-x power basis.
The finite schedule avoids at most n*(n-1)/2 point collisions and n tangent
directions. Characteristic p exceeds n and the bound.

Output the closed cycle (f(z),x(z),y(z)) minus deg(e)*Eref; this is a signed
divisor representation, not necessarily the old degree12 KM representation.
Retain original and moved source supports and the exact principal identity.
Check zero, the second branch class, public pullG/pullQ, and the existing
generic P,-P,2P,P+pullG fixtures. Compare direct regular output with the pinned
GB image where z=x. Execute forced moving on the same nonzero inputs.

Representation controls use three actual quartic points on one retained
vertical fiber (old x power-basis rank1) and a length3 cycle at one actual
smooth rational point. Reject an altered y, changed source support and
changed principal identity. Keep constructor, checker and claimed scope
distinct; this is not independent verification.

## Transpose map

The reverse correspondence has projection degree9, not3; the dual is its
degree-zero divisor action. Substitute actual target points into U and V,
recover source support by exact elimination/saturation and reduce with the
unchanged source Cantor arithmetic. Retain all boundary fibers and their
coverage rather than silently discarding them. Check phi(psi(Q-Q0))=3(Q-Q0)
in the actual target Jacobian and source incidence for every admitted fiber.
Closed-target and projective-boundary interfaces remain open until their
own execution covers them; a rational-point test does not complete those APIs.

## Limits and deliverables

Each native invocation <=600seconds,1GiB address space, deterministic fixed
inputs. Immutable copied source/header/protocol, pre-call source/binary/input
hashes, logs/exits and post-call hashes. Source changes require a new attempt.
PCC registration precedes numerical citation. Deliver source-linked report,
diagram and inspected PDF, update the existing problem record, commit and
publish a regular PR. Do not call all map interfaces complete if reverse or
projective cases remain unsupported.
