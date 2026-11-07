# p1tool: the native instrument for P1 (IDEA-20261107-RR-DICKSON-TORUS)

2026-11-07. This is the instrument for proposal P1
([`../../../ideas/idea-generator.md`](../../../ideas/idea-generator.md), lines 41–135).
It is a standalone Rust crate. It writes msolve 0.6.5 input files for four
polynomial systems on one planted ECDLP decomposition instance, and it turns
msolve's verbose log into counted metrics. **No claim cell was run.** No Dickson
cell with m ≥ 3 was run at T24 or T31. Every number below is a calibration or a
smoke test, and is an unverified assertion [pcc:unverified].

- **Source.** [`src/main.rs`](src/main.rs). Dependencies: `serde_json`, `sha2`. Arithmetic uses `u128` products, for p < 2³².
- **Logs.** [`../calibration/`](../calibration/). `provenance.txt` holds the binary, source and msolve hashes.
- **Build.** `CARGO_TARGET_DIR=<dir> cargo build --release`, then run `<dir>/release/p1tool` directly.

## 1. Systems

There are two systems, **RR** and **T**, and each takes one of two membership
towers. The planted target is R = P₁ + … + P_m. Every x(P_i) lies in the base set.

### Towers

| kind | condition | base set (tower coordinate w) | x | step | terminal c |
|---|---|---|---|---|---|
| `kummer` | 2^t ∣ p − 1 | μ_{2^t} = ⟨ζ⟩ | x = g·w (g random, or `--g`) | y ↦ y² | 1 |
| `dickson` | p ≡ 3 (mod 4), 2^t ∣ p + 1 | V = {w + w⁻¹ : w ∈ u·μ_{2^t} ⊂ T}, where T is the norm-one torus of F_{p²} = F_p(i) | x = w | y ↦ y² − 2, since D_{2k} = D_k² − 2 | c = u^{2^t} + u^{−2^t} = D_{2^t}(v) for every v ∈ V |

The Dickson base set is a port of `examples/pkm_tower_pilot.rs` `dickson_torus`.
`check_torus` panics unless all of the following hold:

- ζ has exact order 2^t;
- u ∈ T \ F_p;
- every w + w⁻¹, computed with the general F_{p²} inverse, lies in F_p and equals V[k];
- y² − 4 is a non-residue for every y ∈ V;
- no level of the tower is zero on V;
- D_{2^t}(V[k]) = c;
- c ≠ ±2.

The protocol is `research/pkm_dickson_torus_20261015/PROTOCOL.md` §2. In both
kinds the base points are the x in the base set for which f(x) is a nonzero square.

### RR: Riemann–Roch norm-divisibility

This is a port of `research/ic_tree_gluing_20261103/round7_rr/genrr.sing`, sat = 0.
m must be odd, m = 2h − 1. The auxiliary variable Z is eliminated by taking coefficients.

**Unknowns**

- A(Z) = Z^h + Σ a_i Z^i, monic.
- C(Z) = Σ_{i ≤ h−2} c_i Z^i.
- U(Z) = Z^m + Σ u_i Z^i. U is monic in the tower coordinate, and its roots are the w_i.
- U_x(Z) = Σ u_i g^{m−i} Z^i + Z^m. This is g^m·U(Z/g), with g = 1 for Dickson.

**Equations**

1. The coefficients in Z of A² − f·C² − (Z − x_R)·U_x, with f = Z³ + aZ + b.
2. A(x_R) − y_R·C(x_R). This fixes the sign: ΣP_i = R exactly.
3. The tower. V₀ = Z. For j = 1…t: V_{j−1}² − s = Q_j·U + V_j, with s = 0 (Kummer) or s = 2 (Dickson), and V_t = c.
   - V_j has deg ≤ m − 1 and coefficients r⟨j⟩_i.
   - Q_j has coefficients q⟨j⟩_i. Its degree is 2·deg V_{j−1} − m at the first expanded level and m − 2 after that.
   - A level where V_{j−1}² − s still has degree < m (and j < t) is left unexpanded, with no variables. This follows genrr.sing.

### T: the cherry-tree chain

This is a port of arm T of `research/ic_tree_gluing_20261103/gen.sing`, for m = 2…6.
The target is x-only: S₃ accepts ±R.

**Cherries.** Cherry c has (s_c, q_c) = (w₁ + w₂, w₁w₂).

