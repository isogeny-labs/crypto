# Instance gate before Lean edits

Goal: retain projective numerator/denominator residues for the existing auxiliary
Vélu tower, then omit redundant denominator-inverse witnesses without admitting
pole models. This is an encoding result, not a sufficient-degree or ECDLP cost
claim. KB unavailable (wrapper exit127 in previous component); source review
uses Mathlib Coprime Basic/Lemmas, the shared norm/retained laws, PKM notes and
src/cryptanalysis/isogeny_eval/small_degree.rs. [pcc:unverified]

1. **Structure above.** `CommRing R`, existing `IsCoprime A B` (Bezout/unit-ideal
   predicate), `IsUnit tau`, ordinary ring powers/addition/multiplication.
   For the fixed instance use `AdjoinRoot U` over `ZMod 19`; initial pair is
   `(AdjoinRoot.root U,1)`. No custom polynomial syntax is the primary object.
2. **Theorem used as stated.** Mathlib `IsCoprime.mul_left/mul_right`,
   `pow_left`, additive-multiple invariance, `of_isCoprime_of_dvd_left`,
   `isUnit_of_dvd`. The missing reusable composition is preservation by
   `N=A²-xi*A*B+tau*B²`, `D=A*B-xi*B²` when tau is a unit. If no such law
   appears in the narrow existing-library search, add it in crypto-lean first.
   Terminal denominator invertibility is an application of existing
   `IsCoprime.isUnit_of_dvd`, not a new copied generic law in this study.
3. **Object as instance.** The actual semantic residue ring of a monic support
   U, with the public three-step F19 Vélu constants from A:y²=x³+2x+2,
   order8 point T=(16,11) and coset point G=(12,5). Proposed steps are
   (xi,tau)=(14,1),(16,5),(17,3); terminal c=15. Native correctness checks must
   establish these constants, fiber and target binding before model claims.
4. **Assembly probe.** First type-check the generic coprimality chain using
   the named existing laws, then its F19 quotient-ring instance. The probe
   is not yet built at map creation; retain actual build/failed outputs.
   Report no formal endpoint until its actual build/axiom/proof gates pass.
5. **Missing components only.** Shared projective-step composition law;
   instance scalar-unit facts and exact tower constants; native residue/source
   binding. Native parser/program correctness, auxiliary point counting and
   the growing-family degree/yield theorem are not discharged by this law.
6. **Inheritance.** Addition/multiplication/powers: CommRing instance; quotient
   semantics: existing AdjoinRoot construction; initial coprimality:
   `isCoprime_one_right`; scalar units: algebraMap of units in ZMod19; stage
   coprimality: proposed shared composition law; terminal denominator unit:
   existing divisibility/coprimality theorem. Native source binding remains open.

Proof sketch before code: set q=A-xi*B. Rewrite N as
`A²+B*(-xi*A+tau*B)`, then apply coprimality of A²,B plus additive-multiple
invariance to obtain coprimality of N,B. Next inherit coprimality of B,q from
A,B; power B and multiply by the unit tau. Rewrite N as `tau*B²+q*A`, then
apply additive-multiple invariance to obtain coprimality of N,q. Multiply the
two right coordinates to obtain coprimality of N,B*q; rewrite B*q=D. These
are polynomial ring equalities followed by the named laws, not an unfolded
reproof of Bezout. At a finite terminal N=cD, D divides N, so the existing
coprimality law makes D a unit. No inverse witness is assumed as a hypothesis.
[pcc:unverified]

| Statement | Kind | Owner | Structure / hypotheses |
|---|---|---|---|
| Vélu projective step preserves coprimality | Reusable law | crypto-lean | CommRing; IsCoprime A B; IsUnit tau |
| Three-step F19 quotient pair | Instance obligation | study | AdjoinRoot U; actual public constants |
| Finite terminal has invertible denominator | Endpoint | study | Above instance plus N=15D |

No raised Lean limits, custom axioms, native_decide or copied generic laws.
The shared native coefficient machinery is reused; any new formal report names
the exact pinned dependencies, actual build and axiom receipts.
