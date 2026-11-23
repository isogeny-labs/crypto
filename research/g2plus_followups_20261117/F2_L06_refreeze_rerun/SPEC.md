# F2 implementation specification

Frozen with PILOT.md and PROTOCOL.md (`frozen.sha256`). The instrument is already
written. The coordinator implemented and piloted it, and its source hashes are frozen
in section 1. **The executor does not write or change code.** The executor verifies the
hashes, builds from source, runs PROTOCOL.md section 9, and writes RESULTS.md (and
DEVIATIONS.md if needed). If any instruction here seems wrong, the executor stops and
reports. It does not fix anything. Standing: `[pcc:unverified]`.

This file states what the code does, precisely enough to audit or re-implement it
without a research choice. Sections not marked "F2" are identical to L06 `SPEC.md`
(sha256 recorded in section 13) and are summarised here.

## 1. Files, provenance and frozen hashes

```
l06r/Cargo.toml, Cargo.lock   package "l06r", no dependencies, empty [workspace], opt-level 3
l06r/.gitignore               "target/"
l06r/src/main.rs              CLI (section 10)                         [F2: modified]
l06r/src/rng.rs               SplitMix64 with draw counter, seed tags    [F2: modified]
l06r/src/order.rs             stages A, B, C and assembly (section 4)    [F2: modified]
l06r/src/enumnorm.rs          N2, N3 by norm-character enumeration       [F2: new]
l06r/src/enumchi.rs           N1..N3 by square tables (G1 reference)      [unchanged]
l06r/src/fp.rs poly.rs cantor.rs crit.rs gsp.rs lagr2.rs                 [unchanged]
l06r/src/tests.rs             15 unit tests (section 11)                 [F2: 4 added, 1 call adapted]
analyze.py                    analysis and verdict (section 12)          [F2: modified]
pilot/run_pilot.sh            the coordinator's pilot runner (not part of the decision run)
analyze_l06_orig.py           verbatim copy of L06 analyze.py, for diffing only
COPIED_FROM_L06.sha256        sha256 of the L06 files copied here (taken in L06's directory)
SOURCES.sha256                sha256 of every frozen source file below
```

Files copied from L06, with their sha256 at copy time (`COPIED_FROM_L06.sha256`): every
file of `l06/src/`, plus `l06/Cargo.toml`, `l06/Cargo.lock` and `analyze.py`. Unchanged
copies: `cantor.rs` 49914eeb...e8781107, `crit.rs` 41fe9958...923918c92,
`enumchi.rs` 64f4c995...2cd, `fp.rs` 1c87f74b...a01, `gsp.rs` 48271a72...ae518,
`lagr2.rs` 9f371927...cfa58, `poly.rs` 762214d9...a5b (full hashes in
`COPIED_FROM_L06.sha256`). In `Cargo.toml` and `Cargo.lock`, the package name changed
from `l06` to `l06r` and nothing else.

Frozen source hashes. The content of `SOURCES.sha256`, verified with
`sha256sum -c SOURCES.sha256`:

```
c6bc61fe743279675547191f58df87eff495c3cd61ac99c39969c20cd8c30a3e  l06r/Cargo.toml
ce58bd399d53250116d0c35dc2c55cfc5f6fdda43f7525962bda1a5aebf4e0cd  l06r/Cargo.lock
b53ee26e6ff1da219b0563333a76d2c9ec19eec713d6ee54e11a1edacb0e9f3d  l06r/.gitignore
49914eeb77058e89f47224435d123ba547814ed2ae04cb19ed897760e8781107  l06r/src/cantor.rs
41fe995862f9bf28d2cd85d8ea090b937cc579d9bc8564ac9d5ad08923918c92  l06r/src/crit.rs
64f4c9958772c097e492abdb6f479b367d74bc19268add951d79e4a4bfb9d2cd  l06r/src/enumchi.rs
32180ce1267f4f5c2f4608198ee6251f414e8a938654c0b9da6b8fdc638e1e96  l06r/src/enumnorm.rs
1c87f74b3bf90f62035e7fbdff46a8be179ccabe2638eda5bb0a199186832a01  l06r/src/fp.rs
48271a725a1fa4755a0486500a82601173519036a4d9ae017e2a9342037ae518  l06r/src/gsp.rs
9f37192750cef6b564e8ba4616237c2a9a55a60271475cfbc51d55a3ce8cfa58  l06r/src/lagr2.rs
a386f6a4d0e3b081a0a257ac7ca5a19d68c914d4dcd3281b189bcb23b5d86848  l06r/src/main.rs
c639c8564d1781b1fc91af2ddafd01b0b72a4deb3c5cb20906521674963c1913  l06r/src/order.rs
762214d93f6167ad4d4f3a9c60ea50e3c347019c719ab86162d469ab95d78a5b  l06r/src/poly.rs
71d8efdc24e2bf80a0e825e4fc739c7374df657c13d3ccc514ca5924c9144114  l06r/src/rng.rs
bd8b3a659078901e07f44308b1ab358f41a44ce9069b8b14a308fd1580e2a9c5  l06r/src/tests.rs
0e16242cdc3d19af1e2439433f45592933e79359f3c3c1e8f2c2572f3fad4b72  analyze.py
b275a1bc9feb496b5d0c7aff8234d345f351ee56aa9ce99898267a86405d1058  pilot/run_pilot.sh
```

## 2. Randomness (F2)

SplitMix64 exactly as in L06 (seed 0 gives `0xE220A8397B1DCDAF`, `0x6E789E6AA1B965F4`).
F2 adds a counter field `count`, incremented by 1 on every `next()` call, including
calls made inside `uniform` rejection loops and `bit()`.

```
TAG_L06      = 0x4C06_0000_0000_0000   (L06; used only by copied unit tests and G10)
TAG_DECISION = 0x4C26_0000_0000_0000   (census, --tag decision)
TAG_PILOT    = 0x4C36_0000_0000_0000   (--tag pilot)
TAG_GATE     = 0x4C46_0000_0000_0000   (G1 gate-enum, --tag gate)
curve_seed_t(tag, p, i) = tag ^ (p << 24) ^ i      assert p < 2^13 and i < 2^24
X_ORDER = 0xD1B5_4A32_D192_ED03   stage A stream: SplitMix64(curve_seed ^ X_ORDER)
X_RES   = 0x2545_F491_4F6C_DD1D   stage B stream: SplitMix64(curve_seed ^ X_RES)
X_G2    = 0x8CB9_2BA7_2F3D_8DD7   gate G2 stream: SplitMix64(curve_seed ^ X_G2)
coefficient stream: SplitMix64(curve_seed); c0..c6 = uniform(p) in that order
reference sampler: SplitMix64(0x4C06_5500_0000_0000 ^ (g << 32) ^ (l << 16) ^ m)  (L06, unchanged)
```
Stream position: `pos(s) = s * gamma^{-1} mod 2^64` with `gamma = 0x9E3779B97F4A7C15`.
A stream that made `n` calls occupies positions `pos + 1 .. pos + n`.

## 3. Base arithmetic (unchanged from L06 SPEC section 3)

Fields `F_p` with `p < 2^13`, Legendre table, Tonelli-Shanks, `nu` = the least
non-residue, deterministic Miller-Rabin (bases up to 37), polynomials as fixed arrays,
distinct-degree factorisation, `cycle_type` (degrees sorted descending plus a trailing
1), `k = len(degrees)`, and a squarefree test by `gcd(f, f')`.

## 4. chi for census curves

4.1-4.2 (unchanged): Cantor arithmetic on `y^2 = F(x)` (`F = f` for J, `F = nu f` for
the twist). `random_class` adds three random `F_p`-points, redraws on the identity, and
returns the identity after 100,000 failures.

Common quantities: `N1 = 1 + sum_x (1 + legendre(f(x)))`, `s1 = p + 1 - N1`,
`C = 1 - s1 (1 + p^2) + p^3`, `Ct = 1 + s1 (1 + p^2) + p^3`,
`B = 15p(p+1) + 20 isqrt(p^3) + 20`. `bsgs_all(F, D, L, R, s, cap = 4096)` returns all
`n` in `[L, R]`, `n = L mod s`, with `nD = 0`, and an over-cap flag (L06 4.3).

