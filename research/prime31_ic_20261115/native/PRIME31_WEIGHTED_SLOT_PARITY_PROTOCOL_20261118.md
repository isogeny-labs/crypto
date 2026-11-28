# Prime31 weighted-slot parity gate — preregistered 2026-11-18

**Claim class:** exact group-algebra lemma plus bounded, producer-run native Rust feasibility experiment; [pcc:unverified]. No non-generic complete ECDLP claim.

## Frozen domain and arithmetic

Use the already established ordinary cofactor-one curve over `p=2147483647 = 2^31-1`, `E:y²=x³+847514846x+2087444724`, prime order `n=2147446831`, generator `G=(625037715,2100627251)`. Import `prime31_algebraic_ic::{Arithmetic,Point,PRIME31,check_curve}` from the repository's existing `research/prime31_ic_20261115/native` Rust crate, pinned source revision `1df714645aa683cf975bc76fcacb9842cb4780fd`. Pin Rust 1.100.0. Do not substitute a smaller/weak curve, point field or scalar oracle.

Use **first B rational abscissae** with canonical least y and fixed B in {8,16,31}. These are original-curve points, not known-log inputs. Use m=8 ordered positive point slots and mod-2 parity, not unordered signed-multisets.

- Uniform control: each slot has multiplier 1.
- Distinct scalar-weight candidate: eight publicly known multipliers `[1,3,5,7,11,13,17,19]`; slot i contains `[c_i]P_j` for the same B unknown-log basis points. Every found equation is `sum c_i log(P_{j_i}) = log(R) mod n` and remains original-curve verifiable.
- Translation-only control: slot i contains `P_j+T_i`. By commutativity it is just a translate of the uniform control and must not be promoted as symmetry breaking.

Test the algebraic lemma `(Σ_{P∈A}[P])^(2^k) = Σ_{P∈A}[2^kP]` over `F_2[E(F_p)]` for any A. For m=8 the uniform parity support has exactly B elements because n is odd prime and [8] injective. For arbitrary m, the parity support is contained in `B^{popcount(m)}` point sums.

## Frozen measurement

Use exact mod-2 toggling of 4+4 *ordered* slot sums. For each four-slot half, precompute two ordered pair tables and XOR 4-sums in a `HashMap<Point,Word4>`, retaining one witness iff parity is odd. For each residual R, XOR matches between the left and right parity sets, and replay one eight-slot witness (if any) under the **original Rust group law**. This is a **generic group MITM diagnostic**, explicitly *not* the required nonenumerative algebraic extractor.

Use deterministic SplitMix64 seed `0x57534c4f54503131`; select 12 fixed public residual points as `[1+(splitmix64 % (n-1))]G`. Caller-only generator scalars must not enter membership/recovery, other than constructing R and final verification. Include uniform-control diagonal R=[8]P0 and weighted candidate R=sum c_i P0. Reuse identical residuals for all B. Enumerate a B=2 exhaustive word panel to assert parity agreement with the 4+4 oracle on its entire finite support, including negative controls.

Measure half-index construction and twelve public residual checks (wall clock as uncalibrated diagnostic), count original Rust group additions, Fp multiplications and inversions, distinct parity half-support sizes, odd residuals and exact replay. Save a deterministic JSON artifact from the exact-source CI run. Compare the result against **fully charged** incumbent 256/1024-target IC/BSGS/rho only as a *future* gate, not by comparing unrelated stage times. Do not promote 12-target parity coverage into a reliable general probability estimate.

## Falsifiers and continuation

- Uniform B31 must have parity support exactly 31 although many ordinary eight-leaf decompositions exist; any discrepancy fails the experiment.
- Translation-only construction must retain the same parity support translated by sum T_i.
- Distinct scalar-weight parity must match exhaustive B=2 control for every tested target; every recovered witness must group-sum exactly to its public R.
- All costs from half-index construction and unsuccessful parity checks must be reported.
- If distinct slot weights deliver useful odd parity on B31, test whether a **polynomial/F4/tensor counting** oracle can decide it *without* 4+4 enumeration. Coordinate with active RR9 PR #4692 and parity theorem PR #4698 rather than duplicating their implementations.
- To establish a cheaper complete novel/non-generic solver: new algebraic oracle without generic enumeration; naturally discovered independent rank-31 relation rows, existing Rust 1,024-blinded-log recovery, zero hidden fallback, complete original-curve verification, then same-host comparison to optimized shared BSGS, rho and IC frontier.

## Sources and prior-lead boundary

Existing source: `research/prime31_ic_20261115/native/examples/prime31_s9_b31_half_sum_oracle.rs`, prior B31 control PRs #4623/#4635/#4647, RR9 extractor #4692, S9 obstruction #4451, asymptotic binary-parity theorem #4698, exact count construction #4668. Prior published prime-field IC: Petit–Kosters–Messeng (2016), Amadori–Pintore–Sala (2018). For independent slot hashing and odd singleton yield see the pinned Bochum manuscript https://github.com/bochum-crypto/subexponential-ecdl (section on odd decomposition). The research-KB MCP and repository CLI are not available in this connector-only runtime; search of GitHub primary notes and open PRs is a bounded fallback, **not** novelty clearance. No paid compute or merge authorized.
