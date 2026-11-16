# L03 implementation specification (frozen with PROTOCOL.md)

Read [`PROTOCOL.md`](PROTOCOL.md) first. This document leaves the implementer no
research choice. Where it says **port verbatim**, copy the named lines from the
repository source, change only what is listed, and keep every counter increment
exactly where the source has it. Write nothing outside
`/home/user/crypto/research/g2plus_leads_20261116/L03_base_rebalance/` (abbreviated `LEAD/`).
No git commands that change state.

## 1. Files

```
LEAD/
  PROTOCOL.md  SPEC.md  frozen.sha256           (coordinator, frozen; do not edit)
  instances.jsonl                               (extracted by the executor, PROTOCOL §9)
  sim/Cargo.toml
  sim/src/main.rs        CLI, JSON output
  sim/src/rng.rs         mix64, Rng
  sim/src/jac.rs         Poly, Fk, JDiv, JCurve, GroupOps, scalar mult, instance_from
  sim/src/rel.rs         ModU, splits_completely, roots_of, disc_character, helpers
  sim/src/la.rs          Nat, mulmod/addm/subm/powm/invm, Rel, Row, combine, Dsu,
                         build_rows, peel, mat_vec, mat_t_vec, m_apply,
                         berlekamp_massey, wiedemann, pivot_inversion_steps
  sim/src/run.rs         fb_size, grid, run_ic (the 2LP arm), RunOut
  analyze.py             Python 3 standard library only
  raw/   analysis/       outputs
```

`sim/Cargo.toml`, exactly:

```toml
[package]
name = "l03sim"
version = "0.1.0"
edition = "2021"

[dependencies]

[workspace]

[profile.release]
opt-level = 3
debug = false
```

No dependencies (no `rand`, no `serde`). Rust standard library only. Single thread.

## 2. `rng.rs`

```rust
pub const GOLDEN: u64 = 0x9E37_79B9_7F4A_7C15;
pub fn mix64(mut z: u64) -> u64 {           // verbatim from src/cryptanalysis/residual_walk.rs:336-341
    z = z.wrapping_add(GOLDEN);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}
#[derive(Clone, Debug)]
pub struct Rng { pub state: u64 }
impl Rng {
    pub fn new(seed: u64) -> Rng { Rng { state: seed } }
    pub fn next_u64(&mut self) -> u64 { let out = mix64(self.state); self.state = self.state.wrapping_add(GOLDEN); out }
    /// uniform-ish in [lo, hi), hi > lo; modulo bias accepted and declared
    pub fn range(&mut self, lo: u64, hi: u64) -> u64 { lo + self.next_u64() % (hi - lo) }
}
pub fn run_seed(p: u64, curve_seed: u64, seed: u64) -> u64 {
    mix64(p.wrapping_mul(1_000_003) ^ curve_seed.wrapping_mul(GOLDEN) ^ seed.wrapping_mul(0xD1B5_4A32_D192_ED03))
}
```

The run seed does **not** depend on `cc` (common random numbers, PROTOCOL §3).
Everywhere the source calls `rng.gen_range(a..b)` on a `StdRng`, call `rng.range(a, b)`
(for `usize` ranges cast through `u64`). `rng.clone()` stays a clone.

## 3. `jac.rs`: port verbatim from `src/cryptanalysis/genus_ladder.rs`

