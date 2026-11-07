# p2coset: instrument for IDEA-20261107-TWIST-PURE-COSET (P2)

Status: instrument only, validated at p < 2^16 (development smoke tests). No claim cell has been run.
Labels: `[pcc:unverified]` for every number below (development logs, not pcc-ledgered claims).

Source of the proposal: `research/split_prime_ecdlp_20261107/ideas/idea-generator.md`, P2 (lines 136-227).
Tower bases: `research/pkm_dickson_torus_20261015/PROTOCOL.md` (section 2, torus case: `V = {w + w^-1}`,
`w in u*mu_{2^t}`, `u` in the norm-1 torus of F_{p^2}, `c = u^{2^t} + u^{-2^t}`, steps `D_2`, `2^t | p+1`).

## Build and run

```
CARGO_TARGET_DIR=/tmp/claude-0/-home-user/0af71fd6-cb66-5cbf-9292-d617eb53cca4/scratchpad/targets/p2 cargo build --release
B=$CARGO_TARGET_DIR/release/p2coset      # run the binary directly
$B selftest
```
Standalone crate (own empty `[workspace]`), deps: `serde_json` only; field arithmetic is u128-backed, `p < 2^32`.
Every subcommand takes `--p P` and `--cseed S` (default 1: deterministic curve search; `a, b != 0` so `j` is not 0 or 1728).

## Systems

Let `V_c` be the coset, `x_i in V_c` the m unknowns. Kummer kind: `V_c = {x : x^(2^t) = c}` (needs `2^t | p-1`),
`V_c = x0 * mu_{2^t}`, `c = x0^(2^t)`. Dickson-torus kind: `V_c = {x : D_{2^t}(x) = c}`, `D_2(y) = y^2 - 2`
(needs `t >= 2` and `2^t | p+1`; `c = 2 re(u^{2^t})`; candidates need `u^{2^{t+1}} != 1` so that `u mu` and `u^-1 mu` are
disjoint, and `c != +-2`). `V_c` is the full root set of the tower polynomial in F_p (selftest checks `#{x : tower(x) = c} = 2^t`).
A point is an E-abscissa iff `f(x) = x^3 + ax + b` is a nonzero square (n is an odd prime, so f has no F_p root).

`emit` writes an msolve 0.6.5 file with `m*t` unknowns `x1..xm, y{i}_{j}` (j = 1..t-1) and `m*t + 1` equations:
- `S_{m+1}(x1..xm, xR) = 0`, xR a constant. m = 2: Semaev `S_3 = (u-v)^2 z^2 - 2((u+v)(uv+a)+2b) z + (uv-a)^2 - 4b(u+v)`.
  m = 3: `S_4 = Res_z(S_3(x1,x2,z), S_3(x3,xR,z))` (native, `(a2 b0 - a0 b2)^2 - (a2 b1 - a1 b2)(a1 b0 - a0 b1)`).
- per i: `y_j - T(y_{j-1}) = 0` (j = 1..t-1, `y_0 = x_i`) and `T(y_{t-1}) - c = 0`; `T(y) = y^2` (Kummer) or `y^2 - 2` (Dickson).
The planted target is `R = sum of m distinct base E-points with random signs` (R != O). Before writing, the program checks that the
planted assignment zeroes every equation (the `.emit.json` records `planted_verified`). The system is overdetermined by one
equation (as in PKM towers); the zero-dimensional quotient has the planted solutions.

## CLI

```
curve    --p P [--cseed S]
cosets   --kind kummer|dickson --p P --t T --C C --seed S [--cseed S] [--out cosets.json]
coverage --kind K --p P --t T --C C --seed S --select max|med|min|random|idx:K     (m=2 always; m=3 when 2^t <= 64)
emit     --kind K --p P --t T --m 2|3 --C C --seed S --select ... --tseed N --out file.ms   (prints meta JSON)
parse    --log msolve.log
purity   --p P --m 2|3 [--cseed S]      (m=2: p <= 10^4; m=3: p <= 200)
selftest
```
`cosets` reports c_mean, c_max, argmax, the histogram of c_E, and the exact `E[max of C iid Binomial(2^t, q)]` (computed from
the pmf, `E[max] = sum_k 1 - F(k)^C` with `ln_1p`/`exp_m1`) for both `q_emp = c_mean / 2^t` and `q_curve = (n-1)/(2p)`,
plus the Gaussian-law ratio of the proposal for comparison. `coverage` enumerates `{+-P_i +- P_j}` (A+A with A = all 2 c_E
signed base points, i = j included) and, for 2^t <= 64, `A+A+A`; it counts distinct points R != O and divides by n.
`select` = coset used: `max`/`min`/`med` by c_E (ties to lowest index), `idx:K` the K-th candidate (`random` = idx 0).
`emit` needs `c_E >= m` of the selected coset.

## Metrics (`parse` of `msolve -g 2 -t 1 -v 2`)

