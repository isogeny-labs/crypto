# Prime-field IC closure: report on the seven follow-ups (2026-11-15)

Items 1–4 (m8-runs, hyp-G, f4-model, lifting) ran in parallel, then items 5–7 (lead-atlas, prior-art, pcc). Every result below is **[pcc:unverified]**. None of it improves on the index-calculus baseline, so no leaderboard row or `benchmarks/README.md` row is affected and no gap to rho needs reporting. Nothing has been committed, pushed or opened as a PR. The pcc work is held in a local `git stash`.

**What was blocked for every item:**
- **research-kb was not checked.** It could not be reached (no executable, no MCP tool, server source is vault-encrypted). Items 2 and 3 did not query it.
- **No EKS runs.** The AWS keys were rejected by STS, and `aws`/`kubectl` are not installed.
- **No S3 uploads** of raw data or checkpoints.
- **No complete `bench.record/v1`** for any item.
- **No director/executor card relay.**
- **Conductor** is not installed.

## Per item

### 1. m8-runs: PKM F4 degree at m = 8–10, plus repeats at m = 6–7
- **Outcome:**
  - m = 8, 9, 10 are **INCONCLUSIVE** under the frozen rule. D stays at 7 for 3–4 N steps, then the next rung ran out of memory or time (censored at D ≥ 8, or ≥ 7 for m = 10).
  - m = 6 and m = 7 re-read **KILL**.
  - An extension run of m = 8, N = 12 with 10 GB and 1800 s was censored again.
- **Checker:** sound-with-gaps.
  - Slopes, spans, exponent bounds and check counts were re-derived and match.
  - Re-running m = 7 at N = 7 and N = 8 gave D = 6 and D = 7 again.
  - Each KILL rests on a single unit step in D. For m = 7 that step is at the smallest cell.
  - The two draws per rung are effectively one observation.
  - Wall-capped cells ran on a VM with load around 4.8, so they are not reproducible bounds.
- **Change to the closure's standing:** none. A4-PKM is still a toy-scale, fragile KILL at m = 4–7 and still open at m ≥ 8. Calling the m = 6/7 repeats "confirmations" overstates them.
- **Blocked:**
  - The deciding cells (m = 8 at N ≥ 12, m = 9 at N ≥ 13, m = 10 at N ≥ 12) need more than 10 GB and more than 30 min each, which means EKS with a node of at least 64 GiB.
  - A degree-7 closure certificate at those cells would turn "≥ 8" into a value.
  - The benchrec record is still open: it was initialised after the run started and was not wrapped by `benchrec run`.
  - Raw data has not gone to S3.

### 2. hyp-G: hypothesis G and Theorem C
- **Outcome:**
  - **G as literally stated is false at toy scale.** Clause (ii) fails while clause (i) holds on 20 of 65 instances.
  - **No counterexample to how Theorem C uses G.** At the C2 threshold, none of the 11 failures meets C1's size condition.
  - **New proofs, on paper only:**
    - Lemma L (the line lemma);
    - Prop T (δ ≥ k − m + 1 for every target);
    - RM-T (the Plotkin-type floor that PROOF.md only conjectured);
    - Theorem C′, which needs no hypothesis but holds **only in the XL/RAIC model**, with k ≥ m and no 2-torsion abscissa.
  - **Gap found in PROOF.md:** its Theorem C argument misses that Δ_R depends on the target.
  - **C′ floor clears rho** asymptotically for 3 ≤ m ≤ 11. At concrete sizes it clears only m ≤ 7 at 128 bits and m ≤ 8 at 256 bits.
- **Checker:** sound-with-gaps.
  - All scripts reproduced byte-identically.
  - Lemma L, Prop T, RM-T and Lemma Δ were checked line by line with no errors found.
  - C′ inherits PROOF.md Lemmas 4, 6 and 9 and axioms A1–A3, which were not re-checked.
  - Lean covers only two trivial lemmas.
  - The preset was written after the grid launched.
  - 65 of 80 grid instances were covered.
  - No novelty: Lemma L is a BCH-type bound and RM-T is the standard Plotkin argument.
