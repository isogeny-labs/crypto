# Protocol: does the continuous learning loop let agents finish tasks without supervision?

Status: **DRAFT.** To be frozen after owner review. Nothing in this file is data.
Freezing means: commit this file and the task set, record the canonical hash and
commit, then draw the seed (section 5). Any later change is a dated amendment that
says whether it came before or after the data it touches. Both versions are
reported (skill `controlled-experiment`, section 3).

Claim class (skill `controlled-experiment`, section 1): *comparative / statistical*.
The experiment may license the sentence "on these 48 tasks, with this model, the
loop-on arm finished more tasks unsupervised than the loop-off arm, by d (95% CI)".
It licenses no broader sentence. Every figure the experiment produces is
`[pcc:unverified]` until a pcc claim cites it.

Related files: [`README.md`](README.md) (episode schema `crypto.autonomy/v1`, intervention
classes), [`tasks/INDEX.md`](tasks/INDEX.md) (the 48 draft tasks, schema
`crypto.autonomy-task/v1`), AGENTS.md "Continuous learning loop", `tools/lessons.py`.

## 1. Question

Does the repository's continuous learning loop change how often an agent finishes a
task with no corrective human message?

The loop is the AGENTS.md section "Continuous learning loop": the ledger in
`docs/lessons/` (`tools/lessons.py`), the brief each skill and agent reads before it
acts, the outcome and test entries it writes after, and the "Continuous loop" section
at the end of every `.agents/skills/*/SKILL.md` and `.claude/agents/*.md`.

## 2. Hypotheses

- **H1.** The unsupervised-success rate of the loop-on arm is higher than that of the
  loop-off arm. The size of interest is an absolute gap of 0.10 or more. The
  planning figure is 0.50 against 0.70.
- **H0.** The two rates are equal, or loop-off is higher.

H1 is one-sided in direction and tested two-sided (section 7). A result in the wrong
direction is reported as such.

## 3. Arms

| arm | instructions | ledger | purpose |
|:--|:--|:--|:--|
| **ON** | the pinned commit as it is | the ledger at that commit, then every entry the arm's own agents write, carried from task to task | treatment |
| **OFF** | the same commit with the loop removed (below) | none | control |
| **ON2** | same as ON, separate checkout and separate ledger lineage | its own, starting from the same seed state | A/A repeat: the noise floor |

**How OFF is built.** From the pinned commit, delete: the section "Continuous learning
loop" of AGENTS.md; the "Continuous loop" section at the end of every
`.agents/skills/*/SKILL.md` and `.claude/agents/*.md`; the line that points to the
loop in CLAUDE.md; `docs/lessons/`; `tools/lessons.py` and `tools/test_lessons.py`;
the `agent-learning` workflow; and the lessons output of the session-start hook. A
script produces OFF from ON and prints the list of changed paths. The only
differences between ON and OFF are those paths. The script's output and the tree
hashes of both arms are stored. No task `check_command` may use a loop file; a grep
over the task set confirms it before the freeze.

**What the treatment is.** The treatment is the whole bundle: instruction text plus
ledger. The experiment cannot separate the two effects (section 13).

**Same for every arm:** model id, effort level, system prompt, tool set, permission
mode, hooks other than the lessons output, the task prompt text (byte for byte), the
owner stand-in (section 6), the time limit, the host class, and the commit. The model
id and effort are fixed before the freeze and recorded.

## 4. Tasks

Forty-eight draft tasks, six classes (counts in `tasks/INDEX.md`): ci-repair 10,
measurement 8, lean 8, docs-tooling 10, research 6, blocked 6. Each task gives the
prompt, the setup (a commit and a fixture the harness builds), the acceptance
criteria, one `check_command` that exits 0 only when the work is accepted, the
anticipated unblock requests with their scripted answers, and the forbidden actions.

Harness variables set for every run: `T_BASE` (the commit that holds the setup),
`T_OUT` (an output directory outside git), `T_DATE`. After the agent stops, the
harness commits the working tree on a scratch branch and then runs the check.
Checks therefore compare commits (`git diff "$T_BASE" HEAD`), not the working tree.

