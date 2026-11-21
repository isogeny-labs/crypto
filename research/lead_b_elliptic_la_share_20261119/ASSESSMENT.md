# Lead B: does the factor-base rebalance lever exist on the elliptic IC lanes? No-go.

Desk assessment, 2026-11-19. No new measurement was made: every number is copied from committed
files, and three were spot-checked against the files (marked ✓). Every number is `[pcc:unverified]`.

## Hypothesis and kill criterion (stated before reading the data)

- **Hypothesis.** In genus-4 hyperelliptic IC, linear algebra was 76–81% of native work at the
  default factor base, and shrinking the base (cc 2.0 → 1.4) cut cost 12–17%
  (`research/genus4_prime_order_20261118`). The hypothesis is that the elliptic IC lanes are also
  LA-bound with an untuned base.
- **Kill criterion.** If the LA share is below ~50% on a lane, or its base is already tuned per
  size, the lever is weak there, and we stop.

## Findings

| Lane | Base-size rule | Already tuned? | LA share | Evidence |
|---|---|---|---|---|
| Prime `ic.pipeline` (mitm pair table) | `size=N` or `size=auto` = round(0.5·r^(1/3)) | Yes, per size. ✓ Over-shrinking raised w = 1 cost 13–19% at 24 and 28 bits. | 0% of charged GAE; row ops 0.3–2.7% of native | `research/prime_base_oracle_tuning_20261029/README.md:28-30,84`; `benchmarks/20261107-ic-auto-base` |
| Table-B k-sum (prime and binary) | base A = 9–14, frozen from train | Yes (bracketed at 0.71×/1.41×) | 0.0–0.4% | `research/ic_tableb_ksum_20261115/PROTOCOL.md`; `docs/ic/LEADERBOARD.md` |
| Koblitz k-sum fold / lazy orbit | orbit columns | Swept as arms | negligible (6 row ops) | `research/koblitz_ksum_fold_study_20261115/validator/unpriced-n23.json` |
| Koblitz compact orbit (N83) | K per n | Yes (n = 41: 100–125; 53: 240–280; 61: 500) | LA uncharged in its unit; Table C 0.1–2.7% | `research/ic_improve_20261015/02-freerel-production`; `research/n83_padd_rank_20261019` |
| Semaev root-finding | 8 columns | A best-\|F\| block was swept | trivially small | `research/semaev_rootfind_e2e_20261012` |
| F_{p^k} D4 | 7 columns | not needed | ✓ 9·10⁻¹⁰ % | `research/gaudry_quartic_d4_ecbench_20261013/README.md:26-34` |

**Verdict: no-go on every elliptic lane.** Linear algebra is at most ~3% of cost, and the bases are
already tuned. The genus-4 lever came from the Jacobian lanes' large factor bases (p^(1−1/g)
columns), and the elliptic lanes do not have one.

## Redirect: where the lever does exist

The genus-3 plane-quartic GHS lane is the one place outside hyperelliptic genus 4 where LA dominates.
In it, E0/F_{q^3} → genus 3 over F_q (Diem). ✓ The phase shares at the top size, from
`research/quartic_n4v2_20261020/README.md:122-124`, are:

| Arm | Linear algebra | Relations |
|---|---|---|
| B0 | 87% | 13% |
| D | 82% | 18% |
| DL | 64% | 35% |
| DL1 (D + SGE + scalar Lanczos) | 34% | 65% |

- **Why this lane matters.** It is an elliptic-curve transfer target, so it is where leads B and D
  meet.
- **What was already done.** SGE and Lanczos are applied.
- **Not found.** No factor-base-size sweep against the LA/relations balance for DL1 was found.
- **Candidate next step.** A frozen base-size sweep for DL1 on held-out curves, mirroring the
  genus-4 prime-order part A. This is the only remaining place where lead B's mechanism could pay,
  and it is on a lane that transfers from an elliptic curve (a GHS-weak class only).

## Sources checked

- Desk research by a read-only agent over `src/cryptanalysis/ic_framework/plugins.rs`,
  `research/prime_base_oracle_tuning_20261029`, `research/ic_tableb_*`,
  `research/koblitz_ksum_fold_study_20261115`, `research/n83_*`, `research/semaev_rootfind_e2e_20261012`,
  `research/gaudry_quartic_d4_ecbench_20261013`, `research/quartic_n4v2_20261020`,
  `.claude/skills/dreg-triage`.
- Three numbers re-verified by hand (✓).
- research-kb was not available in this container.
