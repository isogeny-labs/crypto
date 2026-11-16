# CT22 — Frobenius-equivariant 4-torsion quotient and invariant split Richelot kernel

**State (2026-11-11): [pcc:unverified].** The model is a finite, explicit
symplectic-module calculation, not the real degree-seven Kohel action on
elliptic 4-torsion and not a balanced-divisor Jacobian isogeny. No speedup,
ECDLP advantage, or completed Richelot chain is asserted.

## Sources and distinct scope

Reviewed parent crypto #3877, CT20 #3920 and CT20b #3921, CT21 #3922,
cryptanalysis #1483 and the current queue and 2026-11-11 session log.
Earlier CT20b enumerated 15 *branch pairs* on J[2]; CT22 independently
enumerates the **64 product 4-torsion lifts, the quotient symplectic form,
all 15 Lagrangian subgroups and their Frobenius action**, and proves a
specific invariant of every compatible odd-degree continuation.

Prior art: Howe--Leprévost--Poonen, "Large torsion subgroups of split
Jacobians of curves of genus two or three", Section 3.2,
https://math.mit.edu/~poonen/papers/large.pdf ; Bruin--Doerksen,
"The arithmetic of genus two curves with (4,4)-split Jacobians",
https://arxiv.org/abs/0902.3480 ; Magma's general Galois-stable Richelot
kernel and split-codomain documentation,
https://magma.maths.usyd.edu.au/magma/handbook/text/1615 .
All constructions and the polarization formula below are standard
consequences of HLP gluing; no new theorem of algebraic geometry is claimed.
Research-KB MCP and executable wrapper unavailable in this runtime;
encrypted atlas not opened. In particular no global novelty claim is made.

## The exact finite group-module model

Let A=E_1 x E_2 and let G be the graph of the F_2-linear gluing
isomorphism E_1[2] -> E_2[2]. After selecting compatible symplectic
bases, identify A[4] with V=(Z/4)^4 and set

    G={(0,0,0,0), (2,0,2,0), (0,2,0,2), (2,2,2,2)}.

For the quotient P:A -> J=A/G, the geometric two-torsion of J is

    J[2] = [2]^{-1}(G)/G.

Thus #A[4]=256, #[2]^{-1}(G)=64, and #J[2]=16.
The inverse-image classes are *not* the 16 rational points on
J(F_101); these are geometric 4-torsion cosets.

The product symplectic pairing on V is

    omega(v,w)=v0*w1-v1*w0+v2*w3-v3*w2 (mod 4).

For v,w in [2]^{-1}(G), omega(v,w) is even, and the induced alternating
F_2 form on J[2] is omega/2 (mod 2), independent of representatives
modulo G. The Rust test checks that independence across all 16 x 16
quotient classes and all 4 x 4 shifts by G, and tests nondegeneracy
(each nonzero class has eight orthogonal classes).

Here p=101, #E_1(F_p)=109, #E_2(F_p)=113, giving Frobenius
traces -7,-11. Both traces are 1 (mod 4) and p is 1 (mod 4).
A common compatible mod-4 Frobenius chart is

    F=[[0,3],[1,1]] on each elliptic 2-dimensional block.

It satisfies F^2-F+I=0 (mod 4), so **F^3=-I and F^6=I** on
E[4]. For every primitive order-four point R, its Frobenius orbit
has exact length six, while its x-coordinate (which identifies R
with -R) has exact Frobenius orbit length three. Consequently,
**all primitive 4-torsion x-coordinates lie in F_(101^3), and the
full points first lie in F_(101^6)**. This is a useful concrete
field-degree bound for the next actual Kohel-kernel lift calculation:
evaluate the rational x-isogeny in the cubic field and track its
y-sign in the sextic field. It follows from the already-known
Frobenius polynomial, not from constructing real E[4] points here.
The Rust test checks F^3=-I and the exact point/x periods on all
twelve primitive torsion classes of one elliptic 4-torsion block.
Modulo 2 it cyclically permutes
the three nonzero 2-torsion points. The test confirms that F
preserves G, the induced pairing and group operation. It finds
15 total maximal isotropic 2-planes in J[2], **exactly three**
Frobenius-stable, and no Frobenius-fixed *nonzero* geometric
2-torsion point. This agrees with CT21's three stable branch
pairings for the source and both supported target charts.

