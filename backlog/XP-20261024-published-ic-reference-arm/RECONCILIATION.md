# Proposal: two registers for "improvement" (not an edit of AGENTS.md)

Proposal only. `AGENTS.md` and `CLAUDE.md` are untouched; the owner decides.

## 1. The problem in the existing wording

`AGENTS.md`, "Success criterion: iterative improvement of the index-calculus baseline" (line 74),
defines the baseline as "the best current verified index-calculus pipeline, frozen with its
protocol" (the "Baseline." bullet), and says "Each landed improvement becomes the new baseline"
(the "Ratchet." bullet). `CLAUDE.md` restates it: "every verified, measured improvement over the
current index-calculus baseline is a success", and "Beating Pollard rho is the long-term goal".

Read literally, "the current index-calculus baseline" is the lab's own previous producer. Each
ratchet step is then a success, and its ratio is internal. The stated external goal is different:
beating the best *published* index calculus at matched (q, n, m). The two are conflated in
reports: every "x times" in the program is against rho or the lab's previous producer
(`ic-regime-review-20261024/REVIEW.md` §1 item 4; §3 item 5). The leaderboard's own Table B
header names the reference as matched rho only.

## 2. Proposal: two registers, never mixed in one number

**Register I, internal ratchet.** Baseline = the lab's previous best producer on the same
instances. Reported as now (delta vs previous baseline, plus gap to matched rho "as the distance
remaining"). Wording of success unchanged. Label: `ratchet`.

**Register E, external published baseline.** Baseline = the reference arm of card
`XP-20261024-published-ic-reference-arm` (frozen protocol, pinned solver builds). A row may say
"x times published" only if a Register-E reference row exists for the same (family, n, m,
instance) and unit. Label: `vs-published`. A producer is never described as improving on
"published index calculus" from a Register-I ratio.

## 3. Lines of `AGENTS.md` it would amend (quoted)

1. (line 85) "**Baseline.** For each curve and size, the baseline is the best current verified
   index-calculus pipeline, frozen with its protocol, receipts and end-to-end `S` (every phase,
   cold). Name it before you optimise." -> add: "Name which register it is: `ratchet` (the lab's
   previous best) or `vs-published` (the frozen reference arm). A report states both when both
   exist."
2. (line 100) "**Ratchet.** Each landed improvement becomes the new baseline." -> add: "of the ratchet
   register only. It never replaces the published reference arm."
3. (line 103) "**Distance to rho.** Report the gap to the matched rho on every row, and to precomputation
   rho ... where setup is excluded." -> add a third distance, "to the published reference arm
   where one exists", and say "no reference arm yet" where none does.
4. "A claim to *beat* rho still needs every reference rule" -> extend with: "A claim to beat
   published index calculus needs a Register-E row from a frozen reference-arm protocol, the same
   unit (GAE), the same instances, and a replay."
5. `CLAUDE.md` line 8 success-criterion paragraph: "Beating Pollard rho is the long-term goal" -> add
   "and beating the published index-calculus baseline is the external checkpoint on the way".

## 4. What stays the same

The "Advance and engineering are both successes" rule, the relabelling/accounting exclusions,
the "gap is the distance remaining, not a pass/fail gate" rule, and the validation gate
(`docs/VALIDATION_GATE.md`) are untouched. The proposal changes only what the word "baseline" is
allowed to mean in an external claim.

## 5. Interim practice, no rule change

Until the reference arm exists, reports that quote a multiplier add one clause: "against the
lab's previous producer, not against published index calculus (no published reference arm
exists, card XP-20261024-...)". The leaderboard Table B header and benchmarks rows could carry the
same clause as a column `register`.

## 6. Risks of the proposal

- A weak or mis-tuned reference arm makes every Register-E ratio look good. PROTOCOL §6
  (tuning parity) and the A/A controls are the guard.
- At toy sizes the reference arm may be cheaper than every lab producer, which would be a
  finding for the program, not a failure of the register split.