- **Change to the closure's standing:** Theorem C no longer depends on G, but only in the XL model, as a paper proof, and with an exponent floor that holds for bounded m only. A4-PKM under F4 is unchanged.
- **Blocked:**
  - m ≥ 12 asymptotically (m ≥ 9 at 256 bits) needs a weak-degree floor above τ_m·mk.
  - F4/polynomial calculus and the Boolean-bits case were not attempted.
  - Prop T, RM-T, Lemma Δ and C′ are not in Lean.
  - The 15 MB of raw data is not in S3.

### 3. f4-model: Theorems A and B under F4
- **Outcome:**
  - **Theorem A** holds for any solver. It uses only "an attempt costs at least 1", and the argument is on paper.
  - **Theorem B under the first-fall-degree model** holds for m ≤ 4, but the result is near-tautological.
  - **Theorem B under msolve's actual F4:** there is no theorem. On cost measured as nnz or time, the measured routes behave like exponent 1 at toy sizes.
  - **A5 at m = 4 is inconclusive** (pre-asymptotic).
  - **Boolean bits at m = 4 has no data:** msolve segfaults.
  - **New top-form lemma** (m ≤ 4): it gives D_ff exactly and explains why F4 runs at degree d + constant while XL runs at about m·d.
- **Checker:** sound-with-gaps.
  - The headline sets F4 nnz against XL **column counts**, so it does not show that F4 costs more than XL.
  - The claim that the minimum proxy is "always rows reduced" is false for bits m = 3.
  - The "267 cells" are unique tags; the file has 275 rows.
  - The top-form lemma is partly checked by sampling, not proved over Z.
  - The Koszul-syzygy step is unjustified in the note, though it holds.
  - The preset formula was corrected after the sweep, so verdicts quoted with the corrected formula are post hoc.
- **Change to the closure's standing:** "F4 escapes XL" is now quantified on the degree side only. There is still no F4 or polynomial-calculus lower bound. A5 at m = 4 and Boolean bits at m = 4 stay open.
- **Blocked:**
  - A degree or size lower bound for polynomial calculus on Semaev systems.
  - The top-form lemma for m ≥ 5.
  - The graph family under the first-fall model.
  - A5 at m = 4 needs d ≫ 24, and d = 16 already took about 680 s.

### 4. lifting: Silverman–Suzuki with growing rank, and 256-bit lifting with r ≥ 7
- **Outcome:**
  - **Lemma 3H (paper only):** excludes r = 7, 8, 9 at 256 bits **if** the height ratio ρ is at most 32.6, 10.8 and 4.94 respectively. Without a proven height ratio these cases stay open; that would need λ ≥ about 41 nats for r = 7 and about 268 nats for r = 8.
  - **r = 2 without an assumption:** whether it is excluded depends on the lift sizes. The margin is +7.0 with the previous round's lifts and −7.5 with this round's.
  - **Silverman–Suzuki with lifts from a table of known points:** ruled out, cost at least √(2q).
  - **Silverman–Suzuki with fixed rank:** ruled out asymptotically.
  - **Direct lifting with rank s = Ω(log p / log log p):** open.
  - **Bug in the previous round's `min_form`:** it inflated ρ_G. Its "largest ρ_G 2.68" was not re-checked.
- **Checker:** sound-with-gaps.
  - The numbers reproduced and both manifests verify.
  - Turning Lemma 3H into a probability bound relies on a random-map heuristic.
  - "Excluded" must read "conditional on a proven ρ".
  - "s₀ = 13–31 is enough" should read "not excluded by counting".
  - The 3.58× calibration factor comes from a truncated run.
  - The DET positive control is 45 of 48 trials, not 45 of 45.
- **Change to the closure's standing:** the r ≥ 7 margins at 256 bits are now conditional exclusions, not unconditional ones. The table-lift and fixed-rank Silverman–Suzuki variants are recorded as ruled out. The asymptotic lifting result (Theorem 2) is unchanged.
- **Blocked:**
  - A proven Lang-type height bound with an explicit constant.
  - A toy for r = 9.
  - The calibration runs at e = 16 and e = 32.
  - Reading Silverman–Suzuki 1998 and Huang–Kueh–Tan in full.
  - Lemma 3H in Lean.
  - Raw runs in S3.