- Its remainder chain runs modulo T² − s_c T + q_c, with R₀ = T and R_j = R_{j−1}² − s mod (T² − sT + q).
- R_j = h⟨c⟩r⟨j⟩_1·T + h⟨c⟩r⟨j⟩_0 for j < t, and R_t = c.
- The cherry is glued by S₃sym(g·s, g²·q, X_c).

**Single leaves.** A single leaf w has its own chain: u₁ = w² − s, u_{j+1} = u_j² − s, and u_{t−1}² − s = c.

**Separators.** X₁… (and Y for m ≥ 5) are joined by S₃. The shapes are exactly those of gen.sing.

**Kummer reduction.** With s = 0 and c = 1 both systems are genrr.sing and gen.sing. §3 shows this byte for byte.

### Size at m = 3

These counts come from `emit` and are the same for both kinds.

| system | N | equations | input degrees (count) | N formula, m = 3 |
|---|--:|--:|---|---|
| RR, t = 4 | 18 | 20 | 2 (×18), 1 (×2) | 5t − 2 |
| T, t = 4 | 13 | 14 | 4 (×2), 3 (×6), 2 (×4), 1 (×2) | 3t + 1 |
| RR, t = 5 | 23 | 25 | 2 (×23), 1 (×2) | |
| T, t = 5 | 16 | 17 | 4 (×2), 3 (×8), 2 (×5), 1 (×2) | |
| RR, t = 3 | 13 | 15 | 2 (×13), 1 (×2) | |
| T, t = 3 | 10 | 11 | 4 (×2), 3 (×4), 2 (×3), 1 (×2) | |

At m = 5, t = 3 the RR system has N = 14 and the T system has N = 18.

The proposal's N = (2m − 1)t + 2m gives 26 at m3 t4; the actual N_RR is 18.
The degree-1 inputs are the target-sign equation and the leading coefficient of
the first expanded tower level.

## 2. CLI

Run every command `nice -n 10`, one msolve job at a time.

```
p1tool selftest
p1tool curve --p P --seed S
p1tool count --p P --a A --b B
p1tool emit --system rr|t --kind kummer|dickson --m M --t T \
       (--p P --a A --b B [--g G] --seed S | --round7 instances.txt [--k K]) \
       --out CELL.ms [--plant CELL.ms.plant.json]
calibration/msolve_cell.sh CELL.ms CELL.log [CELL.gb]
p1tool parse CELL.log
p1tool check-gb --gb CELL.gb --plant CELL.ms.plant.json
```

**`selftest`.** Checks the field arithmetic, the identity D_{2^t}(w + 1/w) = w^{2^t} + w^{−2^t}, and BSGS against the naive point count. Builds torus base sets at T24, T31, 1279 and 1999. Checks that Kummer is refused at a torus prime and Dickson at a split prime. Emits round-7 RR m3 t4 and compares its sha256 with the recorded `SYSTEMS.sha256` value. Plants systems of both kinds and both system types.

**`curve`.** Finds a prime-order curve y² = x³ + ax + b with j ∉ {0, 1728}.

- The exact order is taken from BSGS over the Hasse interval: it is the unique prime annihilator of a random point.
- For p < 2²² it is also compared with the naive count. Above that, uniqueness follows because 2n exceeds the top of the Hasse interval.
- Eight random points are checked with n·P = O.
- The output also includes n, j, a generator and v₂(p ± 1).

**`count`.** The naive Legendre-sum order, for p < 2²⁵. It is an independent check.

**`emit`.** Samples the planted instance from `--seed` and writes the `.ms` file.

- **Same instance for both systems.** The RR and T conditions are both enforced at sampling time, so the same seed gives the same instance to RR and to T. The conditions are: x_R ∉ {x_i}, exactly one sign pattern summing to R, and separators defined.
- **Planted check.** Every equation is evaluated at the planted assignment, and the norm identity is checked as a polynomial identity. Any nonzero result aborts with exit 1.
- **Outputs.** A JSON summary on stdout (N, equations, degree histogram, sha256, instance) and `CELL.ms.plant.json` (summary plus every planted variable value).

**`msolve_cell.sh`.** Runs `msolve -g 2 -t 1 -v 2 -f CELL.ms -o CELL.gb` under `ulimit -v ${VLIMIT:-1572864}`, `timeout ${TLIMIT:-1200}` and `nice -n 10`. It appends `exit=<rc> wall=<s>`, as `../../../../ic_tree_gluing_20261103/run.sh` does. `KEEPGB=1` keeps the basis file.

