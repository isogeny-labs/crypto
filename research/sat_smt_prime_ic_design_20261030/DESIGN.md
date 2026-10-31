# SAT/SMT for prime-field index calculus: an algorithm design, its obstruction, and its crossover

**Date.** 2026-10-30. **Status.** Design plus a small local feasibility probe.
**No speedup, no rho comparison, no leaderboard claim.** The probe is a
single-machine dev check on a shared Mac, not an ecbench measurement; the
crossover numbers in §4 are analytic and unmeasured.

## 0. Verdict

Asked for: a *novel* SAT/SMT solver algorithm for the decomposition step of
prime-field index calculus (IC).

1. **A plain SAT/SMT encoding cannot win in `F_p`, and there is a structural
   reason, not just a measured one** (§2, the *free-quotient carry*). Binary-field
   SAT (WDSat, CryptoMiniSat-XOR) works because Weil descent hands the solver
   GF(2)-linear structure. `F_p` has none, and the modular reduction removes the
   propagation a CDCL solver would otherwise get from the low bits.
2. **The one construction that removes the obstruction** is to take the modular
   congruence *out of the SAT core* and give it to a lattice theory solver, which
   leaves a purely integer problem with small numbers (§3, **LQ-CDCL**:
   CDCL modulo a linearised-monomial lattice theory). This is novel as a solver
   architecture, but it is the Coppersmith/Jochemsz–May reach in different clothes:
   it improves the IC relation-collection exponent only by `p^{1/6}` at best (m = 2).
3. **That gain does not pay at any size we can complete** (§4): the lattice
   constant puts the crossover with the existing pair-table oracle at roughly
   `p ≈ 2^{100}`–`2^{150}`, and the whole method stays above rho by `p^{1/3}` or more.
   Recommended disposition: record as *open, not measurable below ~100 bits*,
   with the falsifier in §5. Do not spend EKS time on it.

## 1. What the repository already says (so this is not re-derived)

- Binary SAT/XOR PDP oracle: `LEAD-ICE-sat-xor-pdp-oracle` (open next step: price
  conflicts at 0 / 0.1 / 1 GAE; identity says the CNF-XOR model is a complete
  enumerator of `V^m mod S_m` with no above-leaf pruning).
- Prime-field SMT frontend: `research/prime_field_smt_20261005/RESULT.md` — all
  three frozen 20-bit cvc5 formulations (`QF_BV`, `QF_FF`, `QF_FF`+S3) hit the
  120 s limit with `unknown`; the native exact lookup finished in 146 probes.
- Off-the-shelf SMT as PDP oracle (binary): `research/pdp_smt_oracle_20261008`.
- Root finding already replaces Gröbner (`RESEARCH_SEMAEV_ROOTFIND_VS_GROEBNER.md`).
- Small-root reach: `RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` §2.2
  (`δ_max = 1/6` at m = 2 for the basic lattice; needed `1/2`) and
  `LEAD-ICP-coppersmith-smallroot-semaev`.

The generic ceiling (`S·T² ≳ n`, same note §1) applies to any table-style or
enumeration-style decomposition, SAT included.

## 2. The free-quotient carry (why bit-blasting `F_p` is the wrong shape)

Small-x factor base `F = {P : x(P) < B}`, `B = 2^k`. For `m = 2` the decomposition
is a root `(x_1, x_2) ∈ [0,B)²` of `S_3(x_1, x_2, x_R) ≡ 0 (mod p)`. Lift to the
integers: `S(x_1, x_2) = q·p` with `S` having 9 monomials and coefficients reduced
mod `p`, so `|q| ≲ 9·B^4` — i.e. `q` has about `4k` unknown bits.

A CDCL solver propagates well on arithmetic circuits when low-order or
high-order bits of the operands fix bits of the result. Here:

- **Low-bit (Hensel) propagation.** Reducing `S(x) − q·p ≡ 0` modulo `2^j` for
  `j ≤ 4k` gives one `j`-bit constraint on the low `j` bits of `x_1, x_2` **and**
  the low `j` bits of `q`. The `j` bits of `q` are free, so the constraint
  removes nothing from `x`. The range `j ≤ 4k` covers the entire width of `x`
  (`k` bits), so low-bit propagation is vacuous end to end.
