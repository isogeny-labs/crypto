# CT335 — preregistered split-Prime31 Richelot codomain and dual-formula audit

Date: 2026-11-23 UTC. Status at registration: **plan only**, no CT335 scientific execution. [pcc:unverified]

## Frozen input and ownership
- Existing CT334 JSON blob `01be355bf299c01419ac3a43a53adc4f8f7d61ea`, row `[1,1,1,1,1,1]`, defines `p=2147483647` and `f(x)=prod(x-a), a=1..6`. The source model and parent CT334 JSON remain byte-for-byte unchanged.
- Work only in new CT335-prefixed files of `research/chained_2power_descent_20261123/`. Do **not** read or rewrite EKS-only 695/701 source fixture, native Cantor/norm-point lane, cost16, two-hop C/D or genus-three RREF work. No paid compute or timing benchmark.
- Relevant formulas: Kunzweiler, *Efficient computation of (2^n,2^n)-isogenies* (2024) §3.2, https://link.springer.com/article/10.1007/s10623-024-01366-1 ; Magma handbook Richelot §, https://magma.maths.usyd.edu.au/magma/handbook/text/1615 ; Visser (2022) slide 9, https://warwick.ac.uk/fac/sci/maths/people/staff/visser/genus2crypto.pdf .
- KB/Conductor not exposed in this runtime; reviewed source-linked CT331–CT334 and concurrent #4992/#5005 instead. No claim of novelty.

## Fixed algorithm / claims permitted
Enumerate the fifteen unordered perfect matchings of roots 1..6. For each matching, let `g_i=(x-a_i)(x-b_i)` in canonical matching order; compute `delta=det([g_i constant, linear, quadratic])`. For nonzero delta, set `h_1=(g_2' g_3-g_2 g_3')/delta`, `h_2=(g_3' g_1-g_3 g_1')/delta`, `h_3=(g_1' g_2-g_1 g_2')/delta`. Compute `H=h_1*h_2*h_3`, check its degree is 5 or 6, and `gcd(H,H')=1`. Distinguish delta=0 (decomposable quotient; not a smooth Richelot-Jacobian output) from smooth Jacobian-type codomains. Do not treat a rational kernel alone as a new smooth Jacobian.

As an algebraic duality diagnostic only, repeat the same determinant/bracket operation on the `h_i` wherever the first codomain is smooth; test whether each dual quadratic is a common-scaled/permuted original `g_i` and the dual sextic a scalar multiple of `f`. Record the exact scaling and its quadratic character. A doubled curve equation does *not* independently verify a divisor-map composition or a DLP-preserving full transfer.

Freeze the complete 15-row table in sorted matching order with `g_i,delta,h_i,H,gcd/smooth,dual` and strict summary totals. Include deterministic positive controls and **effective** negative controls rejecting deliberately modified bracket/determinant and duplicate-root input; explicitly report if a control fails. Use the original and a different arithmetic implementation (SymPy finite-field `Poly`) for independent-library algebraic recomputation, with the same coordinator as producer. The hash of executed source and JSON must be recorded; run normal and `python3 -O` with byte-identical outputs. Preserve any failures and report actual outcomes.

## Boundaries and next gate
This is a bounded source-only Fp algebra test, **not** 695/701 fixture custody, a native Rust/Cantor divisor map, a genuine composite morphism verification, a cross-factor map census, or complete single-target IC/rho/BSGS comparison. All full-cost physical `C169/C13a/C13b/N2_actual/D`, cold same-host, memory and independent PCC remain held by existing owners; no result may be promoted without the signed validation gate. Keep PR #4997 draft until stacked requirements.
