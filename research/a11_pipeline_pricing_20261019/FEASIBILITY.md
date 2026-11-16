# A11 pipeline pricing: feasibility verdict (2026-10-19)

**Verdict: the swap is NOT feasible today. ecbench has no method that builds or
solves a PKM-tower summation system, so there is no incumbent presentation to
swap A11 into and no S for either arm. Nothing was measured; no result is
claimed.** Branch `agent-b-a11-pricing`. Conductor: `conductor check` ran
(scope `research/a11_pipeline_pricing_20261019`) and reported no conflict.

## 1. Pipeline map (file:line)

| step | where |
|:--|:--|
| PKM Kummer tower as an ecbench factor base | `src/cryptanalysis/ic_framework/prime_tower.rs:1-24` (docs), `PrimeKummerBase` at `:53`, `build` at `:75`: `V = g*mu_{2^t}`, lifted to curve points by `lift_abscissae` (`:~107`). It builds a point list only. |
| Isogeny-tower base | same file, `PrimeIsogenyBase` (aux-curve search charged to the FB phase, `methods.rs:3249`) |
| Method wiring | `solve_ic_prime`, `src/cryptanalysis/ecbench/methods.rs:2110-2184`: base chosen at `:2122-2134` (`prime-kummer` at `:2129`), oracle chosen at `:2142-2154` |
| Prime-curve oracles that exist | `subtract`, `mitm`, `mitm-prime-batch`, `mitm-fiber`, `rootfind-mitm`, `rootfind-pairs`, `rootfind-subtract` (`methods.rs:2147-2153`; impls `ic_framework/plugins.rs:1380,1417,1504,1611`, `rootfind.rs:162,277`, `fiber.rs:410`). All decide a relation by group-law additions (`rootfind.rs:1-32`: "by the group law alone, never by an algebraic system"). |
| What one relation attempt costs in the counted unit today | point additions through the `GroupOps` ledger (one each), plus counted-not-charged Legendre symbols and square roots in the FB phase (`docs/ecbench/README.md:456-474`). No Groebner/F4 term on any prime-curve arm. |
| Where PKM tower systems ARE solved | only in diagnostics: `examples/pkm_tower_pilot.rs:1-12,26-` ("prices no pipeline, reports no S"), library engines `src/cryptanalysis/f4_fp_tower.rs:1-60` and `f4_tower_generic.rs`, and A11 in `examples/fb_designs_slope.rs` (`kummer_eqs:438`, `tower_system:802`, `norm_chain_system:893`, `check_norm_chain:1286`; header `:63` "prices no pipeline"). Nothing under `src/cryptanalysis/ecbench/` references `f4_fp_tower`, `f4_tower_generic` or `sig_fp_tower` (grep). |
| Design note for the missing oracle | `research/notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md` (not in this sparse checkout; cited by `examples/pkm_tower_pilot.rs:3-9`): "designs an algebraic decomposition oracle ... and pre-registers the one measurement that decides whether to build it". It has not been built as an ecbench plug-in. |
| Precedent for charging F4 in ecbench units | `solve_ic_fp4` (`methods.rs:2193-`): F4-style work priced in F_p multiplications at the curve's multiplications per addition; `f4_fp::field_ops_total` (`f4_fp.rs:84`) is the counter the A11 runs report as `field_ops`. |

So the A11 comparison "norm-chain vs Kummer tower" exists only as
`field_ops` of one F4 solve per system, in a harness that is not ecbench. A
relation attempt in ecbench for a `prime-kummer` base costs group additions
and never touches a tower system. A11 changes the *equations* given to a
solver, not the base set `V = g*mu_{2^t}` (same set, different presentation),
so it cannot be expressed as a factor-base option either: `prime-kummer:t=T`
yields the same `FB1` id and the same ecbench cost whichever presentation a
solver would use.

## 2. What would have to change

