# Odd and mixed-degree PKM towers under F4: does the degree mix change the cost at equal |V|?

**DRAFT, not yet frozen: the ladder top and budget await the pre-flight probe.** 2026-11-14. Study id `EXP-PKM-ODD-001`.
Machine-readable form: [`protocol.json`](protocol.json) (`experiment.protocol/v1`). Runner:
[`run_cells.sh`](run_cells.sh). Analysis: [`analyze.py`](analyze.py) (SHA-256 recorded in `protocol.json` at
freeze). Instrument: [`examples/pkm_odd_tower_f4.rs`](../../examples/pkm_odd_tower_f4.rs).

## 1. Question

A Petit–Kosters–Messeng Kummer tower writes membership in `V = g·μ_S`, `S = q_1⋯q_{n'}`, as the chain
`x_{j+1} = x_j^{q_j}`, `x_{n'}^{q_{n'}} = c`. Every Gröbner measurement in this repository so far used
`q_j = 2` (or radix-`2^k` gradings of the same 2-power tower: `research/pkm_radix_grading_20261104/`).
Odd and mixed degrees were designed but never run, because the tower engine `f4_fp_tower` stores tower
monomials as bit masks and cannot express a rule of degree above 2
(`research/curve25519_leads_wave4_20261104/l11_odd_kummer_design/PROTOCOL.md`, Q-B).

**Question.** At `m = 2` (and `m = 3` on a small ladder), on refutation systems with the same `p`, curve, `g`
and target policy, does writing `V` with steps of degree 3, 5, or mixed 2/3 change F4's counted work
(`field_ops`) by a factor of 2 or more against the degree-2 tower *at the same information* `log₂ S`? If it
does, does the factor grow with `S`? Does the order of a mixed sequence (2,3,… vs 3,2,…) matter at equal `S`?

**Why it matters.** The prime-field barrier note's reading is that the cost is governed by information
(`log₂|V|`), not by how the membership is written. A degree mix that changes F4's work by a factor that grows
with `S` would contradict that reading. A constant factor would be engineering. No outcome here moves an
exponent at cryptographic size, and no outcome is a claim about Pollard rho.

## 2. Claims and their classes

| id | claim | class | decided by |
|:--|:--|:--|:--|
| C1 | On every non-censored system, F4's verdict (reached `1` or not) equals the exhaustive verdict over `V^m` | exact computation | every row (`agree`) |
| C2 | Every planted system is consistent under F4, and the planted point satisfies every equation | exact computation | `PL-*` cells |
| C3 | Arm `X` changes refutation `field_ops` by the reported ratio `r_X(S)` against the interpolated degree-2 tower | exact computation per system; comparative across the ladder | main ladder |
| C4 | The ratio grows, fades or holds with `S` | comparative, descriptive only over the ladder below | §5 rule |
| C5 | Order of a mixed sequence: `A32 / A23` at equal `S` | exact per system; comparative | `n = 4, 6` |
| — | Anything about larger `S`, `m ≥ 4`, cryptographic sizes, isogeny towers or rho | out of scope | not established |

## 3. Arms, controls, cells

**Arms** (degree sequences; all Kummer, `V = g·μ_S`):

| arm | sequence | `S` on the `m = 2` ladder | `log₂ S` |
|:--|:--|:--|:--|
| A2 (reference) | `2^t`, `t = 4…10` | 16 … 1024 | 4 … 10 |
| A3 | `3^k`, `k = 3…6` | 27, 81, 243, 729 | 4.75 … 9.51 |
| A5 | `5^k`, `k = 2…4` | 25, 125, 625 | 4.64 … 9.29 |
| A23 | `2,3,2,3,…`, length `n = 4…7` | 36, 72, 216, 432 | 5.17 … 8.75 |
| A32 | `3,2,3,2,…`, length `n = 4…7` | 36, 108, 216, 648 | 5.17 … 9.34 |

**Fixed across arms.** One prime `P1 = 1399680001` (`P1 − 1 = 3·2^10·3^6·5^4`, so every `S` above divides
`P1 − 1`), one seed `20261114`, hence one `g` (the tower's first draw), one curve
`E: y² = x³ + ax + b` (seed-derived), and one target sequence. A refutation target is the first draw of the
target generator that no tuple of `V^m` decomposes (exhaustive check), so arms with the same `V` (A23 and A32
at `n = 4, 6`) get the same targets. Instrument: `pkm_odd_tower_f4` at this PR's commit, one binary (SHA-256
recorded by `run_cells.sh`), grevlex, degree bound `D = 64`, `RAYON_NUM_THREADS=4` on every cell.

**Controls.**
- *Correctness (C1, C2):* every row records the exhaustive verdict beside F4's. Planted cells (`PL-*`, one draw
  each, `S ≤ 256`) run first. Any disagreement or failed planted row makes the study `instrument_invalid`.
- *Negative / null control:* `--control null`. Each summation polynomial is replaced by one with `S₃`'s
  monomial support and random nonzero coefficients (seeded), under the same tower, run on every main cell.
  If tower and null give the same §5 band, the effect belongs to the system's shape, not to the curve.
- *Positive control of the detector:* the ratio procedure applied to A2 against itself shifted by one bit
  (`W(t+1)/W(t)`, from the A2 rows). It must report `≥ 2` at every adjacent pair, i.e. the procedure sees a
  one-bit difference in `|V|`. If it does not, the study cannot call a factor-2 effect and reports so.
- *A/A:* cell `AA-M2-A3-k5-tower` reruns `M2-A3-k5-tower` under a new name. Counts must be bit-identical. This
  is a determinism check: the counted metric has no variance at fixed input and threads.