4.3 **Stage A** (`Force::Auto` only, stream X_ORDER): exactly L06 4.3-4.4. J: up to 4
`D1` (over cap means redraw), then 7 filter classes. Twist: `D'` plus 7 filter classes,
congruence `n' = 2 + 2p^3 - n (mod 2(p+1))`. F2 changes, both making an impossible
state into an instrument failure: an empty filtered J set gives `status = error`
(L06: `error` too), and an empty twist pair set gives `status = error` (L06 called it
ambiguous). If the J set has more than 16 elements, or the pair count is greater than
1, or a cap is hit, stage A returns "no result" and stage B runs. One pair `(nJ, nJtw)`
gives `s2 = ((nJ + nJtw)/2 - 1 - p^3)/(1 + p)` (assert exact) and
`s3 = (nJtw - nJ)/2 - s1 (1 + p^2)`, `resolved_by = "bsgs"`.

4.4 **Stage B** (F2; runs when stage A gives no result, or under `Force::B`; stream
X_RES):
```
N2 = n2_norm(f)                        (section 5.2)
s2 = (s1^2 - (p^2 + 1 - N2)) / 2       (assert exact)
center = C + s2 (1 + p); S = isqrt(400 p^3); L = center - S; R = center + S  (assert L > 0)
repeat up to 4 times: D = random_class(J); (cand, over) = bsgs_all(J, D, L, R, 1, 4096); stop if not over
dj = 8 x random_class(J); dt = 8 x random_class(twist)      (always drawn, after the D draws)
if a non-over cand exists:
   total = C + Ct + 2 s2 (1 + p)
   kept = [n in cand : n dj_k = 0 for all k, and ntw = total - n > 0 with ntw dt_k = 0 for all k]
   n_candidates_B = len(kept)
   len 1 -> nJ = kept[0], s3 = center - nJ, nJtw = total - nJ, resolved_by "n2"
   len 0 -> status "error" (the true order always survives)
   len > 1 -> stage C with cand_B = kept
else n_candidates_B = 4097 -> stage C with no candidate list
```

4.5 **Stage C** (F2; `p <= 1031` only, constant `STAGE_C_MAX_P`). If `p > 1031`, then
`status = "unresolved"` and the draw stops here.
```
N2 = n2_norm(f); s2 as above (assert it equals stage B's s2 if B ran)
N3 = n3_norm(f); s3 = (s2 s1 - s1 S2 + S3)/3 with S2 = p^2+1-N2, S3 = p^3+1-N3 (assert exact)
nJ = C + s2(1+p) - s3; nJtw = Ct + s2(1+p) + s3
if cand_B exists and nJ not in cand_B: status "error"
resolved_by "n3"
```

4.6 **Assembly** (all paths): assert `C + s2(1+p) - s3 == nJ` and
`Ct + s2(1+p) + s3 == nJtw`. `chi = [p^3, -p^2 s1, p s2, -s3, s2, -s1, 1]`.
`gate_weil` as in L06. **G2** (stream X_G2, used for nothing else): 3 `random_class(J)`
and then 3 `random_class(twist)`. `gate_annihilation` is true iff every class is
nonzero and is killed by `nJ` (respectively `nJtw`). (F2 adds the "nonzero" part.)
`gate_mod2`, `prime` and `c2` are as in L06. Draw counts `order`, `res` and `g2` are the
`count` of each stream (0 if the stream was not used).

Counter placement. The per-draw counters are `rng_draws.coef`, `.order`, `.res`, `.g2`,
`n_candidates_J`, `n_candidates_pair` and `n_candidates_B`. Each is set once per draw by
the code above. There is no operation counter, because no cost metric is measured.

## 5. Enumeration