`D_max_deg` (maximum `deg` column over F4 rounds), `widest_cols`, `max_rows_x_cols` (largest matrix), `sum_rows_x_cols`
(work proxy: sum over F4 rounds of rows x cols), `sum_rows`, `quotient_dim` ("Dimension of quotient"), `deg_sequence`,
`wall_s_descriptive`, `cpu_s_descriptive` (descriptive only; 0.00 at these sizes).
msolve invocation: `ulimit -v 1572864; nice -n 10 msolve -g 2 -t 1 -v 2 -f in.ms -o out`. msolve supports characteristic < 2^31 only
(documented limit, not tested here), so cells at p near 2^32 cannot use msolve.

## Smoke tests (p < 2^16, instrument validation only)

`smoke/run_smoke.sh` (needs `BIN=...`); outputs in `smoke/`: `cosets_coverage.jsonl`, `f4_smoke.jsonl`, per-cell `*.emit.json` and `*.log`.
Kummer p = 12289 (`3*2^12+1`, n = 12329 prime, curve cseed 1); Dickson p = 65519 (`p+1 = 2^4 * 4095`), C = 300, seed 1.

- `selftest`: 14/14 PASS (BSGS order = Legendre count at p = 1009, 10007, 65521; S_3 and S_4 vanish on 200 random genuine sums;
  S_3 random-zero rate 0/2000; both kinds: 2^t distinct elements, tower ends at c, V_c is the full F_p root set; binomial order statistic checks).
- cosets, t = 3: Kummer c_mean 4.15, c_max 8, exact E[max] 7.79 (ratio 1.88); Dickson c_mean 3.95, c_max 8, exact E[max] 7.65.
  t = 4: Kummer c_mean 7.95, c_max 14, exact E[max] 13.46; Dickson c_mean 7.96, c_max 13, exact E[max] 13.48.
  The Gaussian law of the proposal gives 1.84 at t = 4, C = 300 versus exact 1.69 (it overshoots).
- coverage m = 2 (Kummer t = 4): c_E = 14 gives 392 points (0.03179 of n), c_E = 8 gives 122, c_E = 3 gives 18. Values equal `2 c_E^2` up
  to rare collisions (14: 392 = 2*196; Dickson t = 4, c_E = 13: 338 = 2*169), so coverage scales as c^2 as claimed. m = 3 (2^t <= 64 so computed): 3392 points at c_E = 14.
- F4 (msolve, m = 2, three target seeds each, sum rows x cols): Kummer t = 3: c_E = 8: 34653/35381/34653 versus c_E = 4: 35381/34653/34653.
  t = 4: c_E = 14: 264413 x3 versus c_E = 8: 264413/265644/264413. Dickson t = 3: c_E = 8: 35381/34653/34653 versus c_E = 4: 34653 x3.
  D = 4 and widest_cols identical in every cell (126 at t = 3, 276 at t = 4). Differences between cosets are within the target-to-target spread (<= 2.1%).
  m = 3: t = 2 (c_E = 4 both): D = 12, widest 369, sum 330920; t = 3: Kummer c_E 8 versus 4: sum 19207127 versus 19350524, D = 12; Dickson 19544373 versus 19213760.
  Toy sizes only: with p ~ 10^4 and 2^t <= 16 the quotient dimension is 2 (m = 2) or 6 (m = 3), so this says the instrument works, not that the claim holds.
- purity (development check; `smoke/purity.jsonl`), 9 curves over 3 primes: see below.

### Purity check (development result)

m = 2 (S_3), all (x1, x2) in F_p^2, all roots xR that are E-abscissae; p in {7919, 8191, 9973}, cseed 1, 2, 3:
31.6M, 31.0M, 30.9M, 34.2M, 33.9M, 33.3M, 49.7M, 50.8M, 51.0M solutions, **all class EE, zero with a twist summand**.
m = 3 (S_4), all (x1, x2, x3) in F_p^3, xR over E-abscissae; p in {181, 193, 199}, cseed 1-3 (181 yields only two distinct n):
EEE 2.5M-5.7M per curve; ETT/TET/TTE patterns 8160-9870 each, all of them **trivial cancelling pairs**
(`x_a = x_b` twist with `x_c = xR`, i.e. `P_a = -P_b`, `P_c = R`); **non-trivial solutions with a twist summand: 0** in all 9 curves.
So the lemma holds for m = 2 as stated, and for m = 3 only after excluding trivial cancelling pairs.

## Frozen-protocol commands (suggested)

```
$B selftest
$B curve --p P --cseed 1
$B cosets   --kind K --p P --t T --C C --seed S --out cosets_PTC.json
$B coverage --kind K --p P --t T --C C --seed S --select max      # and --select idx:0 (seed-random) and --select med
$B emit     --kind K --p P --t T --m M --C C --seed S --select SEL --tseed N --out cell.ms
( ulimit -v 1572864; nice -n 10 msolve -g 2 -t 1 -v 2 -f cell.ms -o cell.out > cell.log 2>&1 )
$B parse --log cell.log
$B purity --p P --m 2 --cseed CS     # p <= 10^4;  --m 3 for p <= 200
```
