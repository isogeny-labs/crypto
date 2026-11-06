---
name: experiment-protocol
description: Design, preregister and freeze an unbiased controlled experiment as an experiment.protocol/v1 record before any data is taken, and bind the run's unified experiment.export/v1 to it. Pointer to the canonical skill in aburan28/crypto-autoresearcher; in this repository the controlled-experiment skill decides whether a claim needs an experiment and what its controls are, and this record is where that design is written once.
---

# Experiment protocol (pointer)

The canonical skill, schemas and checker live in
`aburan28/crypto-autoresearcher`:

- skill: `.claude/skills/experiment-protocol/SKILL.md`
- rules: `docs/experiment-protocol.md`
- schemas: `schemas/experiment-protocol.v1.json`, `schemas/experiment-export.v1.json`
- checker: `tools/experiment_protocol.py` (`validate`, `freeze`, `seeds`,
  `power`, `blind`, `from-ecbench`, `from-rows`, `check-export`, `flow`)
- worked example: `examples/experiment-protocol/ecbench-l2-smoke-subset/`

Read the canonical `SKILL.md` and follow it. This pointer adds no rule and
lowers none; where it and the canonical skill disagree, the canonical skill
wins. This repository's `AGENTS.md` measurement rules (boundary, table,
ratio; §5a pricing band; §10 baseline discipline; §12 ecbench) and the
`controlled-experiment` skill still bind every design here.

## Using it from this repository

Use a sibling checkout at `../crypto-autoresearcher`, fetched to
`origin/main`; if there is none, clone it beside this repository or add it
to the session. Never copy the tool into this repository: two copies of an
instrument drift apart, and research tooling here is native
(AGENTS.md "Implementation language").

```sh
A=../crypto-autoresearcher/tools/experiment_protocol.py
python3 $A validate research/<study>/protocol.json
python3 $A freeze research/<study>/protocol.json --commit auto --filed-before-affected-data
python3 $A from-ecbench research/<study>/sessions/<session> --protocol research/<study>/protocol.json \
    --out research/<study>/export.json --export-id <study>-<session>
python3 $A check-export research/<study>/export.json --protocol research/<study>/protocol.json --verify-files
```

`protocol.json` sits beside `PROTOCOL.md` in the study directory and is
committed before the first data point; `export.json` sits beside the
sessions, never inside one (ecbench sessions are sealed and re-audited by
CI). An ecbench spec is the instrument; the protocol is the preregistration
that names it, its controls, its seeds and what would count.
