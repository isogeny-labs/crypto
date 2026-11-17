# Skills

The skills of this repository, the order they run in, and the rules they
share. Each skill is a folder holding one `SKILL.md` (YAML front matter with
`name` and `description`, then the procedure); `.claude/skills/<name>` is a
relative symlink to it (`tools/check_skills_parity.py`). When a skill was
edited during a session, read it from disk again: a cached copy may be stale.

The layout follows the Trail of Bits constructive-cryptography skills
(`trailofbits/constructive-cryptography/skills`, MIT): rules are stated once,
with a number, and each skill points to the rule instead of restating it in
weaker form. AGENTS.md remains the contract; this file is the index.

## Flow

```text
research-kb (prior art)  →  controlled-experiment / experiment-protocol (design, freeze)
   →  ecbench-measure | ic-autolab | perf-ipc | profile | isogeny-volcano (run, via isolab-eks / queue-experiment)
   →  lean-instance-gate (any Lean statement or proof)
   →  ecbench-bounds (cost record, cost chain: AGENTS.md §3a)
   →  report-evidence + research-visuals (report, graph, PDF)
   →  audit-runs | review-run | sheriff (independent checks)
```

| Skill | Fires |
| --- | --- |
| `research-kb` | before a research task, lead, novelty or "not tried" claim, or handoff |
| `controlled-experiment`, `experiment-protocol` | before any empirical or comparative claim; freeze the protocol before data |
| `isolab-eks`, `queue-experiment`, `ecbench-independent-runner` | where and how a run executes |
| `ecbench-measure`, `ecbench-extend` | cross-method ECDLP measurements; adding a method |
| `ic-autolab`, `dreg-triage` | index-calculus iterations and parameter sweeps |
| `perf-ipc`, `profile` | cycles, IPC, where time goes |
| `optlevel` | how close a kernel runs to its machine's roofline (not a speed claim) |
| `curve-algebra`, `ecdlp-surfaces`, `isogeny-volcano`, `isogeny-volcano-diagrams` | curve structure, transfers, isogeny graphs |
| `lean-instance-gate` | before any Lean definition, statement or proof; when a proof grows long or needs `maxHeartbeats` |
| `ecbench-bounds` | what a method costs; whether a change beat it |
| `report-evidence`, `research-visuals` | writing up any run or result |
| `audit-runs`, `review-run`, `sheriff` | independent re-checks after a result is frozen |
| `iacr-eprint` | posting a paper |

## Before writing anything (every skill)

Every skill opens by answering these for the object it is about to produce
(a spec, a definition, a Lean statement, a cost row, a report), and writes the
answers in the step's record:

1. **Where is it in the whole?** What sits above it and is stated against
   its name, interface or budget.
2. **What changes, at which layer?** One component; its interface stays.
3. **What is the sketch?** The composition, the line of the cost or proof
   chain it is (AGENTS.md §3a; `lean-instance-gate`), or the frozen protocol.
4. **What is missing below?** Every block the sketch cites that does not
   exist, each with its first consumer. Build each just before that consumer.
5. **What does the source say, and what is already decided?** The paper,
   the decisions in AGENTS.md and the session logs, the lead atlas. Do not
   reopen a settled design and do not forget one (AGENTS.md "What supersedes
   what").

If an answer contradicts the plan or an accepted design, stop and report.

## Shared rules

1. **Instantiate, never re-derive.** A result is an existing theorem,
   bound or method applied to the new object; the only new work is showing
   the object fits (`lean-instance-gate`, `ecbench-bounds`).
2. **Sketch before code.** A proof or a cost claim is written first as a
   chain whose lines either restructure (cost- and meaning-neutral) or
   replace one component (AGENTS.md §3a).
3. **Generic first, owner first.** A missing block goes into its owning
   layer (crypto-lean, ecbench, a shared tool), stated generally, before
   its first consumer uses it.
4. **Noticing is stopping.** A worker that finds a rule violation in its own
   or upstream work (a line that does not close by its named laws, a cost
   line that both restructures and replaces) messages the supervisor at
   once and continues only with independent items.
5. **Rules are not suggestions.** Work found to follow a rule only partly is
   changed or removed in the same round, not queued for later.
6. **Never restate a rule in weaker form.** Point at the skill or AGENTS.md
   section instead.
7. **Learn continuously.** Every skill and agent ends with a "Continuous
   loop" section: read `tools/lessons.py brief --scope <name>` before acting,
   record the outcome and lesson tests after, and fold adopted lessons back
   into its own text by PR (AGENTS.md "Continuous learning loop").

## Stop-sign tables

A skill that has failure modes ends with a table, one row per sign, each
with the move to make instead:

| Sign | Move |
| --- | --- |
| what the agent observes | what it does instead of continuing |

A sign is a reason to stop, not a note to record and carry past.
