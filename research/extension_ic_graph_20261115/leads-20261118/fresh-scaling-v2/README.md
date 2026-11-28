# fresh-scaling v2: amended draft protocol (growing sizes, fresh targets)

Lane key `fresh-scaling-v2`, 2026-11-19. This is a **design amendment, not a
run**. Nothing was dispatched and no paid compute was used. No pcc claim is
registered, and the protocol stays a **DRAFT for owner freeze**. Every number
is a model or a producer observation labelled [pcc:unverified]. The v1 draft
in [`../fresh-scaling/`](../fresh-scaling/) is unchanged.

v2 amends v1 for the round-2 red-team findings F1–F5 and F8
([`../CRITIC_ROUND2.md`](../CRITIC_ROUND2.md)). The finding-by-finding map is in
[`CHANGES.md`](CHANGES.md).

## Files

| File | What it is |
|---|---|
| [`protocol.draft.json`](protocol.draft.json) | `experiment.protocol/v1` v2 draft. Nothing is pinned: no hash, commit, nonce, worker build or analysis program. |
| [`CHANGES.md`](CHANGES.md) | Each critic finding, its resolution, and where the change sits. |
| [`tools/rule_check.rs`](tools/rule_check.rs), [`tools/run.sh`](tools/run.sh) | A standalone Rust check with no crates. Part A runs v1's `estimate.csv` through both rules. Part B is a Monte Carlo of both rules. Part C is the v2 runtime estimate. |
| [`rule-check.txt`](rule-check.txt) | Its output. |
| [`estimate-v2.csv`](estimate-v2.csv) | Per-size and per-arm runtime model for v2. |
| [`dry-run/`](dry-run/) | Generator dry run for the two new rungs, p101 and p1511, three targets each, under the placeholder nonce `DRYRUN-NOT-A-BEACON-20261119-V2`. **Not evidence.** |

## What changed and why

### F1. The exponent decision can now fail

**Problem.** v1 decided P1/C2 on a global OLS fit of `ln C_total` against
`ln l`. The descent costs a roughly constant 7.6 s at every p, which flattens
the small sizes, so the fit "passes" even when the collection phase scales as
`l^(1/2)`.

**Rule now.** The decision follows SUCCESS_CRITERIA T2 item 4 and lesson
L-eda09df2bc11. It uses α\*, the largest **per-phase** upper exponent limit
(counted phases, all band weights), together with the **top-pair local slope**
of the complete M2 total, between p1153 and p1511. Both are compared with
θ = α_rho_ref − δ, where δ = 0.05.

| Verdict | Condition |
|---|---|
| FAIL | the top-pair lower limit ≥ θ, or (G-cov passed and) some phase's lower limit ≥ θ |
| PASS | G-cov passed, every phase upper limit < θ, the top-pair upper limit < θ, and α\* ≥ α_derived = 1/3 |
| HELD | as PASS, but α\* is below α_derived by more than its interval (T2 item 5: a fixed cost, a phase not yet dominant, or a counting gap) |
| UNRESOLVED | anything else |

The global fit is still printed, but it decides nothing.

**Re-run of the lane's own `estimate.csv`** (`rule-check.txt` Part A, point
values; rho_ref α = 0.488, θ = 0.438):

| IC model | v1 global α_total | v1 verdict | Per-phase α (non-descent / descent) | Top pair | v2 verdict, G-cov passed | v2 verdict, G-cov failed |
|---|---:|---|---|---:|---|---|
| e=2 | 0.119 | PASS | 0.333 / 0.000 | 0.263 | HELD (α\* is on α_derived) | UNRESOLVED |
| e=3 | 0.151 | PASS | 0.500 / 0.000 | 0.422 | **FAIL** | UNRESOLVED |

**Simulation** (Part B): 200 simulated studies × 400 stratified bootstrap
resamples. Each study has n = 32 per size, an NB(2, 1/1100) descent and
Rayleigh rho, on both ladders. The amended rule:

- FAILs the e=3 models 200/200. v1 PASSes them 200/200.
- FAILs the lesson's two-phase case (α 0.25 plus a 1%-share α 0.75 phase)
  200/200. v1 PASSes it 200/200.
- FAILs the null (every phase at α = 1/2) 200/200. v1 gives 3–7 PASS, 7 FAIL
  and the rest UNRESOLVED.
