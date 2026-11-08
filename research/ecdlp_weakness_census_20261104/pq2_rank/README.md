# pq2_rank: exact degree lower bounds for the m = 2 descended S₃ system

Target: `HEUR-GB-PQ-ASSUMPTION-2` (Petit–Quisquater Assumption 2: the degree of
regularity, i.e. the maximal degree reached by the Gröbner solver, equals the first
fall degree up to o(1); Kousidis–Wiemers read it as `D_reg = m² − m + 1 + o(1)`, which
is 3 + o(1) at m = 2). At m = 2 the first fall degree of the Weil-descended third
summation polynomial system is 2 for every n (Kosters–Yeo Cor. 4.11), so the literal
reading predicts solver degree 2 and the Kousidis–Wiemers reading predicts 3 for all
large n. This census computes, exactly over F₂ and with no Gröbner basis computation,
a lower bound on both quantities for n from 8 to 56.

Command (from this directory): `cargo run --release -- 8,9,...,56 3 20 results/pq2_rank 44 16`
(the arguments are the list of n, instances per n, the largest n for one-shot degree 4,
the output prefix, the largest n for iterated degree 3, the largest n for iterated
degree 4). Rust only, no dependencies, single thread; seed `0x5eed20261104`.
Outputs: `results/pq2_rank.json`, `results/pq2_rank.csv`, `results/run.log`.

## Instance

* Field F_{2ⁿ} with an irreducible trinomial or pentanomial found by the Ben-Or test.
* Ordinary curve y² + xy = x³ + a x² + b with random a and random b ≠ 0.
* Factor-base subspace V ⊂ F_{2ⁿ}: random F₂-basis of dimension n' = ⌈n/2⌉.
* Target abscissa x_R of a random point; instances whose solution set is empty are
  skipped (then 1 lies in the ideal and no degree question arises).
* Descended system: with x₁ = Σ aᵢvᵢ, x₂ = Σ bⱼvⱼ, S₃(x₁, x₂, x_R) = (x₁x₂ + x₁x_R +
  x₂x_R)² + x₁x₂x_R + b is bilinear in the bits: n equations in N = 2n' variables, plus
  the N field equations aᵢ² + aᵢ, bⱼ² + bⱼ. The program checks bitwise, on 20 random
  assignments per instance, that the descended equations agree with the direct S₃
  evaluation, and that S₃ vanishes on the abscissae of a true sum of points.

## Quantities, all exact

