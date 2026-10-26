# Amendment 3: n = 19 from the interrupted sessions; restartable units

Written 2026-10-18 20:30 UTC, before Parts B and C.

A third container reclaim stopped `A-n19-fast` at 23 of 32 executions,
about 15 minutes in. The reclaims are not tied to idleness; their spacing
was 2.5 h, then 15 min. A multi-hour ecbench session cannot be completed
here, and a session cannot resume.

**n = 19 is reported from the union of the interrupted sessions.**
`A-n19-02` and `A-n19-fast` measured the same workloads (same spec seeds;
`A-n19-fast` is `SPEC-A-n19` without its chain arm). Coverage by arm:

| arm | workloads with a verified run |
|---|---|
| `rho-strong`, `rho-frob`, `std-unsplit-g4` | 0–5 |
| `clo-unsplit-g4` | 0–4 |
| `clo-chain-g4` | 0–3 |

- The remaining workloads of each arm are **censored by container
  lifetime**. They are reported as such and never imputed.
- Paired figures for an arm use only the workloads on which both arms
  ran.
- Where both sessions ran the same (workload, arm), both records are kept,
  and their deterministic counts must agree. That agreement is an extra
  replication check.
- Both sessions are audited with `--allow-interrupted --replay 10`. The
  first interrupted session, `A-n19`, is audited the same way.

**Parts B and C and the audits** run from `resume3.sh` as restartable
units:

- each Part B cell is a unit, skipped when its output is complete;
- an ecbench session whose directory exists but is not `complete` is
  moved to `<name>-interrupted-<k>` and rerun, never overwritten;
- the operator commits after each unit.

The protocol's inputs, specs, seeds and binaries are unchanged.
