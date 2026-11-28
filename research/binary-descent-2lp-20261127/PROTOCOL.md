# Protocol: double-large-prime Weil descent (arm D2) against matched rho

Study `b2lp-20261127/v1`. Seed policy `isogeny-labs/experiment-seeds/v1`.
Status `[pcc:unverified]`, L0, one shared 4-vCPU / 15 GB cloud container.
**Single target only** (owner instruction): every solve is one cold solve of
one public `Q`; no multi-target amortisation anywhere.

Builds on crypto PR #5261 (`research/binary-weil-descent-20261126/`, arm D,
study `bwd-20261126/v1`), stacked on #5253
(`research/binary-composite-rootfind-20261126/`, study `bcr-20261126/v1`).

## Part A — pilot rule (frozen before the pilot is run)

The pilot only chooses the small-base constant θ of each D2 arm and the rho
lane count of the new rungs. It uses held-out instances only:

- θ pilot: curves `d2pilot-curve|n|ci`, targets `d2pilot-target|n|ci|ti`,
  relation stream `d2pilot-stream|n|ci|ti`; n ∈ {30, 36, 42}, ci ∈ {0, 1},
  ti ∈ {0, 1} (12 cases); grids
  - β = 2/3: θ ∈ {0.5, 1, 2, 4, 8, 16};
  - β = 0.528: θ ∈ {1, 2, 4, 8, 16, 32};
  with the small base `fs = min(F − 1, ⌈θ·F^β⌉)` (F classes including T).
  Rule: for each β, θ* = argmin over the grid of
  `Σ_rungs log( gmean_cases Meq(s=1,u=1) )`; ties to the smaller θ.
- rho lanes for the new rungs n ∈ {51, 54, 57, 60}: held-out
  `pilot-curve|n|ci` (ci = 0, 1), `pilot-target`, `pilot-rho|n|ci|ti|W`,
  8 targets per curve, W ∈ {16, 32, 64, 128, 256}; W* = argmin of mean
  Meq(s=1,u=1), the #5253 rule with a grid widened past #5261's edge W = 64.

Pilot outputs: `data/pilot_theta.jsonl`, `data/pilot_rho_ext2.jsonl`. They
are not evaluation data.

Part B (arms, predictions, analysis) is written and frozen after the pilot
and before any evaluation data.

### Pilot outcome (recorded before Part B was frozen)

- θ pilot (`data/pilot_theta.jsonl`, 144 records, all correct): β = 2/3 →
  **θ = 4** (score 54.004; θ = 8: 54.069, 16: 54.268, 2: 54.965);
  β = 0.528 → **θ = 16** (53.931; 32: 54.061, 8: 54.385).
- rho lanes (`data/pilot_rho_ext2.jsonl`): n = 51 → **W = 128** (mean
  2.085·10⁸ Meq; 256: 2.265·10⁸; 64: 2.521·10⁸), n = 54 → **W = 64**
  (3.989·10⁸; 128: 4.023·10⁸). The n = 57 and n = 60 pilot cells were
  stopped part-way (255 of 320 records kept) once the prediction below showed
  those rungs exceed the wall budget; they choose nothing.
- **Deviation from the pilot rule as written:** none for θ. The rho pilot
  used 8 targets per curve, as written.

## Part B — evaluation protocol (frozen before any evaluation data)

## Question

On binary composite `F_{2^n}`, `n = 3n′`, with #5261's Gaudry–Diem k = 3
descent oracle unchanged: does a double-large-prime variant (small subfield
base, large primes = the rest of the subfield points, relations with ≤ 2
large primes combined through a graph) lower the cold single-target cost
against arm D and against matched negation-map rho, and does its measured
growth support #5261's model that only a 2LP variant could cross rho
(n ≳ 130 before constants)?

## Arms

Shared by every D-type arm, bitwise as #5261: curves, targets, the A1
relation-target stream `icstream-a1|n|ci|ti`, the descent oracle
(`descent.rs` by `#[path]`), the group-law sign resolution, the A2 echelon,
the final `[d]G = Q`.

- **D2** (primary): small base = factor-base columns `1..fs`,
  `fs = min(F − 1, ⌈θ·F^β⌉)`, β = 2/3, θ = 4. Derivation: a target yields a
  full-base decomposition with probability ≈ 1/3! (#5261); one with ≤ 2
  large primes with probability ≈ (1/6)(1 − (1 − α)³) ≈ α/2 (α = fs/F); the
  graph needs ≈ c·F edges before cycles appear, so the relation phase costs
  ∝ F/α = F²/fs targets, and LA ∝ fs² (sparse) — balanced at fs ∝ F^{2/3},
  total ∝ F^{4/3} = q′^{4/3} against rho's q′^{3/2} (Gaudry 2009 for
  n = 3 Weil restriction; Gaudry–Thomé–Thériault–Diem 2007).
