# Validator report: `research/g3_affine_twist_20261121` (affine-type (2,2,2) codomains, twist test)

Date 2026-11-21. Validator: Claude Opus 5.5 (`validator` role), single-threaded on a shared host.
Standing of the study's numbers is unchanged by this report: `[pcc:unverified]` (no pcc claim
filed). This report is a validation of the committed artifacts, not a claim.

## Disposition

**validated-with-errata.** (In the role's vocabulary: `validated`. No headline number
changes.) Every count, rate and interval in `RESULTS.md` reproduces from the raw rows. An
independent F_p implementation reproduces every sampled stored value exactly: 58/58 main
evaluations and 56/56 control kernels. The positive control is a genuine out-of-sample test
for the pair-type chain, not an agreement built in by construction. An independent theta-series
check shows one constant links `prod theta_B` and `S_L` for all 64 formulas, pair and affine
alike, so the control's `kappa_p` transfers to the affine arm. The errata are F5, F11, F13 and
F14. F15 is a repository-policy item. None of them alters 803/753/44, r = 0.516 or
coverage 0.758.

## Artifact set validated

- Commit `0340c083905b5ab4a795fa4a2bc300ed375837f2` (branch `claude/hyperelliptic-index-calculus-td1fdn`),
  all tracked files under `research/g3_affine_twist_20261121/`, plus the frozen
  `research/g3_transfer_gates_20261120/{census.py,theta_arm.py}`.
- HEAD moved during validation to `2a765f254a0dbc07a95b3cf6a9fee53766dd8f2d` (another agent's
  "WIP snapshot: quartic descent fix"). `git diff --stat 0340c08 2a765f2` is empty for every path
  above, and so is the working tree against `0340c08`. The only untracked path is this `validation/` directory.
- `sha256sum -c frozen.sha256`: 73/73 OK. `sha256sum -c data.sha256`: 7/7 OK.
- Environment: validator scripts ran on system Python 3.13.18 with numpy 2.5.3. The producer's
  replay used the producer's venv `scratchpad/venv_g3`: Python 3.13.18, mpmath 1.4.1. The study
  itself does not record its environment.

## Requirement table

| # | Requirement | Result | Evidence |
|---|---|---|---|
| R1 | Frozen hashes match | pass | `frozen.sha256` 73/73, `data.sha256` 7/7 |
| R2 | Freeze before data; D1, D2 honest | pass (mtime evidence only; see F1-F3) | file mtimes; `data/run_failed_launch.log`; scratch `twist/dev_main.jsonl` (p=13 only) |
| R3 | Nothing decided after seeing main data | pass, with note F2 (`analyze.py` unfrozen) | `analyze.py` mtime 00:22:28 is before the first main row (00:25:05) |
| R4 | Control not circular | pass for the pair chain; affine transfer supported by F7 | F4, F7; `validation/retabulate.py` |
| R5 | Independent recompute, ≥40 codomains and ≥40 control kernels | pass: 56 codomains (58 evals) and 56 kernels, all exact | `validation/recompute_fp.py`, `.out.jsonl`, `.log` |
| R6 | Exactly 2 stable affine kernels; independence | pass | F8, F9; `validation/retabulate.py` |
| R7 | Statistics (Wilson, 0.758) | pass | F10 |
| R8 | Literature criterion as used | pass with stated dependencies; see F12 | F7, F12, F13 |
| R9 | Reporting contract | not applicable (no ECDLP cost comparison, no §5a band, no rho row); labels honest | F16 |
| R10 | Deterministic replay of the producer pipeline | pass | F16 |
| R11 | Repository policy (artifact store, visuals) | fail (process only) | F15 |

## Findings

**F1. Freeze timeline (pass; mtime evidence only).** A single commit holds the protocol,
data and results, so git cannot order them. The mtimes do. Formulas were derived
00:13:31–00:17:13 (scratch logs). `run.py` and `formulas/` date from 00:19:44, `PROTOCOL.md`
from 00:20:53. The failed launch ran at 00:20:59. `run_all.sh` and `frozen.sha256` date from
00:21:05. The control ran 00:21:05–00:25:05 and the main arm 00:25:05–00:43:49.
`summary.json`, `data.sha256` and `RESULTS.md` follow, ending at 00:44:59. Every frozen file
except `run_all.sh` predates `PROTOCOL.md`. The two lead-#3 files are also git-evidenced: they
were committed in `982eb05c` on 2026-11-20. The pre-D1 `frozen.sha256` was overwritten, so
"no other change" (D1) rests on those mtimes. The surviving scratch (`scratchpad/twist/`) holds
64 formula JSONs, byte-identical to the committed ones, plus logs and 6 development rows at
p = 13 only. No surviving file shows protocol-prime data before the freeze. Deleted files could
not be detected.

