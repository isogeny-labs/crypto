# Is rho.signed_frobenius_strong the right matched reference on small Koblitz curves? (2026-11-15) `[pcc:unverified]`

Reference audit and recommendation. No file under `docs/ic/`, `scripts/build_ic_leaderboard.py`, `boundary_targets.json` or `src/` was edited. Every number below is `[pcc:unverified]`: counted GAE from one local ecbench build, L0, no pcc claim yet. Frozen protocol: [`PROTOCOL.md`](PROTOCOL.md) (hashes in `REGISTRATION.sha256`, written before any measured data; addendum A1 adds n41/n53 and is dated in the file). Data: `results.json`, `tables/`, `sessions/`, `audits/`, figure `figures/fig1-reference-ratio.{svg,png}`, [`REPORT.pdf`](REPORT.pdf).

## Verdict

1. **v5 is a sound, fully charged, single-target-cold method.** All 7 frozen sessions (48 targets x 3 rounds + warm-up; 9600 records) verified 100% and `ecbench verify --replay-all` reproduced every sampled replay (0 problems); both A/A pairs are exactly 1.000. It uses no known-log table, no precomputation outside the run and nothing that depends on the target except through the start stride. It has one bookkeeping gap against strong: it does not tally the per-step jump-index selection that strong tallies as `partition_hashes_uncharged` (section 1).
2. **The strong reference overstates single-target rho cost on small curves.** v5 / strong at w = 0 is 0.079 (log2 r 15.4), 0.093 (16.0), 0.168 (20.5), 0.261 (22.0), 0.581 (27.8), 0.939 (39.0), 0.962 (44.3): monotone in log2 r, 12x at the smallest size, parity (interval contains 1 at w = 0) from about 2^39. The cause is strong's set-up: 32 full-size scalar multiplications charged 1.5 log2 r additions each (738 of 932 GAE on K1 n29 (the validator quoted 943); 76-79% of charged cost at log2 r <= 16, 57% at 22, 32% at 27.8, 2% at 39); v5 builds a weight-2 table from shared ladders (23 GAE on n29).
3. **Distance to rho on the leaderboard's small Koblitz rows is understated.** Best validated IC arm (generator-orbit layout) / rho, w = 0.1: K1 n29 0.142 vs strong but **1.69** vs v5 (so the "IC below rho at every band weight" reading of the n29 gen-orbit row does not survive v5); n23 1.005 vs 3.74; n17 0.49 vs 4.99; n31 0.58 vs 3.28; n37 2.94 vs 5.19. K1 recipe rows: n23 54x vs 202x, n29 766x vs 9134x.
4. **Recommendation (owner decision, section 5):** keep strong as the large-r boundary reference, and for r below about 2^38 also report the distance against v5 with its jump-index selection tallied, as the matched reference; until then quote small-curve rows against both.

## 1. Audit (code read, plus measured counters)

