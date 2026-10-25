<!-- BEGIN SHARED SESSION LOG -->

## Session log

At the start of every session, read the newest dated Markdown file in
`docs/session-logs/`.

Write today's log at `docs/session-logs/YYYY-MM-DD.md`, using the current date.
These paths are relative to the repository or workspace root.
Create the folder and file if missing. Claude and Codex share this directory.

Do not wait for the user to end the session. Append a timestamped entry after
every finished piece of work, after every decision, and whenever we switch
tasks. Preserve previous entries. Record concrete facts from commands and
logs; distinguish verified results from plans or assumptions.

Every entry has four headings:

- What was done (commands, files, features)
- Decisions made (and why)
- Open threads (unfinished, pick up next time)
- Errors (what went wrong, how it was handled)

Commit project handoffs with the related PR so they survive across sessions.
Keep entries about this project; do not copy private global logs or chat transcripts.

<!-- END SHARED SESSION LOG -->

Open pull requests ready for review, never as drafts, whatever the runtime
defaults to, and mark an existing draft ready; only a user request in the task
makes a draft. Enable auto-merge as you open the PR (`gh pr merge <n> --auto
--merge`); a stacked PR cannot auto-merge until it is retargeted to `main`.
This and the repository's other rules are in
[`AGENTS.md`](AGENTS.md) ("Default workflow").

**Success criterion** (owner, 2026-10-15): every verified, measured
improvement over the current index-calculus baseline is a success. Beating
Pollard rho is the long-term goal, reached iteratively, not by one
breakthrough. Report each row's gap to the matched rho as the distance
remaining, not as a pass/fail gate. Relabelling and accounting changes are
not improvements. Full rule: [`AGENTS.md`](AGENTS.md), "Success criterion:
iterative improvement of the index-calculus baseline".

Before surveying for ECDLP leads or proposing one, read the lead atlas in the
sibling crypto-autoresearcher checkout (`knowledge/leads/ecdlp/`,
`python3 tools/lead_atlas.py --brief`, printed at session start). It records
what has already been tried here and in the sibling repositories, and whether
anything changed since its last review (AGENTS.md "Pointers").

Every experiment run, every PR that lands a result, and every research search
leaves a report, a graph, and a PDF: follow the `research-visuals` skill
(`.claude/skills/research-visuals/SKILL.md`; AGENTS.md "Research searches must
leave visual reports"). Performance measurements and speedups also copy their
PDFs, SVG graphs and PNG renders into `benchmarks/<YYYYMMDD>-<slug>/` with a row
in `benchmarks/README.md`, in the same PR (AGENTS.md "Performance results also
land in `benchmarks/`").

Every big improvement also updates the index-calculus leaderboard in the same
PR: regenerate `docs/ic-leaderboard.html`, `docs/ic/LEADERBOARD.md` and
`docs/ic/leaderboard.json` with `python3 scripts/build_ic_leaderboard.py`, and
the lab browser with it. That holds for a phase or component speed-up too, as
its own section outside tables A–C (AGENTS.md "Standing workflow rules" and
§7b).

<!-- conductor:begin -->
Validation gate (owner, 2026-10-19): no row reaches the IC leaderboard or
`benchmarks/README.md` without a replay certificate, a validator report and a
matched rho reference (with the §5a band), named by the evidence directory's
`VALIDATION.json`; pre-gate rows render as "unvalidated (pre-gate)". Rule:
[`AGENTS.md`](AGENTS.md), "Standing workflow rules"; schema:
[`docs/VALIDATION_GATE.md`](docs/VALIDATION_GATE.md).

Before making code changes, obtain or attach to a Conductor task. Run
`conductor check --summary "…" --scope path:…` first — if someone already holds
those files, it will tell you who and what to do about it. Read `.conductor/WORKFLOW.md`
and the active task card. Report scope expansion before editing outside the reserved paths.
Do not publish chat transcripts or secrets as task metadata.
<!-- conductor:end -->

In Claude Code on the web, `.claude/hooks/session-start.sh` installs Conductor
and cairn and starts a control plane when the session starts. Elsewhere
Conductor is not preinstalled: [`docs/CONDUCTOR.md`](docs/CONDUCTOR.md) says how
to install it and connect a session to a control plane. Until then the hooks
warn and let edits through.

The session that opens a PR stays with it until it is merged or closed: it
checks in on CI, reviews and merge state, fixes the failures and feedback its
change causes, and merges once green and authorized (a non-required check
queued 24 hours without starting does not block the merge). Any runtime may do this,
Claude Opus and Fable included. Check in through PR-activity subscriptions or
safety-net check-ins (about 50 minutes, then about 4 hours; stop after 3 quiet
check-ins), and never busy-poll, sleep in a loop, or re-read job logs faster
than that cadence. AGENTS.md, "Default workflow: open the PR, check in, and
land it".

Model tiers (owner, 2026-10-17): frontier/core models reason and decide;
`experiment-director` (Sonnet) writes a frozen command card, the main session
relays it verbatim to `executor-mechanical` (Haiku), and the director assesses
the run. AGENTS.md, "Model tiers". After a relay the main session records what
it cost: `python3 tools/token_usage.py session --units <cards> --unit-name card`
(tokens and list-price cost per agent and tier, cache hit ratio, cost per card;
`autofix-queue.yml` records each attempt the same way). Metrics and the
transcript accounting traps: [`docs/token-efficiency.md`](docs/token-efficiency.md).

Every cross-method ECDLP comparison prices the unpriced native work of
every arm, rho references included, at 0, 0.1 and 1 GAE per operation and
ranks two arms only where the order holds across that band (AGENTS.md §5a).
