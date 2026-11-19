# Prime31: saturated x/y Horner chain for signed W8 — 2026-11-19

**Evidence state:** mathematical derivation and executable original-Rust fixture validator committed. Actual CI result must be read from the [source-specific job](https://github.com/isogeny-labs/crypto/actions/workflows/prime31-horner-xy-chain.yml) and raw artifact, not inferred from the existence of the code. [pcc:unverified]. No full public-target index extraction or complete ECDLP solving improvement is claimed.

## Precise result

On unchanged ordinary cofactor-one E/F_p with p=2147483647, a=847514846, b=2087444724, #E=2147446831, G=(625037715,2100627251), consider 8 signed independently selected factor-base leaves P_i, weighted by 2^i.

The weighted sum admits **one 7-stage recursive Horner chain**
T_7=P_7, T_i=P_i+[2]T_{i+1} (i=6,..,0), Q=T_0.

This requires **7 point doublings plus 7 point additions** in a generic-group evaluation, compared with 10 doublings plus 7 additions for two independent four-slot Horner chains and four extra doublings to realize U+[16]V: total **14 versus 17** group operations on nonexceptional input. The arithmetic advantage is an identity-level count (a 3/17=17.65% reduction in group *operation count*) and is **not a measurement of index extraction or solver time**. Native Rust test counters separately compare literal per-leaf scalar weighting, full Horner and split Horner on the same generated leaves.

### Exact polynomial chart

For a doubling of T=(X,Y), introduce (Xd,Yd), slope lambda and inverse rho:
(2Y)lambda=3X²+a; (2Y)rho=1;
Xd=lambda²-2X; Yd=lambda(X-Xd)-Y.

For adding a signed factor-base leaf P_i=(xi,yi), introduce new coordinates (Xn,Yn), slope mu and inverse sigma:
(xi-Xd)mu=yi-Yd; (xi-Xd)sigma=1;
Xn=mu²-Xd-xi; Yn=mu(Xd-Xn)-Yd.

**All eight identities per step have degree ≤2.** The inverse-witness equations are algebraic saturation constraints; omitting them creates spurious solutions where denominators vanish. For the eight leaves require H_B(x_i)=0 and y_i²=x_i³+a x_i+b, with H_B(X)=prod_(j=1)^8 (X−x(B_j)). The public target Q supplies two terminal linear equations.

Total explicit system: 16 leaf variables and 7×8 intermediate variables = **72 variables** (70 if the last point is substituted with public Q), 56 quadratic-or-less step equations, 16 leaf equations (degrees 8 and 3), two terminal equations: **74 equations**. A rational affine solution with both denominators nonzero at each step is **equivalent** to a valid signed weighted factor-base decomposition of Q. This is a reversible mathematical construction over its stated ordinary-affine chart, not a complete all-chart index extractor.

### Why low degree alone is not a breakthrough

The *number of unknowns* increases dramatically from the seven RR9 coefficients. In a dense 72-variable monomial model there are C(72+d,d) columns up to degree d: at d=2, 2701; d=3, 67525; d=4, **1,282,975**; d=5, **19,757,815**. These are **loose monomial ceilings**, not measured sparse F4 sizes, degrees of regularity, or complexity lower bounds. The sparse recurrence may permit elimination / block sparsity, but this must be measured rather than assumed. Exception charts include T_i=O, y(T_i)=0, and x(P_i)=x([2]T_(i+1)); these cannot be treated as absence of a relation.

The code implements a **certificate construction and verifier using planted signed leaves**, including polynomial residual checks, curve/factor predicates, group-law replay, poisoned slope and mutated-target checks. The planted witnesses are test-oracle inputs only; the public system depends only on curve, factor-base x coordinates and Q. A genuinely public-only unknown-index F4 solver remains unimplemented.

## Frozen measurement

- Protocol committed *before implementation* in HORNER_XY_CHAIN_PROTOCOL_20261119.md.
- Native original-crate example: research/prime31_ic_20261115/native/examples/prime31_horner_xy_chain.rs.
- Independent BigInt affine verifier: research/prime31_ic_20261115/native/verify_prime31_horner_xy_chain.py.
- Deterministic 64 signed leaf-tuples × 4 original windows, SplitMix seed 0x4858595f43484149. Test-only targets generated as exact weighted sums; Q's generating scalar is not supplied to any candidate.
- Count: group additions, chart exceptions, all 7×8 ordinary polynomial identity residuals, same-witness target mutations, mutated-slope controls; first 12 witness certificates published.
- CI runs original pinned native tests, focused Rust tests, release JSON, then independently replays serialized group identities. Do not claim passed validation until that source run completes.

## Prior lead / duplicate-work audit

| Lead and owner | Established or current status | Decision |
| --- | --- | --- |
| RR9/F4 public unknown-index extractor, #4692 | Draft, candidate integration not established | **Owner for solver gate**; hand off x/y system only, no parallel F4 rewrite |
| Native weighted W8 four-window complete graph/rank solver, #4728 | 1024/1024 logs verified with *generic 4+4 grading*, 14.1s producer on its runner | Reuse end-to-end Rust harness only once genuine extractor exists |
| Semaev F3 pair algebraic membership, #4809 | Separate public-only two-factor primitive in flight | Do not duplicate its quadratic-root implementation |
| Kummer x-only [16], #4810 | Exact bijection/correctness on curve and twist | Alternative x-only sparse constraint; include as matched ablation |
| Block-scale U+[16]V, #4808 | 452/512 held-out generic-grader hits, k=4; n.s. versus k=5 | Split-chain baseline for x/y recurrence |
| Two-sign interpolation, #4801; unweighted B8 coverage, #4803 | Rigorous structural limitations | No reuse as weighted W8 extractor |
| Shared 2^20 BSGS, merged #4532 | Same-run matched setup 103.840ms + online 66.274ms = **170.113ms for 1024 logs**; 67,924 KiB RSS | Necessary whole-solver frontier; do not compare unpaired runs as speedups |
| Previous RR9 budget #4793 | Historical cross-run sensitivity 0.13955ms/req when all other work free | Inform failure thresholds, **not** new measured crossover |
| Cross-repo genus-2 and CM proposals | crypto-autoresearcher #3338/#3339 give cost obstacles, general transfers unproved | Do not count as prime31 solver advantage |

Literature: Petit–Kosters–Messengur (PKC 2016) on rational-map prime-field bases; Amadori et al. (2018) Semaev relation variants; Kudo–Yokota–Takahashi–Yasuda (2018) hybrid Gröbner attacks; 'Complexity bounds on Semaev's naive index calculus method for ECDLP' (J. Math. Cryptol., 2020). Existing elliptic group-law equations and Horner double-and-add are standard. No novelty claim.

## Next falsifiable experiment queue

1. **Public-only B8 extraction**: translate the 74 polynomials to the existing Rust F4 machinery, measure basis sizes, degree, sparse fill and actual unknown x/± witness recovery. Strictly separate data fed to candidate (public Q/B8) from test-oracle planted witnesses.
2. **Matched algebraic ablation:** full x/y Horner, split U+[16]V via four-link doubling, x-only monolithic [16], and intermediate-eliminated system; bound RAM at 1 GiB and time per target. Include frozen 46 positive and 18 negative targets, exceptions, and reject all false positives.
3. **If and only if passed**, integrate into existing 31-factor-rank, 1024-public-log Rust pipeline. Charge setup, full matrix solve, all misses, verification, extra Fp inversions, memory, and both cold and amortized T∈{1,64,1024}.
4. Run on same host and targets against original optimized BSGS, mixed/weighted index calculus, and optimized negation rho. A whole-solver crossover, not a smaller positive chart, is the success criterion.

### Validation boundary
Mathematical chart equivalence: derived. Rust fixture code: committed. Independent original-source CI: pending until source-specific run reports success. Real F4 extraction: not implemented. Complete nongeneric ECDLP advantage: not established. Independent signed PCC: not obtained.