### 5. lead-atlas: closure fold into crypto-autoresearcher
- **Outcome:**
  - 13 existing leads were updated: annotations, evidence paths and review dates.
  - 1 new lead, `LEAD-ICE-semaev-solver-degree-lower-bound`, as `open_next_step`.
  - A fold note was written.
  - **No disposition changed.**
- **Checker:** sound-with-gaps.
  - The patch applies cleanly to `69d906dc6`.
  - No disposition line was removed.
  - All evidence paths added by the fold exist on main.
  - 8 dangling paths remain; they were already in the leads before the fold.
  - The "18 errors before and after" result and the `--match` ranking were not re-run.
  - The fold was built against crypto `bb70178877`; main is now `cbb393deea`.
- **Change to the closure's standing:** the atlas now records the closure's routes. No route is moved to closed.
- **Blocked:**
  - Commit, PR and `--stamp` in an unlocked crypto-autoresearcher checkout.
  - Four decisions are left for you:
    1. whether to split the lifting lead;
    2. whether to close the class-representative lead;
    3. whether to settle A9 (BKK) with one msolve comparison;
    4. what the next PKM step should be.

### 6. prior-art: prior-art check of the closure note
- **Outcome:**
  - **The required research-kb check was not done.** The search used fallbacks only: the lead atlas, repo grep and abstracts found on the web.
  - **Theorems A and B are pre-empted in substance** by the repo's own 2026-10-31 note (`RESEARCH_ALGEBRAIC_IC_LOWER_BOUND.md`) and by Yokoyama et al. (JMC 2020). Any novelty claim for them should be dropped.
  - **The "PKM beats rho at m = 8" reading depends on the cost model.** Under the 10-31 rule and the 11-13 continuation it reads as KILL. Which rule is right is undecided.
  - No route's verdict is overturned.
- **Checker:** sound-with-gaps.
  - `bars2.py` reproduced byte-identically.
  - The 10-31 note's H_C/H_E are **not** the checker's two hidden assumptions; they are related but different.
  - The 10-31 Theorem 1 matches Theorem B only at the exponent level.
  - The closure ledger **does** cite Yokoyama, though for A6 only.
  - A `kbsrc/` directory was not reported.
- **Change to the closure's standing:** Section 3 (Theorems A and B, and K1) is downgraded from new to re-derivation. The m = 8 PKM upside now depends on which cost model is right.
- **Blocked:**
  - research-kb protocol.
  - Reading the primary papers in full.
  - Reconciling the two PKM cost rules (s < m−3 versus c < (m−1)/2).

### 7. pcc: claims for the closure's main results
- **Outcome:**
  - Five `statement`/interpretive claims were filed: `PCC-20261115-5749d6` (Thm A), `-19b006` (Thm B), `-3a77d5` (routes ruled out at toy scale), `-51ffd7` (PKM m = 4–7), `-ffa5b1` (lifting).
  - All stand **unverified**, and statement claims cannot become verified.
  - `canary`: 69/69. `ledger verify`: chain ok, 1102 entries.
- **Checker:** sound-with-gaps.
  - All 74 SHA-256 pins match, but 22 of them resolve only against main.
  - The PKM manifest checks 25/25 only with main and the mirror overlaid.
  - The claim scopes are honest.
  - The checker's verdicts exist only inside the closure note.
- **Change to the closure's standing:** its results are now citable as `[pcc:ID]` with standing unverified, and nothing more.
- **Blocked:**
  - The ledger applies cleanly only while main stays at `cbb393de`. Re-filing on a newer main gives new claim ids.
  - There is no path to `verified`.
  - The claim scopes predate items 2–4 and 6. For example, the Thm A and B claims do not reflect the prior-art finding, and the scope text "hypothesis G open" is now partly answered. The scopes need a refresh before citing.

## Files that should land in a PR

Mirror root: `/tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/followups/<lab>/repo/`