- *Replication:* a second prime `P2 = 3265920001` (`P2 − 1 = 7·2^10·3^6·5^4`) on a subset (A2 `t = 4, 6, 8`;
  A3 `k = 3, 4, 5`; A5 `k = 3`), tower only.

**Cells.** `m = 2` main ladder: every arm × size above, `tower` and `null`, 3 refutation draws each.
`m = 3` secondary ladder (`n_vars = 3n' + 1`): A2 `t = 2…5`, A3 `k = 2, 3`, A5 `k = 2`, A23 `n = 2…4`,
tower and null, 3 draws. Planted: A2 `t = 4…8`, A3 `k = 3…5`, A5 `k = 2, 3`, A23 `n = 4…6`. Replication
and A/A as above. Full list and order: `run_cells.sh`.

**Order.** Fixed in `run_cells.sh`: planted first, then the main ladder interleaved by `log₂ S` with the arms
rotating, tower before null; then `m = 3`, replication, A/A. Counts do not depend on order. The order only
guards the clock, which is not a study metric.

**Budgets.** `--budget 1800` seconds per system, `--dmax 64`. A system that times out or drops a pair above
the bound is *censored*. It is reported with its partial counts as a lower bound and never imputed. No cell
is rerun with a larger budget to get a different outcome.

## 4. Metrics

- **Primary:** `field_ops`, the `F_p` multiplications of F4's row reductions (counted, exact, per system).
- **Secondary:** `solving_degree_max`, `max_cols`, `steps`, per system.
- **Clock:** `ms` is recorded, not used. The host is unisolated.
- **Reduction:** the three draws of a cell are separate systems. Per-draw ratios are computed; the cell value
  is their median, and the band rule below needs every draw in one band.

## 5. Decision rule (frozen)

Let `W_2(x)` be the A2 reference: the log₂-linear interpolation of the median A2 `field_ops` over
non-censored A2 cells, as a function of `x = log₂ S`, on the same `(m, control, p)`. For an arm cell at
`log₂ S` inside the reference range, `r = W / W_2(log₂ S)` per draw.

1. **Validity first.** C1, C2 and the A/A identity must hold. Otherwise the verdict is `instrument_invalid`
   and nothing below is read.
2. Per `(m, control, arm)`, at the arm's largest non-censored, in-range cell:
   - `loss`: every draw `r ≥ 2`; `gain`: every draw `r ≤ 0.5`; `no_material_effect`: every draw in
     `(0.5, 2)`; `mixed`: draws in different bands.
   - Qualifier `growing` (for `loss`/`gain`): the median `r` moves monotonically away from 1 over the arm's
     last three completed in-range cells. Otherwise the factor is called constant.
3. **Attribution.** Tower and null in the same band → "the effect belongs to the system shape". Different
   bands → "elliptic-specific at these sizes".
4. **Order (C5).** `A32/A23` per draw at `n = 4, 6`, same bands.
5. **Replication.** A3 and A5 at `P2` must give the same band as at `P1`, or the verdict is `not_replicated`.
6. **Descriptive only:** least-squares slope of `log₂ field_ops` against `log₂ S` per arm. No interval, no
   decision.

**Pre-registered prediction (this author's, from the barrier note's information reading):**
`no_material_effect` for A3, A5, A23 and A32 at `m = 2` and `m = 3`, on tower and null alike, and no order
effect. **It is falsified by** `gain` or `loss` for any arm on the tower at `m = 2`.

**Multiple comparisons.** Eight primary verdicts (4 arms × `m ∈ {2, 3}`) plus two order cells. They are band
rules on exact counts with no p-values, so no correction applies. Every verdict is reported.

## 6. What does not count

Changing cells, budgets, bands or the reference after seeing a study row. Dropping a censored row. Reading a
clock. Calling a constant factor an exponent. Extrapolating past the ladder. Any change needs a dated
amendment in §8 that states whether it preceded the data it affects.

## 7. Execution, pre-flight and deviations

- **Backend.** This container: local, 4 vCPU, 15 GiB, unisolated (`host: local-ccr-4vcpu`). The repository
  default is Iso Lab on EKS (`AGENTS.md` §9a). This session has no route to it: no `aws` CLI and no AWS
  identity in the container. The primary metric is counted. The backend changes no study value, only which
  cells finish inside the budget.
- **Resume checkpoint.** `scripts/checkpoint.sh gate` fails here ("no way to upload": no AWS profile, no upload
  token). The run is launched with the hook's sanctioned opt-out, `CHECKPOINT_SKIP=no-s3-credentials`, which
  appends a bypass entry to `pcc/ledger.jsonl`. `.resume/RESUME.md` is filled in and committed with this
  protocol, so a fresh session can resume from Git.
- **Model tiers.** The director/executor relay (`tools/task_card.py run`) needs a benchmark record whose raw
  data goes to S3, which this session cannot reach. The runner is launched directly by the main session. This
  is a deviation from `AGENTS.md` "Model tiers", disclosed here.
- **Knowledge base.** The `research-kb` MCP server and CLI are not available in this container. Prior work was
  found by searching both repositories (`git grep`, 2026-11-14). The closest records are cited in §1.
- **Pre-flight (smoke, before freeze; `smoke/`).** Planted and refutation systems at `S = 12…36`: every row
  `agree = true`, every planted point checks. `m = 2`, A2 `t = 8`: 9.2 s, `field_ops ≈ 1.0·10^10`.
  Probe timings for the top of the ladder are in `smoke/` and §8. The smoke rows were seen before this
  protocol was frozen. They are not study data.
- **Checkpointing.** Per cell: `run_cells.sh` skips completed cells and reruns a partial cell from the start
  under the same name (the partial file is kept in `runs/partial/`).

## 8. Amendments

None at freeze.
