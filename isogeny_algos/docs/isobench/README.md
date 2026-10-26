# isobench: benchmarks and a frontier for isogeny problems

`isobench` is to isogeny algorithms what ecbench is to ECDLP methods. Each problem is a
precisely stated computation with:
- deterministic instances;
- competing methods, each charged in one counted unit;
- an independent checker.

Measurements come from frozen specifications and are sealed, replayable records. A **bound**
records what a method costs as cost = C · size^α (or C · 2^(α · size) for the exponential
problems), with bootstrap intervals. The **frontier**
lists, per problem, the bounds no other bound dominates. A **challenge** is a frozen paired spec;
its **verdict** says whether a candidate advances on the incumbent, trades, matches or
regresses, and at which level (exponent or constant).

The current frontier is [`FRONTIER.md`](FRONTIER.md).

The harness lives in the `isogeny_algos` crate:
- `src/isobench/`: `counted`, `problems`, `harness` and `bounds`;
- the `isobench` binary.

It shares ecbench's conventions but not its code. ecbench is typed throughout for discrete-log
instances (a target point, success meaning [k]G = Q, cost in group additions per √r), which do
not describe isogeny problems.

## Problems and units

| problem | computes | size | unit | methods | checked by |
|:--|:--|:--|:--|:--|:--|
| `kernel.codomain` | the codomain of the ℓ-isogeny with kernel ⟨P⟩, P a rational point of order ℓ on y² = x³ + x over p = 4ℓk − 1 | ℓ | fme | `kohel`, `velu`, `xonly`, `sqrtvelu` | Kohel's codomain from the kernel polynomial, built separately from the x-multiples of P (x-only Vélu for the Kohel arm) |
| `kernel.chain` | the ℓⁿ-isogeny with kernel ⟨P⟩ as n ℓ-isogenies, p = 4ℓⁿk − 1 | n | fme | `naive`, `balanced`, `optimal` | the naive strategy's codomain (balanced for the naive arm) |
| `find.kernel` | the kernel polynomial of the normalised ℓ-isogeny E → Ẽ, given E and Elkies' Ẽ (the BMSS problem) | ℓ | fme | the eight BMSS variants | Kohel's codomain of the kernel equals Ẽ; monic of degree (ℓ − 1)/2 |
| `modpoly.phi` | Φ_ℓ mod p, p = 4ℓk − 1 | ℓ | fme | `hecke`, `linear_algebra` | equality with the other construction (Hecke above ℓ = 41), and Φ_ℓ(j(E), j(E′)) = 0 for a Vélu pair E → E′ |
| `count.sea` | #E(F_p) for a random curve over a random prime | log₂ p | fme | `sea`, `sea_no_cycles` | the order certified by point orders (`api::check_order`) |
| `path.ordinary` | a path of ℓ-isogenies, ℓ ∈ {2, 3, 5, 7}, between two ordinary j-invariants of one class | log₂ p | oracle calls | `galbraith` (BFS), `ghs`, `galbraith_stolbunov` | every edge a root of Φ_ℓ; the endpoints |
| `path.supersingular` | a path between two random supersingular j over F_p² | log₂ p | oracle calls | `delfs_galbraith`, `bfs`, `ghs` | every edge a root of Φ_ℓ; the endpoints |
| `csidh.action` | [∏ ℓᵢ^eᵢ] E₀, eᵢ uniform in [−B, B] | B | fme | `tree`, `batched`, `one_at_a_time` | the one-at-a-time action's result (the tree action's for that arm) |
| `klpt` | an equivalent left O₀-ideal of 2-power norm for a random ideal of prime norm | log₂ p | norm bits | `klpt` | N(Iξ) = 2ᵉ recomputed, integral, left order O₀, primitive |

`isobench problems` prints this registry, including the parameters and their defaults.

**`isobench.fme`, field-multiplication equivalents.** The algebraic problems are counted
through `Counted<F>`, a wrapper around the crate's `Field` trait that every algorithm is
generic over:
- **Element operations** are counted exactly: one per multiplication or squaring, and an
  inversion at the calibrated ratio of that field. `pow`, `div` and `batch_inv` run the trait's
  default bodies, so their multiplications are counted one by one. Additions are free.
