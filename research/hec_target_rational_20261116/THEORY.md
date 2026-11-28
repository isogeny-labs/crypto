# Target-conditioned rational interpolation and the exact group-orbit boundary

Let C:y²=f(x), deg f=2g+1, g>=2. Let a full-degree valid Mumford target be D=(u,v), with gcd(u,v)=1; u may have repeated roots. Choose three distinct rational anchor abscissae xi away from u=0, with independently signed ordinates yi. Set

    B=b0+b1 x, C=c0+c1 x, A=v B+u C.

The four homogeneous coefficients (b0,b1,c0,c1) satisfy three linear anchor conditions

    (v(xi)-yi)(b0+b1 xi)+u(xi)(c0+c1 xi)=0.

A generic rank-three matrix therefore determines a projective pair (A,B). This is a standard Riemann–Roch linear system, not a new construction. It forces three anchor points rather than the single point of the pencil.

## Domain, divisibility and degree

Assume gcd(A,B)=1 and gcd(B,u)=1, and B(xi)!=0. Then F=A-yB vanishes on the target effective divisor with its full multiplicities: A-vB=uC and y-v vanishes to the u-multiplicities at nonbranch target support. It also vanishes on all three anchors. For T=product(x-xi), the norm A²-fB² is divisible by uT. With deg A<=g+1 and deg B<=1,

    deg(A²-fB²)<=2g+3,
    Q=(A²-fB²)/(uT), deg Q<=g.

gcd(B,Q)=1 follows from primitiveness. Thus E=(monic Q, A/B mod Q) is a valid reduced Mumford divisor, including degree drops. Its original-group identity is

    [D] + sum_i [Pi-infinity] + [E] = 0.

Repeated nonbranch target support is included. Branch-target support, canceled interpolation points, nonprimitive pairs and higher-nullity spaces are outside this first primitive gate and are recorded separately. This is not the unrestricted family after arbitrary primitive normalization.

## Exact equivalence, not extra relation information

The reduced representative of a Jacobian class is unique. Hence

    E = reduced( -D -P1-P2-P3 ).

Consequently changing the norm construction does not change the residual or its rational splitting for the SAME signed target and anchors. Any gain must arise from cheaper evaluation/batching or a demonstrably better sampling policy, not new independent information from these equations. The comparison must charge primitive checks, nullspace, inverses, residual splitting, catalogue, rank and source/target work. The earlier whole-solve boundary cannot be bypassed by comparing only Gaussian interpolation.

The target sign is fixed: global sign reversal sends D to -D as well. Therefore the old target-free 2^(3-1) sign fold is invalid here. All2^3anchor sign classes must be retained for a fixed D, unless a separate proven equivalence is supplied. The finite gate does not use such a fold.

The same uniqueness argument extends to any primitive target-conditioned norm construction whose residual is already reduced of degree<=g and whose input signed place sum is fixed. A different residual class requires a different signed input, a higher-degree unreduced residual, or a changed map. This does not exclude faster arithmetic or a new algorithm for selecting useful signed inputs. It excludes promoting an equivalent residual presentation alone into a mathematical relation-density gain.

## Sources checked and scope

Current sources read: ../hec_anchor_relations_20261114/rational.rs and RATIONAL-PROTOCOL.md (target-free degree bounds/primitive cancellation); ../hec_profile_strata_20261116/ROW-EQUIVALENCE.md (target-free row equivalence); ../hec_fibre_pencil_20261116/DESCENT-RESULTS.md (failed fixed-pencil guard). This target-conditioned extension uses the same classical divisor identity and reduced-representative uniqueness, rather than a novelty claim.

KB revision9141e386254340a9bdba3036f2bb706a,last refresh2026-11-14T21:44:48Z, ready/lexical-only snapshot predates current studies. Queries in this task: target/Riemann/decomposition/Nagao; results source:6b2208c18d3ecd6afd0494c0c44c1a36 (Nagao extension-field decomposition pointer), source:bc240a58264dfb95434f51aed26b863c (crypto:research/notes/ecc2k130/RESEARCH_ECC2K130_RR_SOLVER_PANEL.md, encoding can lose to direct enumeration), source:2ab33887849712ef97a2f42e2e74c4f1 (crypto-autoresearcher:ledger/proposals/IDEA-20260926-0d1d74.yaml, adjacent-field subspace-membership work). These are pointers/internal notes, not certificates for this construction. Current local evidence and explicit derivation take precedence. Prior primary Joux–Vitse PDF access failed and remains an evidence gap; no priority or absence-of-prior-art conclusion is inferred. Classical Cantor arithmetic is implemented in ../../crates/hec-native-ic/src/jac.rs. No Lean-backed statement or machine-checked group certificate is claimed.