## Stronger conditional invariant: the degenerate Richelot plane is fixed

Let N:J -> A denote the dual norm, with NP=[2].
On geometric J[2], the opposite-branch-point Richelot plane is

    K_opp = ker(N|J[2]) = A[2]/G,

a subgroup of order 4. In the concrete module it comprises precisely
the four classes represented by v with [2]v=0. It is itself
Frobenius-stable. Of the three stable isotropic planes, their
intersection sizes with K_opp are **4, 1, 1**. In the CT20 even-sextic
charts, K_opp is the determinant-zero ("split") Richelot kernel; the
two other stable planes are determinant-nonzero ("nonsplit") kernels.

Let alpha=phi_1 x phi_2:A_0 -> A_1 be the *actual* product of odd-degree
elliptic isogenies, and suppose the HLP diagrams commute:

    Phi P_0 = P_1 alpha,       N_1 Phi = alpha N_0,

with G_1=alpha(G_0), and Phi a principally polarized (7,7)
isogeny. Since alpha is invertible on 2-torsion,

    Phi(K_opp,0)=K_opp,1.

Indeed, if N_0(D)=0, then N_1(Phi D)=0, so inclusion holds;
both groups have order four and Phi is injective on 2-torsion.
The polarization multiplier 7 reduces to 1 modulo 2, so Phi
also preserves the F_2 symplectic form. Since Phi commutes with
Frobenius it permutes the **other two** rational maximal isotropic
Richelot planes. This is a conditional implication about actual
HLP diagrams, *not* a computation of Phi in balanced Mumford
coordinates.

**Consequence for CT21:** a compatible odd-degree hop cannot
transport the source split/degenerate kernel into either smooth
Richelot kernel. The unordered pair of nondegenerate successors
is preserved. CT21 already established that the two nondegenerate
successor curve models in each target chart are related by t -> -t.
Therefore *only when the destination curve-isomorphism class is
being classified*, the uncomputed exchange of the two labels is
immaterial. For a specific homomorphism, isogeny composition,
kernel certificate, and full transport or cost measurement, **the
actual 4-torsion correspondence and labeled kernel remain required**.

## Bounded regression and deliberately wrong gluing

Run from repository root, with any stable rustc (no dependencies):

    rustc --edition=2021 --test \
      research/chained_jacobian_transfers_20261110/ct22_mod4_gluing.rs \
      -o /tmp/ct22_mod4_gluing
    /tmp/ct22_mod4_gluing --nocapture

The test enumerates all 256 elements of V and all 64 lifts; proves
well-defined quotient pairings, nondegeneracy, 15/3 plane census,
4/1/1 intersections and the absence of rational nonzero 2-torsion.
It exercises a model **B=I+F** on each elliptic 4-torsion block.
Its determinant is 3 (mod 4), the same as degree 7 (mod 4).
This is just one illustrative Frobenius-equivariant mod-4
symplectic similitude, **not** a replay of any of CT19's four
Kohel isogenies. A deliberately rotated Frobenius-stable target
graph intersects the correct graph only at zero (1 of 4 elements)
and is rejected as an invalid target gluing.

A producing-session independent Python prototype ran a 256-element
census and printed (256,4,64,16), 15 stable-plane candidates,
three Frobenius-stable planes, intersections [1,1,4], and passed
the model alpha control. This was a self-check [pcc:unverified].
The committed Rust test has its own scoped GitHub Actions workflow;
report its exact-head outcome separately when available.
A successful operational CI run would still not be a signed PCC verdict.

## What remains open

