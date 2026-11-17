---
name: sheriff
description: "Wildly deep integrity audit of research results in this repository and its siblings: execution proofs and STARK/zk verifiers, Lean theorems (holes, axioms, statement fidelity), solve certificates, cost accounting against matched rho, statistics and preregistration, provenance and replays, and every number in papers. Use for /sheriff, 'is this result real', before a claim, paper or benchmark row lands, after a claimed breakthrough, or on a schedule. Pointer to the canonical skill in aburan28/crypto-autoresearcher; never repairs or changes research state."
---

# Sheriff (pointer)

The canonical skill, its checklist and its tool live in
`aburan28/crypto-autoresearcher`:

- procedure: `.claude/skills/sheriff/SKILL.md`
- beat-by-beat checklist: `docs/sheriff.md`
- deterministic half: `tools/sheriff.py` (`sweep`, `canary`, `check-case`)

Read the canonical `SKILL.md` and follow it. This pointer adds no rule and
lowers none; where it and the canonical skill disagree, the canonical skill
wins. This repository's own `AGENTS.md` evidence rules still bind every
verdict.

## Getting the canonical skill

Use a sibling checkout at `../crypto-autoresearcher`, fetched to `origin/main`.
If there is none, clone it beside this repository, or add it to the session.
Never copy the tool into this repository: two copies of an instrument drift
apart.

## Auditing something in this repository

Run the sweep from the autoresearcher checkout, naming this repository as a
prefix. Its siblings are discovered as `../crypto` and `../cryptanalysis`, or
are passed with `--repo NAME=PATH`.

```sh
cd ../crypto-autoresearcher
python3 tools/sheriff.py sweep --target crypto:<path> [--deep]   # e.g. crypto:papers/lll-degeneracy-note
python3 tools/sheriff.py check-case research/sheriff/SHF-...
```

Cases are written write-once under `crypto-autoresearcher/research/sheriff/`
and published by a PR in that repository. This repository receives only the
fixes its owners choose to make: the sheriff repairs nothing here.

## Continuous loop: learn, adapt, act on your own

AGENTS.md "Continuous learning loop"; ledger `tools/lessons.py`.

1. **Before acting**: `python3 tools/lessons.py brief --scope sheriff`. Apply the
   adopted lessons; treat candidates as hypotheses this run can test.
2. **While acting**: resolve a blocker yourself within your authority
   (diagnose the cause, try the documented remedy, then a materially different
   approach) before escalating; AGENTS.md "Persistent scientific research loop".
   Persistence grants no extra execution or spending authority.
3. **After acting**: record the outcome with evidence,
   `python3 tools/lessons.py outcome --scope sheriff --task "..." --result success|partial|fail|blocked --evidence <receipt|commit|job> [--applied L-...]`,
   and `lessons.py test --lesson L-... --outcome confirm|refute --evidence ...`
   for each lesson you applied. A surprise that would change what the next run
   does becomes `lessons.py add --scope sheriff --trigger "..." --action "..." --evidence ...`.
4. **Adapt**: when a lesson scoped to `sheriff` is adopted (two distinct
   confirmations), propose the edit to this file in a PR that cites the lesson
   id; when one is refuted twice more than confirmed, it retires itself.