| | `rho.signed_frobenius_strong` (`koblitz_strong_rho::StrongRho::solve`) | `rho.signed_frobenius_v5` (`ic_boundary::signed_frobenius_rho_v3(RhoV3::v6(true))`) |
|:--|:--|:--|
| walk | signed-Frobenius classes (A = 2n), r-adding walk, 32 jumps `[s]G`, 32 lockstep lanes, batched inversion, normal-basis canonical form | same classes (A = 2n), `k = round(sqrt(F/(weight-1)))` jumps (6 at n29, 747 at n53) of weight-2 ladder sums, tuned walk with look-ahead and cycle window, polynomial-basis canonicalisation |
| target dependence | every walk starts at `cG + Q` (`b = 1`), jumps are multiples of G only | jumps are multiples of G only (`g_only`); Q enters only through the start stride (`[k]T`, stride `T`), as in Kuhn-Struik and Bernstein-Lange single-target walks |
| set-up | `jumps()`: 32 independent scalar multiplications, each **charged 1.5 log2 r additions** (convention of `signed_frobenius_rho`), plus 2 for the start stride: 738 GAE at n29, 2124 at n53 | G ladder (`tg = min(2t, bits-1)` doublings, counted) and jump sums (counted additions): 22.6 GAE at n29, 777 at n53 (750 additions, 27 doublings) |
| verification `[d]G = Q` | one scalar multiplication charged in the search phase | read from the G ladder, charged (`verification_operations`, 7.3 at n29, 34.5 at n53) |
| precomputation outside the run, known-log table, target-dependent information | none: `solve(q, jumps, ...)`; the jump table is rebuilt for every target inside the timed solve and charged | none: the table is rebuilt per run from the run seed, inside `rho_walk_v3`, and every ladder and jump operation is in the `setup` phase |
| start multiples `j * stride` | `lanes - 1 = 31` additions left out of the count (`charge_start_multiples` is off by default; charged in `_v2`) | charged (`start_additions`) |
| natives, priced at w in the band | canonicalisations, **partition hashes (1 per step)**, DP table queries and inserts (DP bits 4: one probe per ~16 steps) | canonicalisations (1 per step), DP table queries and inserts (DP bits `round(0.5 log2 F - 0.8)`, 2 at n29, 9 at n53); **the jump-index selection is not tallied** |
| median steps / `sqrt(pi r / 2A)` | 0.91 to 2.2 (above 1 at small r: 32 lanes and DP bits 4 abandon and restart many short walks) | 0.82 to 1.00 |

Findings. (i) v5 does not leak or assume the target and needs no log table; "jumps in G only" is the standard target-independent walk and strong has the same property. (ii) Unpriced work, strong side: 31 start additions not counted (negligible), and the 32-scalar-multiplication set-up priced at 1.5 log2 r per multiplication, which is an upper-bound convention (the same walk with a fixed-base table, `rho.signed_frobenius_strong_v2`, sets up for 66 GAE at n29 and costs 0.22-0.37 of strong at w = 0.1 up to log2 r 22, 0.75 at 27.8, but 1.08-1.12x at 39 and 44.3). (iii) Unpriced work, v5 side: the per-step jump-index selection; with one extra native operation per walk step charged to v5 (derived from recorded `walk_operations`, a point estimate, not a measurement) v5 / strong at w = 1 moves from 0.63 to 0.95 at n53 and 0.50 to 0.74 at n37, but only from 0.12 to 0.15 at n29. So the advantage below 2^28 is robust to the missing tally; the apparent advantage at w > 0 for r >= 2^39 is mostly this bookkeeping gap. (iv) At w = 0 and r >= 2^39 the two references coincide (v5 / strong 0.94 [0.84, 1.05] at n41, 0.96 [0.86, 1.08] at n53).