- **D2e** (secondary): the same with β = 0.528 = 2/(γ+1), γ = 2.79 the
  echelon exponent #5261 measured (balance of F²/fs against fs^γ); total
  then ∝ F^{2γ/(γ+1)} = F^{1.47}. θ = 16.
- **D2all** (control): every class small; must reproduce #5261's D record.
- **References:** #5261's D records (n = 18..48); rho as in `protocol.json`
  (16 seeds per target).

**Large-prime graph** (`harness/src/graph.rs`). A relation with large
support of size 0 goes to the LA; size 1 is an edge to the special vertex
SV; size 2 an edge; size 3 is discarded. Union-find gives components; a
spanning forest (parent pointers, each holding its edge's relation) gives
paths; the component of the edge's endpoint that does not contain SV (else
the smaller one) is re-rooted. Because classes are `±P` and coefficients are
±1 (±2 for a repeated point), a cycle cancels its large primes over `Z/ℓ`
only if it is balanced; the graph is a signed graph. A component is grounded
when it holds SV or one frustrated cycle (kept as its grounding relation).
An edge yields one relation over the small base when it closes a cycle in a
grounded component or a balanced cycle anywhere, or joins two grounded
components; so yields = edges − rank of the signed incidence matrix, and no
combination is lost. Every mod-ℓ product/addition (`z`, `za`), inversion
(`2·bits(ℓ)` z) and every union-find, parent, re-root and classification
step (`h`) is counted in phase `graph`.

## Rungs, cases and resources

- n = 18..48: #5261's 116 cases exactly (4 curves × 3 targets; 2 × 2 at
  n = 33 and 48). D2 and D2e at every rung; D2all at n ≤ 39.
- n = 51: 2 curves × 2 targets; n = 54: 2 curves × 1 target; D2, D2e, rho.
  D is not run there (its echelon on F ≈ 6.5·10⁴ columns needs ≳ 30 GB).
- n = 57, 60 are **not run**: the prediction below gives an echelon on
  fs ≈ 1.6·10⁴ / 2.6·10⁴ columns at ≈ 3·10¹¹ / 10¹² mod-ℓ units, i.e.
  several hours per solve on this host. Recorded as a resource limit, not as
  a result.
- One process per rung and arm group, `prlimit --as` 10 GiB, wall budgets in
  `protocol.json`; a killed process is a resource-limit record.

## Correctness checks (frozen)

- every log: `[d]G = Q` in the counted solve and `d = d_true`;
  D2's and D2e's d equal D's on every shared case;
- every relation fed to the LA — full or cycle-combined — is checked by the
  group law `Σ e_c·[h]P_c = [a]([h]G) + [b]([h]Q)` exactly in E (uncounted);
- D2all ≡ D: `(d, targets, relations, columns)` equal on every case n ≤ 39;
- graph self-test on planted logarithms (`selftest`): every yield satisfies
  the planted logs, and `1LP + 2LP = yields_cycle + yields_join + groundings
  + tree_edges` (the rank identity);
- replay: n = 30, ci = 0, ti = 0, arm D2, rerun from the frozen manifest and
  compared bitwise (all fields but wall times).

## Cost unit and band

#5253's Meq unchanged (field m, s; inversion at its census; 1 per table map,
hash/graph step, mod-ℓ product or addition). Band per §5a: non-field units at
0, 0.1, 1; squarings at 1 and 0.25. An order between arms is stated only
where it holds across the band. Wall time is indicative only.

## Analysis (frozen; `b2lp_harness analyze`)

- per case: D2/rho, D2e/rho, D/rho (rho = mean of 16 runs on the same Q),
  D2/D, D2e/D, D2/D2e; per rung the geometric mean with a 95% hierarchical
  bootstrap (curves, targets, rho seeds), B = 10000 primary, 2000 band;
  stream `boot|b2lp|main`;
- rho A/A (seeds 0–7 vs 8–15);
- phase split: relation phase (fb, setup, targets, descent, rootfind, lift,
  resolve), graph, LA; relation-phase/rho;
- census: fs, F, targets/q′, yields, full/1LP/2LP/discarded counts, yield
  weight, groundings, components;
- exponents: OLS slope of log₂ cost on log₂ ℓ, bootstrap within rung, for
  rho, D (n ≤ 48), D2 and D2e (all rungs, n ≥ 36, n ≥ 48) and their relation
  phases;