- **Bulk kernels** (products, squares, truncated, cyclic and prepared NTT products, dot
  products) keep their fast paths and are priced from the pinned calibration
  [`calibration.json`](calibration.json): the measured time of that kernel at that length, in
  multiplications of the same field, interpolated in log–log between powers of two. An
  unbalanced n × m product is charged as ⌈max/min⌉ balanced products. A square root is charged
  1.5 log₂ q multiplications.
- **Determinism.** Counted runs switch the AVX-512 IFMA path off, so thresholds, call patterns
  and counts are identical on every host. The raw tally (kind, length, calls) is kept in every
  record, so a record can be re-priced under another calibration.
- **Recalibration.** `isobench calibrate` writes a new calibration with a new id, and every
  session records the id it was priced under. A recalibration is a new unit: sessions under
  different calibrations are not compared.

**`isobench.oracle_calls`.** Path finding is charged in neighbour-oracle root-findings, one per
degree asked: a call to the Φ_ℓ(j, Y) oracle with |ells| degrees costs |ells|. This is the
isogeny-graph analogue of ecbench's group operations. Delfs–Galbraith is charged its F_p²
walk steps plus two root-findings per F_p BFS node (degrees 2 and 3). Galbraith–Stolbunov is
charged one per walk step, since it solves one Φ_ℓ per step.

**`isobench.norm_bits`.** KLPT is judged by what it outputs: log₂ of the output norm (e for
norm 2ᵉ). Smaller is better, and e ≈ 3.5 log₂ p is the classical figure.

**Wall time.** Every record also has `wall_ms`: the method call alone, on the plain field with
IFMA on, in the same child process, excluding instance generation. Its answer must equal the
counted run's. Wall times are evidence only from isolated runs (`--isolate`, below).

## Running

```sh
cd isogeny_algos
cargo build --release --bin isobench
B=target/release/isobench
$B problems
$B run docs/isobench/specs/codomain-127.json --out /tmp/s --cal docs/isobench/calibration.json \
   --isolate 3 --bench-tool ../tools/isolated_bench.py
$B verify /tmp/s --cal docs/isobench/calibration.json --replay-all
$B bound fit --arm sqrtvelu --out docs/isobench/bounds/records/codomain-127-sqrtvelu.json /tmp/s
$B frontier build --bounds docs/isobench/bounds/records --out docs/isobench/FRONTIER.md --json docs/isobench/frontier.json
```

**Spec** (`isobench.spec/v1`, see `specs/`). It contains:
- `problem`, `params`, `sizes` and `instances_per_size`;
- `arms`, each with a `name`, a `role` (reference, baseline, candidate or control) and a
  `method`;
- `measurement`, with `rounds` and `seed`.

The spec id `ISS1h…` is the hash of the spec. `plan` expands it deterministically:
- every (size, instance, round) gets one algorithm seed shared by all arms, so the arms are
  paired;
- the arm order is reversed on odd rounds.

**Session** (`run --out DIR`). The directory holds `spec.json`, `plan.json`, `host.json`,
`records.jsonl` and `session.json`, plus `isolation.jsonl` when the session was isolated.
- Every execution is a fresh `isobench exec` child process.
- With `--isolate CPU` the child is pinned through `tools/isolated_bench.py`, and the record
  carries whether the run was contended.
- The parent checks each answer with the problem's verifier and writes a record sealed by the
  hash of its own content (`ISR1h…`). The record's status is `verified`, `wrong_answer`,
  `exhausted` (budget spent) or `error`.

**Audit** (`verify DIR --replay N | --replay-all`):
- re-hashes every file and re-checks every seal;
- re-plans the spec and checks it against `plan.json`;
- re-verifies every answer;
- re-executes N records (or all of them). Each replay must reproduce the answer, the cost, the
  raw tally and the work counters exactly.

## Bounds, frontier, challenges

