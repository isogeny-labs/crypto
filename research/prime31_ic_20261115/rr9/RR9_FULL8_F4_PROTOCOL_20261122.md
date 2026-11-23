# Prime31 B8 full-eight RR9 F4 extraction screen — frozen protocol

**Frozen 2026-11-22 before implementation or measurement. [pcc:unverified].** This is a bounded mechanism and cost screen on the ordinary cofactor-one curve `p=2147483647`, `a=847514846`, `b=2087444724`, `n=2147446831`. It does not substitute for natural B31 relation yield, rank, or a complete fresh single-target comparison.

## Prior-work gate and hypothesis

The seven-of-eight B8 ideal is already known to split into 128 rational signed charts on these controls (`RR9_SEVEN_QUOTIENT_SPLIT_RESULT_20261117.md`). Evaluating its eighth equation by enumerating those charts is an exact generic-equivalent control. The new candidate runs the existing Rust sparse F4 engine directly on **all eight original coefficient quadrics**, formed from public residual `R` and the fixed B8 polynomial `H_B`, without entering signed leaves, a support mask, a coefficient witness, or a target scalar. The test asks whether a complete F4 basis reveals a unique positive chart or a unit negative at a cost below the 11,440-column D9 Krylov extractor. A positive result would be a lower-cost sign-independent *implementation path* at B8, not evidence of a non-generic ECDLP advantage.

## Frozen inputs and caps

- B8 abscissas: `(0,1,2,3,5,6,8,9)`.
- Public positive residual: `(1911579686,548820096)`.
- Public negatives: `(625037715,2100627251)` and `(1923055775,307731458)`.
- Polynomial: `M=(X²+rX+r²+a)V²-(X-r)A²-2sAV`, with `A=a0+a1X+a2X²+a3X³` and `V=X³+v2X²+v1X+v0`; input quadrics are all eight coefficients of `M-H_B` below degree eight.
- Run order: positive, first negative, second negative. One deterministic F4 call per case; no seed search. F4 maximum degree 9, cooperative wall budget 60 seconds per case, at most 3,000,000 symbolic nonzeros and 1,000,000 dense entries. Outer process cap 210 seconds. Do not automatically enlarge a cap after observing a result.
- Record source SHA, full wall and CPU time, per-case F4 wall, maximum degree, basis size, staircase, peak nonzeros/dense entries, multiply-adds, timeout/oversize/censored status, and peak RSS if available. Charge public equation construction, F4, extraction and checks. Preserve all three case outputs, including failed/censored cases.

## Soundness and decision rule

The positive counts only if F4 terminates with no pairs left, the completed basis contains seven independent affine linear equations yielding seven coefficients, and an independent implementation checks all eight original equations and the full signed point sum against public R. A basis with unresolved pairs or an unverified candidate is censored.

A negative counts as certified only with a replayable unit identity from the original eight quadrics. A completed F4 unit basis without an independently replayable identity is a solver observation, not a certificate. Existing D9 negative unit receipts may be cited as separate controls; they do not certify this new F4 trace by themselves.

Compare complete same-host wall and CPU with the existing D9 positive and 128 signed-system direct control, without pooling hosts or calling a component speed a complete solve. If the positive F4 call exceeds its cap or yields no independently checked coefficients, record the precise limiting phase and stop this B8 F4 branch. If it passes but is slower than the signed control, retain it as a correctness route and screen B31 all-multiplicity cost before implementation. A B31 proposal must handle `rad(M)|H_31` and exceptional/repeated-root strata, produce natural relation rank, and satisfy a frozen full single-target budget against matched optimized rho.
