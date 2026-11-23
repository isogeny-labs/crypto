# CT218 — full norm-projected genus-4 toy ECDLP index calculus

2026-11-18. Existing [crypto PR #4715](https://github.com/isogeny-labs/crypto/pull/4715). **Finite complete toy solver, not a demonstrated cold ECDLP cost advantage.** [pcc:unverified].

## Algebra and solver

E/F101: Y²=X³+X+46, #E(F101)=109 and G=(2,37). C/F101: y²=x⁹+x³+46 (genus 4), π(x,y)=(x³,y). With α=π* and β=π_*, β∘α=[3], giving rational projector e=α[73]β. Each actual fully split Jacobian relation

    D0 + α([k]G) = Σ[P_i−∞]

pushes forward to

    Σ log_G(π(P_i)) − Σ log_G(π(Q_j)) = 3k mod109,

where the Q_j are explicit known rational support points of offset D0. No offset column is silently dropped. Since gcd(3,100)=1, cubic x↦x³ permutes F101, so this special model has 108 finite rational cover points. Folding y→−y yields **54 signed factor-base unknowns**; the only known-log anchor is the public generator lift. The algorithm does not use BSGS, rho, target known scalar, or a free source DLP oracle to construct or solve the factor-base matrix.

## Completed CI execution

Exact-head source [cc5a7e1a](https://github.com/isogeny-labs/crypto/commit/cc5a7e1a14a6bd699aa3f59c2f7709fff948611a), [GitHub Actions #44418260757](https://github.com/isogeny-labs/crypto/actions/runs/44418260757), oracle job [#135204145637](https://github.com/isogeny-labs/crypto/actions/runs/44418260757/job/135204145637) **passed** on 2026-11-18. The executable is producer-written: no independent PCC signature.

| Scope | Verified result |
|---|---:|
| Deterministic offset cosets | 13 |
| Actual Jacobian candidate trials | 1,402 |
| Fully split genus-4 relations / norms checked | 61 / 61 |
| Independent relation rows + generator anchor | 53 + 1 |
| Signed factor-base matrix rank / nullity | **54/54 / 0** |
| Dependent relation consistency checks | 8 |
| Factor-base source logs checked after solving | **54/54** |
| Public elliptic source DLPs recovered and checked | **109/109** |
| Target T=(64,19) logarithm | **37** |
| Online trial count / J additions for T | 3 / 4 |
| Reusing setup for all 109: online trials / J additions | 2,276 / 2,385 |

The program uses actual CT216 independent Python Cantor arithmetic, exact fixed-offset norm identities, incremental modular rank, and full Gaussian elimination. It checks candidate logs by public elliptic multiplication **after** matrix solving, not as an oracle to the solver.

## Cold cost and scope

A single-target cold solve involves at least 108 source-walk J additions +26 offset composition J additions +1,402 relation candidate J additions +4 online additions = **1,540 Jacobian additions**; 111,925 modular F109 LA multiply/subtract updates are additionally recorded. This excludes important field/polynomial/root costs, memory, transfer, and verification. Direct elliptic BSGS from CT217 took 25 elliptic additions for the same target; group types are not cost-equivalent and 1540/25 is **not** a measured ratio. It is implausible this toy candidate is single-target competitive; no identical-host strongest rho/BSGS/IC comparison is available. Shared 109-target performance is amortized, not cold. No practical prime-field ECDLP advantage is established.

## Reproducibility and next steps

    python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct218_full_norm_ic.py --self-test
    python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct218_full_norm_ic.py > /tmp/ct218.json
    cmp /tmp/ct218.json research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct218_f101_full_norm_ic.json

Next exact-head CI verifies frozen byte equality. Port signed norm-offset relations into existing Rust genus-ladder and LA, add adversarial controls, then preregister complete same-host cold setup/transfer/relations/LA/memory/verification versus optimized rho/BSGS/IC. Scaling beyond F101 is unknown, including ordinary prime fields such as 2³¹−1. Original 695/701 norm-point fixture and physical C169/C13a/C13b/D still blocked by documented S3 HTTP403; no synthetic substitute permitted.

Existing CT216–217 sources, crypto #4215, cryptanalysis #1523, genus4 #4693 and classical Gaudry (2000), Diem (2006), GHS Weil descent papers were consulted; the shared research-KB MCP/wrapper was unavailable. No novelty claim for norm/pullback or genus-4 IC.

**No paid compute, protected merge, PCC, native CT218 solver, or speedup claim.**