1. A new `DecompositionOracle<PrimeCurve>` plug-in (`pkm-tower-f4`, `m = 2..3`)
   in `ic_framework/`: per target, build the S3 (or S3 chain) system over the
   tower for the target abscissa, solve with `f4_fp_tower`/`f4_tower_generic`,
   read out the solutions in `V^m`, lift to signed curve points, and verify
   `sum = R` with counted group additions (the harness's witness check).
2. A presentation option on that oracle (`presentation=tower|norm-chain`),
   porting `norm_chain_system` (+ its soundness check) from the example into
   the library, default `tower`, with every planted/derived solution checked.
3. A pricing rule converting F4 field multiplications to the counted unit
   (S) at the curve's multiplications per addition, as `solve_ic_fp4` does,
   stated and calibrated; the solver's non-multiplication work stays named in
   `unpriced`. Trace replays must reproduce `field_ops` exactly (the
   committed A0/A11 rows already show trace-matched replays, so this is
   feasible).
4. Frozen-source replay compatibility (the ecbench library is rebuilt
   against frozen snapshots; see `docs/ecbench/README.md` on `pdp3_koblitz.rs`),
   a spec, a preregistration, then the interleaved/replayed/paired session.
5. Expectation, not a result: at m = 3 on p1 = 786433 (`p-1 = 3*2^18`) the
   committed F4 cost is already 2^12-2^30 field multiplications per system,
   while matched rho on a ~20-bit group is ~2^10 steps, so the S ratio to
   rho is likely to be large for both arms; whether A11 changes that cannot
   be said without the measurement.

**Effort estimate:** roughly 1.5-3 engineer-weeks: oracle plug-in and
witness lifting (4-6 days), A11 port with checks (2 days), pricing
calibration and replay determinism (3-4 days), tests/clippy/frozen-replay
compatibility (2 days), preregistered session plus audit (2-3 days on this
4-core host for the small cells only).

## 3. What the existing evidence does show (not an S result)

`field_ops` per F4 system, m = 3, p1, from
`research/factor_base_designs_20261015/runs/analysis-amendment4.txt:19-24,106-112`
(see `reports/run-20261019T1634Z/` for the graph): A11 is lower than A0 at
t = 2, 3, 4 (4663 vs 508104; 4788601 vs 238512767; 855071386 vs
15920614779) and its fitted cost growth per bit is steeper (kappa 8.769 vs
7.468). This is a stage diagnostic of the solver axis (`AGENTS.md` section
2): it is not S, not a speedup, and not a pipeline price.

## Paper bound, 2026-10-20: the oracle is not worth building to price A11

Before building the oracle above, a bound from committed rows asked whether
an A11 algebraic oracle could ever compete per discrete logarithm with the
group-law oracle ecbench already runs. Native computation:
`examples/a11_oracle_paper_bound.rs`. Output: `paper_bound.txt`. Report:
`reports/run-20261020T0630Z/REPORT.pdf`.

- **Measured group-law arm** (PKM20: `prime-kummer:t=8`,
  `rootfind-mitm:m=3`, p = 946177, 32 verified solves): 27 015, 29 332 and
  50 187 GAE per discrete logarithm, with its unpriced counters at 0, 0.1
  and 1 GAE (§5a).
- **Algebraic lower bound**, every term chosen in A11's favour:
  - row-reduction multiplications only;
  - 50 field multiplications per GAE;
  - one relation per decomposable target;
  - N + 1 relations per logarithm, with no linear algebra and no setup;
  - t > 5 held at the t = 5 count.

  Its cheapest point is 7.1·10⁹ GAE per logarithm (A11, t = 3).
- **Verdict:** at least 1.4·10⁵ times the group-law arm at every weight in
  the band. A three-order-of-magnitude error in the model would not change
  the order. The 1.5–3-week oracle is not worth building to price A11.
  A11 stays a constant-factor gain inside F4 that does not reach the
  pipeline.
