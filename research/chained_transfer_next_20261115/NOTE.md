# Chained transfer: the next open case, chosen and frozen (2026-11-15)

Status: research note and a frozen protocol. No experiment was run and no
code was written or changed. Every statement about a measurement, a cost or a
prediction is `[pcc:unverified]`: no pcc claim exists for any of them.

## What was asked

Choose the next open case of the chained-transfer line, other than O3 (being
rerun separately), and freeze the protocol of its cheapest decisive
measurement. The case list comes from
`research/chained_transfer_proof_20261112/PROOF.md`, section 5 ("What is not
established") and the summary table, together with
`problems/prime-field-chained-transfer-no-advantage.md`.

## Sources checked

- research-kb: **not run.** `scripts/research-kb status` exited 127 with
  "research-kb: no executable found; set RESEARCH_KB_BIN or put research-kb on
  PATH", and `crypto-autoresearcher/research-kb/.venv/bin/research-kb` is not
  in this container. No `source_id` was obtained, so this note makes no
  novelty, "not tried" or "no prior art" claim. The negative-form queries
  ("chained transfer index calculus no advantage", "non-hyperelliptic bielliptic
  quartic index calculus", "plane quartic Diem index calculus rho") were not
  run. Rerun them before this note is quoted.
- Lead atlas: **not run.** `../crypto-autoresearcher` is absent
  (`/home/user` holds `crypto`, `cryptanalysis`, `finite-fields` only), so
  `tools/lead_atlas.py --brief` and `knowledge/leads/ecdlp/` could not be read.
  "Not tried" statements below are limited to what a search of this checkout
  found (repo paths below), not to the lab as a whole.
- Local files read (`repo:path`, read in full or the parts cited):
  - `crypto:research/chained_transfer_proof_20261112/PROOF.md` (T1 to T9, section 5)
  - `crypto:problems/prime-field-chained-transfer-no-advantage.md`
  - `crypto:research/full_jacobian_ic_rp_20261114/PROTOCOL.md` (style, O3 rerun)
  - `crypto:research/prime_leads_chase_20261113/L4-lifting/REPORT.md` (O2)
  - `crypto:research/bielliptic_quartic_20261005/README.md` and
    `crypto:src/cryptanalysis/bielliptic_quartic.rs` (the code of case O4)
  - `crypto:research/leads_chase_20261112/J1_adapter_report.md` (plane-quartic code, read for names only)
  - `crypto:src/cryptanalysis/chained_transfer/{full_jacobian_ic,negation_remeasure,ic_control,pushforward}.rs`
    (names and signatures only)

## Open cases

PROOF.md numbers only O1 to O3. Its section 5 also lists, without ids, the
hypotheses under which T5 to T7 are silent. This note gives those ids O4 to O6
so the table can name them; the ids O4 to O6 are this note's labels, not
PROOF.md's.

| id | statement | measurement exists? | cheapest discriminating test | expected cost on this 4-vCPU VM |
|---|---|---|---|---|
| O1 | No efficient cross-characteristic DLP-preserving map exists (open, not claimed). | None, and none is possible: nothing in the repository constructs such a map. `PROOF.md` section 5. | None that is a measurement. A construction, or a barrier theorem, is a research result, not a run. | Not a run. 0 compute hours; the research effort is unbounded. |
| O2 | Lifting to characteristic 0 (xedni) beats `C0/p` per trial; the only bound is conditional. | Partly. Toy-scale xedni trials and a margin table at 256 bits are in `crypto:research/prime_leads_chase_20261113/L4-lifting/REPORT.md`; the success-rate study is in `cryptanalysis:research/xedni_lift_success_20261112/` (seen as a directory listing, not read). Not a measurement on the transferred instance. | The report's own next action: measure the height-to-discriminant ratio rho0 on planted dependent configurations at 128 to 256 bits with 9 to 10 lifted points. | At least 40 h. Needs rank 8 to 9 curves over Q and lattice work; `gp`, `sage` and `cypari2` are not installed here. Estimate, not a measurement. |
| O3 | Index calculus drawing relations from all of `J_C(F_p)`. | Yes, genus 2 only: `research/full_jacobian_ic_20261113/`; rerun with `r >= 0.4p` frozen in `research/full_jacobian_ic_rp_20261114/PROTOCOL.md`. | Excluded from this choice: being rerun separately. | Excluded. |
| O4 | Non-hyperelliptic covers with `g <= d^2 - d + 1` (the smallest is `g = 3`, `d = 2`): T6 is silent and T5 does not apply, so nothing says the relations are fibres. | Partly, and not against rho. `bielliptic_quartic.rs` recovers a supplied scalar on `v^4 = x^3+ax+b` for `p <= 257` (known-answer controls). Its README states "Timing, calibrated total operations, rho comparison, candidate ID and speedup remain null", and the field-size gate is `MAX_PRIME = 257`. | Run the same line-relation mechanism at growing `p` with every native counter against the matched negation rho (this note's PROTOCOL.md). | Run: 0.5 to 1 h (stage 2 included). Code: new module, about 1 to 2 days of agent work. |
| O5 | The hyperelliptic hypotheses of T5 fail: `d > g`, reference `c^*O` not equivalent to `d` times the point at infinity, relations of weight above `g`, or `Q` in the finite set `N`. | None. Every cover measured has `d <= g` (`PUSHFORWARD.md`: `g = 2, d = 2`; `PUSHFORWARD_DEG3.md`: `g = 4, d = 3`). | Build a genus-2 curve with a degree-3 map to `E` (a (3,3)-split Jacobian) and repeat the fibre classification. Weight above `g` is, by the argument below, the O3 relation source. | Run about 1 h. Code: 2 to 4 days, because no such cover constructor exists in `src/cryptanalysis/chained_transfer/`. |
| O6 | The measured results are toy-size: `p <= 12,841` (degree 2) and `p <= 251` (degree 3). | Yes, that is the caveat itself. | Extend the subgroup-restricted arms to `p` near 10^6. | About 3 h using existing arms, if `hyper64` accepts that `p`; unchecked. T5 to T7 are argued in PROOF.md `[pcc:unverified]`, so the run checks code against theorems and adds little. |

Argument for the O5 weight remark `[pcc:unverified]` (not checked by any run):
an effective divisor `D` of weight `g + 1` with `[D - (g+1) inf]` in `S`
splits as `D = P3 + R` with `P3` one place, and then
`[R - g inf] = [D - (g+1) inf] - [P3 - inf]` is outside `S` for almost every
`P3`. Its smoothness is the generic whole-Jacobian event, with the log of
`P3` as one more unknown. So weight above `g` inside `S` is the O3 source, not a
new case. A counting reading of the same kind: with relations restricted to
`S`, the number of available classes is `r`, so rows reach at most
`r (B/p)^g / g!` against `B` columns, which is below one for every `B <= p`;
this has not been tested.

## Recommendation: measure O4 next

O4, and only O4, on the following grounds.

1. It is outside the region PROOF.md covers. PROOF.md T6 needs `g > d^2 - d + 1`, which
   is 3 for `d = 2`; `g = 3` fails it. The relations that this family's index
   calculus uses are line sections, which have weight 4 (above `g`) and class
   `K`, not fibres. So neither T5 nor T7 says what its cost is.
2. The code already exists and its own README leaves the comparison null:
   "Timing, calibrated total operations, rho comparison, candidate ID and
   speedup remain null" `[pcc:unverified]`. The gate is `p <= 257`. The measurement is therefore
   a scale-up and an accounting, not a new construction.
3. It is the one open case where the band can order arms differently across
   prices. The line relations cost field multiplications, not group
   operations, and the projected linear algebra costs multiplications mod `r`.
   At 0 GAE per native operation the arm is counted only by its group
   operations, which are few. At 0.1 and 1 GAE the linear algebra counts. That
   is the situation the §5a band and the calibrated reading are for, and the
   O3 line has not met it.
4. It is cheap: under one hour of run time, and the prediction is a growth rate,
   so a small `p` range decides it.
5. It is not O3. It uses the Jacobian of a non-hyperelliptic genus-3 curve
   only through line sections and the norm map to `E`; no Jacobian arithmetic
   is performed, and the `r` of the instance is the order of `E` itself, so
   the O3 defect (an `r` not tied to `p`) cannot recur: `r` is within
   `2 sqrt(p)` of `p + 1` by Hasse.

What the measurement can and cannot settle. The published bound for plane
quartics is `O~(p)` (as the README says) against `sqrt(p)` for rho, so a
"closed" verdict is the expected result. The factor base of this family is a
constant fraction of the group (the README's fixed replay at `p = 53` has
45 rational quartic points and 11 folded columns, about `p/5`), so even the individual
log after precomputation is not cheaper than a baby-step table of the same
size. The protocol includes a control arm that finds relations by elliptic
additions with the same factor base, so the measurement separates "the
quartic source saves group operations" from "the linear algebra dominates".

## What this note does not do

- It does not run the measurement, build any binary or change any source file.
- It does not claim the case is novel; see "Sources checked".
- It does not rank O1 or O2 as less important, only as not cheaply
  measurable on this VM.
