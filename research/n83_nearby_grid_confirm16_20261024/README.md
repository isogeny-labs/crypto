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

Regenerate and check the immutable fixture from the repository root:

```sh
python3 -m research.n83_nearby_grid_confirm16_20261024.generate_fixture
python3 -m unittest research.n83_nearby_grid_confirm16_20261024.test_select
```
