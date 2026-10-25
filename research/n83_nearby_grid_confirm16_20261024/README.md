# N83 independent 16-point confirmation fixture

These 16 public N83 points were sealed before the result of the
K1102–K1120 Tier 1 screen at GitHub run 40136901841 was inspected. Each
point has its own canonical, one-target workload ID. The deterministic
scalar sidecar is for independent replay and stays outside IC and rho child
inputs. `exclusion_audit.json` checks the earlier public-point inventory,
the preceding nearby-K point, and the 19-base screen point.

The confirmation candidate set is fixed before timing by `select.py`: take the three
lowest paired online ratios and the three lowest paired cold ratios from
the **first admitted** run of the exact screen protocol and workload,
breaking ties by ascending K, and add K1107. Deduplicate the union. If no
screen run passes its complete Tier 1 gate, candidate selection remains
unknown. The selector independently replays a raw screen output directory
before applying the rule. No confirmation timings or factor-base choice are
present here.

Run every selected candidate on all 16 points as separate one-target IC
and same-point rho pairs with alternating order, independent scalar replay,
complete online phase and supplementary cold accounting, and Tier 1 host
receipts. Preserve every failed, noisy, timed-out, and OOM row. Selection
over these 19 signed-Frobenius orbit bases requires the complete admitted
confirmation; wider factor-base families require a separate paired
comparison.

`protocol.py`, `run.py`, `verify.py`, and `aggregate.py` implement that
confirmation. Two distinct GitHub Actions Tier 1 runners each receive eight
points. For every point, the selected candidates rotate in order; each has
one independent IC and same-point rho child, with their order alternating by
point and candidate. K1107 fresh-process controls bracket each point.
Every child is capped at 900 seconds for IC or 120 seconds for rho, with a
240-minute producer cap per stratum, 4 GiB child RSS cap, and 4 GiB free-disk
preflight. The verifier checks the exact candidate/base digest, relation,
scalar, one-target online phases, cold process cost, resource receipt, and
same-point rho result. All child resources must pass, visible CPU steal must
stay below 0.5% of that child's CPU wall, and each point's online and cold
K1107 controls must agree within 20%. Failed rows retain their status and
null costs.

The two-stratum aggregate requires 16 verified rows for every selected
candidate and distinct runner names. It reports every canonical
`IC1...W...R1` row. Its online ranking uses each candidate's geometric mean
of the 16 one-target online times; fresh-process cold ranking is
supplementary. For the descriptive fastest candidate, exact two-sided sign
flips test paired log online ratios or cold millisecond differences against
every other candidate. The threshold is `0.05 / C(selected_count, 2)`.
If any stratum or pair fails its gate, the corresponding selection stays
`BOUNDED_UNKNOWN`.

The [workflow](../../.github/workflows/n83-nearby-grid-confirm16.yml) first
runs fixture and schedule checks. Its measured jobs are skipped until a
reviewed `dispatch.json` with schema
`n83.nearby-grid-confirm16-dispatch/v1`, status `SEALED_FOR_DISPATCH`, and
numeric `screen_run_id` and `screen_attempt` is committed after the first
admitted 19-base screen. The same workflow can also be invoked manually with
those two inputs. It downloads and independently replays the raw screen before
building or timing either stratum, then uploads each stratum's raw receipts
and the aggregate separately. The earlier run `40136901841` stopped at its
frozen disk preflight and cannot nominate candidates.

Regenerate and check the immutable fixture from the repository root:

```sh
python3 -m research.n83_nearby_grid_confirm16_20261024.generate_fixture
python3 -m unittest research.n83_nearby_grid_confirm16_20261024.test_select
python3 -m unittest research.n83_nearby_grid_confirm16_20261024.test_execution
python3 -m research.n83_nearby_grid_confirm16_20261024.run --fixture-preflight
```
