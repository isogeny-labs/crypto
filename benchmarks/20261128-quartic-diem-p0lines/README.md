# Quartic Diem 2LP IC: lines through P0 (frozen rule INCONCLUSIVE)

Evidence for `research/quartic_diem_p0lines_20261128`, validated with errata on commit c271f320e40. `VALIDATION.json` passes the gate checker. Every number is `[pcc:unverified]`.

- **Change.** The `p0` arm seeds the relation set with the lines through P0 and each factor-base point. Without them, the line rows leave the base-point shift free, and one linked decomposition row is spent removing it (diagnosis: PR #5269).
- **Grid.** The parent's 20 prime-order plane quartics at p = 4099, 16411, 65537 and 131101, with solver seeds 1–6, three arms and 360 paired runs. Held-out seeds 5–6 are the primary panel.
- **Decision (frozen rule): INCONCLUSIVE.**
  - **Primary panel.** The pooled paired GAE₁ ratio p0/nofb is **0.990 [0.934, 1.045]**; at w = 0 it is 1.018 [0.861, 1.181].
  - **All seeds 1–6 (secondary).** The ratio is 0.951 [0.921, 0.981].
  - **Second attempts.** 25/120 → 12/120 on all seeds, and 10/40 → 8/40 on seeds 5–6.
  - **Not an improvement.** There is no leaderboard change.
- **Mechanism (exploratory).**
  - **Overhead.** The P0 lines cost at most 3% when both arms finish in one attempt.
  - **Rescues.** A rescued run costs 0.62× its nofb pair; 15 of 25 failures were rescued.
  - **Unrescued failures.** They come from factor-base columns beyond the pair-enumeration frontier (9 of the 11 failed p0 attempts) or from too few decomposition rows reaching the core (2).
- **IC / ideal negation rho.** At p = 131101, w = 1: p0 **1.91 [1.82, 1.99]** against nofb 2.02 [1.85, 2.20]. UNRESOLVED at every prime.
- **Errata.** E1–E3, in [`ERRATA.md`](../../research/quartic_diem_p0lines_20261128/ERRATA.md).

Files: `figure.svg`/`figure.png`, `report.pdf`, `validator-report.json`/`.md`, `replay-certificate.json`, `VALIDATION.json`.
