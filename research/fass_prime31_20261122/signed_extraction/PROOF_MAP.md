# Signed-extraction proof map

Tracking: [crypto PR #4976](https://github.com/isogeny-labs/crypto/pull/4976).
Written source: [THEORY.md](THEORY.md). All statements below retain
unverified repository PCC standing. [pcc:unverified]

| Statement | Exact hypotheses and written argument | Formal declaration / status |
| --- | --- | --- |
| Actual cyclic subgroup from a complete inventory | Rational on-curve generator, exact repeated-addition inventory starting at O, distinct entries before the declared first return to O; THEORY section 1. No inference from an annihilator test alone. | No Lean declaration; written group-law argument |
| Complete signed extraction, Proposition 1 | Prime p>3, nonsingular short-Weierstrass E, split squarefree selector, finite sign-closed base in an actual subgroup, repetitions allowed, complete and input-bound affine support and witness roots, all rational lifts, exact group replay; target either finite or O. Split by P1+P2=O. | No Lean declaration; no full implementation refinement proof |
| Prefix-identity chart, equation (8) | O excluded from the base; finite Q contributes exactly (P,−P,Q) for P in the base iff Q is in the base; Q=O contributes none. Includes nonidentity two-torsion P=−P. | Written proof; finite native cases are controls only |
| Identity-target chart, equation (4) | Q=O and nonzero finite base summands; finite prefix R=−P3 has x in the actual base support W. The right modulus is F_B under both left-cover policies. | Written proof; not substitution of a finite coordinate for O |
| Complete EMPTY corollary | All certificate/input/root/sign/chart obligations completed, with no cap or error; complete output set empty. An empty affine grid alone is insufficient when Q belongs to the base. | Written consequence of Proposition 1; formal and implementation proofs open |
| No zero witness polynomial, equation (9) | p>3 and nonsingular E; a≠t gives a nonzero quadratic coefficient, a=t with f(a)≠0 gives a nonzero linear coefficient, otherwise constant f'(a)^2 is nonzero. | Written coefficient calculation; covers two-torsion targets and intermediate points |
| Output bound, equation (10) | Same hypotheses; at most two roots per left witness polynomial, at most two distinct lifts per coordinate, unique P3 determined by a pair of signed points; identity branch counted separately. | Written loose upper bound; no algorithmic runtime or memory bound |
| Coordinate mask is not a cancellation classifier | Explicit F97 order-four d1 and order-five d96 controls, actual group equations in THEORY section 6. Retain all collision/sign branches. | Written fixture arithmetic; native finite controls tracked in evidence |
| Exact cancellation baseline, equation (14) | Finite sign-closed base of size N, repetitions allowed, finite Q in the base; inclusion-exclusion over the three possible inverse slot pairs gives 3N−3+1[Q=−Q], including exactly N prefix-identity triples. | Written group-law counting argument; predicts only cancellation-containing triples, not total output |
| Exact integer signed row, equation (13) | Canonical ordinate representatives, one sign for y=0, actual point occurrences and signed coefficients; optional target subtraction specified separately. | Written group-law identity; no relation-rank or useful-row inference |
| Quotient triangular decomposition | Inherited monic split squarefree F,G and all divisibility/Bezout/partition identities from quotient_query/THEORY.md. | Inherited written status; frozen predecessor remains unchanged |

Capability check: no `lean` or `lake` executable was found in the session.
No placeholder file, admitted theorem, custom axiom, native arithmetic
check or weaker statement is presented as formalization. A future formal
development must instantiate the actual elliptic-curve group and point
coordinate structures, the finite selector/root contracts and the quotient
certificate theorem, then establish the set equality including both identity
charts. It must build the modules and retain the toolchain, dependency
revision and transitive-axiom audit under the repository Lean instance gate.

Finite native source qualification, same-host replay, an independent saved
output reader and rejection controls address a different obligation:
specific implementations and recorded inputs. Their claims and receipts do
not replace the universal proofs listed here.
