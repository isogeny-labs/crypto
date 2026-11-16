# R53 proof-to-claim map

`transfer-lean53-02` compiles and audits successfully under Lean4.34.1 and
mathlibd13f23b723b8a846827a245b89c10fc7d3f11612. The audit reports exactly
three theorems,zero failing,zero trusted. `alpha_product` and `alpha_isUnit`
use propext;`discriminant_change` also uses Classical.choice and Quot.sound.
No native_decide,sorry or additional axiom is used. This is a self-reviewed
mapping of propositions to evidence,not an independent scientific review.

| Mathematical claim | Lean statement | Native / external obligations |
|---|---|---|
| Full Gram change implies determinant change | `Transfer53.discriminant_change` | Actual M,G,P arrays;all1024entry products;trace semantics;normal-RR source identification. Native `discrim53-02` supplies the arrays/products. |
| Torsion membership plus coprime-kernel identity gives the alpha inverse numerator in the source quotient | `Transfer53.alpha_product` | Actual G,N,psi3,psi7,alpha and x,y,z,U,V,Delta;the two full products;source equations vanish. `alphaunit53-01` rechecks those actual products. |
| Alpha is a unit wherever the source equations vanish and Delta is a unit | `Transfer53.alpha_isUnit` | Same actual data and a unit Delta. Applies in an arbitrary commutative quotient ring;it does not assume that the rank64algebra is a field. Coefficient-denominator specializations remain external. |

The native trace checker builds all31universal companion traces,all32normal
generator traces and the complete Gram pairings. The Lean determinant lemma
does not prove these arrays implement algebraic trace. The retained complete
normal RR algebra and source multiplication bridge are mathematical inputs
to that interpretation,documented in R50/R51/R52.

The native marked Tate stage has all4unit products,a full marked-point product
and a full curve-change product. No new Lean theorem claims to formalize the
entire rational map or its extension to a proper curve. The all5target-equation
packet stage is exact native quotient evaluation;its actual terminal outcome
is recorded in results.json. F2descent,exceptional branches,actual level11
specialization and intended subgroup action are separate constructions.

The first proof workflow compiled0but its audit launcher failed to locate an
external lean process. The corrected runner exposes the same pinned binary
on PATH and compiles/audits in a fresh directory. Both workflows are retained.

The first packet workflow is rejected independently of mathematics. Its
driver was admitted before the Lean exit receipt,and a subsequent live Bash
runner edit caused an unintended second native invocation in the same raw
directory. Both owned processes were stopped;intermediate custody may have
been overwritten. `interrupted.json` records this exception. Those files are
retained as rejected evidence,excluded from valid historical executed-byte
pins,and never used for a mathematical claim. The fresh packet workflow
requires completed Lean compile/audit receipts before admission.