- **Bound** (`bound fit`, `isobench.bound/v1`, `ISBND1h…`).
  - **Points:** one arm's verified records. Each instance is averaged over its rounds.
  - **Per size:** the mean, with a bootstrap interval in which instances are resampled within
    each size.
  - **Fit:** by least squares in log space, with bootstrap intervals for α and C (2000
    resamples, fixed seed), under the problem's model. The same fit is made for wall time.
    - `power`: cost = C · size^α (log–log). The algebraic problems use it.
    - `exp2`: cost = C · 2^(α · size). With size = log₂ p this is C · p^α. The path problems
      use it; their costs grow exponentially in log₂ p, and a power law in log₂ p would give an
      exponent with no meaning. The declared α is 1/4 for both: Galbraith and GHS on ordinary
      classes (√(class number)), and Delfs–Galbraith on supersingular graphs.
  - **Scaling:** the exponent is a scaling claim only with at least four sizes.
  - **Re-checking:** `bound check` re-derives a committed bound from its sessions, which must
    give the identical document.
  - **Exact counts:** counted costs that depend only on the size (Vélu at a given ℓ, a chain of
    a given n) have zero-width intervals.
- **Frontier** (`frontier build`). Bounds are grouped by problem and parameters. A bound A
  dominates B when A's mean-cost interval at the largest size both measured lies entirely below
  B's, unless A's α interval lies entirely above B's. A cheaper method that scales worse is a
  crossover: dominance would be an extrapolation past the measured sizes, so both stay on the
  frontier. A method name that appears in two sessions of one group is labelled with its
  session.

  Ties stand. `--check` fails when `FRONTIER.md` or `frontier.json` is stale.
- **Challenge** (`challenge seal`, `ISCH1h…`). A frozen paired spec with three arms:
  - `incumbent`;
  - `candidate`;
  - `aa`, the incumbent run again as an A/A control.

  Run the spec (`challenge spec FILE > spec.json`), audit the session with `--replay-all`, then
  `challenge verdict`.
- **Verdict** (`ISVD1h…`). The ratio Σ candidate / Σ incumbent over paired executions, with a
  bootstrap interval, decides the outcome for the counted unit:
  - `advances` when the interval lies below 1;
  - `regresses` when it lies above 1;
  - `matches` otherwise.

  Wall time is judged the same way, and a cost/wall disagreement is a `trade`. Wall time counts
  only if its own A/A interval includes 1.

  The verdict is `inadmissible` in any of these cases:
  - the cost A/A interval excludes 1;
  - an execution is not verified;
  - there are fewer than four paired runs per size.

  The level is `exponent` only when the two arms' α intervals are disjoint over at least four
  sizes. Otherwise it is `constant`.

## What a result here is not

- **fme is a model.** It counts what the wrapper sees and prices bulk kernels by measured time.
  A method that does its work outside the `Field` interface (in integers, or in a hand-rolled
  kernel) would be under-charged. Every method registered here does its field arithmetic
  through the wrapper; a new method has to as well.
- **Wall time and fme can disagree.** For example, sqrt-Vélu's product trees cost more per fme
  than Vélu's straight-line loops at small ℓ. The verdict reports both.
- **Path instances are heuristic.**
  - *Ordinary pairs* end a random walk of 4 log₂ p + 8 · 2^(log₂ p / 4) steps over
    ℓ ∈ {2, 3, 5, 7}. This is long enough to leave the start's neighbourhood, but it is not
    proven uniform in the class.
  - *Supersingular pairs* end 2-isogeny walks of 3 log₂ p steps from j = 1728, which mix in the
    Ramanujan 2-graph.

  The fitted α describes these instance distributions.
- **SEA sizes stop at 160 bits.** A `count.sea` answer is accepted only with a point-order
  certificate for #E or its twist, which needs the order factored by trial division and Pollard
  rho. At 192 bits two of three random instances leave cofactors (72 × 93 and 46 × 135 bits)
  beyond that effort for both orders. An uncertified count is not accepted as verified.
- **Not compared here:** external tools such as PARI/GP cannot be operation-counted. The
  wall-time comparisons against them stay in `results/p16-external/`, `results/p17-sea/` and
  `results/p18-families/`.
