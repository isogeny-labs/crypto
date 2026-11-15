# Coprime base-change carrier family: theorem and bounded replay

Frozen before the q=4 replay, 15 November 2026 UTC. The target is a new *isogeny-class carrier construction* from the exact q=2 trace ±9 witness, not a GHS vulnerability claim. No subgroup map has been evaluated and no full attack cost is measured here. [pcc:unverified]

## Prior work checked first

- [Menezes–Teske (2004/2006), Section 5.2](https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf) defines the n=7 W1 family. The coefficient condition and q2/q4 finite screens are in `research/ghs_n7_membership_20261114/`.
- [Borodin–May (2025), Section 4](https://arxiv.org/pdf/2508.16886) gives general characteristic-two hyperelliptic enumeration and smoothness criteria.
- The earlier repository `research/ic_next10_20261014/10-torelli-toy/README.md` already found genus-seven carriers for traces ±13, including `v=0xb9,u=0x8051`, exactly the +13 class rediscovered in `research/ghs_n7_prym_trace_20261115/`. That predecessor targeted the two E_a with b=1 and did not report the ±9 W1 classes. The current ±9 trace-class result is under independent review.
- Standard facts used: finite-field tensor products, base change of Weil restriction, Tate's finite-field isogeny theorem. The proof is written in `THEOREM.md`; the algebraic base-change step is not a new attack method.

## Mathematical claim under test

Fix E_-/F128 with `y²+xy=x³+x²+8`, trace -9, and Y_-/F2 with `y²+v y=u`, `v=0x8f,u=0xc07a`. The exact q2 census gives `J(Y_-) ~ Res_(F128/F2) E_-`. For every s≥1 with gcd(s,7)=1, put q=2^s, K=F_(q^7), E_s=E_-×_(F128)K, Y_s=Y_-×_(F2)Fq. Then `J(Y_s) ~ Res_(K/Fq) E_s` over Fq. The same holds for the +9 twist pair `E_+:(a,b)=(0,8)` and `Y_+:(v,u)=(0x8f,0x802f)`. The W1 coefficient b=8 remains in the union of the two W1 cubic arms for this coprime tower. The theorem states an isogeny-class family, not an explicit evaluable map or a rho-beating algorithm.

The trace recurrence for either sign is `t_0=2, t_1=±9, t_s=t_1 t_(s-1)-128 t_(s-2)`. For s=2, the minus arm has `t_2=-175`, and the predicted q4 target counts are N_m=4^m+1 for 1≤m<7, N_7=4^7+1-7(-175)=17610. These are exact predictions to replay independently from the q2 source count.

## Native replay and controls

- Implement a separate Rust point counter for both ±9 twists over F_(2^(2m)) for m=1,...,7, using irreducible binary moduli found by trial division, and compare against `5,17,65,257,1025,4097,17610` for each arm. Count each affine `(x,y)` solution directly through m=6 and cross-check with the Artin–Schreier trace criterion. Use the trace criterion at m=7 because 16,384² direct pairs exceed the local replay bound. Record the algorithm used per m.
- Check smoothness via the q2 census witness and Borodin–May criterion; this replay does not repeat the prior q2 classification.
- Stop on any field irreducibility failure, point-count mismatch, or runtime over five minutes. Output raw counts, field moduli, source hash, toolchain and checkpoint ID. No cloud spend.
- This q4 replay is an arithmetic cross-check of one base-change instance. The theorem's all-s scope rests on the finite-field tensor and Weil-restriction base-change proof, not on extrapolating the measured q4 row.

## Amendment A: source and W1 presentation replay

The target replay completed with the predicted seven counts in both twist arms. Before claiming a source-to-target q4 isogeny-class match, independently embed the original F128 coefficient into the selected F_(2^14) representation: find the lexicographically first root w of `w^7+w+1=0` in `F2[z]/(0x4021)`, set b=w³, and count `E(a,b): y²+xy=x³+a x²+b` for a=0,1 by the binary-elliptic trace criterion. Expected group order for both twists is `4^7+1-(-175)=16560`. Also find an old q2 W1 presentation `b=(r(v+1))²` with r,v in one cubic Frobenius kernel, then check the same r,v lie together in one q4 cubic kernel for σ:x→x⁴. Record old and new arms and exact masks. Native source: `src/q4_source.rs`. Stop on any mismatch. This is a new control, not a change to the target prediction or theorem.

## Amendment B: simplicity and genus floor

Before the finite factorization check, the paper argument in `GENUS_FLOOR.md` predicts that the base-changed restriction `A0 × F_(2^s)` is simple exactly for `7∤s`. The proof rests on the exact discriminant `−431`, absence of a quadratic-field integer of norm two, Capelli's prime-binomial criterion, and factorization of Frobenius polynomials. An independent Sage check will compute the source quadratic-field discriminant and norm obstruction and factor `X¹⁴−t_s X⁷+2^(7s)` for both signs at `s=1,...,10`. Predicted outcome: irreducible at all tested s except 7, whose polynomial is a seventh power of a quadratic. The check is a finite control of the symbolic proof, not its basis. Preserve exact commands and output in `reports/simplicity-20261115/` and stop on a disagreement.

## Amendment C: distinguish the two s=7 polynomials

The first Sage run reached s=7 and failed the assertion that `X¹⁴−t_7 X⁷+2^49` equals the seventh power of the q2 quadratic. That assertion was a protocol error: the former is the Weil polynomial of `Res_(F_(2^49)/F_(2^7)) E_7`, whereas `A0 × F_(2^7)` has Weil polynomial `(X²−t_1 X+128)^7`. Finite-field linear disjointness fails when `7|s`, exactly as stated in the main theorem. Preserve the failed run and change the control to check two separate polynomials at s=7. For `7∤s` they coincide and must be irreducible. For s=7, the base-changed `A0` polynomial must be the seventh power, and the distinct restriction polynomial must be reducible because `α^7` is a seventh power in the quadratic field. No all-s isogeny identity is asserted at s=7.

## Amendment D: correction of W0/W1 family labels

The frozen protocol used the W1 coefficient presentation as if it established W1 curve membership for both twists. [Menezes–Teske, Sections 5.1–5.2](https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf) additionally requires the **absolute** trace of `a`: zero for W0 and one for W1. At q2 the source `E_-(a,b)=(1,8)` is W1 and `E_+(a,b)=(0,8)` is W0. For coprime base-change degree s, absolute `Tr(a=1)=s mod 2`, so the minus curve is W1 only for odd s and W0 for even s; the plus curve remains W0. The q4 coefficient witness and all point counts are valid, but the prior q4 W1 membership interpretation is withdrawn. A direct genus-seven W0 descent for the +9 q2 source is recorded in `../ghs_n7_isogenous_w0_20261115/`. The isogeny-class base-change identity and genus-floor theorem are unaffected.