- **High-bit (interval) propagation.** The coefficients are `~p`-sized, so
  `S(x)` varies over `~p·B^4` and the top bits of `x` pin `S` only to a window
  containing many multiples of `p`.
- **Consequence.** Information reaches `x` only when `q` is determined, i.e. at
  the leaves. This is the prime-field restatement of the binary lead's identity
  "complete enumerator, no above-leaf pruning", and it predicts the SAT effort
  tracks `B^m` (all x-tuples) rather than the pair table's `B^{m−1}`.

Second-order Taylor/CVP tricks do not rescue this: the quadratic terms of `S`
have `~p`-sized coefficients and are not small modulo `p` (this is exactly why
Coppersmith needs a degree-`d` shift set).

## 3. Algorithm: LQ-CDCL (CDCL modulo a linearised-monomial lattice theory)

**Idea.** Remove `q` by never bit-blasting the congruence.

1. *Linearise.* For target `R`, `S_3(x_1,x_2,x_R) = Σ c_{ij}(R)·x_1^i x_2^j`
   (`0 ≤ i,j ≤ 2`, 9 monomials, `c_{ij}` known mod `p`). Introduce integer
   variables `y_{ij}` for the monomials with exact bounds `0 ≤ y_{ij} < B^{i+j}`.
2. *Split the constraints.*
   - **Integer core (SAT/SMT):** `y_{10}=x_1`, `y_{01}=x_2`, `y_{ij}=y_{i0}·y_{0j}`-style
     consistency, bounds. All numbers are small (`≤ B^4`), so interval and bit
     propagation are sound and effective. No modulus appears here.
   - **Lattice theory `T_lat`:** the single congruence `Σ c_{ij} y_{ij} ≡ 0 (mod p)`.
     Its solutions in the box form a coset-free lattice `Λ` of dimension 9 and
     determinant `p`.
3. *Theory propagation.* On each partial assignment (fixed high bits of `x_i`
   shrink the box), `T_lat` LLL-reduces `Λ` with the box-scaled basis and runs a
   bounded Fincke–Pohst enumeration of `Λ ∩ box`:
   - empty ⇒ **conflict clause** over the assigned high bits (exact, because the
     enumeration is exhaustive within the box);
   - non-empty ⇒ candidate `y`-vectors are handed back to the integer core,
     which checks the monomial consistency exactly and either returns a model
     or learns the clause excluding that vector.
4. *Branching.* Decide the top `t` bits of `x_1,x_2` (cube-and-conquer over the
   box). Higher levels of shifts (Jochemsz–May: monomials `x_1^i x_2^j` up to
   level `ℓ`, dimension `(2ℓ+1)²`) strengthen `T_lat` at higher cost.
5. *Output.* Each satisfying model is a decomposition; it is replayed through
   the group law, as in `prime_field_smt`.

What is new relative to the repository's leads: CDCL as the *driver* of
hybrid guess-and-lattice with exact (not heuristic) theory conflicts, and the
carry argument (§2) as the reason to separate the congruence from the circuit.
What is not new: the lattice reach (it is exactly the Coppersmith numbers of
the breakthrough-program note).

## 4. Cost model and crossover (analytic, unmeasured)

Notation: `B = p^δ`, `m = 2`. The baseline is the existing pair-table / direct
subtraction oracle, `≈ p/B` operations per relation, `B` relations, so total
`≈ p` independent of `B` (linear algebra aside).

- Success probability per random target: `≈ B²/p = p^{2δ−1}`.
- LQ-CDCL with no guessing needs `δ ≤ δ_lat` (`1/18` level 1, `1/10` level 2,
  `0.119` level 3, `1/6` limit). One lattice call per trial, so cost per relation
  `≈ L·p^{1−2δ}` (`L` = cost of one theory call in lookup-equivalents), total
  `L·p^{1−δ}`. Best at `δ = δ_lat`: gain over the baseline `p^{δ_lat}/L`.
