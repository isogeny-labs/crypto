# Prime31 run 54: exact 25-variable, 27-equation signed-coset chart

Date: 2026-11-21 PST. **Status: new mathematically equivalent nonexceptional polynomial input + independent 4,090-witness replay. There is no blind algebraic root extraction or whole-ECDLP performance advantage.**

## Live overlap and prior art (checked first)

- **Existing native Rust F4 engine:** `src/cryptanalysis/f4_fp.rs` from main. `Poly`, `F4Options`, `f4` are reused in `examples/prime31_coset_signed25.rs`. Its documented fast packed path covers at most 16 variables; 25-variable degree-seven systems have not been shown tractable.
- **Public-only unknown-index Prime31 oracle:** PR [#4692](https://github.com/isogeny-labs/crypto/pull/4692); the exact positive/negative gates there own promotion.
- **Transformed-slot Horner/F3:** PR [#4812](https://github.com/isogeny-labs/crypto/pull/4812) has a smaller, different 14-var/15-eq degree-eight chart with original-Rust correctness controls, not F4 blind extraction.
- **Quotient/RR9:** PR [#4822](https://github.com/isogeny-labs/crypto/pull/4822), potentially four variables on a different relation predicate; avoid duplicate work.
- **Archived Prime31 runs 49–53:** MQ7 (64 vars/66 quadratics), signed-x-only7 (29 vars/31 equations max degree21), and seven-cubic public factor-base setup. No new curve selection; (p=2^{31}-1), (A=847514846), (B=2087444724), ordinary prime-order group of size 2147446831.
- Published prime-field background: Amadori–Pintore–Sala, *On the discrete logarithm problem for prime-field elliptic curves*, FFA 51 (2018), DOI [10.1016/j.ffa.2018.01.009](https://doi.org/10.1016/j.ffa.2018.01.009); Semaev summation polynomial approach and its unresolved practical rootfinding bottleneck. This alternative input is **not** asserted to be novel in the academic literature.
- **Knowledge base:** MCP `kb_status`/`search_knowledge` unavailable in this runtime, and repository CLI unavailable without checkout. Audited live repository paths and named PRs instead. The audit is bounded, not an exhaustive prior-art proof.

## Exact algebraic construction

For seven distinct public signed coset constants (c_i\in\mathbb F_p^\times), and public (Q=(q_x,q_y)\in E(\mathbb F_p)), introduce only the 14 leaf variables ((x_i,y_i)), 10 intermediate partial-sum variables ((X_j,Y_j)) for (j=1,...,5), and one guard variable (h): **25 variables total**.

Leaf constraints (14):
```
y_i^7 - c_i = 0
y_i^2 - (x_i^3 + A*x_i + B) = 0       i=0,...,6
```
Set (U_0=(x_0,y_0)), (V_j=(x_{j+1},y_{j+1})), (W_j=(X_{j+1},Y_{j+1})) for (j<5) and (W_5=Q). Let (d_j=V_j.x-U_j.x), (e_j=V_j.y-U_j.y); for (j>0), (U_j=W_{j-1}).

For each of six ordinary additions impose **two**, not three, equations:
```
d_j^2 * (W_j.x + U_j.x + V_j.x) - e_j^2 = 0
d_j   * (W_j.y + U_j.y) - e_j * (U_j.x - W_j.x) = 0
```
One saturation equation (h\prod_{j=0}^5d_j-1=0) simultaneously excludes all exceptional zero denominators. Total **14+12+1=27 equations**, maximum total degree **7**. No intermediate on-curve equations are needed: they follow from the ordinary addition law.

**Equivalence proof on the ordinary chart.** The saturation equation forces every (d_j\ne0). Define (lambda_j=e_j/d_j). The first equation uniquely forces (W_j.x=lambda_j^2-U_j.x-V_j.x), the second forces (W_j.y=lambda_j(U_j.x-W_j.x)-U_j.y). These are exactly the EC addition coordinates. By induction, the final public (W_5=Q) equals the sum of all seven leaves. Conversely, any ordinary seven-leaf decomposition gives the unique five intermediate points and guard inverse. The leaf equations select the *signed* (y^7=c) coset; no x-only sign-spurious solutions are added.

**Scope:** excludes any chart with a vertical pair or doubling during the chosen addition order; an exceptional-case branch and no-witness handling remain mandatory for a complete ECDLP solver. It is not a proof of coverage of all group targets.

## Executed independent replay (Python BigInt affine EC arithmetic)

Input: archived run52's `adaptive4096.csv` (SHA256 `07dfd381c1d069f4610e907ca3e7f9922ebb599ed473a18dca8e010652bb6e6a`) and `eight_leaves.tsv` (SHA256 `127fe818a2712ad942af86db78bfba8e87cc5bbac432c9d00c790f84ef1b0472`).

- 4,090 of 4,090 available seven-leaf signed point-sum witnesses satisfy **all 27 equations** and independent point addition. Six prior seven-leaf misses remain unresolved, not reclassified as no-solution proofs.
- 4,090 of 4,090 `(q_x+1,q_y)` target mutations reject the same witness.
- The first public-input system contains **203 nonzero terms**, with max degree exactly seven; canonical sorted-coefficient fingerprint SHA256 `e9a82c8cd73a84a14731e5b6e9b887643e084b7489cbdc92b61ad2f1d5cd7152`.
- The Rust example recreates the system **from public (Q,c_i) only** and checks one frozen witness and its wrong-target mutation separately; no witness enters the system builder. GitHub Actions compilation and native Rust validation on this commit are pending.

## Candidate comparison — do not confuse polynomial encoding with solving

| Chart | Variables | Equations | Max degree | Status |
|---|---:|---:|---:|---|
| Run50 MQ7 quadratized | 64 | 66 | 2 | Correctness positive; no blind F4 root |
| Run52 x-only signed7 | 29 | 31 | 21 | Correctness positive; no blind F4 root |
| **Run54 signed7 slope-free** | **25** | **27** | **7** | **4,090 verified assignments; F4 cost unknown** |
| #4812 transformed-slot Horner/F3 (different 8-slot predicate) | 14 | 15 | 8 | Original-Rust correctness CI; blind F4 unresolved |
| #4822 quotient/RR9 (different predicate) | smaller | — | — | Consult exact registered gate; do not compare directly |

At total degree five, the raw 25-variable monomial envelope is `binom(30,5)=142,506`, versus 278,256 for 29 variables; this is **not** an F4 row count or complexity forecast. The existing backend's packed-16-variable optimization does not apply to this chart, so a smaller input may still be slower.

## Prior benchmark gates, prioritized next experiments

1. **Native Rust CI / arithmetic parity:** source-bound `examples/prime31_coset_signed25.rs`, full exact polynomial construction and witness mutation check; fix compile issues before any timing claims. The earlier run53 Actions #44984788855 failed *only because cold Cargo compilation exceeded a 3-minute execution-step timeout*, not because a mathematical test failed. The PR workflow now separates cold build, caches dependencies and time-limits executions.
2. **Registered genuine blind F4:** a capped, public-only unknown-index solve on 16 unseen cases, 8 positive plus 8 negative if the predicate gives them. Test #4812's 14-var chart and #4822's RR9 chart before spending on this 25-var candidate; distinct equations, coverage and sign contracts must be preserved. `--bounded-f4` in this branch runs a **degree-3 admission probe only**; do not interpret an incomplete Gröbner basis as UNSAT.
3. **Full seven-of-eight cost gate:** frozen run50 histogram requires 7,297 seven-leaf calls plus six eight-leaf fallbacks over 4,096 Q. With uniform per-call cost, seven-leaf extraction must be under `4090/7297=0.5605043` of eight-leaf extraction just to improve extraction costs. Negative attempts, memory, exceptional branches and fallback must all be charged.
4. **Complete rank-112 recovery and cold/amortized cost:** replace generic MITM in relation collection and individual-log descent with verified actual algebraic outputs, then include factor-base acquisition, relations, modular linear algebra, retries and complete fallback. Benchmark 1/32/128/1024/4096 unknown targets against the strongest *same-runner native Rust* negation-BSGS, optimized rho and existing index calculus; independently replay all (kG=Q) certificates and uncertainty estimates.

**Result boundary:** This run discovers and verifies a more economical signed polynomial input, **not** an ECDLP speedup or an extraction oracle. Existing best control run50 adaptive MITM was 2,484.9 ms for 4,096 points versus negation-BSGS 585.3 ms, and was generic; no new complete non-generic timing was obtained.