- crossover projection (a model, labelled so): targets/F fitted as
  `exp(a₀)·(fs/F)^{p_α}·F^{p_F}` on all unsaturated D2/D2e cases, relation
  plus graph cost per target `α + βn′ + g`, LA either the fitted echelon
  `2^{l₀}·fs^γ` or a sparse-LA model `3·w̄·fs²` (w̄ = 12, not measured), rho
  `S_top·√(2^{3n′}/h̄)` with S_top the mean of the top two rho rungs; first
  n′ ≤ 64 (step 1/4) where each ratio ≤ 1, for the frozen rules and for a
  base re-balanced at every n′ under the sparse model; and the straight-line
  extrapolation of the measured slopes.
- gap reported as the remaining distance (ratio and CI), never pass/fail.

## Predictions (written before evaluation data)

From the abstract-graph simulation (`b2lp_harness predict`: F classes,
Poisson(1/6) decompositions per target with uniform classes and signs, the
same `LpGraph` and `Echelon` code; 3 replicates per cell, 1 at n = 51) priced
with #5261's measured relation Meq per target and rho S. Full table:
[`data/predict_summary.md`](data/predict_summary.md); raw:
`data/predict.jsonl`.

| n | D2 fs/F | targets/q′ | yields | LA Meq | D2 total | pred D2/rho | pred D2/D |
|---|---|---|---|---|---|---|---|
| 18 | 32/33 | 2.56 | 32 | 5.2e3 | 3.2e5 | 34 | 0.94 |
| 30 | 257/513 | 3.05 | 242 | 2.0e5 | 9.3e6 | 49 | 1.07 |
| 36 | 646/2049 | 3.96 | 615 | 6.4e6 | 6.2e7 | 58 | 1.24 |
| 42 | 1626/8193 | 5.21 | 1576 | 1.8e8 | 5.2e8 | 59 | 0.64 |
| 45 | 2581/16385 | 6.17 | 2523 | 1.0e9 | 1.8e9 | 64 | 0.33 |
| 48 | 4097/32769 | 7.33 | 4034 | 5.6e9 | 7.7e9 | 97 | 0.18 |
| 51 | 6503/65537 | 8.74 | 6428 | 2.7e10 | 3.2e10 | 145 | — |

D2e (θ = 16, β = 0.528) is predicted within ±20% of D2 at every rung
(n = 48: 6.8·10⁹, D2e/D 0.16).

Hypotheses:

- **H1 (exactness).** Every D2, D2e and D2all log is correct; every yield
  passes the group-law check; D2all ≡ D on every case n ≤ 39; the graph
  self-test passes.
- **H2 (targets).** D2's targets/q′ is within ±25% of the predicted value at
  every rung (2.6 at n = 18 rising to 7.3 at n = 48, 8.7 at n = 51), i.e. it
  grows roughly like F^{1/3} once fs < F.
- **H3 (cycle yield).** Yields ≈ 0.94–0.99·fs; the share of 2LP rows among
  used partial relations ≥ 0.8 at n ≥ 45; groundings ≤ 1% of yields; mean
  yield weight rising from 3 to 12–15 at n = 48..51.
- **H4 (LA size and split).** LA on ≈ fs columns; LA share of D2's cost below
  10% at n ≤ 33, above 50% at n ≥ 48 (predicted 72% at n = 48, 83% at
  n = 51); graph share below 2% everywhere.
- **H5 (gap).** No parity: D2/rho between 25 and 200 at every rung. D2/D
  within 0.8–1.4 at n ≤ 39, ≤ 0.5 at n = 45 and ≤ 0.3 at n = 48.
- **H6 (exponent).** D2's whole-solve slope on n ≥ 36 is above rho's
  asymptotic 0.5 (predicted 0.59), because the echelon's fill-in on weight-12
  yields grows like fs^{≈2.8}, not fs². The relation phase alone grows with
  slope ≈ 4/9·(1 + small n′ drift), below rho's.
- **H7 (crossover, model).** With the echelon, neither frozen rule crosses rho
  for n′ ≤ 64; only the sparse-LA model with a re-balanced base can, and
  #5261's "n ≳ 130" is the order of magnitude expected there.

**Falsifier.** Parity at a rung if the upper 95% bound of D2/rho or D2e/rho
is ≤ 1 there (then the rho A/A floor and full verification are required
before it is stated). H2–H6 are falsified by the stated ranges.

## Deviations and what is not tested

- Any change after the freeze is an amendment frozen before its own data;
  every record is kept, failures included.
- Not tested: multi-target amortisation (excluded); sparse LA (another lane
  builds it; not depended on); single-large-prime variant; Koblitz rungs;
  GHS; isolab / L2; n ≥ 57 (resource); D at n ≥ 51 (memory).
- Prior art: research-kb MCP is not available in this session (as in
  #5261); the derivation cites Gaudry 2009 (J. Symb. Comput. 44) and
  Gaudry–Thomé–Thériault–Diem 2007 (Math. Comp. 76) from memory.