- With guessing (`δ > δ_lat`, `t` top bits per variable): per-trial cost
  `2^{2(δ−δ_lat)log p}` and the `δ`-dependence cancels, giving
  `L·p^{1−2δ_lat}` per relation and total `L·p^{1−2δ_lat+δ}`, increasing in
  `δ`; with `δ < δ_lat` the total is `L·p^{1−δ}`, decreasing in `δ`. The
  minimum, `L·p^{1−δ_lat}`, is at `δ = δ_lat`. Larger factor bases do not help.
- Break-even needs `p^{δ_lat} > L`. With `L ≈ 10^3` (dimension-9 LLL against a
  ~100 ns lookup; assumed, not measured here) and `δ_lat = 1/18`: `p ≳ 2^{180}`.
  With level 3 (`δ_lat = 0.119`, dimension 49, `L ≈ 10^5`–`10^6`, assumed):
  `p ≳ 2^{140}`–`2^{165}`. The `1/6` limit (`L ≈ 10^3` ignores that the limit needs
  unbounded dimension) would give `p ≳ 2^{60}`, which is not attainable.
- Against rho: even at the best case the total is `p^{1−δ_lat} ≥ p^{5/6}` vs
  `p^{1/2}`; the gap to rho is at least `p^{1/3}` and does not close.

These numbers inherit `δ_lat` from the cited program note and were not
re-derived here. The `L` values above were guesses; §5.2 replaces them with a
first measurement, which moves the level-1 crossover **up** (`p ≳ 2^{186}`–`2^{247}`)
and leaves level 3 at `p ≳ 2^{132}`–`2^{178}`.

## 5. Falsifiers (cheapest first)

1. **Carry law.** On the bit-blasted `S_3` instance, CDCL conflicts for the
   exhaustive (UNSAT) case should grow like `B^m`, not `B^{m−1}`, and should be
   insensitive to the structure of `p`. §6 reports a first look.
2. **Theory-call cost `L`.** Measure LLL + bounded enumeration on the 9- and
   49-dimensional `S_3` lattices (fplll / sage) in ns, and one pair-table lookup
   in ns on the same host, then recompute the break-even of §4. If `L < 10^2`
   at dimension 9 the level-1 crossover falls to `p ≈ 2^{100}`.
3. **Planted recovery.** At `B^{18} < p` (level-1 reach) the lattice should
   recover planted `(x_1, x_2)` in one call with no search; failure refutes
   the lattice construction itself.
4. **Anything touching a recorded row** goes through isolab and the validation
   gate. This design triggers none of that.

### 5.2 Falsifier 2, first measurement (2026-10-30)

Files: `lll.gp`, `lookup.c`, raw output `timing_results.txt`. Single run per
cell on a shared Mac at load average ≈ 21–23, no isolation, no spread
estimated. Treat as order of magnitude.

- **Lattice call.** PARI/GP 2.17.3 `qflll` (not fplll) on the linearised-`S₃`
  lattice: one congruence mod `p`, 9 monomials, coordinates weighted by
  `Y_max/B^{i+j}` with `B = p^{1/18}`. CPU time per call: 0.156 ms (60-bit `p`),
  0.244 ms (100), 0.385 ms (160), 0.661 ms (256). LLL only; no enumeration
  or resultant step.
- **Level-3 proxy.** The real Jochemsz–May shift lattice was **not** built. A
  49-dimensional lattice (monomials `x_1^i x_2^j`, `0 ≤ i,j ≤ 6`, same
  weighting) was timed with modulus `p` (6.8–16.4 ms) and `p³` (51–117 ms) to
  bracket the larger entries of a real shift lattice.
- **Baseline unit.** A direct-subtraction candidate (`x(R−F_i)` and `x(R+F_i)`
  with one batched inversion per 1024 candidates, 2 hash lookups), 62-bit
  modulus, synthetic keys: 49 ns (table 1 MB), 75 ns (17 MB), 123 ns (268 MB).
  A bare hash lookup is 14–43 ns.
- **`L` (lattice call ÷ candidate).** Dim 9: `1.3·10³`–`1.3·10^4`
  (my guess of `10³` was at the low end). Dim 49 proxy: `5.5·10^4`–`3.3·10^5`
  (mod `p`), `4.2·10^5`–`2.4·10^6` (mod `p³`).