5.1 (unchanged, `enumchi.rs`, used only by gate-enum): `F_{p^2} = F_p[t]/(t^2 - nu)`,
`F_{p^3} = F_p[t]/(t^3 + a1 t + a0)` with the first `(a1 outer from 0, a0 inner from 1)`
having no root in `F_p`, square tables, `N_k = 1 + sum (1 if f(x) = 0, 2 if square)`,
then Newton.

5.2 (F2, `enumnorm.rs`, used by stages B and C). The same fields and the same cubic
modulus rule, but no tables. The quadratic character of `w != 0` in `F_{p^k}` is
`legendre(Norm(w))` (`w` is a square iff its norm is a square in `F_p`, because the norm
is a surjective homomorphism of cyclic groups of even order). `Norm` in `F_{p^2}` is
`a0^2 - nu a1^2`. In `F_{p^3}` it is the determinant of the multiplication-by-`a` matrix
in the basis `1, t, t^2` (columns `a`, `a t`, `a t^2`, with
`(b0, b1, b2) t = (-c0 b2, b0 - c1 b2, b1)`). `n2_norm = 1 + sum over all p^2 x`;
`n3_norm = 1 + sum over all p^3 x`. Horner is evaluated in the extension.

## 6. Criterion mod l (unchanged, L06 SPEC 6)

`h^*[i] = h[d-i] q^(d-i) / h[0]`. The brute force over monic `h` (degree 3, `h[0] != 0`)
gives `fire`, `n_h` and `ext_deg` (least `k` with `T^k = 1 mod h`, minimum over valid
`h`, cap 20,000). The factor test gives `fire_fac` and `n_pairs`. The class is
fire, none or undetermined.

## 7. Reference sampler, 8. J[2] table (unchanged, L06 SPEC 7 and 8)

## 9. Output formats

`raw/census_p{P}.jsonl`, one line per draw:
- not squarefree: `{"p","i","block","status":"not_squarefree","rng_draws":{"coef"}}`
- P stream, reducible `f`: `{"p","i","block":"P","status":"skipped_reducible","cycle_type","rng_draws":{"coef"}}`
- otherwise: `{"p","i","block","status","f":[c0..c6],"cycle_type","k","n_candidates_J",
  "n_candidates_pair","n_candidates_B","resolved_by","rng_draws":{"coef","order","res","g2"}}`,
  and when `status = ok` also `"s1","s2","s3","nJ","nJtw","prime","c2","gate_weil",
  "gate_annihilation","gate_mod2","l2":{...},"crit":{"2","3","5","7"}` as in L06.
  `status` is one of `ok`, `unresolved` or `error`. `resolved_by` is one of `bsgs`,
  `n2`, `n3`, or `""`.
- final line: `{"type":"run_summary","tag","p","draws","stop_reason","u_count","prime_count","status_counts"}`.

`raw/gate_enum_p257_{auto,B,C}.jsonl`: `{"tag","force","p","i","N":[N1,N2,N3],
"chi_enum":[s1,s2,s3],"status","resolved_by","n_candidates_J","n_candidates_pair",
"n_candidates_B","chi_census":[..] or null,"agree":bool or null,"gate_annihilation",
"rng_draws":{...}}`.

`raw/gsp_reference.json` and `raw/l2_lagrangians.json`: as L06 (the code is unchanged).
`lagr2` output is byte-identical to L06 `raw/l2_lagrangians.json` (checked in the
pilot).

## 10. CLI

```
l06r lagr2 --out PATH
l06r gsp --samples N --out PATH
l06r gate-enum --tag {decision|gate|pilot|l06} --p P --count K --force {auto|B|C} --out PATH
l06r census --tag {decision|gate|pilot|l06} --p P --u-count U --prime-target T --draw-cap C --out PATH
```
Census loop (unchanged): block U while fewer than `u_count` squarefree draws have been
seen (counting the current one), then P. The loop stops after the draw at which
`prime_count == prime_target`, or at index `draw_cap - 1`.

## 11. Unit tests (G0: all 15 must pass)

Tests 1-11 are L06's (L06 SPEC 11), unchanged except that test 6 calls
`chi_search(..., Force::Auto)`. Tests 1-11 use the L06 tag through `curve_f` and
`curve_seed`, which wrap `TAG_L06`.

