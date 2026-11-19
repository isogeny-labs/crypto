# Next research line after the PKM close-out: proposal (2026-11-19)

Status: proposal only. No experiment was started and nothing was measured for this note. Every number below is
quoted from a committed study or log and is [pcc:unverified]; this note does not re-run or re-prove any of it.

## Starting point

The PKM prime-field line is closed (`research/notes/pkm-structured-factor-base-closeout-20261119.md`). Kummer-coset,
random and small-x factor bases cost the same inside the repository's m = 2 index calculus. With a table
decomposition the cost is about `p/S + S^2`, so about `p^(2/3)` against rho's `p^(1/2)`. The whole solve costs 114 to
1,308 times the theoretical rho cost at 20 to 28 bits [pcc:unverified]. The close-out lists what would reopen it: an
oracle with per-target cost `o(S)`, a change in the distribution of the sums, or a change of group.

Success criterion in use (`docs/ic/SUCCESS_CRITERIA.md`): T0 is any gain over the baseline at any size, counted as verified only once a pcc claim exists [pcc:unverified]. T1 is a
cold, one-target, fully charged interval wholly below matched rho at a stated size. T2 is a smaller fitted exponent over
six sizes spanning 24 bits. T3 is subexponential. Distances to rho are reported as the distance remaining.

Sources read for this note: `docs/ACTIVE.md`, `tools/goal_brief.py` output, `docs/session-logs/2026-11-18.md` and
`2026-11-19.md`, `docs/ic/SUCCESS_CRITERIA.md`, and the study READMEs cited per candidate. The research-kb server and
CLI were not queried (not available in this container); the novelty statements below mean "not found in the files
read", not "not tried".

## Candidate 1: hyperelliptic and non-hyperelliptic Jacobian transfers (genus 2 to 16)

**Question.** Does the degree-2-place factor base (arms D and E of the merged genus 2-16 programme) keep a measured
advantage over the lab's 2LP arm and over matched rho as the genus grows, and where does it cross over with p?

**Why it could move the baseline.** It is the only open line with a measured cost already below expected matched rho
in some cells. The P1 table gives cost relative to expected rho of 0.43, 0.13, 0.072 and 0.024 at g = 6 for p = 31,
53, 101, 211, and 0.066 at g = 8, p = 31 [pcc:unverified]. Crossover p*(g) is about 270 at g = 6 and about 830 at g = 7
(g = 7 and 8 are extrapolations) [pcc:unverified]. Tier reachable: T0 now; T1 on a Jacobian only when the group is
public, the map is a transfer from an elliptic curve, and every phase is charged (section 5 of SUCCESS_CRITERIA).
Standalone Jacobian DLP rows are a different problem and do not count toward the prime-field ECDLP goal.

**What exists.**
- `research/ic_genus2_16_speedup_20261117/` (P0, P1, P2, `PLAN.md`), merged in PR #4598.
- Frozen follow-ups F1 (L03 base constant), F2 (L06 refreeze rerun), F3 (L01 end-to-end pencil) under
  `research/g2plus_followups_20261117/`; none has a completed run.