- HELDs a pure fixed cost 200/200. v1 PASSes it.
- PASSes e=2 in 190–197 of 200, with the rest HELD.

**Limit.** The frozen driver emits no per-phase CPU time, so the per-phase
statistic needs counted phases, and therefore G-cov. With G-cov failed, as it
does today, the only route to FAIL is the M2 top pair:

- It catches the two-phase case (200/200).
- It does **not** catch e=3: the top pair is 0.40 against θ ≈ 0.45, because
  the descent is still about 14% (deferred) to 54% (table) of the modelled cost at p1511.

So without G-cov, C2 can be FAIL or UNRESOLVED, never PASS. An opt-in
per-phase CPU timer is listed as an owner decision.

### F2. G-cov means complete

G-cov now requires an emitted, non-null count on the bridge fixture and on
every pilot target for **every** row of
[`../complete-accounting/README.md`](../complete-accounting/README.md)'s
inventory (#1–#19). Row #16 must carry its excluded count, and rows that do not
apply carry explicit zeros. Gate G-inv first adds rows that inventory lacks:

- #20: ic-table's quadratic root table;
- #21: the Karatsuba-priced rho reading's model terms.

The v1 descent-multiplication match survives as a sub-check.

**Today G-cov fails.** These rows have no emitted count in the 11c9a031 build:

- #1–#3: the counted linear stage exists only as a patch;
- #4 and #5;
- #9–#11, #13 and #14;
- #18–#19: rho's own null rows;
- #20: not yet inventoried.

Until it passes, M1 is "lower bound, complete_cost null, lower-bound order,
unranked" (F5).

### F3 and F4. Strongest arms on both sides, and an expected-cost reference

**IC arms:**

- **Primary:** `ic-table` (Run50 `--method extension-sparse-remainders --root-table`).
  On Run50's matched 3,415-residual stream it had the lowest non-descent
  counted work, 1.805e12 against 2.488e12 for deferred and 2.791e12 for
  original IC. It also had the lowest process time, 15.1 s against 27.8 s.
  Lesson L-7cdab2147415 applies.
- **Secondary:** `ic-deferred` (the graph candidate). It uses the same worker
  seed, so it consumes the identical residual stream. That makes the new claim
  C5, deferred against table per size, a paired reading with no NB term.

**Instrument.** It moves from the nagao-e2e worker at fcda1e22, which has
neither method, to the Run50 driver at 11c9a031, which runs every IC and rho
arm in one binary.

**Rho arms.** Each reference is fixed per metric before any data:

- **M2:** `rho-neg-b16`, expected about 76 s at p1009.
- **M1:** `rho-neg-b1`, expected 1.549e14 bitops, the lowest of the
  registered configurations.
- **Strongest priced M1 reading:** `rho-neg-b1-karatsuba-priced`. It is the
  same walks priced in rho-fold's counted Karatsuba/Chung–Hasan tower, and
  gives 9.784e13 at p1009. It is labelled *priced, not measured* (F11).
- **Optional measured arm:** `rho-neg-b1-karatsuba`. It enters only if the
  opt-in tower lands and passes rho-fold's pre-dispatch controls before
  freeze.
- **Fastest external rho: unavailable.** Magma, PARI/GP and Sage are not
  installed here. The lane knows no E(F_{p^6}) record implementation, and
  research-kb is down, so this is not a literature claim. It is named as
  missing under AGENTS.md §15, for the owner.

**Reference cost (F3).**

- Each configuration has its own derived seed per target, so no walk is
  reused.
- The reference is the mean over 32 independent targets with a bootstrap
  interval. A step-normalised reading, `(C_i/steps_i)·E[iter](l_i)`, is
  reported beside it.
- CTL-rho-steps requires the interval of the mean steps/E[steps] to contain 1.
- An observed-cheapest configuration is shown only as a sensitivity row.

**Dropped arms.** `rho-plain-b1` is dominated in expectation. `ic-original` is
dominated in non-descent work and kept as a bridge-only continuity row.
`rho-plain-b16` stays as the slope negative control.

### F5. Wording

The draft never writes "cheapest", "fastest" or "ranked" about a counted lower
bound. M2 order statements are made only from intervals, in seconds on the
stated host.

### F8. The p2017 rung is dropped; p1511 and p101 are added

**Decision.** Drop p2017. Replace it with p1511 at the top and add p101 at the
bottom.

**Why p2017 cannot run.**

- **Every arm refuses it.** The dry-run l = 1.683e19 is above 2^63. The Run50
  driver's `PublicCurve::validate`
  (`examples/jv_sieve_deferred_pair_compare.rs:158-160`) refuses
  p ∉ 101..=1511 or l ≥ 2^63. So do the IC pipelines:
  - `jv_extension_deferred_pair_dlp.rs:416-417`
  - `jv_extension_quadratic_pair_dlp.rs:421-422`
  - `jv_extension_sparse_dlp.rs:334`
  - `jv_sieve_two_large_prime_dlp.rs:160-161`

  The same guards are present at v1's fcda1e22. So v1's top rung would have
  produced no data at all.
- **The guards hide unreduced sums.** A read-only static audit of the 11c9a031
  closure (`git archive` into scratch) found u64 additions mod l with no
  widening. They wrap once l > 2^63:
  - the rho walk's coefficient update, `jv_rho_strong.rs:719-726`
    (`a = ws.a + aj; if a >= n {a -= n}`). At l ≈ 0.91·2^64, about 40% of
    uniform pairs overflow, so the walks would carry wrong logs;
  - its cycle escape, `jv_rho_strong.rs:759` (`2 * best.1 % n`);
  - the IC sparse kernel's `add_raw` and Shoup `raw`
    (`jv_sparse_kernel.rs:128-147`, used in dot, kernel and Krylov updates);
  - `residual_walk.rs` `add_mod`, which is not imported by the IC closure;
  - `jv_cover.rs` rho_e* and run_cover_dlp_mode, which this audit did not
    trace onto the driver's methods.

  The u128-widened `add_mod`s in `jv_extension_*_dlp.rs` and rho's
  `collision_scalar` are safe. Lifting the guard therefore needs an audited
  u128 path. That is out of scope here.

**The new rungs.**

- **p1511** is the instrument's ceiling. Its l ≈ 2.975e18 < 2^62, so every sum
  of up to four reduced residues fits in a u64.
- **p101** keeps the span at 6·log2(1511/101) = **23.42 bits**. That is the
  most the domain allows, and it is short of T2's 24 bits, which is an owner
  decision.
- **Dry run.** Both rungs were generated and checked, 6/6 ok, with the
  unchanged v1 generator (`dry-run/`). Generation took 29 s in total;
  p1511 needed 6–92 attempts per target.

**New gates.**

- **G-dom:** refuse l ≥ 2^62 at admission. The generator needs this check, or
  a frozen admission filter, before freeze.
- **G-ovf:** both planted pilots at every size go through ic-table,
  ic-deferred and rho-neg-b16 in a build with `-C overflow-checks=on`, and
  must match the measured build byte-for-byte in scalar, residual count and
  tally. This applies lesson L-61e5ea1f9e0d: a small-p control cannot see an
  overflow near the largest modulus.

## New compute estimate (model; `estimate-v2.csv`)

Seven sizes × 32 targets × 7 measured arms: ic-table, ic-deferred,
ic-table-aa, rho-neg-b1, rho-neg-b16, rho-plain-b16 and rho-aa.

| p | 101 | 127 | 223 | 383 | 661 | 1153 | 1511 | total |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| CPU-h (e=2 / e=3) | 0.23 | 0.23 | 0.26 | 0.42 / 0.40 | 1.16 / 1.14 | 5.06 / 5.09 | 10.98 / 11.17 | **18.3 / 18.5** |

- **Totals.** Rho is 16.1 CPU-h of the total. The planted pilot, with
  overflow checks assumed to cost 1.5×, adds 0.5 CPU-h, and the bridge fixture
  0.12 CPU-h per host. With ×1.2 overhead the total is **about 23 CPU-hours**,
  against about 55 for v1.
- **Budget stop.** The draft's budget stop is 30 CPU-h.
- **Optional arm.** A measured Karatsuba arm would add about 2.9 CPU-h. That
  figure is an unmeasured proxy.
- **Model basis.** The IC model is
  `T_nd(1009)·(p/1009)^e + 2200 × 3.8 ms`. Its constants come from Run50
  process seconds: table T_nd 2.14 s, deferred 14.85 s.
- **No IC run exists at any p other than 1009.**

## Commands run

```
df -h /                                   # 13 GB free
git archive 11c9a031 src/cryptanalysis examples/jv_sieve_{deferred_pair,graph}_compare.rs Cargo.toml \
  | tar -x -C <scratch>/r50src            # read-only static audit (F8)
git grep / git show at fcda1e22 and 11c9a031   # domain guards, add_mod
RUSTUP_TOOLCHAIN=1.98 CARGO_TARGET_DIR=/home/user/cargo-target-round2 CARGO_INCREMENTAL=0 \
  cargo build --offline --locked --release -j 1 --manifest-path ../fresh-scaling/gen/Cargo.toml
  # v1 sources hash-checked unchanged after the build; binary 98b623a4…, rustc 1.98.1
fresh-scaling-gen batch --nonce DRYRUN-NOT-A-BEACON-20261119-V2 --sizes 101,1511 --n 3 --out <scratch>/DRY
fresh-scaling-gen check <scratch>/DRY/p*/public-*/public.json      # 6/6 ok -> dry-run/check.jsonl
tools/run.sh   # rustc 1.98.1 -O -C overflow-checks=on tools/rule_check.rs; writes rule-check.txt, estimate-v2.csv
scripts/research-kb status                # "no executable found"
```

Hashes: `tools/rule_check.rs` a6d92420…, binary 1b195d7e…, `rule-check.txt`
f7cbf943…, `estimate-v2.csv` c079d71f…. The protocol pins all of them again at
freeze. The Monte Carlo is seeded, so a rerun gives the same output.

## Gaps and open items for the owner

These are also listed in `protocol.draft.json` under `owner_decisions`:

1. **δ and α_derived.** Freeze δ = 0.05 and α_derived = 1/3, or change them.
   α_derived is the lane's own full-factor-base model; it was not re-derived
   from Joux–Vitse here.
2. **Ladder span.** Accept the 23.42-bit ladder, or fund a widened, audited
   u128 domain to reach T2's 24 bits and a held-out size.
3. **M1 reference.** Choose the registered M1 rho reference: the registered
   tower, or a Karatsuba tower that is priced now or measured after
   rho-fold's protocol.
4. **G-cov.** Land the counted linear stage and the null-count rows before the
   run. Without them C2 cannot PASS, and M1 stays an unranked lower bound.
5. **Per-phase timer.** An opt-in per-phase CPU timer in the worker, outside
   the counted interval, would let M2 decide per phase without G-cov.
6. **External rho.** Provide a fastest external rho or a host with one, or
   waive the requirement under §15.
7. **Authorization.** The pilot is about 0.5 CPU-h and eligible to run
   locally. The measured run is about 23 CPU-h on isolab-eks.

**Not done here, needed before freeze:**

- the G-dom check in the generator and the v2 default ladder;
- the Rust analysis program;
- the G-inv rows (#20, #21);
- the worker successor with complete counts;
- re-checking the static overflow audit dynamically (G-ovf).

research-kb and ecdb were unavailable, so no KB source ids are recorded. No
literature or novelty claim is made.

## Sources read

- `../CRITIC_ROUND2.md`
- `../fresh-scaling/{README.md,protocol.draft.json,estimate.csv,gen/src/main.rs}`
- `../complete-accounting/README.md`
- `../rho-fold/{README.md,results.txt,protocol.draft.json}`
- `../descent-walltime/README.md`
- Run50: `run-study.json` (outcomes, phases), `*.argv.json`, `*.time.txt` and `README.md`
- `docs/ic/SUCCESS_CRITERIA.md` (T1, T2)
- AGENTS.md: §15, and the "Implementation language" section
- the lessons L-eda09df2bc11, L-61e5ea1f9e0d, L-2ff461066555 and L-7cdab2147415, from the critic's scratch ledger
- code at 11c9a031 and fcda1e22:
  - `jv_rho_strong.rs`
  - `jv_sparse_kernel.rs`
  - `residual_walk.rs`
  - `jv_extension_*_dlp.rs`
  - `jv_sieve_two_large_prime*.rs`
  - `jv_cover.rs`
  - `examples/jv_sieve_{deferred_pair,graph}_compare.rs`