- **Break-even `p*` from `p^{δ_lat} = L`:**

| construction | `δ_lat` | `L` | `log₂ p*` | if `L` were 10× lower |
|---|---:|---|---:|---:|
| level 1, dim 9 | 1/18 | `1.3·10³`–`1.3·10^4` | 186–247 | 126–187 |
| level 3 proxy, mod `p` | 0.119 | `5.5·10^4`–`3.3·10^5` | 132–154 | 104–126 |
| level 3 proxy, mod `p³` | 0.119 | `4.2·10^5`–`2.4·10^6` | 157–178 | 129–150 |

Caveats that cut both ways: fplll is likely faster than PARI `qflll` (the
"10× lower" column allows for that), while the candidate cost at 128–256-bit
`p` is several times the 62-bit figure used here (which lowers `L` in
candidate units and so also pushes `p*` down). I did not measure either.
Verdict on falsifier 2: **not falsified**. The break-even stays above
`2^{100}` in every case, so LQ-CDCL cannot produce a measured improvement
at sizes the repository can complete.

## 6. First look: bit-blasted S₃ in z3 (feasibility only)

Instrument: `probe.py` (z3 5.1.0 binary, `QF_BV`, modular reduction after every
multiplication, `x_i` as `k`-bit vectors, `m = 2` direct; `m = 3` chained
through a full-width intermediate). One shared Mac, no isolation; wall times are
indicative only. Each row is a single random instance (one seed), so spreads
are not estimated. `planted` = target is a sum of two factor-base points;
`random` = random `x_R` (almost always UNSAT).

`m = 2` (z3 `:sat-conflicts`, wall in seconds; the `n = 20` `k = 7` random cell
reached the 60 s limit in an earlier run and is not in the table; `n = 28`
`k = 6` random is included, `k = 7` not run):

| n (bits) | k | B | planted: result, conflicts, wall | random: result, conflicts, wall |
|---:|---:|---:|---|---|
| 20 | 3 | 8 | sat, 166, 0.3 | unsat, 20644, 2.8 |
| 20 | 4 | 16 | sat, 1234, 0.4 | unsat, 8012, 1.1 |
| 20 | 5 | 32 | sat, 476, 0.5 | unsat, 5607, 2.0 |
| 20 | 6 | 64 | sat, 2572, 1.3 | unsat, 8120, 7.1 |
| 20 | 7 | 128 | sat, 22603, 14.8 | (60 s limit hit, earlier run) |
| 28 | 3 | 8 | sat, 5437, 1.3 | unsat, 263, 0.8 |
| 28 | 4 | 16 | sat, 4026, 0.6 | unsat, 11090, 1.7 |
| 28 | 5 | 32 | sat, 62174, 7.3 | unsat, 9933, 4.0 |
| 28 | 6 | 64 | sat, 101569, 21.1 | unsat, 28560, 16.1 |

`m = 3` chain, `n = 20`, `k = 4` (`B = 16`): planted and random both reached the
100 s limit (`timeout`). Smaller `k` had too few factor-base points to plant.
The `m = 3` log was overwritten once by an overlapping sweep, so the planted
`timeout` line is from the first read of the log, not the final file.

**Reading.** The solver is orders of magnitude slower than the lookup oracle,
which needs `B` field operations. The `random` (UNSAT) cases do not show the
clean `B^m` growth the carry law predicts at these sizes; with one seed and `k`
this small the width of the modular-multiplier circuit, not `B`, dominates the
effort, so the probe neither confirms nor refutes falsifier 1. Treat §2 as an
argument, not a measured law.

## 7. Disposition

- Add to the lead atlas under `LEAD-ICE-sat-xor-pdp-oracle` / `LEAD-ICP-coppersmith-smallroot-semaev`
  as "LQ-CDCL: architecture variant, break-even `p ≳ 2^{100}`, not measurable
  at completable sizes". No new research run authorised by this note.
- Falsifier 2 was run in PARI (§5.2): break-even `log₂ p* ≈ 104`–`247`. If
  pursued further, the remaining unknowns are an fplll timing, the real
  shift lattice, and the multi-limb candidate cost at 128–256 bits.