- P3 (non-hyperelliptic small-e models: g = 4 plane quintic with two nodes, g = 6 smooth plane quintic), not started.
- Order-free prime-field hyperelliptic IC (PR #4676, done per `docs/ACTIVE.md`), curve-transfer slice (PR #4683, open).
- Many `research/hec_*_20261118` studies on the genus-4 and pencil routes.

**Cheapest first measurement.** Run F1 (base constant) as frozen, locally, at g = 6, p = 101 and 211, with the
existing `genus_ladder` instrument. It re-measures an arm that already has a count-identity gate, needs no new code,
and tells whether the 0.024 cell is stable. Then decide on P3.

**Blockers.** The P1 cells need #J below 2^63 so that every solve is certified, which forces small p at large genus;
nothing is claimed at cryptographic p. No pcc claim, validator report or benchmark record exists for P1 or P2 (session
log 2026-11-18), so none of it can reach the leaderboard. A transfer claim needs the elliptic-to-Jacobian map and its
cost reported. No credentials needed for F1 locally.

**Effort.** F1: about one session. P3: two to three sessions (new curve models). Validation and records: one more.

## Candidate 2: Koblitz K1 small rows, generator-orbit arm and lazy orbit-member columns

**Question.** Which lever takes the generator-orbit IC arm at K1 n23, n29 and n37 below matched rho v5, not only below
strong rho?

**Why it could move the baseline.** Tier T0 now, T1 at a small size if it works. The matched-rho audit
(`research/koblitz_rho_reference_20261115/`) found the best IC arm at w = 0.1 is 0.142 of strong rho but 1.69 of v5 at
n29, and 1.005 against 3.74 at n23, 2.94 against 5.19 at n37 [pcc:unverified]. So the distance is small at n29 and
n23, which makes these the closest rows to a T1 reading. The P2 sibling build cut the k = 5 table build by 14 to 18
percent at w = 0 but ties at w = 1 by pricing, leaving 4.6 to 5.3 times rho on the Table B prime curves [pcc:unverified].

**What exists.**
- `research/koblitz_rho_reference_20261115/` (reference audit, owner decision pending on v5 below about 2^38).
- `research/ic_lead_chase_20261116/` with the frozen lazy orbit-member column sweep (c in 1, 2, 3; n in 17, 29, 31),
  queue entries `experiments/queue/EXP-LAZYCOL-N{17,29,31}-001.json`, status `blocked`.
- `research/ic_ksum_sibling_20261118/`, `research/ic_ksum_prune_20261115/`, `research/k2_decisive_20261115/`.
- PR #4565 (lazy columns), PR #4651 (K1 small rows and Table B k-sum lanes, open).

**Cheapest first measurement.** Add the missing per-step jump-index tally to v5 (a source change the audit names) so
the matched reference is fully charged, then rerun the n29 gen-orbit arm against it locally at L0. This fixes the
yardstick before any lever is tried and costs one small source edit.

**Blockers.** The lazy-column sweep needs live budget headroom in the lab AWS account (last readback over the monthly
cap), an approved spec, an image built from the bound commit, and a perf-ipc `w_cal` host. This container has no lab
AWS identity. The v5 tally change is an owner decision (audit section 5). K2 at k = 7 and 8 stalls in the blocked GF(2)
closure and D3 needs a 24 GB node.

**Effort.** Tally and local rerun: one session. A new lever on the arm: two to four sessions. The frozen sweep: one
admitted run once the budget unblocks.

## Candidate 3: Semaev decomposition-oracle constants (F-L F4 split, radical root finder, linear two-summand, root-finding 100x)

**Question.** Can the relation-phase oracle be made enough cheaper, in counted operations, to move the exact Semaev
lane toward cold one-target rho parity?

**Why it could move the baseline.** Tier T0, and only where the counted cost S moves. The root-finding plan states
that its levers change wall time and field work per addition but not the number of charged additions, so they are
constant-factor engineering and not an IC improvement unless S changes (`PLAN.md`). Lever 1 measured 2.2 to 3.1 times
over the best existing arm [pcc:unverified]. The quartic radical solver is 1.3 to 1.9 times cheaper than a fair
Cantor-Zassenhaus baseline in field multiplications, and a net loss on the (1,3) Frobenius class
[pcc:unverified]. The closed PKM result says superlinear oracles lose to enumeration, so the lever is the constant,
not the slope.

**What exists.** `research/rootfind_baseline_100x_20261117/` (RESULTS, PLAN), `research/radical_quartic_20261117/`,
`research/semaev_linear_two_summand_20261117/`, `research/semaev_oracle_replacements_20261117/` (projection join and
compiler falsifier), PR #4679 (Boolean Faugere-Lachartre split benchmark, open), PR #4682 (Semaev rho parity, open),
PR #4562 (merged split).

**Cheapest first measurement.** Land lever 1 as an oracle id with witness recovery and ledger parity against
`rootfind-mitm`, then report the ratio in both counted GAE and wall time. If S is unchanged, record it as a
constant-factor result and stop.

**Blockers.** Main's compile and clippy breakage (PR #4650 lane) and 28 examples and tests that fail to build (open item [pcc:unverified]). No
hardware PMU here, so roofline and cycle counts need the lab runner. No credentials needed for the local steps.

**Effort.** Lever 1 oracle with parity: one to two sessions. Levers 2 to 4: two sessions each. High risk of a result
that is real in wall time and invisible in S.

## Candidate 4: exact signed decision test for the hashed-network oracle (T3 route, paper level)

**Question.** Is there an exact, uniformly subexponential decision test for the short-box formulation including its
negative instances (large raw root sets with empty signed queries)?

**Why it could move the baseline.** It is the only open line aimed at T3. The merged audit deduces expected time
`2^(O(b/(log b)^(1/4)))` for arbitrary cofactor from the pinned preprints, which is `r^o(1)` (PR #4665/#4659)
[pcc:unverified]. The same studies find the standalone multiplication layer has exponential rank, and the repository's
own cost model gives the implemented solver at least `2^190` to `2^1181` against rho's `2^60` to `2^250` at 120 to 500
bits [pcc:unverified]. So the line gives no measured gain at any size; its value is in resolving whether the
exponent claim survives.

**What exists.** `docs/leads/2026-11-18/subexponential-proof/`, `docs/leads/2026-11-18/novel-decomposition-oracle/`
(REPORT, EXACT, DENSE, FOLLOWUP), `docs/leads/2026-11-18/fast-ecdlp-v2-assessment/`,
`research/btn_scaling_exponent_20261118/`, PR #4668 (done), PR #4685 (obligation graph, open).

**Cheapest first measurement.** Implement the floor-sum cancellation test of the EXACT note at two slots and check it
against brute force on small prime fields. Expected raw intersections are O(1) per the note, so a count that grows
with p falsifies the hash model before anything larger is built.

**Blockers.** Mostly mathematical: the signed existence test with uniform startup is the stated next proof
obligation. No AWS or credentials needed. Any claim needs Lean or an independent review (the lab marks AI-written
paper proofs as unverified).

**Effort.** Floor-sum test: one session. Full obligation: open-ended; treat as a research bet.

## Candidate 5: execute the frozen ecbench Phase-1 cold panel

**Question.** What is the measured cold one-target gap to matched rho, per row, on the smallest runnable Koblitz and
prime cells?

**Why it could move the baseline.** It gives no gain itself. It supplies the distances the success criterion asks to
report, and the only T1 and T2 evidence needs this panel (at least 32 targets below 64 bits, six sizes for an
exponent). Candidates 1 to 3 each need it to say whether a lever mattered.

**What exists.** PR #4681 (open), `research/ecbench_phased_panel_20261117/`, `research/ecbench_e2e_frontier_20261031/`,
the frozen specs in `research/ic_lead_chase_20261116/specs/`.

**Cheapest first measurement.** Run one frozen cell at L0 locally as a script test only (the lead-chase smoke run did
this for n = 17: 54 records, 27 replays, 0 problems), and read it as a harness check, not a measurement.

**Blockers.** Admission on EKS needs live budget headroom, an approved spec, an image and `w_cal` (same as candidate
2). No lab AWS identity in this container.

**Effort.** One cell locally: under one session. The panel: depends on budget.

## Not ranked

Graph-IC on extension fields (`research/extension_ic_graph_20261115/`) is an extension-field lead whose frozen pilot
ended with no completed IC warm-up and no ratio; it does not speak to prime fields. H115 Frobenius-Kummer
(`research/h115_frobenius_kummer_resolvent_20261117/`, PR #4625) is frozen with obligations but has no measurement.

## Ranking and recommendation

| rank | candidate | best reachable tier | first step needs credentials | risk |
|---|---|---|---|---|
| 1 | 2: Koblitz K1 small rows | T0, T1 at n23 or n29 | no | medium |
| 2 | 1: Jacobian transfers | T0, T1 only with a charged transfer | no | medium, scope |
| 3 | 3: Semaev oracle constants | T0 only if S moves | no | high, wall-time only |
| 4 | 5: Phase-1 panel | none (baseline) | yes for admission | low |
| 5 | 4: exact signed test | T3 on paper | no | high, theory |

**Recommendation: start with candidate 2.** It is the only line where the distance to matched rho is already near 1
on a validated row (1.005 at n23 and 1.69 at n29, both against v5 at w = 0.1 [pcc:unverified]) and a T1 reading at a
stated small size is therefore one or two levers away. Its first step is small and local (charge the v5 jump-index
selection, then rerun), and it fixes the yardstick that every later claim on these rows needs. Candidate 1 has larger
measured ratios but they hold at small p and large genus on a different group, and no validator report exists, so its
gain cannot yet be counted toward the prime-field goal. Candidate 3 can be real in wall time and still leave S
unchanged, which the criterion does not count. Candidate 5 is necessary infrastructure but produces no gain, and
candidate 4 has no measurable output at any current size. Candidate 1's F1 rerun can run beside candidate 2's first
step, since they share no files.

Caveat for all of it: no row here is a leaderboard entry. The validation gate needs a replay certificate, a validator
report and a matched rho reference before any row is ranked.
