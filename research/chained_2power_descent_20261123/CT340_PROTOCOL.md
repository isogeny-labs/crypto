# CT340 protocol — explicit complementary elliptic pullbacks and dual-composition test

Date 2026-11-23 UTC. Existing lane draft #4997, frozen predecessor head `186fc8076629ecfb46e4677a6c77801726762d78`, published CT339 standalone Python SHA-256 `c798c3201da743091be50e1a3c3cd63200f2e995e72d17991518db3ca5345695`, JSON SHA-256 `eb986754a030062db0aa629989edf1ac2705695f5d8b1863fc19dab28c085492`. This protocol must precede executing the new CT340 science code. [pcc:unverified]

## Sources, scope, reservations

Use prior CT336–339 exact maps plus the Richelot / degree-two elliptic subcover literature: https://magma.maths.usyd.edu.au/magma/handbook/text/1615 and https://arxiv.org/abs/math/0312413 . The repository KB/Conductor are unavailable in this coordinator runtime, so no literature novelty assertion. Keep original 695/701 norm-point fixture, dedicated `C169/C13a/C13b/N2_actual/D` work, EKS admission, #5007 native F3/E7, #4992 and #4990, the parallel Rust owner, and existing benchmark reservations untouched. No new scheduled task, paid compute, timed benchmark, root-library build, CI rerun or PR merge. One coordinator, at most three workers only if native workers actually exposed; no worker signatures are assumed.

## Exact math construction

Use `C_p:y²=∏_{j=1}^6(x-j)`, `p=103`, `h=7/2`, `a=h-1=5/2`, `z=1/(x-1)`, `eta=y*z³`, and `F(z)=∏_{j=1}^5(1-jz)`. Preserve CT339 cubic elliptic models `Eplus: v²=∏_{r=1,3,5}(u-r²/4)`, `Eminus: Y²=∏_{r=1,3,5}(1-r² X/4)` with `X=1/t²,` `Y=y/t³`, `t=x-h`. The pushforward `Phi:J(C)→Eplus×Eminus` is exactly CT339's map.

Define the candidate dual `Psi(P,Q)=pi_plus^*(P-O)+pi_minus^*(Q-O)`. All divisors are based at Weierstrass point `W1=(1,0)` via the odd-model Mumford representation.

For finite `P=(u,v)∈Eplus` with `d=a²-u ≠0`, take `U(z)=z²-(2a/d)z+1/d` and `V(z)=v*z³ mod U`. For finite `Q=(X,Y)∈Eminus` with `d=a²X-1 ≠0`, take `U(z)=z²-(2aX/d)z+X/d` and `V(z)=Y*(1-a*z)³ mod U`. Require each normalized pair pass the original `norm` check `U | V²-F`. For `P=O` or `Q=O`, pull back zero. For `P=Tplus=(25/4,0)` or `Q=Tminus=(4/25,0)` the fiber includes the odd-model point at infinity `W1`; return `[W6-W1]` with `z=1/5,eta=0`. For `Q=(0,±1)` use the ramified fiber `2[Q_infty±-W1]` with `z=0,eta=±1`. Any further exceptional denominator must fail rather than silently skip. Do not assume same group law on chart-unrelated curves.

## Frozen bounded gates

1. Load original CT339 Python source byte-for-byte and verify SHA-256 before importing; rerun original CT339 with byte-for-byte matching frozen output, separately in Python normal and `-O`. Do not alter original.
2. Enumerate exactly all `#Eplus(F103)=92` and `#Eminus(F103)=100` rational group points including identity. Construct `Psi` on every point (92+100=192 arms), requiring zero rejects, valid normalized Cantor classes, and explicit recorded distribution of exceptions (infinity, two-torsion fiber W1, Eminus X=0, generic).
3. For every `P` check `Phi(Psi(P,O))=([2]P,O)`; for every `Q`, `Phi(Psi(O,Q))=(O,[2]Q)`. This forces all diagonal and cross-factor maps, not merely finite polynomial identities. Assert at least one ramified and one W1 exceptional case in each applicable arm.
4. For the fixed original eight CT339 generator classes and all `k=0..20`, check `Psi(Phi([k]D))=[2]([k]D)` from actual reduced Cantor classes (168 checks). Check the original order-23 transfer generator and `[2]G` as an independent additional gate. No excluded source classes.
5. Two non-vacuous wrong-sign/wrong-basepoint mutant checks must fail with nonzero process exit in normal and optimized mode; compare canonical stdout byte-for-byte across normal and optimized execution; report SHA-256 and exact counts. Source and result are both unverified until replayed from the *committed* source and hashes.

Mathematical status: the abstract identity `Psi∘Phi=[2]` follows from the complementary genus-two involutions, but this run tests only a fixed finite panel and is not a general formal or signed proof. This intentionally (2,2)-split positive control is not a non-weak ordinary prime-field ECDLP speedup. Preserve all setup, LA, memory and strongest rho/BSGS/IC requirements for the separate cold cost gate.

## Artifacts

Commit source (unmodified imported CT339 and new additive CT340 code), canonical raw JSON, source-bound report with counts/negative results, updated HANDOFF and timestamped four-heading session log on the same draft PR #4997. Keep unchanged upstream source hashes and record worker count (zero unless agents actually used), commands, source/record hashes, unfinished dependencies, and next safe step. A failure is reported and frozen, not silently replaced.
