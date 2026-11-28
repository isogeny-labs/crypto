# EXP-R31-RR9-NORM — coefficient norm and signed-fibre certificate (prospective, 2026-11-17)

**Source and goal.** Continue open PR #4466 (the original-curve Riemann–Roch L(9O) predicate) using the same Rust `prime31_algebraic_ic::{Arithmetic,Base,Point,PRIME31}` source, same actual ordinary cofactor-one curve `p=2147483647; E:y^2=x^3+847514846x+2087444724; n=2147446831`, and both existing first31 and Dickson-depth-six B31 bases. This is **not** a new complete unknown-log solver or novel attack. Freeze the test before source changes. No solver may receive caller-only discrete-log labels. [pcc:unverified].

## Lemma and proposed polynomial encoding

For a degree-8 effective affine signed-leaf divisor `D=P_1+...+P_8`, repeated points allowed and counted, there is a 1-dimensional nullspace of its eight confluent evaluation rows on `L(9O)`, because `L(9O-D)` has degree one on the elliptic curve. Choose nonzero `F(X,Y)=U(X)+Y V(X)`, `deg U<=4, deg V<=3`, from that nullspace, unique up to scale. If `R=sum P_i != O`, then the ninth zero of `F` is exactly `-R`, so its pole order is 9, `V_3 != 0`, and its **x-norm** obeys the exact coefficient identity:

`N_F(X)=U(X)^2-(X^3+aX+b)V(X)^2=-V_3^2 (X-x(R)) prod_{i=1}^8 (X-x(P_i))`

including repeated factors. This follows from the genus-one principal divisor criterion and the two sheets `Y=±sqrt(X^3+aX+b)` of the x-map. No proof of novel complexity follows.

The converse *certificate* when `F` is supplied: require `F(-R)=0` (full signed orientation), norm factorization, `V_3!=0`, and eight factors from `H(X)^8`, where `H=prod_{x in base}(X-x)`. `N_F(x(R))=0` **alone is insufficient**, because it also holds for `+R` and base-root sign ambiguities. If `R=O`, `V_3` may vanish and this affine norm identity has a different pole-degree branch; reject, never label it false. The polynomial is an algebraic constraint on unknown coefficients `U,V`; obtaining them from public `R` *without enumerating supports* is still the hard extraction problem.

## Preregistered Rust verification and falsifiers

1. Extend the existing RR9 example with 8×9 modular RREF of the original-curve value/Hermite-jet rows; assert **rank exactly eight** and recover the unique normalized kernel `F`. Do not use elliptic addition in kernel, norm or polynomial-factor routines.
2. On the 64 existing deterministic distinct eight-leaf positive fixtures (32 per base), assert `F(P_i)=0`, `F(-R)=0`, `F(-R-G)!=0` for the frozen wrong residual `R+G`, `deg N_F=9`, and coefficient-by-coefficient equality to `-V_3^2 * (X-x(R)) prod(X-x(P_i))` in Fp, **not** just polynomial zero at sampled roots.
3. Repeat the full identity on the two bases × three frozen repeated-leaf fixtures with multiplicities {3,4,8}, including full Hermite jets. Reject infinity explicitly. Test the ninth-order adversarial wrong residual separately (not a positive norm certificate).
4. Preserve the prior 128 determinant signed decisions and the existing double/multiplicity/ninth-jet adversarial controls. Emit counts and all norm coefficients or a reproducible digest with witness metadata, retaining arbitrary sign/multiplicity failures.
5. Native Rust 1.100.0 release tests in source-specific GitHub Actions, pinned merged APS source and existing RR9 workflow. No 96-Q unknown-log timing and no strongest-rho ordering will be claimed from predicate-only controls.

## Next target-conditioned experiment, contingent on this certificate

Let `F=U+YV` have eight free projective coefficients after the single linear constraint `F(-R)=0`. Seek solutions to the polynomial divisibility condition `(N_F/(X-x(R))) | H(X)^8`, with degree-eight quotient and signed fibre recovery. First B4/B8: reproduce *all* exact finite signed solution fibres (including diagonals) versus the prior exhaustive 26,400 decisions; track root-finding algebra degree, Groebner matrices, memory and censored instances. Then B31 (not generic enumeration of its 48,903,492 abscissa multisets), require fresh complete rank32 unknown-Q solves and matched same-host tuned WZ/batched rho. This norm is an alternative representation of the **same classical S9 relation ideal**; exploiting it may or may not reduce extraction cost. A symbolic solver that enumerates f or hides a half-sum table fails the non-generic gate.

Prior lines checked before registering: #4188 has an m4 pair-support norm/GCD extractor (do not claim novelty for norm itself); #4228/#4349 selected-root quotient, #4373 censored m8 F4, #4391/#4454 complete but MITM-equivalent APS, #4451 zero-fibre obstruction, #4466 RR9 Hermite, #4468 tuned-rho CI build repair; Vieta #4410, auxiliary IC cryptanalysis #1536, and crypto-autoresearcher #3309 remain partial/negative. PKM 2016, APS 2018, and classical elliptic Riemann–Roch are prior art.

**Success means the exact certificate passes, not a cheaper ECDLP solver.** Measurement acceptance remains 96 fresh blinded full unknown-target logarithms, complete cold accounting and paired 95% intervals against strongest matched generic and published index-calculus frontier. No paid compute or leaderboard changes.