**Before any of this lands:**
- Apply the checker's wording corrections listed above to each README/NOTE.
- Upload raw data to S3 by SHA-256.
- Write or close the `bench.record/v1` records.
- Run a research-kb pass.

### Target repo: crypto (`/home/user/crypto`, current main `cbb393deea`)

- **m8-runs/repo**
  - Land:
    - `research/pkm_m8_runs_20261115/` (26 files: README, tables, analysis, checks, preset and extension, graphs, `scripts/`)
    - `benchmarks/records/20261115T1919Z-pkm-kummer-m8-f4-degree.json`, which must be closed or re-run under `benchrec run` first
  - **Exclude:**
    - `.git/` and `.benchrec/` (scratch).
    - `.resume/RESUME.md`: main tracks a different effort's file at that path.
    - `benchmarks/progress/{PROGRESS.md,progress.json,progress.pdf,progress.svg}`: these were generated in a scratch repo, so regenerate them on the real tree instead.
- **hyp-G/repo**
  - `research/prime_field_ic_closure_20261112/finish/theorem/hypG/`: `NOTE.md`, `code/` (15 files), `lean/` (`LineLemma.lean`, `check.log`), `out/` (78 files including `PRESET_hypG.txt` and `RAW_MANIFEST.sha256`; raw `out/exh/` not included)
- **f4-model/repo**
  - `research/prime_field_ic_closure_20261112/f4-model/`: `README.md`, `PRESET.txt`, `MANIFEST.sha256`, `code/` (6 files), `out/` (10 files including `f4_vs_xl.{svg,png,pdf}`)
  - The Semaev pickle and `runs/` (19 MB) stay out of git and should go to S3.
- **lifting/repo**
  - `research/prime_field_ic_closure_20261112/finish/lifting/followup_20261115/`: `README.md`, `MANIFEST.sha256`, `code/` (12 files), `out/` (12 files including `margins_256.svg` and `RAW_SHA256.txt`)
- **prior-art/repo**
  - `research/prime_field_ic_closure_20261112/SOURCES_CHECKED.md`
- **pcc/repo**
  - Evidence: 54 files under `research/prime_field_ic_closure_20261112/`, covering:
    - `chase/equation-budget-B/`
    - `closure/{galois,ratmap,ratmap-verify,rootfam,semaev-scaling,semaev-verify}/`
    - `finish/{pkm-msolve,pkm-msolve-check,lifting,lifting-check,theorem,theorem-check}/`
  - `pcc/ledger.jsonl` and `pcc/claims/PCC-20261115-{5749d6,19b006,3a77d5,51ffd7,ffa5b1}.json`: **do not copy these if main has moved past `cbb393de`.** Instead copy the evidence in, then run `followups/pcc/file_claims.sh` on fresh main, which gives new ids.

### Target repo: crypto-autoresearcher (`isogeny-labs/crypto-autoresearcher`, vault-encrypted; apply in an unlocked checkout)

- **lead-atlas/repo**, under `knowledge/leads/ecdlp/`:
  - 13 edited leads:
    - `ic-engine/LEAD-ICE-prime-field-semaev-solving-degree.yaml`
    - `ic-prime/LEAD-ICP-{algebraic-membership-factor-base,char0-lifting-xedni-smoothness,extension-field-embedding,generic-table-decomposition,isogeny-transported-relation-yield,pkm-tower-algebraic-oracle,prime-ic-constant-engineering}.yaml`
    - `structure/LEAD-STR-{class-representative-ic-selection,special-j-ic-semaev}.yaml`
    - `transfer/LEAD-TRF-{global-lift-small-height,padic-canonical-section,same-field-genus-g-embedding}.yaml`
  - 1 new lead: `ic-engine/LEAD-ICE-semaev-solver-degree-lower-bound.yaml`
  - The fold note: `notes/2026-11-15-prime-field-ic-closure-fold.md`
  - The same changes as one patch: `/tmp/claude-0/-home-user/ad79db08-cd5f-5158-81c2-aa769b65fafd/scratchpad/followups/lead-atlas/atlas-fold.patch` (against `69d906dc60`). Re-check the crypto evidence paths against current crypto main before landing.

### Target repo: cryptanalysis
- None.