**`parse`.** Applies the rules of `research/ic_tree_gluing_20261103/parse.sh` (round 7) to the F4 round rows `deg sel pairs R x C dens% NEW new Z zero`.

| field | meaning |
|---|---|
| `D` | maximum `deg` over the round rows |
| `lastprod` | maximum `deg` of a round with NEW > 0 |
| `Dsel` | maximum `deg`, including a round cut off without a `new` field |
| `widest_cols`, `widest_rows`, `widest_cells` | the round with the most columns, its rows, and rows × cols |
| `max_cells`, `max_cells_dims` | the round with the largest rows × cols |
| `sumcells` | Σ rows × cols over all rounds |
| `widest_cols_by_degree` | the widest round at each degree |
| `quotient_dim` | read from `Dimension of quotient:` |
| `exit`, `wall_s` | read from the trailing line |
| `status` | `ok`; `cap` (exit 139/134/137, or an allocation message); `timeout` (124); `error`; `unterminated` |
| `D_is_lower_bound` | true unless `status = ok` |

Wall time is descriptive only.

**`check-gb`.** Parses msolve's reduced basis and evaluates every element at the planted point.

- `planted_on_variety` means every element vanishes there and the ideal is not the unit ideal.
- `planted_is_unique_solution` means the basis is N linear univariate elements, so msolve returned exactly the planted relation.

## 3. Positive control A: emitter fidelity (reproduced byte for byte)

Singular 4.3.2 was installed for cross-checking only. It regenerated round 7's
systems on round 7's own instances (`instances.txt`, `instances_r3.txt`) with the
unmodified generators:

```
OUTDIR=$S/sing_rr Singular -q research/ic_tree_gluing_20261103/round7_rr/genrr.sing
CELLS="m3_t4_k0_T m3_t5_k0_T m3_t4_k1_T m3_t6_k0_T" OUTDIR=$S/sing_t Singular -q research/ic_tree_gluing_20261103/gen.sing
INSTFILE=instances_r3.txt CELLS="m5_t4_k0_T m5_t5_k0_T" OUTDIR=$S/sing_t Singular -q research/ic_tree_gluing_20261103/gen.sing
p1tool emit --system rr --kind kummer --round7 research/ic_tree_gluing_20261103/instances.txt --m 3 --t 4 --k 0 --out m3_t4_k0_RR.ms   # etc.
p1tool emit --system t  --kind kummer --round7 research/ic_tree_gluing_20261103/instances.txt --m 3 --t 4 --k 0 --out m3_t4_k0_T.ms
```

**Result.** All 12 files are byte-identical (`cmp`). The six RR files are m3 t4,
t5, t6, t7 and m5 t4, t5. The six T files are m3 t4 k0, m3 t4 k1, m3 t5, m3 t6,
m5 t4 and m5 t5. The Singular regeneration also matches the recorded
`round7_rr/SYSTEMS.sha256` for all five recorded RR inputs. Records are in
[`../calibration/controlA.txt`](../calibration/controlA.txt), re-verified with
the final build.

The `selftest` subcommand rechecks RR m3 t4 against the recorded hash
`2ceb4c59…b82385a`.

Byte identity needs the y_R convention: y_R is the square root whose symmetric
representative is negative. genrr.sing's `sqr` returns that root in all six
round-7 RR cells.

## 4. Positive control B: the round-7 measurement (reproduced exactly)

The inputs are the files of §3. The limits are round 7's: `msolve -g 2 -t 1 -v 2`,
1.5 GiB, 1200 s, `nice -n 10`, one job at a time. Logs are in
[`../calibration/controlB/`](../calibration/controlB/), and `parse_sh.tsv` is the
output of round 7's own `parse.sh` on them.

```
calibration/msolve_cell.sh m3_t4_k0_RR.ms calibration/controlB/m3_t4_k0_RR.log && p1tool parse calibration/controlB/m3_t4_k0_RR.log
```

