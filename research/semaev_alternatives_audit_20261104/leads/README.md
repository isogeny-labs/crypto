# Lead files for the 2026-11-04 Semaev-alternatives audit

These are drop-in entries for `knowledge/leads/ecdlp/` (lead-atlas schema: YAML frontmatter,
`status` in {open, blocked, closed-obstructed, closed-known, dominated, exhausted}, a `next_action`
that is executable, and evidence pointers). They are written here because the lead atlas is not
present in this checkout (`crypto-autoresearcher/knowledge/leads/` is absent).

| file | replaces / advances | status |
|---|---|---|
| `LEAD-ICP-coppersmith-presentation-audit.md` | advances `LEAD-ICP-coppersmith-smallroot-semaev` | open |
| `LEAD-ICP-vieta-conic-sieve.md` | new (Idea 2) | closed-obstructed (Lean: arity 2 cannot win) |
| `LEAD-ICP-gamma0-binary-forms.md` | new (Idea 3) | closed-obstructed (census measured) |
| `LEAD-ICP-sqf-global-relations.md` | new (Idea 10) | exhausted (measured; not a proof) |
| `LEAD-ICP-m4-instance-set.md` | new (Idea 15) | closed-known (instrument) |

Evidence for all five: `REVIEW.md`, `experiments/*.json*`, `scripts/*.py` in this directory.
Costs are deterministic operation counts priced at 0 / 0.1 / 1 GAE (AGENTS.md §5a); wall time is
recorded but never ranked on.