**F2. `analyze.py` was not frozen (note).** It was last modified at 00:22:28, after the freeze
and while the control was running. Control data for p = 17 and 19 existed at that time. No
main-arm data existed yet. Its rules match `PROTOCOL.md` exactly: n ≥ 50 and 100%/0% agreement;
the exclusions (no map, inconsistent Smith symbol, `S_L = 0`); the u ∈ {1, −1, 2, −2} pattern
test; and the statistic `kappa_p chi(c) chi(S_L)`. Rerun on a copy of the data, it regenerates
`data/summary.json` byte-identically. I found no evidence that anything was decided after the
main data. The analysis script belonged in `frozen.sha256`.

**F3. D1 and D2 are honest (pass).** `data/run_failed_launch.log` shows only
`ModuleNotFoundError: No module named 'fpeval'` for every job. That fits `python -I`, which
implies `-P` and so drops the script directory from the path. The frozen `run.py` docstring
still says `python -I`, consistent with only `run_all.sh` having changed. D2: the development
rows are at p = 13, which is not a protocol prime.

**F4. Circularity (Q2): the control is a genuine test of the pair-type chain.**
- Parameters fitted on protocol-prime control data: 8 sign bits (`kappa_p`, one per prime). The
  preregistration constrains them to one of 4 global patterns, u ∈ {1, −1, 2, −2}.
- Parameters fixed before the freeze, not on F_p data:
  - the Thomae partition matching, the 8th roots `eps_c` and the duplication pairing come from
    exact analytic identities at one real configuration, with residuals at most 4.4e-14
    (`formulas/*.json` reports);
  - development at p = 13 (D2). Since `chi_13(-1) = +1`, p = 13 cannot separate u = +1 from
    u = −1. That explains why P0 (`kappa_p = +1`) was predicted and then refuted. Development
    could only have selected conventions that change the global constant, and `kappa_p` absorbs
    exactly those.
- Independent checks, recomputed by `retabulate.py`:
  - 1,569 compared kernels, 1,569 agreements with `kappa_p = chi_p(-1)`. Subtracting the 8 used
    to fix `kappa_p` leaves 1,561 out-of-sample agreements.
  - Both Smith outcomes occur within each of the 48 control curves (48/48), where `c` is fixed.
  - The null `chi(c)` alone agrees on only 817/1,569 = 0.52.
  - The pattern over 8 primes matches 1 of the 4 preregistered patterns (u = −1 only). A random
    pattern would match with chance 4/256.
  - Independently of the control, the analytic constant predicts square class −1 (F7).
- Even counting curves rather than kernels as the unit, 48 curves (40 after the 8 fitted bits)
  agree on every kernel.
- The calibration therefore cannot produce the agreement by construction: a global sign cannot
  make the per-kernel variation agree 1,561 times.
- What the control cannot test:
  - the affine formulas (addressed by F6 and F7);
  - the twist factor `chi(c)`, which cancels (F11);
  - the odd-to-even transformation and the `F_{p^7}` roots (addressed by F6 and the model-t gate).

**F5. Erratum: the `S_L = 0` statement is false as written.** RESULTS says "`S_L = 0` occurred
only on kernels with a single (double-root) trigonal map", and the table caption reads "(all on
one-map kernels)". Over all 5,027 evaluated control kernels, `S_L = 0` occurs on 877:
- 151 with one rational map (all of the one-map kernels);
- **726 with no rational map**;
- 0 of the 1,569 with two maps.

`analyze.py` skips no-map kernels before it counts zeros, which produces the stated pattern. The
zero rate is about 5/p (33% of kernels at p = 17, 10% at p = 47).

My probe (`validation/zero_locus_probe.py`, exhaustive over `P^1(F_{p^2})^2`) checked 22 no-map
kernels with `S_L = 0` and 22 with `S_L != 0`, at p = 17, 19 and 29. Every zero kernel has no
genuine trigonal map over `F_{p^2}` and 2 degenerate candidates (common factor or degree drop).
Every nonzero kernel has 2 genuine conjugate maps. The zeros therefore coincide with
Smith-degenerate configurations, which supports the formula. Only the sentence and the caption
need correcting. My reading that the zero locus has several components (about 5/p) is
interpretation, not verified.

