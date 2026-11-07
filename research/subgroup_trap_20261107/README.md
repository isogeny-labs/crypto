# Subgroup traps in lift-dense factor bases: why the C1 control A0 found no relations

2026-11-07. This is a follow-up to
[`c1_twisted_line_ladder_20261027`](../c1_twisted_line_ladder_20261027/README.md)
(PR #2704) and to
[`lift_dense_factor_base_20261027`](../lift_dense_factor_base_20261027/README.md)
(PR #2686).

- The protocol was frozen before any computation, at `7ac2099b8`. Amendment 1,
  which corrects the trap condition, was also committed before any
  computation, at `fe8b2b62e`. See [`PROTOCOL.md`](PROTOCOL.md).
- The run is [`reports/run-20261107T2/`](reports/run-20261107T2/) (a first full run, `run-20261107T1`, agrees on every frozen field); the verdicts
  are in `summary.txt`. Report: [`REPORT.pdf`](REPORT.pdf). Figure:
  `fig_subgroup_trap.svg` / `.png`.

Every statement below is an unverified assertion of this session
[pcc:unverified]. The claim record is listed at the end.

## Answer

- **A0 is subfield-trapped (P1 and P2 PASS).**
  - A0's line generates K = F_Q, so all of its factor-base points lie in
    H = E(F_Q) + ker(σ_Q + 1), which is exactly half of E(F_{Q²}).
  - At k = 5 and 7 every decomposition lands there: ΣN(R) over E(F_{Q²}) is
    exactly m⁴ (14,776,336 and 4,162,314,256), the support is exactly H (484
    and 8,236 points), and the mean on H is about 30,500 and 505,000 per
    target.
  - A random target lands in H with probability about 1/(2Q²). That gives 9
    hits in 20,000 targets at k = 5, and 0 in 8,000 and 1,500 at k = 7 and 9.
    Every frozen count matches, with no exceptions.
- **2E concentration (P3 PASS).** Across all 72 frozen rows (k = 5 to 11, every
  curve and arm), the 309,870 targets with Tr(x_R) ≠ Tr(a) all have count 0.
  The other half carries 2.007× the prediction (median; range 1.60–2.43).
- **No merged claim is affected (P4 PASS).** Every line that #2686 and #2704
  call applicable has [K:F_2] = N: c2pnb, Oakley 3 and 4, and C1 A1, A2 and
  B1–B3.
- **Post hoc correction to Amendment 1.** The borderline case 2[K:F_2] = N is
  not benign. A typical target sees only the share C(4,2)p²(1−p)² of the
  decompositions (see "What it means").
- **Instrument check.** Regenerated targets reproduce all 72 frozen rows
  exactly. A second full run, `run-20261107T1`, matches on every frozen field.

## Registered verdicts

`reports/run-20261107T2/summary.txt` (scored by `subgroup_trap score`):

| prediction | verdict |
|:--|:--|
| P1: A0 `std` and `tw_a` FB ⊆ H; nonzero counts only on H; A0 `tw_b` not confined to E(F_{Q²}) | **PASS** (k = 5, 7, 9) |
| P2: ΣN over E(F_{Q²}) = m⁴; support ⊆ H; \|H\| = #E(F_{Q²})/2 | **PASS** (k = 5, 7; support = H exactly) |
| P3: zero count whenever Tr(x_R) ≠ Tr(a) | **PASS** (0 of 309,870) |
| P4: no applicable line trapped | **PASS** |

Post hoc (not registered), the borderline case:

Descriptive check on the 2E half of the frozen targets. Fraction with zero count:

| k | A0 `tw_b` (borderline) | A1 `tw_a` (full degree) | predicted generic share 6p²(1−p)² for `tw_b` |
|:-:|:-:|:-:|:-:|
| 5 | 7335/9948 (74 %) | 3076/10011 (31 %) | 0.29 |
| 7 | 2903/4051 (72 %) | 1097/3993 (27 %) | 0.37 |
| 9 | 555/747 (74 %) | 186/753 (25 %) | 0.37 |

## What it means

- **Borderline lines (post hoc).** A0 `tw_b` has u ∈ F_4, so [K:F_2] = N/2.
  - Its factor base splits into A ⊂ E(K) and B ⊂ ker(σ_K + 1). Each is a
    group of size about Q², but a line point covers only about Q of it.
  - A 4-sum reaches a generic point of 2E only with exactly two points from
    each piece. The rest of the mass sits on sets of relative size O(1/Q).
  - So the mean is right but the typical target is starved: about 73 % of
    its 2E targets get zero, against 25–31 % on a full-degree line.
  - This is algebra written after the data; the table above is descriptive.
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
  K = F_2(line, a, b), and use the line only when [K:F_2] = N.
  - [K:F_2] < N with 2[K:F_2] ≠ N traps the factor base in a proper
    subgroup (Amendment 1).
  - 2[K:F_2] = N leaves the typical target only the share
    C(n, n/2)·p^{n/2}(1−p)^{n/2} of the decompositions (post hoc). This joins the 2026-11-06 rule to skip u ∈ b^{1/4}·F_q
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
    this is accounting, not an improvement [pcc:unverified].
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

## Claim record

- `PCC-20261107-286789` (measurement; proof: `reports/run-20261107T2/validator-report.json`
, a recompute of the headline numbers from the pinned raw rows by `subgroup_trap validate`). It is not covered by a signed verdict, so every statement
  here is [pcc:unverified].
