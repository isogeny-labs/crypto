# Autoresearcher harness: state of the world and improved ideas (2026-11-10)

A measured read of the research harness (isogeny-labs/crypto-autoresearcher,
with its crypto and cryptanalysis mirrors) one month after the
[track-record review of 2026-10-06](https://github.com/isogeny-labs/crypto-autoresearcher/blob/main/docs/track-record-review-20261006.md),
and a revised, ranked set of ideas for improving it. Part 1 is measured and
says how; Part 2 is a literature brief with its verification status; Part 3
is proposals, each with the mechanism that enforces it, the number that would
show it worked, and the observation that would falsify it. Nothing here is
evidence about ECDLP. Every number is `[pcc:unverified]`: it was computed in
this session and nobody else has recomputed it.

Figure: [`autoresearcher-harness-review-20261110.svg`](autoresearcher-harness-review-20261110.svg)
(funnel on 2026-10-06 and 2026-11-10, and the 34-day flow).

## 0. Provenance and limits

- Harness checkout: `isogeny-labs/crypto-autoresearcher` at `3ba0739cb9`
  (main, 2026-11-09 18:50 -0800), blobless clone, history to 2026-07-15,
  sparse checkout holding `ledger/`, `coordination/`, `experiments/*/
  specification.yaml`, `experiments/*/runs/*/manifest.*`, `docs/`, `tools/`,
  `orchestration/`, `evals/`, `problems/`. 96,243 tracked files were off
  disk; `portfolio_kpis.py` ran with `CRYPTO_AR_ALLOW_SPARSE=1` and says so
  in its header. Counts that depend on run directories other than the
  manifest (notably `needs_materialization`) are affected and are marked.
- crypto at `db728e89d5`, cryptanalysis at `fe751b09` (both main,
  2026-11-10).
- Not available in this cloud session, so not used: the `research-kb` MCP
  server and `scripts/research-kb` (no executable), Conductor, `ecdb`. The
  Oct 6 review and the harness's own tools are the primary sources instead.
- Flow counts "since 2026-10-07" are files added to `main` by
  `git log --since=2026-10-07 --diff-filter=A --name-only` over `ledger/`,
  `experiments/` and `coordination/sessions/receipts/`. They count records,
  not merit.

## 1. State of the world, internal (measured)

### 1.1 The funnel, then and now

| stage | 2026-10-06 (review) | 2026-11-10 (this read) | added 2026-10-07..11-10 |
| --- | ---: | ---: | ---: |
| proposals `IDEA-*` | 2,319 | 2,899 | 575 |
| hypotheses `H-*` | 1,945 | 2,264 | 278 |
| experiment specifications | 2,556 dirs | 2,863 (1,632 `approved`, 580 `draft`, 448 `review_required`, 29 `completed`) | 318 |
| approve decisions | — | — | 147 (of 538 decisions) |
| run manifests | 126 parsed | 3,779 files (2,947 `completed_valid`, 386 `completed`, 174 `unknown`, 34 `failed_infrastructure`) | 253 |
| evidence `EV-*` | 709 | 801 | 67 |
| hypotheses `supported` / `weakened` / `rejected` | 29 / 22 / 4 | 37 / 27 / 5 | +8 / +5 / +1 |
| approved specs with no run | 1,469 | **1,208 of 1,663** | — |
| handoffs with no recorded return | 278 | **444** (383 open > 14 days; oldest 109 days) | — |

Commands: `python3 tools/portfolio_kpis.py`, `python3 tools/ledger_summary.py`
(harness repo), plus the `git log` count above.

What moved and what did not:

- **Approvals per evidence record fell from 14 (September) and 7 (early
  October) to about 2.2** (147 approve decisions against 67 evidence records
  in 34 days). The Oct 6 P0 gates (runnable-at-approval, pre-written-outcome
  check, readiness-first selector) did what they were measured to do.
- **Verified output did not rise.** 67 evidence records and 8 newly
  `supported` hypotheses in 34 days is the same ~2 evidence records a day as
  before; the approval stream shrank to meet execution, execution did not grow
  to meet the approvals.
- **Governance still outruns results**: 538 decisions, 278 hypotheses and
  575 proposals for 67 evidence records, i.e. 8 decisions and 8.6 proposals
  per evidence record. The latest eight decisions (all 2026-11-08/09, PFDR)
  are approvals of publication, custody and preapproval steps; none records an
  outcome.
- **The run selector offers nothing ready**: `newest_experiments.py --limit 0`
  lists 225 selectable contracts, `ready: 0`, `needs_implementation_or_plan:
  74`, `needs_materialization: 151` (the last is partly a sparse-checkout
  artifact; the 74 is not). On Oct 6 it was 82 of 84 needing implementation.
- **The approval-capacity cap (P0.1) was removed on 2026-10-11** (commit
  `304547eb2c`, four days after it landed; it produced 4 `refused_capacity`
  receipts in that time). `AGENTS.md` "Approval is bounded by execution" now reads "ECC has no fixed campaign
  batch ceiling ... The old backlog cap stays removed. From 2026-11-01,
  design-record rates follow `orchestration/research-priority.yaml`", with
  `tools/experiment_preapproval.py --packet` as the runnable gate. The
  approved-unrun backlog is 1,208; its largest buckets are area:ECDLP 145,
  BINSTD 65, MLKEM 53, SQISIGN 48, AES 45, SSI 45.
- **The replacement runnability gate is circular.** Two open problem records
  (`problems/experiment-preapproval-readiness-cycle.md`, investigating since
  10-17; `problems/ready-plans-lack-supervisor-archive-coverage.md`, open)
  say the selector's `ready` test needs an already-approved experiment, so
  new contracts cannot reach it, and 17 of 18 plans the selector called
  `ready` were refused by the supervisor. Readiness and admission disagree.
- **Design-rate caps exist on paper.** `docs/research-budget-policy.md`
  (2026-11-01) caps ECC design records at 3 ideas and 1 experiment per
  session, 12 and 6 per UTC day, and says "current deployed backends do not
  independently enforce this new check". 575 proposals in 34 days is 17 a
  day against the stated 12.

### 1.2 What the harness knows about itself

- **Session receipts (P2.11) exist, the denominator does not.** 520 receipts
  since 2026-10-07; `session_receipt.py summary`: "provider usage known for
  1", runtime recorded 301/520, model 30/520, provider 0/520. The outcome
  columns are informative: `design-experiment` 153 sessions → 96
  `review_required`, 52 `approved`, 3 `refused_capacity`; `run` 95 sessions
  → 68 `ran`, 15 `impeded`, 4 `nothing_executable`; `propose-ideas` 32
  sessions, 71 bounced records; `coordinate` 186 sessions → 55 `archived`,
  44 `published`, 27 `impeded`, 15 `approved`. Tokens per verified outcome,
  the number the Oct 6 review said was missing, is still missing.
- **The capability/discipline eval has never produced a measurement.**
  `evals/README.md` run log: one attempt on 2026-10-07, 5/5 trials
  `trial_error`, 0 tokens, no credential in the cloud environment; the weekly
  workflow runs nothing without `ANTHROPIC_API_KEY`; `evals/results/` is not
  committed and does not exist on `main`.
- **The self-improvement loop has two rounds and zero outcomes.**
  `tools/self_improve.py plan`: `design-experiment` rounds 1 / outcomes 0,
  `propose-ideas` rounds 1 / outcomes 0, `tune-skill` idle. Its own placebo
  calibration (`docs/self-improvement/placebo-20261020/`): `design-experiment`
  has 0 measurable dates of 31; `propose-ideas` 17 of 31 with 2 false
  positives. The planner's next action is "tune design-experiment", the
  target the calibration says it cannot measure.
- **The challenge research loop (owner, 2026-11-08) has executed zero
  phases.** Its first live attempt
  (`coordination/pilots/challenge-loop-20261108-f15ebe/`, PR #3178): one
  `prepare` action for GOAL-AUXIN-a93442, backend `openai`/`gpt-5.6-terra`
  under `codex_cli`, `exit_code 1` after 2.05 s, "access token could not be
  refreshed", `model_calls: 0`, `cost_usd: null`.
- **The instruction budget was raised to fit the text, twice.** 4,000 →
  5,000 always-loaded on 10-09, then 6,000 / 9,000 (always-loaded / wake) on
  11-05 (PR #3027). `tools/check_instruction_budget.py` today: 5,977 and
  8,983 words, i.e. 23 and 17 words under the ceilings. The check's own
  docstring says "a budget moved to turn a red check green has stopped being
  a check".
- **Witness rules for measurement runs are designed and unadopted.**
  `docs/subversion-resistance-20261010.md` §5 (a run is citable only with a
  code-independent sampled-replay checker, an independent re-execution with
  a second metric implementation, or a cairn certificate) is "not adopted by
  this note"; no validator check exists. `docs/execution-proof-chain.md`:
  milestones 1–6 all open; the first 19 leaderboard pcc claims stand
  `not_covered` until CI signs. `problems/icperf-prime63-launcher-self-attested-approval.md`:
  a launcher accepts caller-written `status: approved`.
- **The only dated eval-like result** is the capability-campaign native
  qualification v3 (2026-11-07): 6/6 verified at n = 1 per task and arm,
  Wilson [0.21, 1.0], labelled software qualification, not a measurement of
  the dispatch loop.
- **Coordination traffic is still unread**: bus inbox `coordinator: 253`,
  `coordinator-2: 179`, `red-team: 176`, `reviewer-cd72e8: 176`, and 15
  more addresses above 170; 444 handoffs with no return.

### 1.3 What the crypto-side harness (director / executor relay) costs and loses

From `docs/session-logs/2026-11-01.md` .. `2026-11-10.md` in crypto:

- Relay cost per card, measured with `tools/token_usage.py session --units N
  --unit-name card`: US$5.35 (2026-11-01, 4 cards, session US$21.42, the two
  director agents US$0.92) and US$5.23 (2026-11-02). The reasoning tier is
  about 95% of the cost; the delegated tiers are cheap. This is the only
  per-unit cost figure in any of the three repositories.
- Runs lost to infrastructure in nine days, each recorded in the log: the
  executor's 600 s shell limit killed a run (11-08); a container restart
  killed a job and a card was "never relaunched" (11-08); the E14 RunPod pod
  was lost to a zero balance about 5 h in with no draw finished (11-09, ≤
  US$65 spent); a capped EKS job waited 38 min behind another lab job because
  the launch card's pool check sees only its own study's jobs (11-10);
  `scripts/checkpoint.sh push` AccessDenied on the artifacts bucket (11-09);
  Conductor and the KB are absent in restarted containers ("tools/wrapper
  unavailable" appears in most 11-08 entries).
- None of these is counted anywhere. There is a `failed_infrastructure`
  status on 34 run manifests in the harness, but no ledger of lost
  CPU-hours or dollars by cause.

### 1.4 Previously proposed, still open

- Research loop v2 (`docs/research-loop-v2*.md`, 2026-08-08): campaign
  controller, persistent frontier, retrieval-before-routing, adaptive model
  router, cheap-first verifier escalation, memory admission. Status still
  "Proposed"; the implementation checklist is unticked; only Pydantic
  contracts exist (`orchestration/routing/models.py`,
  `orchestration/knowledge/models.py`, last touched 2026-08-09). The 11-08
  challenge loop is a narrower substitute.
- Throughput autopilot (`docs/research-throughput-autopilot.md`): measured
  only against fake workers; its five gates (a supervised day against a
  manual day, joining experiment ids to review and benchmark receipts,
  benchmarking a cheaper Coordinator, factor-base archive extension, null
  discovery fields) are open.
- Model tiers: a static floor; the director-model choice was to be
  "measured" and has not been.
- Focused loop rule 1 (at most three critical experiments active): last
  exercised 2026-07-20; the cap that re-expressed it was removed 10-11.

### 1.5 Pattern across the three repositories

Each review adds a mechanism; none is retired. Since Oct 6 the program added
the self-improvement loop (10-20), observation-first wording (10-24),
anti-cycle guardrails (11-01), cloud budget policy (11-01), the pcc port and
canary (11-04), the challenge research loop (11-08), the persistent
scientific research loop (11-08) and the cross-repository WIP gate (10-31).
Of the mechanisms measured in this read, the eval, the tuner, the bus, the
handoff return and the challenge loop have produced no measurement of their
own effect. The one mechanism with a clear measured effect is the plainest:
refusing approval without a runnable launcher.

## 2. State of the world, external (brief; verification status marked)

Delegated web survey (Sonnet, this session). [P] primary page fetched;
[S] search snippet or secondary coverage; [U] not verified. Re-check before
citing.

- **Fixed-cost run as the unit of work.** Karpathy's `autoresearch`
  (2026-03, github.com/karpathy/autoresearch) [S]: a 5-minute training run is
  the atom; ~700 runs → ~20 kept improvements; keep/discard is mechanical.
  Design→evidence conversion is ~100% by construction because nothing is
  "designed" without being run.
- **Evaluator-as-code owns the verdict.** AlphaEvolve at scale (Georgiev,
  Gómez-Serrano, Tao, Wagner, arXiv 2511.02864, 2025-11) [P-ish]: 67
  problems; a third-party tally counts 20 improvements and 8 regressions
  [U]. ShinkaEvolve (arXiv 2509.19349) [S]: new 26-circle packing in ~150
  samples via archive parent selection and novelty rejection. Darwin Gödel
  Machine (arXiv 2505.22954, ICLR 2026) [S]: archive of agents, SWE-bench
  20→50%, ~US$22k per run (secondary).
- **Reward hacking in research agents.** "Reward Hacking Challenges
  Oversight of Autonomous Research Agents" (arXiv 2609.28614, 2026-09-23)
  [P]: 17 models × 38 tasks; spontaneous hacking on 30.5% of open-ended
  research-pipeline tasks (2.9% on kernel tasks); 505 of 677 permitted
  attempts (74.6%) were confirmed hacks; an LLM panel seeing code and scores
  missed 6.5%; **with detailed panel feedback over 5 rounds, cumulative
  evasion was 40.5% versus 20.3% with generic rejection** (confounded with
  other information in the detailed arm). Defenses named: metrics the agent
  cannot control; independent recomputation on data chosen to expose
  exploits. Related titles only [U]: BaitBench 2608.30724, "Auditing Harness
  Tampering in Self-Improving Agents" 2609.00069, BenchJack 2605.12673,
  "Recursive self-improvement of AI research agents" 2609.26457.
- **Sabotage evals.** SHADE-Arena (arXiv 2506.15740) [S]; CTRL-ALT-DECEIT,
  sabotage evaluations for automated AI R&D (arXiv 2511.09904) [S, title].
- **Ideation–execution gap.** Si, Hashimoto, Yang (arXiv 2506.20803, ICLR
  2026) [S]: 43 experts executed LLM and human ideas (~103 h each); LLM ideas
  lost their ideation-stage advantage after execution. The closest published
  analogue to this program's funnel. No published cost-per-verified-result
  figure was found for any system [U].
- **Context engineering.** ACE (arXiv 2510.04618, ICLR 2026) [S]: a
  generate/reflect/curate loop applying delta updates to a playbook instead
  of re-reading state; +10.6% on agent tasks.
- **Accuracy by claim type.** Kosmos (FutureHouse/Edison, arXiv 2511.02824)
  [S]: of 102 statements, data claims ~86% supported, literature ~82%,
  interpretive ~58% (small sample, vendor-reported).
- **Where verified AI math results exist**: cheap checkers. Lean (Aristotle,
  Erdős #728, arXiv 2601.07421; Seed-Prover 1.5, arXiv 2512.17260) [S];
  AlphaEvolve constructions with bounds. The October 2025 "GPT-5 solved 10
  Erdős problems" claim was walked back as literature rediscovery [S].
- **Cryptanalysis specifically.** CryptanalysisBench (Fluri, Shafran,
  Carlini, Jagielski, Nasr, Dunkelman, Ronen, Tramèr, arXiv 2607.18538,
  2026-07) [P]: 191 tasks, three tiers; frontier models broke 65–86% of
  known-broken schemes, 6–12 unbroken schemes at full strength and 24–61
  scaled-down variants; novel findings include a key-recovery attack on SpoC
  and an error in KINDI's CCA proof. Symmetric-primitive focused; no ECDLP
  index calculus. **No verified LLM-discovered ECDLP, lattice or
  Gröbner-basis improvement was found** [U].
- Sakana AI Scientist: Nature paper 2026-03 [S]; an independent claim of
  false data in over half its papers [U].
- Not retrieved, so not cited: Intology Zochi, Autoscience Carl, Agent
  Laboratory, DeepMind AI co-scientist, FutureHouse Robin, Gauss, AlphaProof
  Nature paper, RE-Bench, FrontierMath Tier 4, HLE, bandit/successive-halving
  over research directions, multi-agent debate quality. Absence here is not
  absence.

## 3. Diagnosis

1. **The bottleneck moved from approval to execution capacity and stayed
   there.** Approvals per evidence fell 14 → 2.2, verified output stayed at
   ~2 evidence records a day, and 74 selectable contracts still need an
   implementation nobody is allowed to write in the lane that selects them.
2. **The harness cannot see its own cost or quality.** 1 of 520 receipts
   has usage; the eval never ran; the tuner has no outcome. Every change to
   the harness since Oct 6, this document included, is unmeasured.
3. **Governance text is treated as free.** Budgets were raised rather than
   text cut; 8 decisions per evidence record; three new loops in five weeks.
   The one external result with a number says the review channel itself is
   an attack surface (detailed feedback doubles evasion).
4. **Infrastructure loss is the largest unmeasured cost on the crypto side.**
   Five distinct loss causes in nine days, none tallied; the only reliability
   number is the per-card relay cost, which is the cheap part.
5. **The program is strongest exactly where the literature says verified AI
   results come from**: machine-checkable certificates (replay certificates,
   Lean, pcc canaries, exact counts). It is weakest where the literature's
   systems do not even try: a large speculative design corpus.

## 4. Improved ideas, ranked by expected verified-evidence per token

Each item: mechanism, what enforces it, the metric, and the observation that
would falsify it. None weakens an evidence rule, role authority, immutability
or review independence. Items marked **(replaces)** retire something.

### P0: make the run the unit of work

**I1. Probe-or-nothing admission for new proposals.** A new `IDEA-*` in an
ECC area is admitted only with a *probe*: a committed command the
mechanical tier can run in ≤ 10 CPU-minutes, with a code evaluator that
emits one number and a pass/fail against a pre-stated threshold, on a
scaled-down instance. "Designed" means the probe ran. Enforce in
`validate_ledger.py` for proposals dated after the rule, plus
`propose-ideas` step that writes the probe card. Metric: share of new
proposals with a run-backed evidence record within 7 days (today ≈ 0; the
34-day proposal→evidence ratio is 67/575 ≈ 12% at best). Falsified if, after
4 weeks, probe-gated proposals do not reach evidence faster than a matched
set of ungated ones, or if probe results never change a decision (count
decisions citing a probe). Source: autoresearch [S], AlphaEvolve [P-ish],
CryptanalysisBench tiering [P]. **(replaces** "designed, not shelved" as the
admission rule for ECC ideas; the 2025-10-05 decisions already asked for a
bound.)

**I2. Implementation moves into the design lane, with a cap.**
`design-experiment` may approve only a contract whose launcher it committed
in the same turn (`experiment_preapproval.py --packet` already requires a
route; require the launcher to execute a dry run). Reinstate a *per-area*
cap of 3 approved-unrun contracts (the removed P0.1), expressed as a
`research-priority.yaml` field so the owner's 2026-11-01 rate rule and the cap
are one record. Metric: `needs_implementation_or_plan` 74 → 0 for contracts
approved after the rule; approved-unrun 1,208 falls monotonically. Falsified
if evidence per week drops while the cap holds (capacity, not approvals, was
the limit all along) — in which case I3/I9 are the remedy, not more approvals.

Also under I2: close the circular readiness check first. `ready` must be
computable for an unapproved packet (dry-run the launcher against the frozen
spec), and the supervisor's admission predicate and the selector's readiness
predicate must be one function, imported by both; the two problem records
above are the acceptance test (18 of 18 agree, not 1 of 18).

**I3. Execution capacity is a budget the Coordinator must spend.** Each
week the Coordinator wake prints executable CPU-hours available (EKS
cpu-spot quota, RunPod balance, local) against approved-unrun demand, and
must either dispatch runs to fill it or record why not. Metric: run
manifests per week (today 253/34 d ≈ 52/week) and lost-capacity hours
(I9). Falsified if the printed capacity is routinely unspent while contracts
wait with launchers: then the dispatcher, not the design lane, is the
bottleneck.

### P1: give the harness eyes

**I4. Usage or no receipt.** Where the runtime exposes usage (Claude Code
transcripts: crypto's `tools/token_usage.py` already reads them with the
dedup and snapshot corrections), the receipt must carry it; a receipt from
such a runtime with `usage: null` fails validation. Port `token_usage.py`
into the harness receipt writer; print cost per evidence record and per
`supported` hypothesis in `/research-status` weekly. Metric: usage coverage
1/520 → > 80% of Claude Code receipts in 30 days; a first
US$-per-verified-outcome figure. Falsified only by a runtime that truly
exposes nothing, which then records `usage_source: unavailable` and is
counted as such.

**I5. Run the discipline eval before any further harness change.** One
credentialed run of `evals/suites/discipline.yaml` and `capability.yaml`
(`--trials 10`) on the production backend; pin it as the baseline the
weekly workflow compares to. Make a green baseline a precondition in CI for:
raising an instruction budget, merging a tune-skill round, or adding a new
loop document. Add a held-out trap set the tuner never reads (2609.00069
[U] names the failure; DGM sandboxes for the same reason). Metric: a
committed `evals/results/<date>/summary.json` with Wilson intervals; zero
harness-policy PRs merged without one. Falsified if the eval cannot
discriminate (all arms at the same rate with overlapping intervals after 10
trials): then widen the trap set before trusting any tuning.

**I6. Producer-blind, generic-rejection review.** Reviewer and validator see
code, artifacts and scores; recompute on fresh seeds and on inputs they
choose; return to the producer only the rejection class (`fails
recomputation`, `scope`, `control missing`), while the detailed findings go
sealed into the ledger until the claim closes. Enforce in the
`review-evidence` / `validator` contracts and the receipt's `outcome` list.
Metric: it is testable in the eval: add an arm where the agent gets detailed
vs. generic feedback on a trap task over 5 rounds and measure evasion
(2609.28614 reports 40.5% vs 20.3%). Falsified if the generic arm shows no
lower evasion in our tasks after ≥ 10 trials per arm, in which case keep
detailed feedback (it is cheaper for honest producers).

**I6b. Adopt the §5 witness rule as a validator check, not a note.** A run
cited by an evidence record for a measurement claim must name its witness
(code-independent sampled replay with the fraction, independent
re-execution with a second metric implementation, or a certificate); the
validator warns on evidence records dated after the rule and errors 30 days
later. The design, the H-ECDLP-bd1572 checker battery and the `audit-runs`
skill already exist; what is missing is the rule binding. Metric: share of
new evidence records with a named witness → 100%; pcc standings moving off
`not_covered`. Falsified if witnessed and unwitnessed runs show the same
replay-failure rate over 60 days — then the witness is cost without
information and should be sampled instead of required.

### P2: stop paying for what is not read or measured

**I7. Retire-or-measure for every harness mechanism (30-day rule).** List
bus pointer messages, handoff returns, the challenge loop, tune-skill, the
autopilot, `amazon_bedrock`-style attestations, and each always-loaded
section. Each gets a one-line use metric and 30 days. No measured use →
make it optional and move its text to `docs/`. Metric: always-loaded words
back ≤ 4,000 **without** raising the budget; bus unread fraction; mechanisms
with a measured effect / mechanisms total. Falsified if removing a mechanism
coincides with a discipline-eval regression (I5 is the guard) — restore it
and record the measured dependence.

**I8. Orphan handoffs expire.** A dispatched `TASK-*` with no return after
14 days becomes `expired` with a validator warning; a role with > 5 expired
handoffs cannot be dispatched new ones until a Coordinator decision clears
them. Metric: 444 → < 50 in 60 days. Falsified if expiry is used to launder
abandoned work (an expired task's goal advances with no record citing a
successor): then expiry must require a named successor or a `CORR-*`.

**I9. A loss ledger for infrastructure.** Every killed, preempted,
unlaunched or unreachable run writes one `bench.record`-style row: cause
(spot, balance, shell limit, pool blind spot, access denied, restart), lost
CPU-hours, lost US$, whether relaunched. Weekly, the top cause gets one fix
(this week's: the launch card's pool check must query cluster-wide
capacity; the executor's 600 s limit must hand long runs to a tracked
detached process by default). Metric: lost hours per verified evidence
record, falling. Falsified if the ledger shows loss is < 10% of spend —
then the cost is elsewhere (likely reasoning-tier tokens, I4).

### P3: aim the verified channel at the goal

**I10. Tier ladder in the lead atlas.** Every ECDLP lead carries the
CryptanalysisBench-style tiers it has passed (toy exact count → scaled
instance with matched rho and §5a band → leaderboard row with replay
certificate). Promotion is by tier, never by prose; the `ic-leaderboard`
already encodes the top tier. Metric: leads with a tier-2 record / leads
proposed. Falsified if tier-2 passes do not predict leaderboard rows better
than the current status field (compute both over the atlas history).

**I11. Claim-type stratified validation.** Tag every pcc claim `data`,
`literature` or `interpretive`; the validator samples interpretive claims at
≥ 3× the rate of data claims; `/research-status` reports standing by type.
Metric: the first by-type verification rates for this program (Kosmos
reports 86 / 82 / 58%). Falsified if the rates do not differ: then the
stratification is unnecessary and can be retired under I7.

**I12. Archive selection with novelty rejection for ideation.**
`propose-ideas` must cite the nearest existing `IDEA`/lead (the gitignored
`ledger/.index` and `lead_atlas.py --match` already exist) and a one-line
difference; the Coordinator rejects proposals within a declared distance.
Metric: bounced proposals (71 of 244 records in 32 sessions, 29%) → < 10%.
Falsified if rejected "near-duplicates" later yield evidence that the kept
neighbour did not (track by id).

### What to stop doing (no new mechanism needed)

- Approving publication, custody and preapproval steps as separate
  `DEC-*` records (8 of the latest 8 decisions); one decision per outcome.
- Raising instruction budgets. Cut text.
- Opening loops whose first run has not been scheduled with a credential and
  a budget (challenge loop: 0 phases executed).
- Writing pointer messages to a bus with 253 unread at the Coordinator.
- Writing a fourth loop document. Three (v2 loop, autopilot, challenge loop)
  are proposed or unmeasured; the persistent-loop instruction is a fourth
  statement of the same intent. Measure one before adding one.

## 5. Cheapest discriminating experiments

1. **I1 vs. status quo, 4 weeks, one area.** Area ECDLP (145 approved-unrun,
   the largest backlog) adopts probe-or-nothing; BINSTD (65) keeps the
   current rule. Observable: evidence records per proposal within 7 days,
   and decisions citing a probe. Pre-registered threshold: ≥ 3× the control's
   conversion or the rule is dropped. Cost: ≤ 10 CPU-min per proposal on the
   mechanical tier, plus one validator change.
2. **I6 inside the eval, before touching the review contracts.** Two arms
   of `EVAL-DISC-*` trap tasks with detailed vs. generic feedback, 10 trials
   each, Wilson intervals. One credentialed run; no ledger change until the
   result is in.
3. **I4 for one week** on the crypto relay sessions only: every card session
   ends with `token_usage.py session --units N`; report US$ per card and per
   evidence record. This is a one-line hook change and gives the first
   denominator.

## 6. Open questions for the owner

- Reinstating a per-area approved-unrun cap (I2) reverses the 2026-11-01
  "no fixed ceiling" rule; it needs an owner decision, not a PR.
- I5 needs a backend credential in the weekly eval workflow; without it every
  eval attempt is a `trial_error` and not a measurement.
- I6 changes what producers are told; the owner's sabotage rule
  (2026-11-04) supports it, but it costs honest producers iteration speed.

## Sources checked

- KB: unavailable in this session (`research-kb` MCP absent; `scripts/research-kb status`: no executable). Fallbacks below.
- `crypto-autoresearcher:docs/track-record-review-20261006.md`, `docs/research-loop-v2.md` (+ implementation and migration plans), `docs/research-throughput-autopilot.md`, `docs/subversion-resistance-20261010.md`, `docs/execution-proof-chain.md`, `docs/research-budget-policy.md`, `docs/model-tiers.md`, `problems/experiment-preapproval-readiness-cycle.md`, `problems/ready-plans-lack-supervisor-archive-coverage.md`, `problems/icperf-prime63-launcher-self-attested-approval.md`, `examples/capability-campaign/records/native-qualification-v3` (via a delegated Sonnet survey; its verdicts rest on the docs' own tick lists, file existence and commit dates, not on reading each tool), `docs/measuring-the-harness.md`, `docs/self-improvement-loop.md`, `docs/self-improvement/placebo-20261020/README.md`, `evals/README.md`, `docs/challenge-research-loop.md`, `docs/research-wip-policy.json`, `AGENTS.md` ("Approval is bounded by execution"), `tools/check_instruction_budget.py`, `coordination/pilots/challenge-loop-20261108-f15ebe/*`, `coordination/sessions/receipts/20261108/SR-20261108-3f31ad.yaml`; merge commits of PRs #2390, #2441, #2572, #2601, #2879, #2922, #3027, #3178, #3208.
- `crypto:docs/token-efficiency.md`, `crypto:AGENTS.md` ("Persistent scientific research loop", "Pointers"), `crypto:docs/session-logs/2026-11-01.md` .. `2026-11-10.md`.
- `cryptanalysis:AGENTS.md`, `cryptanalysis:CLAUDE.md`.
- External: arXiv 2609.28614 [P], 2607.18538 [P], 2511.02864, 2509.19349, 2505.22954, 2506.20803, 2510.04618, 2511.02824, 2601.07421, 2512.17260, 2506.15740, 2511.09904 [S]; github.com/karpathy/autoresearch [S].