| cell | D | widest (cols; rows × cols) | sumcells | qdim | round-7 record |
|---|--:|---|--:|--:|---|
| RR-Kummer m3 t4 k0 | 4 | 2,801; 2,989 × 2,801 | 40,232,151 | 1 | D 4, width 2,801, sumcells 40,232,151, qdim 1 (`round7_rr/rows.tsv`) |
| T-Kummer m3 t4 k0 | 6 | 7,077; 9,037 × 7,077 | 151,012,610 | 3 | D 6, width 7,077, sumcells 151,012,610, qdim 3 (`../rows.tsv`) |
| T-Kummer m3 t4 k1 | 6 | 7,077; 9,037 × 7,077 | 151,012,610 | 3 | the same as k0 |

D, the widest matrix, sumcells and the quotient dimension are all identical to
the records. Wall times were 0.71 s, 3.71 s and 3.59 s; the records give 0.47 s
and 3.7 s.

**m3 t3.** Round 7 has no m3 t3 instance. With round 7's g = 780070, g·μ₈ holds
only 2 curve points, so nothing could be reproduced there. A new calibration,
not a reproduction, used round 7's curve with a random g: `emit --kind kummer
--p 786433 --a 779083 --b 163026 --m 3 --t 3 --seed 1|2`. It gave RR **D = 4**,
widest 632, qdim 1, and T **D = 5**, widest 883, qdim 3, for both seeds. The
planted point lies on both varieties. Logs are in
[`../calibration/m3t3_new/`](../calibration/m3t3_new/).

## 5. Dickson smoke tests (tiny primes only)

The curves are prime-order, from `p1tool curve --seed 1`:

| p | a | b | n | v₂(p + 1) |
|---|---|---|---|---|
| 1279 | 10 | 111 | 1249 | 8 |
| 1151 | 294 | 772 | 1123 | 7 |
| 1999 | 1457 | 1780 | 2027 | 4 |

Cells were m3 t3 and m3 t4, seeds 1 and 2, RR and T. That makes 22 systems; the
two p1279 t3 s2 systems had no admissible instance (3 base points).

- **Torus condition.** `check_torus` passed for every instance.
- **Planted check.** Passed for every system.
- **Planted point on the variety.** Yes in 22 of 22 (`check-gb`).
- **Planted relation returned.** RR returned exactly the planted relation (qdim 1, N linear basis elements) in 9 of 11 cells. The exceptions are the two p1151 t4 cells, with qdim 2: a second decomposition exists in the small base. T has qdim 3 at t3, and 3, 6 or 8 at t4, because of the x-only target.
- **D and width, m3 t4, every prime and seed.** RR-Dickson: D = 4, widest 2,801. T-Dickson: D = 6, widest 7,129.
- **D and width, m3 t3.** RR-Dickson: D = 4, widest 632. T-Dickson: D = 5, widest 883.

These numbers are smoke-test values at p < 2000, not claim cells. The table is
[`../calibration/dickson_smoke/smoke.tsv`](../calibration/dickson_smoke/smoke.tsv),
and the instances are in `emit.jsonl`.

## 6. Admissible instances at the claim primes (emit only, no msolve run)

The curves are from `p1tool curve --seed 1`. Both orders are prime.

| prime | a | b | n | order check |
|---|---|---|---|---|
| T24 = 14,680,063 | 3,607,823 | 6,826,282 | 14,683,681 | BSGS; naive count agrees (`count`) |
| T31 = 2,046,820,351 | 289,962,109 | 1,299,273,082 | 2,046,768,077 | BSGS and Hasse uniqueness only |

For a prime-order Kummer control at p₀ = 786433: a = 658,649, b = 132,295,
n = 785,623. Round 7's own p₀ curve has order 786,627, which is not prime.

Emit-only check, seeds 1 and 2:

| cell | T24 | T31 |
|---|---|---|
| m3 t4 | both seeds admissible | both seeds admissible |
| m3 t5 | both seeds admissible | both seeds admissible |
| m3 t3 | seed 1 has no admissible instance (3 base points) | seed 1 has no admissible instance (3 base points) |
| m5 t3 | admissible only for seed 2 | no admissible instance for either seed |

## 7. Commands for the protocol to freeze

```
p1tool emit --system {rr|t} --kind dickson --p {14680063|2046820351} --a A --b B --m M --t T --seed S --out C.ms
VLIMIT=1572864 TLIMIT=1200 calibration/msolve_cell.sh C.ms C.log     # nice -n 10 inside; one job at a time
p1tool parse C.log
KEEPGB=1 … ; p1tool check-gb --gb C.gb --plant C.ms.plant.json        # optional planted-relation check
```

The binary's sha256 is in `../calibration/provenance.txt`.