| source lines | items | changes |
|---|---|---|
| 63–66 | `MAX_GENUS = 5`, `CAP = 24` | none |
| 70–113 | `Poly` | none |
| 115–285 | `Fk` (counted `F_p`: `tick`, `inv`, `add`, `sub`, `neg`, `scale`, `mul`, `divrem`, `rem`, `monic`, `xgcd`, `eval`) | none: `mul` ticks `a.n·b.n`, `divrem` ticks as written, `inv` increments the inversion counter |
| 287–343 | `pow_mod`, `sqrt_mod` (uncounted) | none |
| 345–630 | `JDiv`, `JCurve` (`new`, `p`, `genus`, `f_coeffs`, `fmul`, `finv`, `polys`, `pack`, `u_coeffs`, `v_coeffs`, `is_valid`, `point`, `random_divisor`, `cantor`, `reduce`, `negate`, `mul_ltr`, `mul_rtl`, `key_of`) | `is_prime_u64` → local `fn is_prime(p: u64) -> bool` by trial division up to `sqrt(p)`; `mix64` from `rng.rs`. Do **not** port `count_points` or `jacobian_order` |
| 797–826 | `CountedGroup` impl → plain methods `add(&self, ops, a, b)` (counts `doubles` if `a == b` else `adds`), `double`, `neg`, `key` | no trait; plain inherent methods |
| `src/cryptanalysis/ic_boundary.rs` 1529–1543 | counted scalar multiplication `mul(&self, ops, p, k)` (left-to-right double-and-add from the second-highest bit, `ops.scalar_mults += 1`) | inherent method `JCurve::smul(&self, ops: &mut GroupOps, p: JDiv, k: u64) -> JDiv` |
| 866–903 | `instance_from(curve, order, h, r, target_seed)` | returns `Instance { curve, p, genus, order, h, r, g, t_secret, target }` as in lines 828–839 |

`GroupOps { adds: u64, doubles: u64, scalar_mults: u64 }`, all `u64`, `Default`.
`JCurve::fmul()` and `finv()` read the cumulative `Fk` counters, and `Fk` uses `Cell`
as in the source. `JDiv` derives `Clone, Copy, Debug, PartialEq, Eq, Hash, Default`.

## 4. `rel.rs` and `la.rs`: port verbatim

`rel.rs`, from `genus_ladder.rs` lines 905–1163: `ModU`, `Res`, `mod_u`,
`splits_completely`, `vtrim`, `cinv`, `vrem`, `vgcd_monic`, `vdiv_monic`, `roots_of`
(change `&mut StdRng` to `&mut Rng`), `disc_character`. The `Nat` counter comes from `la.rs`.
`cinv` charges its Fermat inversion as multiplications into `Nat.mul`. Keep this: it
is the lab convention, so relation-phase inversions are already priced inside `rel_mul`.

`la.rs`, from `src/cryptanalysis/chained_transfer/ic_control.rs`:

| lines | items | changes |
|---|---|---|
| 52–62 | `Nat { mul, add }`, `merge` | none |
| 543–591 | `mulmod`, `addm`, `subm`, `powm`, `invm` | `mulmod(a, b, r) = ((a as u128 * b as u128) % r as u128) as u64` (exactly equal to the source's for `r < 2^50`) |
| 865–887 | `REL_FB = 5`, `Rel`, `Rel::endpoints` | none |
| 890 | `type Row = (Vec<(u32, u64)>, u64)` | none |
| 892–928 | `combine` | none |
| 930–972 | `Dsu` (`new`, `add_vertex`, `find`, `edge`, `cycles0`) | none (`ops` counter kept) |
| 975–1071 | `build_rows` | none |
| 1076–1124 | `peel` | none |
| 1127–1188 | `mat_vec`, `mat_t_vec`, `m_apply` | none |
| 1190–1236 | `berlekamp_massey` | none |
| 1238–1317 | `wiedemann` | `&mut StdRng` → `&mut Rng` |
| `ic_boundary.rs` 1399–1401 | `pivot_inversion_steps(m) = 2·(64 − m.leading_zeros())` | none |

## 5. `run.rs`: the 2LP arm

```rust
pub const GENUS: u32 = 3;
pub const RESTART: u64 = 4096;
pub const MAX_TRIALS: u64 = 60_000_000;
pub fn grid() -> Vec<f64> { (-6..=4).map(|k| 2.0f64.powf(k as f64 / 2.0)).collect() }
pub fn fb_size(p: u64, cc: f64) -> usize {           // = genus_ladder.rs:1210-1213 at g = 3
    let nb = (cc * (p as f64).powf(1.0 - 1.0 / 3.0)).ceil() as usize;
    nb.min(p as usize)
}
```

Expected `fb_size` (gate Q5 and unit test T3), grid order `cc = 0.125 … 4`:

| p | 0.125 | 0.177 | 0.25 | 0.354 | 0.5 | 0.707 | 1 | 1.414 | 2 | 2.828 | 4 |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 8209 | 51 | 72 | 102 | 144 | 204 | 288 | 407 | 576 | 814 | 1151 | 1628 |
| 16411 | 81 | 115 | 162 | 229 | 323 | 457 | 646 | 914 | 1292 | 1827 | 2584 |
| 32771 | 129 | 182 | 257 | 363 | 513 | 725 | 1025 | 1449 | 2049 | 2897 | 4097 |

`run_ic(inst: &Instance, nb: usize, seed_state: u64, verify_relations: bool) -> RunOut`
is **`genus_ladder.rs` lines 1276–1529 ported verbatim** for `arm = DoubleLp`
(`max_lp = 2`), `prefilter = true`, `sge = None`, `restart = RESTART`,
`max_trials = MAX_TRIALS`, `rng = Rng::new(seed_state)`. Delete the `Arm::Full`
branches (the `else` branch of the cycle check at lines 1465–1473, and `want = usize::MAX`;
the `if prm.arm != Arm::Full` test becomes unconditional).
Then apply exactly these modifications:

- **M1 (Cantor counter split).** Read `(curve.fmul(), curve.finv(), ops.adds + ops.doubles)`
  at four points: `s0` at function entry; `s1` right after the jump table is built
  (where the source sets `res.setup_ops`); `s2` at the moment `wiedemann` returns a
  solution that is about to be certified (immediately before `let v0 = ...`, line 1511);
  `s3` after `chk = curve.smul(&mut ops, inst.g, t)` (the first certificate multiplication).
  Set `cm_setup = s1.mul − s0.mul`, `ci_setup = s1.inv − s0.inv`;
  `cm_loop = s2.mul − s1.mul`, `ci_loop = s2.inv − s1.inv`;
  `cm_verify = s3.mul − s2.mul`, `ci_verify = s3.inv − s2.inv`, and similarly
  `group_ops_setup`, `group_ops_loop`, `group_ops_verify`. If certification fails and
  the loop continues, the next success re-reads `s2` and `s3`, and the failed certificate's
  Cantor work stays inside `cm_loop`. After the second, independent multiplication
  (`mul_rtl`), restore the `Fk` counters exactly as the source does (lines 1514–1519).
  The JSON fields `cantor_mul` and `cantor_inv` are `cm_setup + cm_loop + cm_verify` and
  the corresponding inversion sum.
- **M2 (relation counter split).** Two `Nat`: `rel_fixed` receives the character-table
  count (`(p−1)/2` mul) and the base-scan counts (lines 1294–1324). `rel_trial` receives
  every `rel` increment inside the `'outer` loop: `disc_character`, `splits_completely`,
  `roots_of`, the `v` evaluations, and the `res.rel.mul += 2`.
- **M3 (`n_full`).** After the base scan, compute in a separate **uncounted** pass
  `n_full = #{x in 0..p : chi[f(x) mod p] == 1}` (Horner, no counter). `beta = fb_cols as f64 / n_full as f64`.
- **M4 (duplicates).** A `HashSet<JDiv>` of the divisors `cur` of accepted relations.
  When an accepted relation's `cur` is already in the set, increment `dup_relations` and
  **still keep the relation** (lab behaviour).
- **M5 (matrix shape).** For the `core` passed to `wiedemann` on the attempt that
  certifies, record: `core_rows = core.len()`, `core_cols = cc`,
  `core_nnz = Σ row lengths`, `core_nonunit = #{entries with v ∉ {1, r−1}}`,
  `core_max_row = max row length`. `cycles_final = dsu.cycles0()` at that attempt.
  `lp_vertices = dsu.parent.len() − 1`.
- **M6 (walk fingerprint, test hook).** `walk_fp1000`: start at 0. For each of the first
  1000 trials (after `cur` is updated, before the degree check), set
  `walk_fp1000 = mix64(walk_fp1000 ^ curve.key_of(&cur))`. If fewer than 1000 trials happen, keep
  what was reached.
- **M7 (LA split).** As in the source: `la` = graph + peel + Wiedemann, with `dsu.ops`
  added to `la.add` at the end. Also output `la_graph_add`, `la_peel_add`,
  `la_wied_mul`, `la_wied_add`, `dsu_ops` separately.
- **M8 (verification mode).** With `verify_relations = true`, do what the source does
  (lines 1435–1444, counting `bad_relations`) and also count `verified_relations`.
  As in the source, set `cantor_mul = cantor_inv = 0` and the `cm_*`/`ci_*` fields to 0
  in this mode.

`RunOut` holds every field of §6. `t_planted = inst.t_secret`.
`certified = (recovered == Some(t_planted)) && both certificate checks passed`.

## 6. `main.rs`: CLI and output

```
l03sim selftest --instances FILE --out FILE
l03sim run --instances FILE --p P --curve-seed CS --seed S --set {S|E|V} [--verify-relations] --out FILE
```

Parse `instances.jsonl` with a hand-written parser: each line is a flat JSON object.
Extract the integers `curve_seed`, `p`, `order`, `h`, `r`, `g` and the integer array `f`.
Select the line with matching `p` and `curve_seed` (exactly one; otherwise exit 2).
`curve = JCurve::new(p, 3, &f)`, `inst = instance_from(curve, order, h, r, curve_seed)`.
Assert `h * r == order`.

**`run`** writes, with one JSON object per line and no spaces, keys in the order below:

1. `{"kind":"config","lead":"L03","p":..,"curve_seed":..,"seed":..,"set":"S","grid":[0.125,...,4.0],"restart":4096,"max_trials":60000000,"seed_state":..,"verify_relations":false}`
2. `{"kind":"instance","p":..,"curve_seed":..,"order":..,"h":..,"r":..,"t_planted":..,"g_u":[..],"g_v":[..],"t_u":[..],"t_v":[..]}`
   (`u` and `v` coefficients of G and T, lowest degree first.)
3. For each `cc` in `grid()` in ascending order, rebuild the instance (fresh `JCurve`,
   so counters start at 0), call `run_ic(&inst, fb_size(p, cc), run_seed(p, cs, seed), verify)`, and write:

```
{"kind":"run","p","curve_seed","seed","set","cc","nb","fb_cols","n_full","beta",
 "trials","candidates","prefilter_rejects","splits","relations","rels_by_lp":[n0,n1,n2],
 "dup_relations","lp_vertices","cycles_final","attempts",
 "core_rows","core_cols","core_nnz","core_nonunit","core_max_row",
 "cantor_mul","cantor_inv","cm_setup","ci_setup","cm_loop","ci_loop","cm_verify","ci_verify",
 "group_ops_setup","group_ops_loop","group_ops_verify",
 "rel_fixed_mul","rel_fixed_add","rel_trial_mul","rel_trial_add",
 "la_mul","la_add","la_graph_add","la_peel_add","la_wied_mul","la_wied_add","dsu_ops",
 "walk_fp1000","recovered","t_planted","certified","exhausted",
 "bad_relations","verified_relations"}
```

Integers are printed as integers; `recovered` is `null` when absent; `cc` and `beta` use
Rust `{}` Display for `f64`. Flush after each line. On stderr, print one line per `cc`:
`p cs seed cc trials certified`. No wall time is printed in the JSON (a `wall_ns` field
is forbidden).

**`selftest`** writes one line per instance:
`{"kind":"selftest","p","curve_seed","g_nonzero":bool,"rG_zero":bool,"order_kills":[b,b,b,b],"t_ok":bool,"pass":bool}`.
Here `rG_zero` means `mul_ltr(G, r)` is the identity. `order_kills[i]` means
`mul_ltr(D_i, order)` is the identity, where `D_i = random_divisor(&mut st)` with
`st = 0xABCD_0000 + i`. `t_ok` means `mul_rtl(G, t_planted) == T`. `pass` is the AND of
all of these. Exit code 0 only if every instance passes.

## 7. Cost formulas (used only in `analyze.py`)

Bands `(I, kappa)`: `lo = (1, 1)`, `mid = (10, 9)`, `hi = (30, 9)`. For a run `x`:

```
F_fixed = cm_setup + I*ci_setup + cm_verify + I*ci_verify + rel_fixed_mul
F_rel   = cm_loop + I*ci_loop + rel_trial_mul
F_la    = kappa * la_mul                      # S_add sensitivity: kappa * (la_mul + 0.1*la_add)
FPM(x; d, lam, band) = F_fixed + F_rel / d + lam * F_la
```

`D = [1, 3, 10, 30]`; `LAM = [1.0, 0.35, 0.255]`; `GRID` = the 11 values in §5.
Lab references (constants in `analyze.py`, labelled `[pcc:unverified]`):

```
RHO_LAB_MID = {8209: 1.425e8, 16411: 2.887e8, 32771: 1.0084e9}
CAL = {8209:  dict(cantor_per_trial=280.36, rel_per_trial=207.86, trials=43964,  core_cols=741.0,  la_mul=6.976e6),
       16411: dict(cantor_per_trial=278.62, rel_per_trial=228.72, trials=102969, core_cols=1196.8, la_mul=1.798e7),
       32771: dict(cantor_per_trial=278.52, rel_per_trial=231.98, trials=242686, core_cols=1933.2, la_mul=4.643e7)}
FB_EXPECT = the §5 table
```

## 8. `analyze.py`

`python3 analyze.py --selftest` runs the tests in §9.2 and exits 0 or 1.
`python3 analyze.py RUNFILES... --verify VERIFYFILES... --out analysis/analysis.json --tables analysis/tables.md`:

1. Load every `"kind":"run"` line. Key each run by `(p, curve_seed, seed, cc)`, with
   `cc` matched to `GRID` by `abs(cc − g) < 1e-9`. Require exactly 9 × 4 × 11 = 396
   metric runs, otherwise record gate `Q_count` failed.
2. **Gates.** Q4: every metric run has `certified` true, `exhausted` false and
   `recovered == t_planted`. Q5: `fb_cols == FB_EXPECT[p][cc]`. Q6: every verify run
   has `bad_relations == 0` and is certified. Q7: at `cc = 2`, pooled over the 12 runs
   of a prime, compute each of
   `(cantor_mul + 10*cantor_inv)/trials` (sum over sum), `(rel_fixed_mul + rel_trial_mul)/trials`
   (sum over sum), mean trials, mean core_cols and mean la_mul. Compare each with `CAL`
   within ±5%, ±10%, ±15%, ±5% and ±20%. Q8: `AA(p) = mean_S FPM(d=1, lam=1, mid, cc=2) / mean_E (same)`,
   pass when `|AA − 1| ≤ 0.05`. Q9: `mean trials(cc=0.25) / mean trials(cc=2) ≥ 3.0` and
   `mean la_mul(cc=4) / mean la_mul(cc=2) ≥ 2.0`, over all 12 runs of the prime.
   Store every gate as `{"id", "pass", "values"}`.
3. **Selection** (S runs, seeds 1 and 2). For each `p, d, lam, band, reading ∈ {primary, S_add}`:
   `meanS[cc]` = arithmetic mean of FPM over the 6 S runs at that `cc`.
   `cc_star` = the `cc` minimising `meanS`; on an exact float tie take the larger `cc`.
   `edge = cc_star in (0.125, 4.0)`.
4. **Evaluation** (E runs, seeds 3 and 4). `G = 1 − ΣE FPM(cc_star) / ΣE FPM(cc=2.0)`.
   Bootstrap: `rng = random.Random(20261116)`, created **once** and reused in this fixed
   order. Loop `band in [lo, mid, hi]`, then `reading in [primary, S_add]`, then
   `p in [8209, 16411, 32771]`, then `d in D`, then `lam in LAM`. For each of `B = 4000`
   resamples: draw 3 curves with `rng.choice` over the prime's sorted curve list; for each
   drawn curve draw 2 seeds with `rng.choice([3, 4])`. Sum the numerator and the
   denominator over the drawn `(curve, seed)` pairs. Store `1 − num/den`. The CI is the
   2.5th and 97.5th percentiles: sort, then take index `int(0.025*B)` and index
   `int(0.975*B) − 1`.
5. **Interaction** (H2, band mid, primary, `lam = 1`): `cc_star_1` and `cc_star_10` from
   step 3; `G_inc = 1 − ΣE FPM(d=10, cc_star_10) / ΣE FPM(d=10, cc_star_1)`, with a
   bootstrap as in step 4 using a separate `random.Random(20261117)`. H2 holds when
   `cc_star_10 < cc_star_1` at ≥ 2 primes and the `G_inc` lower bound ≥ 0.10 at 32771.
6. **Attribution** (E, band mid, primary): `E_d`, `R_d`, `S_{d,lam}` and the total, as in
   PROTOCOL §6.3, using E sums; `G_1 = 1 − R_1`.
7. **H3**: per prime, OLS over all 12 runs at each of the 7 grid points `0.25 … 2.0`
   (84 points) of `ln trials` on `ln fb_cols` gives `slope_T`. OLS over all 132 runs of
   `ln la_mul` on `ln core_cols` gives `slope_L`. H3 holds when `slope_T ∈ [−1.3, −0.7]`
   and `slope_L ∈ [1.7, 2.4]` at every prime.
8. **Rho ratios**: for each `p, d, lam, band`:
   `ratio_lab = meanE FPM(cc_star) / RHO_LAB_MID[p]` (band mid only).
   `ratio_model = meanE FPM(cc_star) / rho_model`, where
   `rho_model = mean over the prime's 3 curves of sqrt(pi*r/4) * (ΣE (cm_loop + I*ci_loop) / ΣE group_ops_loop)`
   (the per-operation FPM is pooled over the E runs at `cc = 2`).
9. **Verdict** (PROTOCOL §6.4) from `G` at `d = 10`, `lam = 1`, band mid, primary.
   `verdict = "inconclusive (gate)"` if any of Q4–Q9 or Q_count failed.
10. Write `analysis.json` with the keys `gates`, `selection`, `evaluation`,
    `interaction`, `attribution`, `h3`, `rho`, `verdict`, `per_cc_means`. `per_cc_means`
    holds, per `p` and `cc`, the means over all 12 runs of trials, relations,
    rels_by_lp, fb_cols, beta, core_rows, core_cols, core_nnz, core_nonunit, core_max_row,
    la_mul, la_add, cm_loop, rel_trial_mul, dup_relations, attempts and FPM_mid (d=1).
    Write `tables.md` with five tables. T1: per-`cc` means at each prime. T2: `cc_star`,
    `beta_star` and `G` [CI] for every `d`, `lam = 1`, band mid. T3: the attribution
    factors. T4: the rho ratios. T5: the gates. The first line of `tables.md` is
    `VERDICT: <supported|killed|inconclusive|inconclusive (gate)>`. Every table caption
    carries `[pcc:unverified]`. Every row with `d > 1` is labelled `synthetic discount`,
    and every row with `lam < 1` is labelled `SGE parameter`.

Printing: floats with 6 significant digits in `tables.md`; full precision in JSON.

## 9. Unit tests with expected values

### 9.1 Rust (`cargo test --release`; put the tests in `#[cfg(test)]` modules)

| id | test | expected |
|---|---|---|
| T1 | `mix64(0)`, `mix64(1)`, `mix64(0x1234)` | 16294208416658607535, 10451216379200822465, 6873671191729682568 |
| T2 | `Rng::new(0)`: three `next_u64` | 16294208416658607535, 7960286522194355700, 487617019471545679; `Rng::new(0).range(5, 17)` = 12 |
| T3 | `fb_size(p, cc)` over the grid for the three primes | the §5 table, all 33 values |
| T4 | `Fk` at p = 7: `mul((1 + x), (1 + x))` | coefficients [1, 2, 1], `mul` counter +4 |
| T5 | `Fk::inv(3)` at p = 7 | 5, `inv` counter +1 |
| T6 | `mulmod(123456789012, 98765432109, 17613478574387)` | 10351459435361 |
| T7 | character table at p = 11 (the `chi` construction of `run_ic`) | `chi[x] = +1` for x ∈ {1, 3, 4, 5, 9}, 0 at 0, −1 otherwise |
| T8 | `disc_character(7, [2,4,1], chi7)`, `[1,0,1]`, `[1,5,1]` | +1, −1, 0 |
| T9 | `splits_completely(7, [2,4,1])`, `([1,0,1])`; `roots_of(7, [2,4,1])` sorted | true, false, [1, 2] |
| T10 | `Dsu`: `new`; `add_vertex` twice (ids 1, 2); `edge(0,1)`, `edge(1,2)`, `edge(2,0)` → `cycles0()`; then `edge(0,0)` → `cycles0()` | 1, then 2 |
| T11 | instance p = 8209, curve_seed 23 (h = 1): `smul(G, r)` is the identity; `mul_ltr(G, t) == mul_rtl(G, t) == T` | true |
| T12 | the same curve: a point `P` from `sqrt_mod` at the first `x ≥ 1` with square `f(x) ≠ 0`; `cantor(P, negate(P))` is the identity; `is_valid(cantor(P, P))` | true, true |
| T13 | Wiedemann on a planted system: `r = 1000003`, 40 columns plus `t`, 60 rows, each row with 3 random columns of coefficient ±1 plus the `t` column with a random coefficient in `[1, r)`, `rhs = A·x0` for random `x0`; RNG `Rng::new(7)` for the system, `Rng::new(8)` for `wiedemann(..., dense_u = true, ...)` | returns `x` with `x == x0` (retry with `Rng::new(9)` once if `None`; the test fails if both return `None`) |
| T14 | end to end: p = 8209, curve_seed 23, seed 1, `cc = 2.0` | `certified`, `recovered == t_planted`, `fb_cols == 814`, `bad_relations == 0` with `verify_relations = true` |
| T15 | common random numbers: the same `(p = 8209, cs = 23, seed = 1)` at `cc = 1.0` and `cc = 2.0` | equal `walk_fp1000`, and both runs have `trials > 1000` |
| T16 | counter split: in T14 without verification, `cantor_mul == cm_setup + cm_loop + cm_verify` and `rel_fixed_mul ≥ (8209 − 1)/2` | true |

### 9.2 Python (`analyze.py --selftest`)

| id | test | expected |
|---|---|---|
| P1 (PC2) | synthetic runs: for every grid `cc`, `F_fixed = 0`, `F_rel = 1/cc` (put it in `cm_loop`, every other count 0), `la_mul = cc²/9` (so `F_la = cc²` at band mid); `d = 1` | `cc_star = 0.7071067811865476`, minimum 1.914213562373095 |
| P2 (PC2) | the same with `d = 8` | `cc_star = 0.3535533905932738`, minimum 0.47855339059327373 |
| P3 (NC1) | one record with `cm_loop = 1000`, `rel_trial_mul = 500`, `la_mul = 100`, `cm_setup = 7`, `rel_fixed_mul = 3`, other counts 0, band mid | FPM(d=1, lam=1) = 2410; FPM(d=10, lam=1) = 1060; FPM(d=10, lam=0.255) = 160 + 229.5 = 389.5 |
| P4 | decision function on `(point, lo, hi)` triples for `[8209, 16411, 32771]`: A `[(0.3,0.2,0.4)]*3` → supported; B `[(0.1,0.05,0.2),(0.12,0.05,0.2),(0.5,0.4,0.6)]` → killed; C `[(0.3,0.2,0.4),(0.3,0.2,0.4),(0.12,0.1,0.14)]` → killed; D `[(0.3,0.1,0.4),(0.3,0.2,0.4),(0.3,0.1,0.4)]` → inconclusive | as stated |
| P5 | the bootstrap percentile indices for B = 4000 | 100 and 3899 |

## 10. Executor notes

- Run the commands of PROTOCOL §9 unchanged, in order. Stop at Q1, Q2, Q3 or Q10
  failures and report them. Do not edit thresholds, the grid, the seeds or the formulas.
- If a repository hook refuses a command, append the refusal verbatim to
  `raw/REFUSALS.txt`, do not work around it, and continue with what remains possible.
- Expected cost: 36 metric processes. The largest (p = 32771) is a few CPU-minutes, peak
  memory under 500 MB. If a process exceeds 10 CPU-minutes, let it finish. Record its
  CPU time from `/usr/bin/time -v` only if it was already being used. CPU time is not a
  metric.