History. The repository clone is shallow (3 commits), so `git log -S` is unavailable; the record is in the documents. `research/notes/index-calculus/RESEARCH_FIXED_REFERENCE_RHO_20261001.md`: the earlier fixture rho stored every step, scanned O(n) for canonical forms and inverted per step, and cost 43x the instructions and 60x the wall time of a strong walk at n = 53 (PR #1090); the fix was a strong single-target walk with distinguished points, normal-basis canonical form and batched inversion. `docs/ic/boundary_targets.json` `rho_reference_minimum` (since 2026-10-01) names that backend and says another rho is admissible "if measured per-step cost (instructions or ns per step, same target) no worse than the strong reference". The rule is about per-step cost, and its evidence note measures n = 37 to 59; it says nothing about set-up, which is what separates the two at small r. v5 (`benchmarks/20261107-rho-v5/`, 2026-11-07) came later and is a registered method in the same binary. The validation gate (`docs/VALIDATION_GATE.md`) requires "the strongest available for that curve": on the curves below 2^28 strong is not that.

## 2. Leaderboard rows that use a Koblitz rho reference

Read from `docs/ic/LEADERBOARD.md` (line numbers of that file at this commit), `scripts/build_ic_leaderboard.py` and `docs/ic/boundary_targets.json`.

| section (LEADERBOARD.md line) | Koblitz curves (log2 r) | rho reference as built | standing | affected here |
|:--|:--|:--|:--|:--|
| K1 Koblitz, one target, pair-table factor base sized for n (192; builder 1432-1504) | K1 n23 `icv1-f2m23-tm5197-1f85e9e1` (22.00), K1 n29 `icv1-f2m29-tm40309-30c52b96` (15.37) | `rho.signed_frobenius_strong`, plain method, band w = 0/0.1/1; gap 5.59/5.66/5.95 and 9.47/9.52/9.86 bits | validated (VAL-20261114-d9190c) | yes, both |
| Orbit-layout table and generator-orbit base (120; builder 1108-1129) | n13 (11.0), n29 (15.4), n17 (16.0), n19 (17.0), n31 (20.5), n23 a=0 (21.0), n37 (27.8) | `rho-frob` = `rho.signed_frobenius_full` (tuned walk, host prices); `rho-strong_full` also run; text: "at n = 29 the IC arm is below rho-frob at every band weight" | validated | yes (section 4: rho-frob is itself 0.26-0.40x strong at n <= 31) |
| Lazy orbit-member index (101) | n13, n29, n17, n19, n31, n23 a=0, n37, n41 (11.0 to 39.0) | `rho.signed_frobenius_strong_full` | unvalidated (pre-gate) | yes below 2^28 |
| Two orbit columns under the lazy oracle (138) | n13, n17, n23 a=0, n29, n37 | `rho-strong_full` and `rho-frob` | unvalidated (pre-gate) | yes below 2^28 |
| Progress table rows 2026-10-15, 10-18 (x2), 10-03 (n17, 4.3x), 10-04 (n43) (11-37) | n29 (15.4), n17 (16.0), n43 (32.1) | `rho-frob` / strong per their studies (not re-read here) | pre-gate / per row | n29 and n17 yes |
| Frontier audit: canonical medium-tier Koblitz cold operations (94) | 6 medium sizes, `ECBND1h05a2ea918f3c`, 1.089x floor | `rho.signed_frobenius_strong` | standing boundary, not an IC row | set-up share at those sizes not checked here |
| F6-IC E_0 ladder (490), F4/F5/F6 cross-family (500), n37 diagnostic (470), n41/n53 panel (511) | m13..m37 K1/E_0 curves, n37, n41 (39.0), n53 (44.3) | `rho.signed_frobenius_strong`, S lower bound | not ranked rows | small m yes; n41/n53 no (parity at w = 0) |
| Table A, one target (228) | n41-n61 (36.6 to 47.2) | clock-unit one-target rho "as built" (rule v8/v9, batched canonicalisation) | validated with caveats (`rho_reference_ok=partial`) | no: a different rho, and r >= 2^36.6 where strong and v5 coincide at w = 0 |
| Table C, 32-target batch (358) | n19 (18.0) to n61 (47.2) | batch rho on the same 32 targets, canonical step | pre-gate | no: set-up is amortised over 32 targets |
| N83 Tier 1 sections (1074, 1202) | n83 | same-point rho online wall | not S | no |
| `research/koblitz_ksum_fold_study_20261115` | n23, n29 | strong (and v5 in the validator) | validation failed (D1, D2); no leaderboard row | its D2 is confirmed here |

## 3. Measurement

Binary `target/release/ecbench` sha256 `805e3238c3d1f8aada4e0448676a265dfa15089b7e91ee7d9a36534c00a1c478` (built 2026-11-16; the tracked sources it reads are the same as at HEAD 29c8e1dbe0 as far as the shallow checkout can show; the working tree carries unrelated uncommitted edits in `f4_fp.rs` and `pq_f4_f2.rs` that no measured method reads). Local virtual machine, `--cpus none --allow-busy` (L0), 4 logical cores, run under the shared CPU lock; counted GAE are deterministic, wall time is not used. Per curve one session of 48 public targets x 3 rounds (+1 warm-up), arms in `PROTOCOL.md`. `ecbench verify --replay-all` OK on all seven sessions. Ratios are paired by target, cluster-bootstrap 95% (`ecbench compare --weight w`, 4000 resamples, seed 20261002).

### v5 against strong, by size (R2)

| curve | log2 r | v5 / strong at w = 0 | w = 0.1 | w = 1 | v5 with jump index counted / strong (derived, point) | strong_v2 / strong (w = 0.1) | A/A strong, A/A v5 |
|:--|--:|:--|:--|:--|:--|:--|:--|
| n29 | 15.37 | 0.079 [0.075, 0.083] | 0.084 [0.080, 0.088] | 0.119 [0.111, 0.128] | 0.079 / 0.087 / 0.149 | 0.239 [0.223, 0.255] | 1.000, 1.000 |
| n17 | 16.00 | 0.093 [0.089, 0.097] | 0.098 [0.094, 0.103] | 0.137 [0.129, 0.145] | 0.093 / 0.103 / 0.174 | 0.221 [0.214, 0.229] | 1.000, 1.000 |
| n31 | 20.46 | 0.168 [0.157, 0.180] | 0.176 [0.164, 0.188] | 0.221 [0.204, 0.240] | 0.168 / 0.187 / 0.302 | 0.304 [0.287, 0.322] | 1.000, 1.000 |
| n23 | 22.00 | 0.261 [0.240, 0.284] | 0.269 [0.247, 0.293] | 0.310 [0.282, 0.341] | 0.261 / 0.288 / 0.428 | 0.371 [0.349, 0.395] | 1.000, 1.000 |
| n37 | 27.78 | 0.581 [0.534, 0.632] | 0.565 [0.519, 0.616] | 0.504 [0.458, 0.554] | 0.581 / 0.613 / 0.737 | 0.754 [0.695, 0.818] | 1.000, 1.000 |
| n41 | 39.00 | 0.939 [0.838, 1.052] | 0.859 [0.766, 0.963] | 0.622 [0.555, 0.698] | 0.939 / 0.937 / 0.931 | 1.081 [0.972, 1.203] | 1.000, 1.000 |
| n53 | 44.26 | 0.962 [0.855, 1.082] | 0.878 [0.780, 0.988] | 0.631 [0.560, 0.710] | 0.962 / 0.957 / 0.945 | 1.119 [0.999, 1.255] | 1.000, 1.000 |

Mean S per arm (`ecbench band`, w = 0 / 0.1 / 1) and set-up share:

| curve | log2 r | strong S (w=0/0.1/1) | v5 S | strong_v2 S | rho-neg S | strong set-up share of charged | v5 set-up share | median steps/ideal strong, v5 |
|:--|--:|:--|:--|:--|:--|--:|--:|:--|
| n29 `icv1-f2m29-tm40309-30c52b96` | 15.37 | 4.522 / 4.630 / 5.608 | 0.357 / 0.388 / 0.669 | 1.029 / 1.104 / 1.782 | 1.921 / 2.039 / 3.101 | 0.79 | 0.31 | 1.84, 1.00 |
| n17 `icv1-f2m17-tm101-00378d4e` | 16.00 | 3.964 / 4.091 / 5.239 | 0.368 / 0.403 / 0.716 | 0.847 / 0.906 / 1.432 | 1.838 / 1.961 / 3.074 | 0.76 | 0.26 | 2.20, 0.84 |
| n31 `icv1-f2m31-tm90707-c95f16f5` | 20.46 | 1.211 / 1.274 / 1.836 | 0.204 / 0.224 / 0.406 | 0.347 / 0.387 / 0.744 | 1.237 / 1.346 / 2.333 | 0.68 | 0.15 | 1.71, 0.82 |
| n23 `icv1-f2m23-tm5197-1f85e9e1` | 22.00 | 0.903 / 0.971 / 1.586 | 0.236 / 0.261 / 0.492 | 0.316 / 0.361 / 0.762 | 1.131 / 1.226 / 2.084 | 0.57 | 0.09 | 1.60, 0.87 |
| n37 `icv1-f2m37-tm534059-32aad96b` | 27.78 | 0.273 / 0.309 / 0.636 | 0.159 / 0.175 / 0.321 | 0.197 / 0.233 / 0.560 | 1.019 / 1.119 / 2.013 | 0.32 | 0.03 | 1.02, 0.99 |
| n41 `icv1-f2m41-tm2308219-7f48b14a` | 39.00 | 0.147 / 0.176 / 0.443 | 0.138 / 0.151 / 0.276 | 0.159 / 0.191 / 0.475 | 0.978 / 1.076 / 1.955 | 0.02 | 0.00 | 0.96, 0.89 |
| n53 `icv1-f2m53-tm56619371-dac20a85` | 44.26 | 0.128 / 0.154 / 0.390 | 0.123 / 0.135 / 0.246 | 0.143 / 0.172 / 0.430 | 1.013 / 1.114 / 2.025 | 0.00 | 0.00 | 0.91, 0.97 |

Descriptive fits (seven points, a saturating curve, so a poor linear model): ln(v5 / strong) = w = 0: -3.56 + 0.088 log2 r (residual sd 0.32); w = 0.1: -3.41 + 0.082 log2 r (residual sd 0.31); w = 1: -2.70 + 0.057 log2 r (residual sd 0.26). The table is the result: the ratio is 0.08-0.26 up to log2 r 22, 0.58 at 27.8 and 0.94-0.96 from 39 (w = 0); it reaches 0.5 near log2 r 26 and 0.9 near 38 by linear interpolation of ln ratio.

### Distance to rho under each reference (R3)

IC arms: generator-orbit layout (`gen-c<c>`, validated 2026-10-22; n23 c = 3 and 4, c = 4 is the validator's pre-declared test) and the K1 recipe. "below" means the 95% upper end is under 1, "above" the lower end is over 1.

| curve | log2 r | IC arm | S(w=0/0.1/1) | IC / strong | IC / v5 | order vs strong | order vs v5 |
|:--|--:|:--|:--|:--|:--|:--|:--|
| n29 | 15.37 | gen-c1 | 0.40 / 0.66 / 2.99 | 0.088 [0.078, 0.099]; 0.142 [0.130, 0.155]; 0.533 [0.504, 0.563] | 1.114 [0.977, 1.270]; 1.691 [1.526, 1.874]; 4.466 [4.126, 4.833] | below at all three | order flips: unresolved/above/above |
| n29 | 15.37 | k1-l10 | 3223.04 / 3547.27 / 6465.33 | 712.802 [674.451, 753.334]; 766.104 [724.851, 809.704]; 1152.894 [1088.767, 1220.797] | 9023.579 [8421.432, 9668.781]; 9134.097 [8512.079, 9801.569]; 9665.203 [8938.272, 10451.255] | above at all three | above at all three |
| n17 | 16.00 | gen-c1 | 1.61 / 2.01 / 5.61 | 0.406 [0.293, 0.561]; 0.491 [0.360, 0.670]; 1.070 [0.813, 1.409] | 4.370 [3.169, 6.027]; 4.985 [3.661, 6.787]; 7.827 [5.939, 10.315] | order flips: below/below/unresolved | above at all three |
| n31 | 20.46 | gen-c2 | 0.56 / 0.74 / 2.29 | 0.464 [0.400, 0.539]; 0.577 [0.503, 0.663]; 1.249 [1.112, 1.402] | 2.762 [2.333, 3.270]; 3.284 [2.804, 3.847]; 5.643 [4.922, 6.469] | order flips: below/below/above | above at all three |
| n23 | 22.00 | gen-c3 | 0.81 / 1.03 / 2.99 | 0.897 [0.788, 1.021]; 1.058 [0.936, 1.196]; 1.883 [1.691, 2.097] | 3.436 [2.995, 3.944]; 3.933 [3.445, 4.489]; 6.072 [5.381, 6.852] | order flips: unresolved/unresolved/above | above at all three |
| n23 | 22.00 | gen-c4 | 0.75 / 0.98 / 2.97 | 0.835 [0.740, 0.942]; 1.005 [0.898, 1.125]; 1.876 [1.698, 2.073] | 3.198 [2.802, 3.649]; 3.735 [3.294, 4.235]; 6.050 [5.399, 6.778] | order flips: below/unresolved/above | above at all three |
| n23 | 22.00 | k1-l9 | 43.81 / 52.82 / 133.86 | 48.530 [47.393, 49.693]; 54.389 [52.923, 55.895]; 84.414 [80.656, 88.347] | 185.906 [171.702, 201.285]; 202.147 [186.439, 219.178]; 272.199 [249.473, 296.995] | above at all three | above at all three |
| n37 | 27.78 | gen-c8 | 0.73 / 0.91 / 2.50 | 2.679 [2.343, 3.063]; 2.936 [2.573, 3.350]; 3.930 [3.451, 4.474] | 4.610 [4.034, 5.268]; 5.194 [4.559, 5.916]; 7.794 [6.898, 8.807] | above at all three | above at all three |

### Interpolated multiplier for the leaderboard rows (derived, not measured)

The rows of section 2 were not re-run. Dividing the reported distance by the interpolated v5 / strong ratio is an approximation (different curves and seeds; a=0 n23 was not measured; below 2^15.4 nothing was measured):

| leaderboard section | curve | log2 r | v5 / strong at this r (w=0; w=1), interpolated in ln ratio | distance under v5 = distance under strong x |
|:--|:--|--:|:--|:--|
| K1 pair-table sized for n (validated) | icv1-f2m23-tm5197-1f85e9e1 (a=1 n23) | 22.00 | 0.261; 0.310 | 3.83; 3.23 |
| K1 pair-table sized for n (validated) | icv1-f2m29-tm40309-30c52b96 | 15.37 | outside the measured range (2^15.4 to 2^44.3); not extrapolated | - |
| gen-orbit (validated), lazy, two-columns, Frobenius-orbit rows | icv1-f2m13-t181-515ee569 | 11.00 | outside the measured range (2^15.4 to 2^44.3); not extrapolated | - |
| same | icv1-f2m17-tm101-00378d4e | 16.00 | 0.093; 0.137 | 10.77; 7.31 |
| same | icv1-f2m19-t797-b6cf2467 | 17.00 | 0.106; 0.152 | 9.43; 6.57 |
| same | icv1-f2m31-tm90707-c95f16f5 | 20.50 | 0.170; 0.223 | 5.87; 4.48 |
| same | icv1-f2m23-t5197-69e76b73 (a=0 n23) | 21.00 | 0.196; 0.249 | 5.09; 4.01 |
| same | icv1-f2m37-tm534059-32aad96b | 27.80 | 0.582; 0.504 | 1.72; 1.98 |
| lazy orbit; n41/n53 panel; F4/F5/F6 ladder | icv1-f2m41-tm2308219-7f48b14a | 39.00 | 0.939; 0.622 | 1.07; 1.61 |
| n41/n53 panel | icv1-f2m53-tm56619371-dac20a85 | 44.26 | outside the measured range (2^15.4 to 2^44.3); not extrapolated | - |

### Post hoc supplement: the reference the validated gen-orbit section used

`rho.signed_frobenius_full` (rho-frob, host-priced and flat in w) and `rho.signed_frobenius` (tuned walk, band-priced) on the same targets, written after the main data (`make_supp.py`, `specs_supp/`, not frozen). The binary is the same sha.

| curve | rho-frob S (host-priced, flat in w) | rho-frob / strong (w=0, 0.1, 1) | rho-frob / v5 (w=0, 0.1, 1) | rho.signed_frobenius (tuned, plain) / strong (w=0) |
|:--|--:|:--|:--|:--|
| n17 | 1.501 | 0.380 / 0.369 / 0.289 | 3.867 / 3.526 / 1.965 | 0.357 |
| n23 | no price row for this curve in full_prices.json (arm errors, 192/192) | n/a | n/a | 0.473 |
| n29 | 1.475 | 0.325 / 0.317 / 0.261 | 4.313 / 3.975 / 2.332 | 0.312 |
| n31 | 0.491 | 0.403 / 0.383 / 0.264 | 2.202 / 1.999 / 1.094 | 0.382 |
| n37 | 0.210 | 0.778 / 0.687 / 0.335 | 1.315 / 1.193 / 0.651 | 0.731 |

At n23 a=1 `rho.signed_frobenius_full` has no price row (`full_prices.json`), so the arm errors on all 192 runs (recorded, not a finding about the method).

## 4. Reads against the protocol

R1 soundness: holds. 100% verified, replays reproduced, A/A 1.000 for both pairs; v5 median steps/ideal 0.82-1.00 (a walk-length inflation flag would be above 1.5). R2: tabulated above. R3: tabulated above. R4: v5 / strong < 1 with an interval excluding 1 at all three weights on n17, n23, n29, n31, n37 and, at w = 0.1 and 1 only, on n41 and n53. The second clause (no unpriced work on the v5 side that strong lacks) fails for the jump-index selection, so by the pre-declared rule v5 is recommended where the advantage survives that gap (point estimates below 1 at every weight for n <= 37), and not as a replacement at n41/n53, where the w > 0 advantage is the bookkeeping gap.

## 5. Recommendation (owner decision; not made here)

- For K1 n23 / n29 and every row below about log2 r 28: report the distance remaining against both strong and v5. On the K1 sized rows the gap grows from 5.8 / 9.6 bits (strong, w = 0.1) to 7.7 / 13.2 bits (v5); on n29 the generator-orbit IC arm moves from 0.14x strong to 1.69x v5 (at w = 0.1; 4.47x at w = 1, 1.11 [0.98, 1.27] at w = 0).
- Whether to make v5 (or `min(strong, v5)`) the stated matched reference below 2^38 is the owner's call. Arguments for: the validation gate asks for the strongest available reference; v5 is a registered, replayable, same-binary method that is 3.8x to 12.7x cheaper at r <= 2^22; strong's 1.5 log2 r per set-up scalar multiplication is a convention, not a measured cost. Arguments against: strong is the name written into `boundary_targets.json` since 2026-10-01 and into every validated row; v5 as registered lacks the per-step jump-index tally; v5 is not a strong-style reference at n >= 41 where they coincide.
- If v5 is adopted, first add its jump-index tally (one native operation per step, as strong's `partition_hashes_uncharged`) so that its band-priced S is comparable at w > 0; the derived sensitivity above shows the effect is large only for r >= 2^37.
- Do not reword "relabelling and accounting" as improvement: moving the reference changes the distance remaining, not any IC cost.

## 6. Errors, deviations, caveats

- Experiment run locally, not on EKS Spot, because the relayed task asked for a small local measurement (CLAUDE.md names EKS Spot as the default for authorised experiments; this was a local reference check of 7 sessions that ran in minutes). Resume-checkpoint push failed (`scripts/checkpoint.sh gate`: S3 upload token expired 2026-11-05), so `.resume/RESUME.md` is written but not in S3; the launcher hooks did not refuse the runs. `conductor` is not installed. The research knowledge base was not queried (no `research-kb` binary/server in this environment).
- The binary's sources could not be diffed against HEAD (shallow clone); the sha is recorded and the sessions' `git_dirty` is true.
- n41 and n53 (addendum A1) and the supplement were added after the first three sessions had run; they do not change R1, R3 or R4. IC arms were not run at n41/n53 (no validated column count).
- Derived quantities (jump-index sensitivity, interpolated multipliers, fits) are computed from recorded counters or ratios and are labelled derived; they are not measurements.
