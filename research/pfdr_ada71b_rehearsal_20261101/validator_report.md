# Validator (review-breakthrough): EXP-PFDR-ada71b rehearsal and proposed lead disposition

The routing was correct: this is a closure-type disposition, so rule 12 applies. V means I verified it by my own computation. R means it is reasoning only.

**Blindness.** Stage 1 used only the named inputs, with one leak to disclose: I saw the `git log` subject "reading O-SUB-BE", which gives the outcome category but no numbers. I hashed my blind outputs at 06:38:37Z and lifted blindness on purpose at 06:38:52Z.

## 1. Blind re-derivation
- **Sizing (V).** n_curves re-derives exactly in all 10 (bits, θ) cells. γ_design matches to 1e-16, and the per-curve plant sizes L follow from the seed rule.
- **Record integrity (V).**
  - 20151 records, all ok; every cell has curves c = 0..n−1.
  - Curve, null, plant and Z/N fields are identical across d.
  - My own order check (Miller–Rabin, Hasse bound, N·P=O) passes on all 6232 curves.
  - Every Z/N count equals the integer-model count.
- **Counts without the kernel (V).** My own counter recounted all 134,340 null and 20,151 structured counts, plus all 26 counts (plants included) in 36 random records. Zero mismatches.
- **Slope δ̂ and 95% CI per line:**
  - θ.30: d1 +0.0047 [−0.0042, 0.0138]; d2 −0.0044 [−0.0127, 0.0039]; d3 +0.0047 [−0.0042, 0.0139].
  - θ.35: d1 +0.0070 [−0.0006, 0.0145]*; d2 −0.0001 [−0.0079, 0.0080]; d3 +0.0066 [−0.0012, 0.0139].
  - All six are valid (minimum M 3670.5) and all read SUB-BE. *The θ.35 d1 lower bound sits on the SUB-BE / SUB-BE-GROWING boundary across bootstrap seeds; either class gives the same reading.
- **Controls.**
  - C3: worst |δ̂ − γ_design| is 0.0128; plant classes as required.
  - C4: 0.6992 and 0.6452, both ALIVE.
  - C5: all 6 intervals contain 0; none ALIVE.
  - C6: mean −0.243, sd 0.915.
  - Selftest passes; the frozen check.py exits 0 on selftest, pilot and census-main.
- **Reading: O-SUB-BE.**

**Where the producer differs.**
- δ̂, S, M, κ, z, z_cal and all 22 classes are identical.
- CI endpoints differ by at most 0.0008 (bootstrap noise).
- Producer's C6 sd of 0.8995 uses ddof=0 (ddof=1 gives 0.9149); both pass.
- Upper95 κ at 32 bits: producer 1.105, mine 1.110.
- Max upper95 δ: producer 0.0144, mine 0.0141–0.0145.

## 2. Verdicts
- **Reading reproduces:** yes (V).
- **Controls valid:** yes (V).
- **Records:** the evidence is adequate, but it is not admissible as a receipt.
  - **Missing:**
    - A Coordinator-committed snapshot. The jsonl files exist only in /tmp and as untracked .gz copies; commit 5f19b996 is on the producer's branch.
    - commit and dirty fields; both are null.
    - The receipt, launch, environment and check logs that the handoff requires.
    - Runs inside the handoff's write scope; they were written outside it.
  - **All of this is paperwork.** The counts are deterministic and I recounted them, so a supervised rerun adds provenance, not new information.
  - **Formal verdict: incomplete.**

## 3. The disposition
**Supported (V):**
- κ shows no growth: max upper95 δ is about 0.014, against 1/3.
- No detectable excess: per-rung κ 0.916–1.058; 28 of 30 intervals contain 1 (the other two are deficits); max |z| 2.15 (pure noise would give about 2.32); pooled upper95 κ ≤ 1.043 per line.

**Not supported as drafted:**
- (a) It sets the lead's own `disposition` field. exhausted_at_scale requires "no named mechanism left", but FAM-1, bbc53e, log-GAP growth and target preselection remain open. Tools read the field, not the YAML comment beside it.
- (b) "No structural relation source on F_p" claims more than these six lines measured.
- (c) The obstruction block lacks kind, revisit_when and measured_by, and the forecloses list is not narrowed.
- (d) (R) The class bounds only the elasticity of κ. It does not exclude κ−1 up to about 0.1 at 32 bits, or a small term growing faster than N^(1/3) that has not emerged by 32 bits. State the model's level gap (about 46c at 16 bits, 1860c at 32 bits) as a model quantity.
- (e) The precision is overstated: write 0.014 and 1.11.

**Wording I would require.** Put it in a new scoped row; the parent lead stays open_next_step with the FAM-1 ladder as its next step.
```
disposition: exhausted_at_scale
obstruction:
  kind: measured
  statement: >-
    x-coordinate window and rank-2/3 label-GAP point bases (theta 0.30/0.35, b 28-2352),
    prime-order E/F_p, 16-32 bits, h=2: no detectable growth of kappa (max upper95 delta
    0.014 vs 1/3) and no detectable excess (kappa 0.916-1.058; upper95 <= 1.14 per rung,
    1.11 at 32 bits). Bounds elasticity only; kappa-1 below ~0.1 not excluded.
  measured_by: [admitted EXP-PFDR-ada71b RUN ids]
  revisit_when: rungs above 32 bits; other theta; h >= 3; a label base with a named mechanism
not_covered: [FAM-1, log-GAP growth, bbc53e, target preselection, j=0/1728]
forecloses: [pair-sum excess of x-window/label-GAP point bases at h=2, 16-32 bits]
```

## 4. CONCUR-WITH-CONDITIONS
1. Admit the records: either a supervised run whose canonical digests match main fe4b23ab… and pilot 73612c5d…, or a Coordinator archive of these exact bytes.
2. Record the disposition as the scoped row above.
3. A committed Coordinator decision cites this review.

If the text is applied literally to the lead's own field, my line is DISSENT.

```yaml
validation_report:
  run_ids: [RUN-PFDR-a4895b, RUN-PFDR-4fa30b, RUN-PFDR-efce3d]
  verdict: incomplete
  independent_recomputation:
    recomputed: [sizing, gamma_design, S/M/kappa/z, 22 slopes and bootstraps, C3-C6, reading]
    disjoint_checker: validator-written counter, integer plant model, Miller-Rabin order check
    on_producer_word: [plant counts in unsampled records, pilot null counts, red-team injections]
  read_sibling_reports: true (Stage 2, deliberate)
```
