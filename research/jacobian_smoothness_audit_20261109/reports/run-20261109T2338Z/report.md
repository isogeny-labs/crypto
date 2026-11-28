---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# Recorded Cantor witness candidates

Both fixed-input cells completed: 25 cycle candidates and 100 complete-model
candidates for $y^2=x^5+1$ over F3. Every recorded candidate passed the
native polynomial assertions and matched its existing, separately
implemented C++-checked table entry. These are candidate certificates;
Lean acceptance is a separate check. [pcc:unverified]

| Recorded value | Cycle | Full model |
|:--|--:|--:|
| Ordered pairs / table matches | 25 / 25 | 100 / 100 |
| High-degree reductions | 6 | 48 |
| Non-one normalization units | 0 | 12 |
| Exit status | 0 | 0 |
| Diagnostic seconds | 0.07 | 0.08 |

![Recorded counters from AUDIT-STOLL-WITNESSES-v1, one complete fixed-input cell per arm; no sampling interval. All 25 cycle pairs also occur in the full 100-pair cell.](counts.pdf){width=96%}

The frozen protocol has a documentation defect: both generic parameter
blocks incorrectly say $X^5-X$ and mode=fixture. Its question, flags,
source and input hash specify the Stoll $X^5+1$ model, and the pinned source
explicitly rejects a different curve or pair count. The actual run follows
those source assertions. **We do not claim full protocol conformance.**
The frozen bytes remain unchanged; see the separate deviation receipt.
[pcc:unverified]

Source `3dd8d410` and protocol hash prefix `cd194533` were committed and
published before calls (remote source/protocol publication `a0bfea7f`).
The exact Rust lock and unchanged pinned vendor backend are retained.
Binary hash prefix `69cb98ed` is checked before arithmetic; the old raw
reference hash prefix is `cf8ec5cf`. Full hashes are in the export/protocol.
[pcc:unverified]

\newpage

# Execution, controls and formal boundary

Flow: two planned, two attempted, two completed, zero failed, timed out,
resumed, interrupted or excluded. One fixed model; complete ordered input
populations of 5 and 10 points. Seeds 202611092901 and 202611092902 label
cycle and full executions. No warmup, retries or stochastic sampling.
Both calls used existing local capacity, one thread, host `762f5fc120c9`,
isolation L0 and zero paid compute. All clocks are scientifically invalid;
reported intervals are diagnostics and support no speed claim.
Hard caps were 120 seconds per cell, 240 total; the earlier 120-second
planning estimate was not a measured generator runtime. [pcc:unverified]

The source checks the frozen inputs and the reference model's 10 valid
pairs, 8 split-u pairs and complete table. It reuses the pinned native
backend and asserts common-divisor identities, Bezout identity,
composition/numerator division, remainder and normalization factor.
The control uses the previously Lean-checked five-cycle. This execution
is on the producer host; it is not an independent-machine validation.
[pcc:unverified]

Twelve full-model reductions use a non-one normalization unit. This
explains why the previous constant-v special relation cannot simply be
applied unchanged to every point. It is not a counterexample to the
qualified Cantor formula, which includes normalization. [pcc:unverified]

Raw output's kernel_proof_status=not_checked is preserved literally.
New Lean files must independently prove every recorded witness equation,
uniqueness of deterministic arithmetic, the full group operation and
subgroup fraction inequality. A failed tactic is not a mathematical
counterexample. The geometric Picard/Jacobian bridge and qualified
asymptotic or large-prime heuristics remain separate obligations.
[pcc:unverified]

Evidence: `../../evidence/stoll-witness-v1/export.json`,
`raw-manifest.json`, both exact raw output directories and
`protocol-deviation.json`; frozen source/protocol in
`../../stoll-witness-v1/`. No signed independent replay/recompute exists;
scientific standing remains unsigned/unverified. Source context is
Stoll2014 Theorem4.18/Example4.19 at the read scopes recorded in
`../../stoll-sources.json`. Original Cantor1987 remains unread after403.
No novelty claim is made. [pcc:unverified]
