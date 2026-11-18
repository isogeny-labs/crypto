# Raw evidence

Frozen output of the sweeps. The research note and the scoreboard cite these
files and do not recompute anything from them.

* `boundary.json` — the counting floor, derived not measured. The rho
  reference it quotes is cited from
  `experiments/ecc2k130_extension_field_boundary.json`
  (`target.log2_rho_reference` = 2^60.809, S = 0.07743), not recomputed here.
* `summary.json`, `<name>.json` — the three-point sweeps, one file per support.
* `run.log.live`, `four_point.log` — progress logs.
* `four_point_summary.json`, `four_weight_two.json`,
  `four_weight_two_frobenius_stable.json`, `four_random_matched.json` — the
  completed four-point collision sweep.

The restored four-point evidence contains three complete supports totalling
37,507,774 signed pairs and 2,365 checked digest-collision candidates, with
zero four-point relations. The apparent 93% endpoint in the first support is
only its last 30-second progress heartbeat; its following JSON record says
18,160,382 of 18,160,382 pairs were consumed. The summary, each standalone
record and all three JSON records embedded in `four_point.log` agree exactly
after omitting the standalone-only empty `relations` list.

An older copy of this README incorrectly said the run had stopped at that
heartbeat and that the JSON files did not exist. Commit `1e85fdf181` restored
the files after sparse-branch merges had deleted main-tree content, but also
restored that stale explanation. Check custody and three-way parity without
rerunning the sweep with:

```sh
python3 research/ecc2k130_relations/verify_four_point_evidence.py
```

The orbit-grouped four-point sweep that replaces it is in

* `four_point_orbit_summary.json`, `four_orbit_nb<N>.json` — one file per
  normal basis, eight of them, every one complete;
* `four_point_orbit.log` — progress log;
* `four_point_orbit_boundary.json` — what the Frobenius quotient is worth
  against the `m = 4` cost boundary, derived not measured, from
  `orbit_boundary.py`. The rho reference is the same cited 2^60.809.

`run.log.live` carries its name for an unglamorous reason. The first
three-point run was launched with its output redirected to `run.log`, and a
later `git stash` of this directory replaced that file's inode, which left the
running process writing to an unlinked file. The stream was recovered through
`/proc/<pid>/fd/1` before the process exited and preserved here. No JSON
evidence was affected: each result file is opened fresh on write.

Section 5 (the cost of a logarithm, as against a homogeneous relation):

* `target_boundary.json` — derived, from `target_boundary.py`. Prices T + 1
  relations against known targets, optimising over support size, relation
  length and meet-in-the-middle split, with memory charged at zero. Carries
  the homogeneous extension past m = 8 alongside, because that is the
  accounting that crosses below the reference and the contrast is the point.
* `amortised_attack.json` — measured, from `validate_amortised_attack.py`.
  The section 5.3 structure run end to end: table of canonical sigma-classes
  built once, single points streamed per target, planted logarithm recovered
  and checked by the point identity [d]P = Q.
* `target_boundary.json` also carries `family_floor` — a lower bound on every
  configuration, derived not searched, and the number parity must be compared
  against.
* `target_decomposition.json` — measured, from `validate_target_model.py`.
  The decomposition rate the above depends on, against exhaustive enumeration
  on small analogues, and the direct check that the reachable sum set is
  sigma-closed.