12. `t12_enumnorm_vs_tables`: `n2_norm`, `n3_norm` equal the table counts of `enumchi`
    for `x^7 + 1` over `F_5`, and for the first 10 squarefree L06-tag curves over each of
    `F_11`, `F_13`, `F_101`.
13. `t13_cascade_p101_all_paths`: for the first 20 squarefree L06-tag curves over
    `F_101` (L06's G1 instances, where L06 had 8 of 200 ambiguous), each of
    `Force::Auto`, `B`, `C` gives `status = ok` with `(s1, s2, s3) == chi_enum`, G2 and
    Weil true, and `C` gives `resolved_by = n3`.
14. `t14_l06_ambiguous_p257_resolved`: L06 census draws `p = 257, i = 18` and `i = 253`
    (L06 status `ambiguous`, cycle type `[2,1,1,1,1,1,1]`). Auto cascade: `status = ok`,
    `resolved_by != bsgs`, and `(s1, s2, s3) = (-6, 175, 2220)` and `(-22, 431, -4532)`
    respectively, equal to `chi_enum`.
15. `t15_seed_tags`: the new tags differ from `TAG_L06` in bits 48-63;
    `curve_seed_t(tag, 4099, 2^24 - 1) >> 48 == tag >> 48`; `curve_f_t(TAG_L06, ...)`
    equals `curve_f`; the coefficient stream count is at least 7; and one `next()` plus
    one `uniform(10)` from seed 5 gives `count = 2`.

## 12. analyze.py (F2)

`python3 -I analyze.py raw > raw/summary.json`. Options are for the pilot only:
`--census-prefix`, `--gate-prefix`, `--tag`, `--g1-counts auto=..,B=..,C=..`,
`--gates-only` (prints gates and no criterion statistic), and `--l06-dir`. The default
is L06's path.
- G0: `test.log` contains `test result: ok. 15 passed` and no `FAILED`/`panicked`.
- G1: per force file, the expected count (auto 200, B 40, C 20), all `ok`, all
  `agree`, no annihilation failure, every tag `gate`, C all `n3`, B no `bsgs`.
- G2-G7: as L06 (G2/G3/G5 on all `ok` rows of the five census files).
- G8: `error` total 0; `unresolved` 0 at `p <= 1031`; every run has a `run_summary`.
  Unresolved counts at 2053 and 4099 are listed in `unresolved_flag_p`.
- G9: the interval-disjointness test of section 2 over all streams of the five census
  files (tag decision) and the three G1 files (tag gate). A stream with the same
  `(tag, p, i, kind)` in several G1 files counts once (it is the same stream). Sort the
  intervals, then check `start_j > end_{j-1}` and the wrap past `2^64`. Each recorded
  `f` must equal the regenerated `f`.
- G10: L06 census `draws` per p from L06 `raw/census_p*.jsonl` run summaries, and L06
  gate-enum index ranges at p = 101 and 257. Regenerate L06 seeds and `f`, then count
  collisions with this run's seeds and `(p, f)`, and count duplicate `f` within each
  census.
- Populations: L06's rules plus the dedup by `f` (keep the smaller `i`).
- Criterion tables, A2, S4: as L06. S5: section 7 of PROTOCOL.md, with the
  recomputation of valid `h` over `F_3` (brute force over 18 monic cubics, `h(0) != 0`).
- Verdict: PROTOCOL.md section 6. The key is `verdict`.

Wilson (`z = 1.959964`): `center = (ph + z^2/2n)/(1 + z^2/n)`,
`half = z sqrt(ph(1-ph)/n + z^2/4n^2)/(1 + z^2/n)`.

## 13. Provenance of the unchanged parts

L06 `SPEC.md` sha256 is listed in L06 `frozen.sha256`. Sections 3, 4.1-4.2, 5.1, 6, 7
and 8 of that file govern the unchanged modules. Their code is byte-identical to the L06
copies hashed in `COPIED_FROM_L06.sha256`.