* `Z`: every solution (a, b) ∈ F₂^N, enumerated through x₁ ∈ V (2^{n'} values) by solving
  the quadratic A X² + B X + C = 0 for x₂ and testing membership in V.
* `k = N − affdim(Z)`: the number of linear polynomials in the ideal with distinct
  leading variables (Boolean Nullstellensatz: the ideal with field equations is the
  vanishing ideal of Z; the |Z| = 1 case is kernel-checked in
  `BooleanNullstellensatz.linear_mem_of_unique_zero`).
* `rho`: rank of the bilinear parts of the n descended equations.
* `low_d` (one-shot): dim(rowspace(M_{≤d}) ∩ P_{≤1}) for the Macaulay matrix of the
  original generators at degree d = 2, 3 (and 4 for n ≤ 20).
* `iter_d` (iterated): the same dimension for the degree-d closure: starting from the
  generators, every multiple of the newly found polynomials that stays within degree d
  is added and the matrix re-reduced, until the rank stops growing. This is what a
  Gröbner run whose steps never exceed degree d can reach (F4 multiplies the new
  polynomials of each step again, so its output at max step degree D lies in the
  degree-D closure). Computed for d = 3 (n ≤ 44) and d = 4 (n ≤ 16).
* `d_lin`: least d with `low_d ≥ k`; `d_iter`: least d with `iter_d ≥ k`.

By `SolvingDegreeBound.no_gb_in_rows_of_low_lt` (Lean, kernel-checked), any Gröbner basis
for a degree-compatible order that lies in a row span whose degree-≤1 part has
dimension < k is impossible. Hence `d_lin ≤ sd` (Caminata–Gorla one-shot solving degree)
and `d_iter ≤ D_F4` (F4's maximal step degree), the latter under the containment
property of F4 stated above, which is not formalised.

## Results

All counts are unverified assertions: no signed validator report exists yet [pcc:unverified].

**Main sweep.** It ran n = 8 to 47, three instances per n, 119 instances in all. It
was stopped during n = 47, where enumerating Z takes about six minutes per instance,
before the binary wrote its own JSON. `log_to_json.py` copies every instance line of
`results/run.log` into `results/pq2_rank.json` and `.csv` and recomputes nothing.
n = 48 to 56 were not reached.

| n | instances | iterated degree 3 reaches k | d_iter | d_lin |
|---|---|---|---|---|
| 8–14 | 21 | 21 | 3 | 4 (20), >4 (1) |
| 15 | 3 | 0 (iter3 = 1) | 4 (iter4 = k) | >4 |
| 16 | 3 | 3 | 3 | >4 |
| 17–20 | 12 | 0 (iter3 = 1) | >3 | >4 |
| 21–44 | 72 | 0 (iter3 = 1) | >3 | >3 |
| 45–47 | 8 | not run | – | >3 |

At every n, low2 = 1, and low3 = 1 for every n ≥ 9. The value of k lies between N − 7 and N.

**Degree 4 beyond n = 16.** A separate sweep at n = 17 to 20 (`results/iter4_n17_20/`)
has fresh instances, because the random stream starts at n = 17. Iterated degree 3
gives 1 and iterated degree 4 gives exactly k on all 12 instances, so d_iter = 4 there.
`results/iter4_n21_24/` does the same at n = 21 to 24 with the same outcome on all 12
instances. Across both sweeps, d_iter = 4 on 24 of 24 instances from n = 17 to 24.
`results/iter4_n25/`, `iter4_n26/` and `iter4_n27/` extend this to n = 27 with the same
outcome on all 9 instances. In total d_iter = 4 on 33 of 33 fresh instances from n = 17 to 27.

**A real F4 run (`f4/`).** msolve 0.6.5 (Berthomieu, Eder and Safey El Din; F4 with
grevlex) ran on the main sweep's own instances for n = 8 to 34, 81 in all. The binary
dumped them with the same seed and arguments, and the dump log equals `results/run.log` on
all 81 lines. `f4/run_msolve.py` records msolve's maximal step degree, and
`f4/summarize.py` merges the runs into `f4/msolve_all.json`.

| check | result |
|---|---|
| linear polynomials in msolve's reduced basis equal k | 81 of 81 |
| msolve's degree equals d_iter where the census value is exact (n ≤ 16) | 27 of 27 |
| census d_iter > 3 and msolve below 4 | 0 |
| msolve maximal step degree, n = 17 to 34 | 4 on all 54 |

One row first recorded 0 linear polynomials because its output was truncated while the
disk was full. The rerun gives k, and the part file `f4/msolve_n26_i0.json` takes
precedence. Three instances were rerun after a machine restart.

**Testing the published step to 5 (`f4/step_n36_37/`).** Kousidis and Wiemers report Magma
degree 4 for n ≤ 36 and 5 from n = 37. To test that with a second implementation, msolve
ran on fresh instances at n = 36 and 37.

| instance | outcome |
|---|---|
| n = 36, instance 0 (control) | maximal step degree 4, linear polynomials = k = 35, 47 minutes |
| n = 37, instance 0 (test) | locally killed twice for memory; on isolab-eks (128 GB node, 4 threads) msolve ran 2, 2, 4 steps at degrees 2, 3, 4 and then started a degree-5 step (1,016,044 × 610,665 matrix) about 60 minutes in, at 53 GB; run still in progress (`f4/eks_n37/interim-0742/`) |
| n = 36 and 37, instance 1 | stopped early to save memory, no result |

On the single-thread retry at n = 37, memory grew about 1.2 GiB per minute over the last
five minutes (0.22 GiB per minute averaged over the hour) and passed 13 GiB on a 15 GiB machine (`msolve_n37_i0.mem.log`). The earlier attempts are kept as
`*.timeout_3600s.log` and `*.killed_t3.*`. On the isolab-eks rerun msolve does reach a degree-5 step at n = 37, so its maximal step
degree there is at least 5, matching the Magma value Kousidis–Wiemers report [pcc:unverified].
The run had not finished at this revision; the final basis check (linear = k = 33) is pending.

**Independent cross-check.** `crosscheck/` recomputes the closure by naive full
saturation in pure Python, sharing no code with the Rust closure. It agrees with Rust
on five dumped instances at n = 14 to 18 at degree 3. At degree 4 the Python closure reaches k on
the three instances it was run on; the Rust run on those dumps had degree 4 disabled. The
independent validator later ran both degrees on all five and found agreement (`validation/`).

**Reproducibility.** A rerun with the same seed and arguments for n = 8 to 40
(`results/rerun_n8_40/`) reproduces the main log line for line, timings aside. Its
JSON also carries |F'| for `HEUR-BK-GG-FACTOR-BASE-SIZE`. Over its 99 instances |F'|/|V| ranges from
0.84 to 1.31 with mean 1.021, and from 0.997 to 1.004 for n ≥ 30, so `|F| ≈ |V|`
holds on this range [pcc:unverified].

**Reading.**

* Literal Assumption 2 says the solver degree equals the first fall degree, which is 2.
  It fails on the 111 instances with n ≤ 44: there d_iter ≥ 3. On 87 of them iter3 < k already
  forces this; on the other 24 the independent validator computed the degree-2 closure (iter2 = 1).
  At n = 45 to 47 no closure was computed, so those 8 instances are open.
* The Kousidis–Wiemers reading predicts 3 + o(1). Every instance with 17 ≤ n ≤ 44 needs
  an iterated degree above 3. The 33 fresh instances at n = 17 to 27 need exactly 4, and
  msolve's F4 reaches exactly 4 on all 54 main-sweep instances at n = 17 to 34.
  A finite range cannot refute an o(1) statement. It does show that the value 3 is not
  reached anywhere from n = 17 to 44.
* Containment. `F4Containment.lean` proves (unsigned pcc claims, [pcc:unverified]) that an abstract F4 run with maximal step degree
  D stays inside every D-closed subspace containing the generators (`run_subset`). So the
  census values are lower bounds on F4's maximal step degree (`no_gb_of_run_of_low_lt`).
  That msolve fits the abstract model is a reading of its algorithm, not a theorem. For the
  one-shot Caminata–Gorla solving degree, d_lin ≤ sd is kernel-checked in `SubspaceCriterion`.
* Direction. msolve's degree is 4 on every instance from n = 17 to 34, one above the Kousidis–Wiemers value 3, and matches their Magma value 4 for n ≤ 36. They report 5 from n = 37 to 48 (`MEAS-GB-KW-TABLE1`), so a window ending at n = 34 does not decide whether the degree stays bounded, and the published data already show it rising. The step from 3 to 4 inside this window, at n = 15 and n = 17, is a second rise. These measurements do not support a subexponential algorithm; they cover m = 2 only, where the heuristics need m to grow with n.
