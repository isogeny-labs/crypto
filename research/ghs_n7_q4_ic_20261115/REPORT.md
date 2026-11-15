# Exact q4 genus-eight GHS index calculus and matched rho

15 November 2026. [Protocol](PROTOCOL.md) was committed at `016bd5f28dd0` and its identity-shortcut exclusion at `49810099be8d` before any relation run. A [post-run amendment](AMENDMENT.md) records the factor-base projection correction without rewriting that protocol. [Magma implementation](attack.magma), [immutable attempt receipts](receipts/), and [independent modular-algebra auditor](audit_receipt.py) make the exact experiment replayable. This is one finite instance in Teske's published n7 genus-eight family. [pcc:unverified]

## Exact workload and verified transfer

The source is `E: y²+xy=x³+a x²+b` over `K=F2[z]/(z¹⁴+z⁵+1)`, with `a=1+z+z⁴+z⁵+z⁶+z⁷+z⁸+z⁹+z¹²` and `b=1+z+z⁵+z⁶+z⁷+z⁸`. It has `#E(K)=16,634=2·8,317`. The source generator and target coordinates are in [attempt 7](receipts/attempt7.json); `P` has exact order 8,317 and `Q=7P` is the public validation target. The attack receives the points `P,Q`, and its relation and descent code does not use seven. Magma constructed a genus-eight hyperelliptic curve over F4, checked `#Jac(C)(F4)=33,268=4·8,317`, and verified `G=h(P)≠0`, `8,317G=0`, and `h(Q)=7G`. Attempt 7 records the F4 generator's embedding in K, the target equation, and the exact Mumford image. The algebraic target model is not canonical under the subfield embedding, so these details are part of the receipt identity. [pcc:unverified]

## Completed relation and recovery path

The degree-at-most-two Mumford factor base yielded 20 accepted relations from 8,316 nonzero multiples of `G`. Every accepted relation was reconstructed and compared with the original Jacobian element; the run recorded zero reconstruction failures. Five factor-base classes occurred. The matrix over `F_8317` had rank five and a unique consistent solution. **None of the five factor-base classes lies in** `<G>`; their matrix values are projected 8,317-primary coordinates, not literal logarithms in `<G>`. Because `#J=4·8,317`, multiplication by four kills the complementary primary component. Magma checked `4Bᵢ=(4Lᵢ)G` for all five solved coordinates `Lᵢ`. Target descent found a nonzero smooth class after 541 offsets: `h(Q)+540G=2B₁`. The projected coordinate is `L₁=4,432`, so `2·4,432−540 ≡ 7 (mod 8,317)`. The returned scalar was checked on both `E` and `Jac(C)`. [Python replay](receipts/attempt7-audit.json) verifies all 20 modular relations, rank, projected coordinates, offset, and recovered scalar without using Magma's matrix solver; divisor reconstruction and projection identities are checked in Magma. [pcc:unverified]

## Same-process cost

Magma 2.30-1 timed the phases with `Cputime` inside the public calculator. The rho implementation is a deterministic three-way Floyd walk on the same `P,Q`, with 588 walk steps and one restart. Its single-run timer is too coarse, so each receipt times 1,000 verified repetitions of that exact solve and reports their mean. The batches ran before GHS in the same process. These timings are an exploratory implementation comparison, not a calibrated T1 or asymptotic claim. [pcc:unverified]

| Receipt | GHS setup CPU | Relations CPU | Linear algebra CPU | Target descent CPU | GHS total CPU | Rho batch CPU / 1,000 | GHS / rho |
|:--|--:|--:|--:|--:|--:|--:|--:|
| [Attempt 6](receipts/attempt6.json) | 0.250 s | 0.350 s | <0.010 s | 0.020 s | 0.630 s | 3.020 s / 1,000 = 0.00302 s | 209× |
| [Attempt 7](receipts/attempt7.json) | 0.240 s | 0.340 s | <0.010 s | 0.020 s | 0.620 s | 3.010 s / 1,000 = 0.00301 s | 206× |

The GHS setup alone took about 80–83 rho-solve means. Relation collection enumerated all 8,316 nonzero source-subgroup multiples, so this particular method does not scale favorably at this size. The 20 relations, factor-base indices, rank, exact projection checks, and target decomposition are printed in each final receipt. The public service reported about 32 MB of memory and 3.77–3.80 seconds per final call, which includes the 1,000 rho repetitions and service overhead and is therefore not the one-target GHS cost. Attempts 1–5 are retained as implementation history; attempts 6–7 are the final projection-checked comparisons. [pcc:unverified]

## Prior work and conclusion

[Teske](https://www.lix.polytechnique.fr/Labo/Francois.Morain/Crypto/IsogenyParty/teske.pdf) already describes the n7 low-genus selector, and [Galbraith–Hess–Smart](https://www.iacr.org/archive/eurocrypt2002/23320028/smart.pdf) already gives the isogeny extension. The [q4 stratum report](../ghs_n7_traceone_genus8_20261115/STRATA_REPORT.md) proves the candidate lies in a finite set whose simple Weil restrictions force target genus at least seven. The present exact curve has a verified transfer and complete recovered target, but the measured genus-eight implementation costs far more than rho on the same subgroup. It therefore does not establish a new vulnerability or new eligible family. An optimized GHS implementation or a larger-field crossover is a separate experiment; this one small instance cannot rule either out. [pcc:unverified]