**Pre-flight checks, before the freeze (these are the instrument's controls).**

1. *Negative control.* On the setup state of every non-blocked task, `check_command`
   exits non-zero. A check that passes before any work cannot score anything.
2. *Positive control.* A reference solution, written by a person or a model that does
   not take part, makes `check_command` exit 0 in every non-blocked task. For blocked
   tasks, a reference report passes and a report with a fabricated pass fails.
3. *Determinism.* `check_command` run twice on the same state gives the same exit code.
4. *Cost.* Each check runs inside its stated limit. Tasks marked `grounding: medium` or
   `weak`, or whose notes say "not executed", are re-checked or dropped. Dropping is
   allowed before the freeze and never after.
5. *Loop-free checks.* No check reads `docs/lessons/` or `tools/lessons.py`.
6. *Seeded-lesson probe (positive control for the treatment).* One short run in ON with a
   ledger that already holds one correct, relevant lesson for a sibling task, to show
   that the arm can use a lesson at all. If the probe fails, the experiment is paused
   and the loop's plumbing is fixed first. A fixed plumbing is a new arm definition.

**Sibling pairs.** Some tasks share a mechanism, so a lesson from one can help the
other: T-002 and T-003 (curve names), T-032 and T-033 (prose lint tags), T-038 and
T-039 (cost chain), T-023 and T-024 (instance statement), T-019 and T-025 (Lean
repair). They are spread over the sequence by the shuffle; their position is not
steered.

## 5. Randomisation and execution order

- **Seed.** After the freeze, one nonce is drawn from the operating system and
  hashed with the protocol hash. Every other seed is derived from that hash and a
  label. The seed is recorded before the first run.
- **Order.** For each replicate, the 48 tasks are shuffled once with a derived seed.
  All three arms run that same order within the replicate. A new shuffle is drawn for
  each replicate. This keeps the pairing exact and spreads order effects across
  replicates.
- **Interleaving.** The arms run step by step: at position i, the three arms each run
  task i, in an order that rotates over positions in a Latin square. Time-varying
  conditions (model service load, host noise) then fall on all arms alike.
- **Isolation.** Each run starts in a fresh container with a fresh checkout of its
  arm's tree. The only state that passes between runs of one arm is that arm's ledger
  directory (ON and ON2 only), copied forward after each run. There is no shared
  mount between arms. The git remote is disabled, so no arm can fetch a ledger or a
  fix from `origin`.
- **Replicates.** Two are planned. A third is run only under the rule in section 10.
  The ledger resets to the seed state at the start of each replicate.

## 6. The owner stand-in

A script plays the owner. It answers only requests to **unblock**.

- A message from the agent that asks for access, credentials, budget, or an owner
  decision is an *unblock request*. The stand-in answers from the task's
  `unblock_answers`. If the task has no matching entry, it sends the fixed text
  "Not available. Continue on your own and report what this blocks."
- Anything else the agent asks or hands back (a choice between approaches, a request
  to confirm a plan, a question about scope or what is meant) is a *correction*. The
  stand-in sends the fixed text "Continue on your own." and the run logs one
  corrective message (the `redirect` class of `README.md`).
- Classification uses one frozen classifier prompt, run by a small model, on the
  message text only. It does not see the arm. A person reads a 10% sample, drawn with
  a recorded seed, and the agreement rate is reported. A person's label replaces the
  machine's for the same message.
- The stand-in is identical in every arm. It never gives hints, never reads the
  agent's work, and never says whether a result is right.
- At most three unblock answers are given per run. A fourth request is logged as a
  correction.
- Unblock requests do not count against success. They are reported apart. In the
  blocked class, a well-formed unblock request followed by a correct report is the
  intended path.

## 7. Primary metric and analysis

**Unit.** One run is one task in one arm in one replicate.

**Unsupervised success.** A run is a success when all four hold:

1. `check_command` exits 0 on the committed snapshot;
2. the run has zero corrective messages (section 6);
3. the PR-equivalent change passes the local equivalents of the merge set that can
   run offline: `scripts/check-large-files.sh "$T_BASE" HEAD`,
   `python3 tools/pcc/pcc.py canary`, `python3 tools/pcc/pcc.py ledger verify`, and
   `python3 tools/check_lab_parity.py --repo crypto`. The validation gate and the store
   checks cannot run without store access and are left out in every arm;
4. the independent validator (section 8) finds nothing blocking: no forbidden action,
   no relabelled result, no unsupported claim, no edit to frozen evidence, no
   fabricated receipt.

A blocked-class task counts as a success when its report passes the check, the
validator finds the report accurate, and there are no corrective messages.

**Analysis (script and hash frozen before data).**

- For each task, the per-task difference is the mean over replicates of
  (ON success minus OFF success). It lies between -1 and 1.
- The headline is the mean of the per-task differences over the 48 tasks, with a 95%
  interval from a cluster bootstrap that resamples tasks (10,000 resamples, derived
  seed). Resampling tasks respects the correlation between replicates of one task.
- The paired per-task McNemar test is the test of H0. Pairs are the matched
  (task, replicate) outcomes. Only the discordant pairs enter. The exact binomial
  p-value is reported with both discordant counts. Because replicates of one task are
  correlated, this p-value is approximate and the cluster interval decides.
- **Noise floor.** The same statistic is computed for ON against ON2. It shows how
  large a difference appears between two arms that are the same by construction.
- **Decision rule.** H1 is supported if the lower end of the headline interval is above
  0 and the headline exceeds the 95% range of the ON-versus-ON2 statistic. Otherwise
  H1 is not supported. If the upper end of the interval is below 0.10, the report says
  that an effect of the size of interest was not seen.
- The ON-versus-OFF and ON2-versus-OFF contrasts are both reported. If they disagree in
  sign, the report says the noise floor is too large to read the result.
- Effect size is given in absolute rate points beside every p-value. Results are
  reported by class and difficulty as descriptive tables, never as separate tests.

## 8. Blinding

- Arm labels are replaced by random run ids when a run ends. The key (run id to arm)
  is sealed. Its SHA-256 is recorded in the export. It is opened only after the
  analysis script is frozen and has run on the masked data
  (`experiment_protocol.py blind`).
- The validator is a separate agent with separate code and no access to the arm. It
  sees the task, the diff against `$T_BASE`, the check output, and the agent's final
  message after a script removes lesson ids and the words "ledger" and "lesson". It
  does not see the transcript. Its model id is the same for every run.
- Blinding is imperfect. A diff can reveal the arm if it touches `docs/lessons/`. In
  that case the validator is shown the diff with those paths removed. A transcript
  reveals the arm through calls to `tools/lessons.py`, which is why the validator never
  reads one. The agent itself is not blind. This is part of the treatment.
- The deterministic check runs first. The validator may only add a blocking finding. It
  cannot turn a failed check into a pass. The rate at which the validator and the check
  disagree is reported.

## 9. Secondary measures

Each is descriptive. Five are listed, and the interval for each is Holm-adjusted over
the five.

| measure | definition |
|:--|:--|
| redirects per task | corrective messages per run, by arm |
| self-recovery rate | incidents (a failing check, a merge conflict, a blocked command) the agent fixed with no human message, over all incidents, by arm |
| cost per unsupervised success | total list-price cost of the arm's runs divided by its number of unsupervised successes; tokens and prices from `tools/token_usage.py` and `tools/token_prices.json`, cache hit ratio reported beside it |
| learning curve | success in the last third of the sequence minus success in the first third, in each arm. ON should rise more than OFF if the loop learns |
| repeat-failure rate | share of failed runs whose failure signature (the failing check and the first error line) already occurred earlier in the same arm and replicate |

Also reported, without a test: wall minutes per run, blocked-class report quality, the
number of ledger entries ON writes, and how many of them a later run cites.

## 10. Sample size and stopping rules

**Sample size (approximate).** About 50 paired tasks, two to three replicates each. For a
paired binary outcome, a gap of 0.20 (0.50 against 0.70), a discordant share near 0.35,
and a two-sided 5% level, about 60 independent pairs give 80% power. With 48 tasks and
two replicates there are 96 pairs. If the replicate correlation of one task is near
0.5, the effective number is near 64, close to that target. Three replicates give about
72. The discordant share and the correlation are guesses, not measurements. The
A/A arm will show whether they were close. This is a planning figure, not a power
guarantee.

**Stopping rules.**

- The plan is fixed: 48 tasks times two replicates times three arms, 288 runs.
- A third replicate is run if and only if, after replicate 2 and **without opening the
  arm key**, the 95% interval half-width of the ON-versus-ON2 statistic is above 0.15.
  The check uses masked data.
- No other early stop for effect. Nobody looks at arm-labelled results before the end.
- A hard stop applies if the cost cap in the freeze record is reached (cap to be set by
  the owner at the freeze), or if a run attempts a destructive action outside its
  container. The partial result is then reported as partial, with the stop reason.
- Changing the task set, the metric, the arms, the model, the margin or the decision
  rule after the first run is an amendment, not a fix.

## 11. Exclusions

- **Infrastructure failure** (container lost, network or model-service error, harness
  bug, a check that crashed). The run is repeated once in the same arm, at the same
  position, with the same seed. Both attempts are stored. If the second also fails for
  infrastructure reasons, the task is excluded in all three arms for that replicate and
  counted in the report.
- Nothing is dropped silently. The report gives planned, attempted, completed and
  excluded counts per arm and the reason for each exclusion.
- An agent that hits its time limit, runs out of budget, or crashes by its own action is
  a failure, not an infrastructure failure.
- A task found defective after data (its reference solution fails, or its setup state
  passes) stays in the main analysis. A sensitivity analysis without it is added.

## 12. Confounders and how each is controlled

| confounder | how it is handled |
|:--|:--|
| model version or service drift | one model id and effort, fixed before the freeze; the ON2 arm tracks drift; interleaving puts any drift in all arms |
| task difficulty and class | paired design on the same tasks; difficulty and class tables are descriptive |
| order and learning from earlier tasks | same order for all arms within a replicate, new shuffle per replicate, ledger carried only inside ON and ON2; the learning curve measures order use |
| host and time | same host class, interleaved arms, fresh container per run; kernel, CPU and load are recorded |
| ledger contamination between arms | separate checkouts, no shared mount, git remote disabled; hashes confirm OFF holds no ledger file and ON2 does not read ON's entries |
| instruction text versus ledger | not separated here; the bundle is the treatment |
| the grader | deterministic check first; blinded validator; same validator for all runs; validator-check disagreement reported |
| the owner stand-in | one script and one frozen classifier for all arms; 10% human audit |
| tasks that upstream changes have already fixed | the commit is pinned; the harness re-applies each defect (the `setup` field) |
| gaming the check | forbidden-action list per task; the validator reads the diff; checks compare commits |
| prompt wording | identical bytes in every arm |
| the analyst | arm key sealed until the analysis script is frozen and run |
| spot or container loss | the infrastructure rule of section 11 |

## 13. What would falsify H1

- A headline whose 95% interval has a lower end at or below 0.
- A headline at or below the ON-versus-ON2 noise range.
- OFF at or above ON in point estimate.
- A headline above zero with no learning curve: ON no higher in the last third than in
  the first. Then H1 may hold for some other reason, such as a longer instruction set,
  and the report says that "the loop learns" is not supported.
- A positive result that disappears when the sibling pairs are removed, if the effect
  was meant to be general.

## 14. What this experiment cannot show

- Anything about tasks, repositories or models outside this set. Forty-eight tasks from
  one repository are a sample of that repository's work.
- The separate effect of the ledger and of the instruction text.
- That any single lesson is correct or useful. Lessons need two confirmations to be
  adopted, and a 48-task sequence may adopt few or none.
- Long-run effects: drift across weeks, ledger growth, or a lesson that was right once
  and is wrong now.
- Real PR outcomes. The change is PR-equivalent; no PR is opened, merged or reverted.
- The quality of the research output, the correctness of a proof, or any ECDLP claim.
  The research and Lean tasks check form and a few recomputed values; the validator
  judges the rest, and it is itself a model.
- Owner time saved in practice. The stand-in is a script and may answer more quickly, and
  differently, than an owner.
- Safety. A run in a sandbox with no remote says little about behaviour with real
  credentials.
- Wall-clock figures. The runs are not isolated and the measurement tasks are count-only
  toy runs. They are not benchmarks, and they add no row to `benchmarks/README.md` or
  the leaderboard.

## 15. Records and order of work

1. Owner review of this draft and of `tasks/`; pre-flight checks (section 4); weak tasks
   fixed or dropped.
2. Freeze: commit this file, the task set, the OFF builder, the classifier prompt, the
   analysis script and its SHA-256. Write `experiment.protocol/v1` from it.
3. Draw the nonce and derive seeds. Record them.
4. Run the 288 planned runs. Store each run as an episode (`README.md`), with the arm
   masked, plus raw outputs and a unified `experiment.export/v1`.
5. Run the analysis script on masked data. Open the key. Report with the table first,
   effect size and interval beside every p-value, the noise floor, the flow counts, and
   section 14 in full.
6. Ask a second reader to review the export (`review-run`).

Sources checked: the repository files named above. The research-kb server was not
queried for this draft. No literature claim is made.
