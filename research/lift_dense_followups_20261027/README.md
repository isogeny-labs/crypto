# Lift-dense factor bases: follow-ups (2026-10-27)

Follow-ups to [`lift_dense_factor_base_20261027`](../lift_dense_factor_base_20261027/README.md)
(PR #2686). They were run in parallel by four Sonnet subagents; the main
session reproduced or independently checked each result before committing.

| study | question | result | main-session check |
|:--|:--|:--|:--|
| [`oakley_lift_dense_20261027`](../oakley_lift_dense_20261027/) | Does the twisted all-lift line exist on the Oakley EC2N groups 3 and 4 (n = 5)? | **Yes on both.** u found by root-finding on line restrictions of the quartic form, and verified directly. 200/200 sampled line elements lift (standard base: 104/200, 99/200). The c2pnb control matches on all 5 curves. This means 2⁵ = 32× relations per decomposition trial; it is a yield statement, not a cost. | Rerun of the frozen binary and seed: every non-timing field identical. |
| [`c1_twisted_line_ladder_20261027`](../c1_twisted_line_ladder_20261027/) | C1 toy ladder (E over F_16 lifted to F_{2^{4k}}, n = 4): does the twisted line give about 16× yield at unchanged per-trial cost? | **Yield:** pooled 17.33 measured against 16.92 predicted (1.024), H-Y2 PASS. **Cost:** counted Macaulay-elimination solver with the same matrix shapes, rank and Hilbert function (396/396); twisted/standard geometric mean 0.961 [0.928, 0.991], UNCHANGED within [0.8, 1.25]. H-Y1 and H-Y3 FAIL, both on the A0 control, a curve whose standard line is already all-lift. The cost is not a full F4 and not timed. | Scorer rerun reproduced the first-pass summary byte for byte; RAW and FREEZE hashes OK. After review, the scorer keeps constant-count rows in H-Y1 (48/54, still FAIL); the first pass is kept as `summary_firstpass.*`. |
| [Lean `LiftDenseUCount`](../lift_dense_factor_base_20261027/lean/) | Formalise Proposition 3 (the u-count). | **Proved:** q − 1 divides the count (any n); exact count at n = 2; the count is in {0, q−1, 2(q−1), q²−1} at n = 3. | Proof gate re-run: PASS, 26 theorems, standard axioms only. |
| [`lift_repulsion_mechanism_20261027`](../lift_repulsion_mechanism_20261027/) | Why do curve lift sets contain fewer all-lift subspaces than random sets? | The Walsh spectrum is a Kloosterman spectrum, so ΣW⁴ = 2q³ − 2q² − 3q on every curve (normalised 2, against 3 for random sets; with ψ(0) = 0). This explains about 70 % of the k = 4 deficit. The 3rd moment carries the scatter. About 48/q remains unexplained. | Independent brute-force Walsh computation on 8 curves: both moment identities exact. |

None of these measures an ECDLP cost, so there is no leaderboard, benchmark
or rho row. The c2pnb and Oakley gains remain per-trial yield statements.
C1 supports the "c_trial unchanged" assumption only for its simplified
solver. Turning the yield into a measured end-to-end improvement needs a
complete toy solve with both bases, timed via isolab-eks.

## Execution route (AGENTS.md §9a)

All three computational studies (Oakley, C1 ladder, repulsion mechanism) ran
locally on the session host, not on isolab-eks. §9a makes isolab-eks the
default for x86 CPU experiments, and exact counting is not an exemption.
The blocker: this session has no AWS identity and no `aws` CLI, so
`aws sts get-caller-identity` cannot run and no EKS job could be submitted.
No backend exception was authorised.

The results are deterministic integer counts and Walsh sums. The main
session reproduced each one, and rerunning on a different host should give
identical numbers. That does not satisfy the route rule. Each study is
therefore **local evidence; the §9a confirmatory EKS run is open**. The next
step is to submit each frozen binary and seed through `isolab-eks` from a
session with the lab identity, then diff the exact-count outputs against the
committed `reports/run-*` files. The C1 cost ratio counts multiplications,
not time, and none of these results is a leaderboard, benchmark or timing
claim.

`ATLAS_LEAD_DRAFT.yaml` is the lead record for crypto-autoresearcher. It
could not be pushed from this session, so it needs to be applied there.