CT22 does not identify the actual 4-torsion matrix of the four Kohel
degree-seven maps, the target branch-point labeling, or the exchange
versus identity action on the *two* nonsplit Richelot kernels.
Compute those in an extension field with actual E[4] coordinates
and verify Frobenius and Weil pairing; then implement P_1/N_1 and
Phi/PhiHat on balanced divisor classes, check both 12,317-class
dual-[7] compositions and all 109 source DLP scalar transports.
Benchmark only with complete costs and direct elliptic controls.
Do not repair/merge the frozen root PCC ledger by rewriting it;
the ledger/archive/S3 budget and independent-verifier gates persist.


## CT22b: actual finite-field elliptic 4-torsion, not just the abstract module

**[pcc:unverified]** A separate standard-library field implementation now
evaluates the **four concrete CT19 degree-seven Kohel maps** on the entire
geometric 4-torsion, including their normalized duals and both CT20
supported product-gluing cases. Source:
`ct22_actual_e4.py`; command:

```sh
python3 -S research/chained_jacobian_transfers_20261110/ct22_actual_e4.py
python3 -m py_compile research/chained_jacobian_transfers_20261110/ct22_actual_e4.py
```

The field is (ℝ) replaced by (ᵓ_{101^3}):
choose (θ^3+6θ^2+13θ+56=0), and represent the sextic field
as (ᵓ_{101^3}(ζ)) with (ζ^2=2). Because 2 is a nonsquare
in the cubic field, primitive order-four points have y-coordinate
(ζ u), (uinᵓ_{101^3}). No generic degree-six
irreducible-polynomial rootfinder is needed.

For a short-Weierstrass two-torsion point (T=(r,0)), the duplication
equation (x([2]R)=r) is the *square* of the quadratic
((X^2-2rX-a-2r^2)). Its two x roots are therefore

[
oxed{x(R)=r ± sqrt{3r^2+a}.}
]

This is an exact algebraic identity from the ordinary duplication
formula, not a guessed quartic factorization. Here the three required square roots
per elliptic factor are found in (ᵓ_{101^3}). We then solve
(u^2=(x^3+ax+b)/2) in the same cubic field and represent the actual
points ((x,ζ u)). The 16 points per factor consist of the
identity, three nonzero 2-torsion points and 12 primitive 4-torsion
points.

The oracle derives the full Kohel x-map from the already committed
monic cubic kernel (h):

[
phi_x(X)=7X+2c_2 -(6X^2+2a)h'/h
+4(X^3+aX+b)((h')^2-hh'')/h^2,
]

and evaluates (y' = y,phi_x'(X)), rather than applying the
two-torsion-only simplification to general points. It separately checks
the target curve equation, all (16^2=256) ordered group-law pairs
per map, full image bijections, Frobenius commutation and
(hatphiphi=phihatphi=[7]) for all 16 source/target points.
The four fixed kernel/target/dual rows come from CT19
(isogeny-labs/crypto #3919 and cryptanalysis #1483); no producer
kernel-search heuristic is imported.

For CT20 B+A and B+B, it forms **the actual 64 product E[4] lifts** of
the four-element 2-gluing graph inside the full 256-element product
group, transports every lift under the paired Kohel maps, and
compares all 16 quotient (J[2]=[2]^{-1}G/G) classes and their full
(16^2) addition table. It checks that the norm-zero 4-element
kernel maps to the target norm-zero kernel and rejects a rotated
target matching (intersection only the identity).

The *local producing* Python run reported all four map/dual/Frobenius
checks and both 64-lift quotient tests passing. This remains
[pcc:unverified] until exact-head CI replay and independently signed
verification. Importantly, the quotient uses actual elliptic
torsion and Kohel maps **but still is not a balanced Mumford-divisor
evaluator for the genus-two Jacobian**. The isogeny action on the
three geometric Richelot pairings has not yet been matched to the
actual target sextic's six named branch points. The next gate is
the explicit branch-pair-to-four-torsion lifting correspondence,
followed by the complete rational-divisor map and solving costs.