**F6. Independent F_p recomputation (Q3): pass, exact agreement.** `validation/recompute_fp.py`,
seed 20261121, uses its own implementation throughout:
- `F_{p^d} = F_p[x]/(g)` with a randomly drawn irreducible `g` (Rabin test), multiplied by numpy
  convolution and a reduction matrix;
- roots of `f` by equal-degree splitting over the `F_{p^7}` subfield of `F_{p^28}`, with orbit
  length 7 checked (so `f` is irreducible);
- fourth roots by Tonelli–Shanks applied twice, each times a random 4th root of unity;
- a random primitive 8th root `zeta8`;
- the relabelling permutation by brute force over `S_8`, taking a random member of the coset;
- `S_L` evaluated from the stored formula terms.

Main arm: 56 codomains (7 per prime, 8 of them degenerate), 58 evaluations (two G2 rows with
both `t`).
- Exact `F_p` value 58/58; sign 58/58; `c = f(t)` 58/58.
- A randomly drawn affine formula the run never used (14 distinct S) gives the same value 58/58.
- `#J` from my own point counts over `F_p`, `F_{p^2}`, `F_{p^3}` matches 56/56 and is prime
  56/56; the Frobenius polynomial matches 56/56.

Control: 56 kernels (per prime: 5 compared, 1 with `S_L = 0`, 1 without a map).
- Value 56/56; a random other pair formula (33 distinct S) 56/56.
- Trigonal map count 56/56 and Smith symbols 56/56, both by exhaustive `P^1 × P^1` search and
  the Sylvester resultant.
- Decision `chi_p(-1) chi(c) chi(S_L)` equals my Smith symbol on 40/40 compared kernels.

Runtime 57 s.

**F7. One constant D for all 64 formulas (supports the transfer of `kappa_p` to the affine arm).**
`validation/universal_constant.py` computes, with my own numpy theta series (R = 10), the 36 even
theta-nulls of `(tau+S)/2` for all 64 S at the producer's derivation configuration (seed 1). The
period matrix `tau` and the a-periods `A` come from the frozen `theta_arm.py`.
- `prod theta_B / S_L(S)` is the same number for all 64 formulas: maximum relative deviation
  6.5e-14 (pair 5.2e-14, affine 6.5e-14). No even theta-null of B vanishes.
- The common value equals `kappa^9 det(A)^18` with `kappa = -(2 pi)^-6` to 1.6e-15. Every
  formula report gives `kappa pi^6 = -1/64`, and this value is `(2 pi i)^-6`, the classical
  Thomae constant.
- With LRZ's normalisation `chi_18 = (2 pi)^54 / 2^28 · prod theta / det(Omega_1)^18` and
  `Omega_1 = A`, this gives `chi_18(B, omega) = -2^-28 S_L`. That predicts `kappa_p = chi_p(-1)`
  a priori, matching the calibration.
- Caveat: the normalisation (and its sign) is known only from the search summary in
  `LITERATURE.md` L5 and from my recall. This prediction is corroboration, not proof.

**F8. Exactly two stable affine kernels (Q4): pass.**
- Enumerating `J[2]` as even subsets modulo complement with the pairing `(-1)^{|S∩T|}` gives
  135 Lagrangians: 105 pair type and 30 affine.
- Exactly 2 are stable under the Frobenius 7-cycle (0 1 2 3 4 5 6) that fixes ∞, and both are
  affine. Group theory agrees: a C_7 lies in 30·8/120 = 2 Fano structures.
- The data's `L_subsets` for L = 0 and L = 1 are exactly these two on all 800 curves, with no
  duplicate curve.
- Prime (odd) `#J` forces `f` irreducible for an odd-degree model, since
  `#J[2](F_p) = 2^{r-1}` where r is the number of irreducible factors of `f`, so Frobenius is a
  7-cycle.

**F9. Independence of the two kernels (Q4): consistent with independence.**
- 2×2 table on the 756 curves with both kernels nondegenerate: ++ 197, +− 188, −+ 197, −− 174.
- phi = −0.019, approximate 95% interval [−0.091, 0.052]. Fisher exact two-sided p = 0.61.
  Expected −− is 177.6 from the marginals and 177.0 at r = 0.516.
- This is a non-rejection, not a proof of independence. The coverage 606/800 is counted
  directly and does not depend on independence. RESULTS' "behave independently, **so** 606/800"
  reads as if it did.
