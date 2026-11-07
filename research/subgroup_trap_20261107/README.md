# Subgroup traps in lift-dense factor bases: why the C1 control A0 found no relations

2026-11-07. This is a follow-up to
[`c1_twisted_line_ladder_20261027`](../c1_twisted_line_ladder_20261027/README.md)
(PR #2704) and to
[`lift_dense_factor_base_20261027`](../lift_dense_factor_base_20261027/README.md)
(PR #2686).

- The protocol was frozen before any computation, at `7ac2099b8`. Amendment 1,
  which corrects the trap condition, was also committed before any
  computation, at `fe8b2b62e`. See [`PROTOCOL.md`](PROTOCOL.md).
- The run is [`reports/run-20261107T1/`](reports/run-20261107T1/); the verdicts
  are in `summary.txt`. Report: [`REPORT.pdf`](REPORT.pdf). Figure:
  `fig_subgroup_trap.svg` / `.png`.

Every statement below is an unverified assertion of this session
[pcc:unverified]. The claim record is listed at the end.

## Answer

__ANSWER__

## Registered verdicts

__VERDICTS__

## What it means

- **A0 was never a valid positive control.**
  - It is y² + xy = x³ + 1, defined over F_2, so the standard line F_Q
    generates only K = F_Q.
  - Its factor base therefore lies in H = E(F_Q) + ker(σ_Q + 1), a group of
    about Q² points, while the targets are sampled from all ≈ Q⁴ points.
  - The identity prediction (about 15.5 per target) is correct on average.
    All of that average sits on a fraction ≈ 1/(2Q²) of the targets.
  - The H-Y1 and H-Y3 failures in #2704 are explained exactly. The verdicts
    stay as registered; nothing is re-scored.
- **No applicable line in the merged studies is trapped (P4).** That covers
  c2pnb208w1 and 368w1 (all valid u, including the 180 and 330 u in
  F_{2^16}), Oakley groups 3 and 4, and every twisted line on C1 A1, A2 and
  B1–B3. The yield and cost-model rows of #2686 and #2704 stand.
- **Construction rule.** Before using a line, compute [K:F_2] for
  K = F_2(line, a, b). Reject it when [K:F_2] < N and 2[K:F_2] ≠ N
  (Amendment 1). This joins the 2026-11-06 rule to skip u ∈ b^{1/4}·F_q
  ([`lift_dense_torsion_pairing_20261106`](../lift_dense_torsion_pairing_20261106/README.md)).
  - A0's line is in both classes: b = 1, so b^{1/4}·F_Q = F_Q.
  - That study left the n = 2 subfield explanation (π_q P = ±P) as algebra
    written after the data. Here the same mechanism is measured exactly at
    n = 4 (P2), with H of index 2 in E(F_{Q²}) as predicted.
- **The 2E factor (P3) is an accounting fact, not a gain.**
  - Every C1 factor base has Tr(x) = 0, so with Tr(a) = 0 it lies in 2E.
    Every target off 2E has zero decompositions, and the 2E half carries
    about twice the prediction.
  - Cryptographic targets have odd order and lie in 2E. For even n, the
    standard base lies in 2E too, so the twisted/standard ratio is unchanged.
  - Derived, not tested here: for odd n (Oakley, n = 5) the plain F_q base
    has Tr(x) = Tr_d(y), which is not constant. The twisted line's yield on
    cryptographic targets is then 2^{n+1} = 64 times that of F_q, not 2^n.
    The extra factor 2 is available to any trace-zero base. It is the
    cofactor mechanism already closed in `LEAD-ICB-cofactor-torsion-quotients`,
    so against a trace-zero comparator the gain stays 2^n. Under AGENTS.md
    this is accounting, not an improvement.
- **ECC2K-130.** N = 131 is prime, so K ∈ {F_2, F_{2^131}}, and no factor base
  with an abscissa outside F_2 is subfield-trapped. The 2E factor there is the
  known 2-bit cofactor cap.

## Not claimed

- Any ECDLP cost, leaderboard, benchmark or rho row.
- The odd-n factor 2^{n+1} is derived, not measured.

## Sources checked

KB unavailable: `scripts/research-kb` reports no executable, and the
research-kb MCP server is not connected in this session. I used the lead atlas
and `rg` over crypto, crypto-autoresearcher and cryptanalysis instead:

- `crypto:docs/ic/factor-base-encyclopedia/REPORT.typ` (§ obstructions):
  "Subgroup confinement" is a known general obstruction. This study gives the
  exact condition for lift-dense lines and measures it.
- `crypto:research/lift_dense_torsion_pairing_20261106/README.md`: the T-stable
  line rule and the unmeasured n = 2 subfield explanation; see above.
- `crypto-autoresearcher:knowledge/leads/ecdlp/ic-binary/LEAD-ICB-cofactor-torsion-quotients.yaml`:
  the E/2E cofactor mechanism (closed_obstructed); P3 is an instance of it.

## Reproduce

```sh
cargo build --release
C=../c1_twisted_line_ladder_20261027/reports/run-20261027T1502Z   # hydrate yield_k{5,7,9}.jsonl from the artifact store first
R=reports/run-20261107T1
./target/release/subgroup_trap audit ../lift_dense_factor_base_20261027/reports/run-20261027T0356Z/c2pnb_input_binary-curve-params.txt \
    ../oakley_lift_dense_20261027/reports/run-20261027T1445Z/rows.json > $R/audit.jsonl
for k in 5 7; do ./target/release/subgroup_trap mass $k >> $R/mass.jsonl; done
for k in 5 7 9 11; do ./target/release/subgroup_trap c1 $C $k > $R/c1_k$k.jsonl; done
./target/release/subgroup_trap score $R
python3 plot.py $R    # layout only
```