- The remaining curves: 44 have one degenerate kernel (at most one per curve); 24 of these have
  a Jacobian on the other kernel and 20 a twist. That gives 174 + 20 = 194 curves without a
  Jacobian codomain.

**F10. Statistics (Q5): pass.**
- All per-prime counts and Wilson 95% intervals reproduce to the printed digits.
- Pooled r = 803/1556 = 0.516067, Wilson [0.4912, 0.5408].
- Coverage 606/800 = 0.7575, Wilson [0.7266, 0.7859].
- P1, checked on the unrounded r: 0.5161 lies in [0.35, 0.65], and both outcomes occur at every
  prime.
- P2: 383/771 = 0.4968 [0.4616, 0.5320] versus 420/785 = 0.5350 [0.5001, 0.5697].
- The binned r(N1) values reproduce.
- Control: pass counts range 54–128 and fail counts 61–140 per prime, as stated.
- Wilson intervals treat codomains as independent. The measured phi ≈ 0 makes that acceptable
  for the main arm.

**F11. The control cannot test the twist factor; G2 "80/80" overstates the informative count
(presentation).** The Smith fibre norm contains `c^3`, and the statistic contains `chi(c)`, so
`chi(c)` cancels identically: the control compares `kappa chi(S_L)` with `chi(prod h(x_i))`
only. The `c^27` factor is tested only by the main-arm model gate (two values of t). Of the 80 G2
rows, 34 are informative (`chi(f(t1)) != chi(f(t2))` and `S_L != 0`), and all 34 give the same
sign. The other 46 rows cannot fail this check. RESULTS should report 34/34 informative.

**F12. Literature versus derivation (Q6).**
- Cited, read through search summaries only:
  - LRZ criterion: "Jacobian over k iff `chi_18(A, E)` is a nonzero square, for a k-rational
    basis E". The summary states it for `k ⊂ C`.
  - its use over `F_p` via lifting (Ritzenthaler 2010);
  - the normalisation constant (L5);
  - Oort–Ueno.
- The producer's own derivations:
  - (a) the explicit `chi_18(J/L_S) = D·S_L` formulas (derive.py: Thomae + duplication, with
    numerically identified partitions and `eps_c`);
  - (b) one D for all S. Supported by F7, and by the argument that the principal polarisation
    of `tau_B = (tau+S)/2` restricts to `2E_J` on `Z^3 + tau Z^3` with unchanged a-periods;
  - (c) the twist factor `c^27`. I agree: weight 18 on a rank-3 basis scaled by `c^{-1/2}` gives
    `c^{±27}`, and the model gate tests it (F11);
  - (d) the post-hoc `i^18 = −1` reading of u = −1, made quantitative by F7;
  - (e) "degenerate ⟹ hyperelliptic" (F13).
- Using the basis `x^k dx/y` of J for B is legitimate: the isogeny has degree 8, prime to p, so
  its pullback is an `F_p`-rational isomorphism on differentials.
- Not verified by me: the LRZ statement itself, its validity over finite fields, and the
  orientation of Smith's Prop. 4. The orientation is inherited from the lab's
  `smith_prop24`/S1 checks. F6 reproduces the symbol, not the theorem.
- research-kb was unavailable here (no executable and no catalog; `scripts/research-kb status`
  exits 127). That is an access limit.

**F13. Erratum (minor): gap in "[der] Degenerate codomains are hyperelliptic".** Simplicity over
`F_p` (from prime `#J`) does not by itself give geometric indecomposability.
`Res_{F_{p^3}/F_p} E` is simple over F_p yet a product geometrically. Its Frobenius polynomial
has the form `x^6 + a x^3 + p^3`, that is `e1 = e2 = 0`. No row of 1,600 (and none of the 44
degenerate rows) has `e1 = e2 = 0`, so the conclusion holds for this data set once that check is
added.

**F14. Erratum (trivial).** "positive control 3.5 min" should be 4.0 min (`data/run.log`:
00:21:05 → 00:25:05). Main arm 18.7 min and derivation 3.7 min match "19 min" and "about 4 min".

**F15. Repository policy (process, not science).**
- `data/control.jsonl` (918,007 B) and `data/main.jsonl` (495,551 B) are data-suffixed files
  over 64 KiB committed to Git. `python3 tools/artifacts/artifacts.py plan
  research/g3_affine_twist_20261121` selects both for the store (AGENTS.md §13). They belong in
  `s3://…` with an `.artifacts.json` manifest.
- The study leaves an SVG but no PDF (research-visuals rule).
- Fixing this is for the producer or the PR owner. Per lesson L-8d86e8fc352c, the pinned
  `RESULTS.md` stays unedited and corrections go into a sibling `ERRATA.md`.

**F16. Reproducibility and reporting contract.**
- Deterministic replay from `git archive 0340c08` with the producer's venv:
  - control p = 17: 630/630 rows identical;
  - main p = 17: 200/200;
  - main p = 47, first 15 curves: 30/30 (timing field `sec` excluded).
- `run.py` draws curves and evaluation randomness from one RNG stream. The population therefore
  cannot be regenerated without running the evaluation. That is not a bias, since RNG
  consumption does not depend on the outcome.
- Not applicable to this study:
  - the ECDLP cost-reporting rules (one unit S, matched rho, §5a band, m=83 / ECC2K-130);
  - certificate rules for DL solves, relations or closures. The study's per-codomain values
    play that role, and F6 re-verified a sample.
- Labels `[lit]/[der]/[comp]/[conj]` and `[pcc:unverified]` are used honestly.
- Lesson L-99ee952350b2 (identity scripts read real keys):
  - `analyze.py` uses `.get` only harmlessly: `r.get("smith")` is intentional, `d2.get` is
    guarded by `len == 2`, and `kappa.get` covers all 8 primes;
  - `retabulate.py` indexes with `[]` and asserts `set(L) == {0,1}` per curve.

**F17. Scope of the per-codomain labels (interpretation, labelled as such).**
- If the affine constant had the opposite sign, r would be 0.484 and coverage 0.724. The
  "coin flip" headline is robust to that; the per-codomain Jacobian/twist labels are not.
- F7 removes the main reason to doubt the sign: D is common to pair and affine S.
- The labels' residual dependencies are external to the data:
  - the LRZ criterion over F_p;
  - Smith's orientation;
  - the common subset map. The map is computed from `tau` only, is shared by all formulas, and
    the pair control validates it.

## Validator files (`research/g3_affine_twist_20261121/validation/`, untracked, not committed)

| file | sha256 |
|---|---|
| `retabulate.py` | `cce6d2f529de65351f7f502f9009532481f148bcd39825d31e2ae634f6a60688` |
| `recompute_fp.py` | `976ccfa4ad6344513b67e744618156f4cb8a869d7a33cf1f487c74f12a3a09a1` |
| `recompute_fp.out.jsonl` | `a46ef2359cfecbae0623503acb110c8d0115a761e008b77ab163696caef3bca9` |
| `recompute_fp.log` | `6e13b1010ff97c3f2a25a121e1221c70f95a757230e66df763b229d28095d35f` |
| `universal_constant.py` | `47f1866df53def559b0aa879de45644235394c273225332429e77fe521b85792` |
| `universal_constant.out.json` | `81fdd8db5255df9ebc016f104d5ae92394d02716f988c1cf00baadb7536b4162` |
| `zero_locus_probe.py` | `55896a384a1e39cb5216865a9ffed8a5e885cdae11052f081663dee3a433b5ec` |
| `zero_locus_probe_p17.json` | `ccf301763ff40a4d5f40289b4b827d8407e0e1659246fc293f24fecb5b0f3823` |
| `zero_locus_probe_p19.json` | `3a0cd5730868b00228717132d5c329ecb55f53cc3aa0a66d6be9b43c743b55eb` |
| `zero_locus_probe_p29.json` | `945894bb7d091a83737b16de756505d4fa072f68918c363fcc71c99e7588282e` |

`recompute_fp.commit` records `2a765f2…`, HEAD when the recompute ran. The study files are
identical at `0340c08`.

Commands, run from `research/g3_affine_twist_20261121/`:

```
python3 -I validation/retabulate.py .
OMP_NUM_THREADS=1 python3 validation/recompute_fp.py . validation/recompute_fp.out.jsonl 20261121
python3 validation/zero_locus_probe.py . 17 8 5   # also: 19 8 6, 29 6 7
# needs mpmath (producer venv site-packages appended to sys.path) and numpy:
python3 validation/universal_constant.py "$PWD"
```

## Not checked

- I did not read the full LRZ, Meagher or Ritzenthaler texts (and the producer did not either).
- I did not prove Smith's Prop. 4 orientation; I inherit it from the lab's prior checks.
- I did not re-derive the formulas from scratch. F7 checks the stored formulas against my own
  thetas at the derivation configuration only.
- Only small primes, as in the study.
