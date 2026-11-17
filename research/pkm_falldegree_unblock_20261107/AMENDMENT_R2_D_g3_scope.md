# Amendment R2-D, frozen 2026-11-09 as PREREGISTRATION Amendment 11: gate G3's scope is the F4 engine's capacity

**Status: FROZEN 2026-11-09 as Amendment 11 of research/prime_fall_degree_20261027/PREREGISTRATION.md, before any stage C G3 row
exists and before `pfd-b` is submitted.** (Originally written as a draft.) Written 2026-11-09 after job `pfd-c-20261108` stopped at its
`g3` stage. Protocol sha256 before: `8c2ff0a389cd0b00c8ce91f8d19830645a97be90279fa89bb2624aee0f0b350d` (Amendments 1–10); after the
append: `ef5e18ea8443d75bc61d925fadbb52406d1fe6fcfaf87d68d99a2361cd657cd2` (`git diff --numstat`: 35 added, 0 deleted). The unfrozen
draft `AMENDMENT_R2_B_m4_null.md` is renumbered to proposed Amendment 12.

## 0. Main session decisions (2026-11-09), frozen with this amendment

1. **Rule:** option 1 of §4.3: keep the `m ≥ 7` rows with the label "G3 not applicable (F4 engine capacity)", with the disclosure of §4.2.
2. **`pfd-c`'s crashed `g3` stage is recorded as "not run (tool defect)", not as a §4 gate failure.** Reasons, with the registered text:
   G3's domain is "on every cell where both run" (§4); Amendment 3 already registers the F4 capacity ("This applies wherever
   `f4_fp_tower` runs (at most 4 free unknowns, `m ≤ 6`)"); `gate3` selects rows by `N` only and does not skip cells outside that
   capacity; it panicked before comparing anything (`gates/g3.jsonl` is empty), so there is no agreement or disagreement evidence.
   This is the main session's reading of the registered text, made before any stage C G3 row exists; the owner may overrule it.
3. **Order:** the stage C G3-only job (`pfd-g3c-20261109`, §4.4) runs BEFORE `pfd-b` and before any `pfd-x` capped run, because a stage C
   disagreement would bear on capped F4 everywhere, and the job is cheap.
4. **Stop rule** (the two questions §3 leaves UNRESOLVED stay undecided: does a disagreement stop capped F4 only or the whole study; is G3
   one gate or one per stage). If the stage C G3 job reports one or more disagreements (a budget timeout counts as one, as `gate3` defines
   it), no further PKM job is submitted and the question goes to the owner with the rows. If it reports 0 disagreements, the study
   continues in the `R2_READY.md` order.
5. **`pfd-b`:** all of its `d-B` and G3 cells are within capacity (`m ≤ 6`); it runs `g1 hky-B d-B g3` as requested. A budget timeout in its
   G3 at `m = 5, 6` is a risk, not a prediction.
6. **Job:** `pfd-g3c-20261109`; cpu 12; memory 56Gi; `seconds` 57600; no instance pin (Amendment 9 pins only above 90,000 s); command
   exactly as §4.4. Cost bound with the Amendment 9 formula, (`seconds`/3600 + 0.25 h) × (Spot $/h + EBS 0.0088 $/h):
   16.25 h × (0.9620 + 0.0088) = **US$15.78** at the highest eligible 16-vCPU Spot price quoted 2026-11-08 (unpinned), or
   16.25 × (0.3243 + 0.0088) = US$5.41 if it lands on r6i.4xlarge (derived; prices cited from `r2c_spot_prices_20261108.txt`). Within US$40.

Section 5 below (the `pfd-x` question) is answered by decisions 3 and 4.
Every number is `[pcc:unverified]`. Evidence classes: *measured* (a command or a source line read here), *cited* (a file says
so), *derived* (arithmetic or reading of code), *heuristic*.

**Blindness.** Sources: the code at `f13a690df0`, the preregistration, the failure log (`gates/g3.log`, `progress.txt`,
`host.txt`, `binary.sha256` of `pfd-c-20261108`, plus object *names* listed under its prefix) and stage A rows (`results/R9/`).
No stage C row was read: nothing under `results/C9/` and no `runs/C-*.jsonl` object. The rule below depends only on
`(kind, m, t, p)` and the engine's source, not on any outcome. Freeze it before anyone reads a stage C row or a stage C G3 row.

## 1. The instrument limit and its evidence

- **Limit (measured, source).** `src/cryptanalysis/f4_fp_tower.rs:189` `pub const MAX_FREE: usize = 4`. A monomial key is a
  `u128`: tower mask in bits 0–63, four 8-bit free exponents in bits 64–95 (`FREE_SHIFT = 64`), degree from bit 96
  (`DEG_SHIFT = 96`). So 4 free variables is a property of the key layout. Raising it is not a one-line change (derived).
  `TowerRing::from_raw` (lines 734–736) writes `f[k]` for `k in 0..self.n_free` into `[u8; MAX_FREE]`. The guard
  `TowerRing::check()` (line 537, `assert!(self.n_free <= MAX_FREE)`) does not run first, because `measure_tower` in
  `examples/pkm_tower_pilot.rs` calls `ring.from_raw` on the tower equations before the engine starts. The run therefore
  panics with an index error and not with the assertion's message (derived, source). `f4_tower_generic` shares the
  constant and panics the same way, at `f4_tower_generic.rs:364` (measured, see below). The closure engine
  (`hky_fp_tower.rs:51`) has `MAX_FREE = 8` (measured, source).
- **Free variables (derived, source).** `pkm_tower_pilot.rs:1148–1158` `tower_ring` builds
  `TowerRing { rules: m blocks of t square rules, n_free: m − 2 }`. `--engine tower`, `--engine sig` and `--engine generic` all
  use it (lines 2264–2318). `system()` (line 1003) gives `n = m·t + (m − 2)` variables for every non-naive control (tower,
  null, null-chain, ladder). The naive control is skipped on these engines (line 2249). So
  **`n_free = m − 2`, whatever the kind (Kummer, isogeny, Dickson), `t`, `p` or control.** Capacity is `m − 2 ≤ 4`, that is
  **`m ≤ 6`**. Separately, `n_tower = m·t ≤ 64` (line 534), which no registered cell exceeds (max `6·8 = 48`, derived).
- **The failure (measured, `gates/g3.log` of `pfd-c-20261108`).** Two pilot subprocesses panicked at
  `f4_fp_tower.rs:736:17` ("index out of bounds: the len is 4 but the index is 4"). `gate3` then panicked at
  `prime_fall_degree_tools.rs:523` ("pilot failed") and at `:552` (join). `progress.txt` shows `end g3 exit 101`, one
  second after `start g3`, and `gates/g3.jsonl` is empty. `binary.sha256` matches `image/binaries.sha256`
  (`pkm_tower_pilot f78f3959…`, `prime_fall_degree_tools 7e8a6703…`). Both panicking threads are consistent with `gate3`
  popping from the end of its key-sorted queue, where the highest `p` and largest `m` (P1, Kummer, `m = 10`, `9`) sit
  (derived, `prime_fall_degree_tools.rs:475–500`).
- **Reproduction with the frozen binaries (measured, local development check, 2026-11-09).** The image's study layer
  (`sha256:a0f7e28e…`, 3.8 MB) was fetched from ECR. Its `pkm_tower_pilot` and `prime_fall_degree_tools` hash to the values in
  `image/binaries.sha256`. `--engine tower --random 2 --targets 0 --budget 120`:

  | p | kind | m | t | cap | exit | |
  |--:|:--|--:|--:|--:|--:|:--|
  | P1 | kummer | 6 | 2 | 6 | 0 | 1 row |
  | P1 | kummer | 7 | 2 | 6 | 101 | panic `f4_fp_tower.rs:736` |
  | P1 | kummer | 7 | 2 | 2 | 101 | panic `f4_fp_tower.rs:736` (the cap does not matter) |
  | P1 | kummer | 10 | 2 | 2 | 101 | panic `f4_fp_tower.rs:736` |
  | P0 | kummer | 6 / 7 | 2 | 4 | 0 / 101 | |
  | P0 | isogeny | 6 / 7 | 2 | 4 | 0 / 101 | |
  | P1 | kummer | 6 | 3 | 4 | 0 | (d-B shape) |
  | P1 | kummer | 5 | 4 | 4 | 0 | (d-B shape) |

  A non-frozen glibc build in `target/release/examples` (sha256 `f91268f5…`; the source files are unchanged since 2026-11-05)
  gives the same pattern. Its `--engine generic` and `--engine sig` also panic at `m = 7`. Its `--engine f4` (raw `f4_fp`, no
  tower ring) runs at `m = 7`, `t = 2` (measured).

## 2. Which registered cells exceed capacity (derived from `run.sh`, `entry*.sh` and the formula above)

| stage / job | engine | cells | over capacity (`m ≥ 7`) | affected? |
|:--|:--|:--|:--|:--|
| A (`pfd-a`: `g1 hky-A g3 d-A cap-A`, done) | hky, tower | `m = 2` only | none | no. G3 ran 144 capped runs with 0 disagreements (cited, `results/R9/…/gates/g3.log`) |
| C closures (`hky-C`) | hky (`MAX_FREE 8`) | P1 Kummer `m = 2…10`, `t = 2, 3`; P0 Kummer and isogeny where `t ≤ tmax0(m)` | `n_free` 5–8 ≤ 8 | no: all 36 closure stages exit 0 (measured, `progress.txt`) |
| C gate (`g3` in `pfd-c`) | tower (capped) | C rows with `N = m·t ≤ 20` | **`t = 2`, `m = 7, 8, 9, 10`**: P1 Kummer, P0 Kummer, P0 isogeny = 12 `(p, kind, m, t)` cells, 24 draws | **yes: the panic** |
| C, `t = 3`, `m = 7…10` (P1 only; P0 has `tmax0 = 2`) | — | `N = 21…30` | over capacity, but already outside G3 because `N > 20` (`--max-n 20`) | would not be reached |
| B closures (`hky-B` in `pfd-b`) | hky | `m = 3…6` | none | **no** |
| B F4 (`d-B` in `pfd-b`) | **tower** (`f4=(--engine tower …)` in `run.sh`) | `m = 3…6`, P1 `t ≤ 8/6/6/6`, P0 `t ≤ tmax0(m)` | none (`n_free ≤ 4`) | **no** |
| B gate (`g3` in `pfd-b`) | tower (capped) | B rows with `m·t ≤ 20`, including the `m = 3` null-chain (`n_free = 1`) | none | **no**: `pfd-b` would not hit this panic |
| `cap:` (`pfd-x-*`) | tower (capped) | Kummer `m = 2` | none | no |
| `m4:` (`pfd-m4-*`), `m4n:` (R2-B draft) | **hky** (closure, not capped F4) | `m = 4` | none | no |

Within capacity, the stage C cells G3 would cover (`m ≤ 6`, `N ≤ 20`) are 28 `(p, kind, m, t)` cells, 56 draws:
- P1 Kummer `t = 2, 3`, `m = 2…6`;
- P0 Kummer and isogeny `t = 2`, `m = 2…6`;
- P0 Kummer and isogeny `t = 3`, `m = 2…5`.

They are 19 run files (`C-t2-m{2..6}`, `C-t2-m{2..6}-p0`, `C-t3-m{2..6}`, `C-t3-m{2..5}-p0`; names from `run.sh`, confirmed by
an object-name listing). `gate3` schedules at most 2 capped runs per draw: at most 112 runs, and at most
`56 × 900 s = 50,400 s` with `--jobs 2` (derived upper bound).

## 3. What the registered text says

- **§3 (cited):** "Capped F4 (`pkm_tower_pilot --engine tower --cap c`) … refuting at cap `c` proves `d_F ≤ c`. The converse
  follows if the capped run is a `c`-truncated Gröbner basis. That is the gate in §4 (G3), not an assumption. Only after G3
  passes may capped F4 extend reach where the closure does not fit."
- **§4 heading (cited):** "Gates (before any measured cell; any failure stops the study)".
- **§4 G3 (cited):** "**G3, engine agreement:** on every cell where both run, `1 ∈ V_{F,c}` (closure) iff capped F4 refutes
  at cap `c`, for each `c` from 2 to the closure's `d_F`. One disagreement stops the use of capped F4 as a `d_F` instrument."
- **Amendment 3 (cited):** "G3 passed: 70 capped-F4 runs on closure-measured systems (`m = 2 … 4`, both primes, `N ≤ 20`)".
  It also says: "This applies wherever `f4_fp_tower` runs (at most 4 free unknowns, `m ≤ 6`)", and "with `t ≥ 2`, `m ≥ 7` lies
  beyond both the closure's limits and the F4 engine's 4-free-variable limit."
- **Amendment 7 B (cited):** "This is the procedure §3 and §5 D allow once G3 has passed." **Amendment 8 (cited):** "Capped F4
  runs only as Amendments 3 and 7 allow, after G3."
- **Ordering, plan text, not registered (cited):** `R2_READY.md` line 155: "rows 1-3 order is fixed by 'capped F4 runs only
  after G3'"; the `pfd-x` rows say "after G3". `README.md` line 106: "`pfd-a` (contains G1, G3) before any `pfd-x-*` or
  `pfd-m4-*` ('capped F4 runs only after G3'); `pfd-c`, `pfd-b` any time after".

**Reading (derived):**
- (a) *G3 not runnable on a cell.* G3's domain is "every cell where both run". A cell where `f4_fp_tower` cannot run is outside
  that domain by the registered definition, and Amendment 3 names the capacity (`m ≤ 6`) explicitly. Nothing registered
  attaches a consequence to such a cell, beyond the obvious one that capped F4 gives it no value.
- (b) *G3 failing.* "One disagreement stops the use of capped F4 as a `d_F` instrument." The §4 heading adds "any failure stops
  the study".
- **The text separates (a) from (b) by domain. It does not say whether a gate tool that crashes before comparing anything
  is a "failure" under the §4 heading: UNRESOLVED.** `pfd-c`'s `g3` produced no comparison (empty `g3.jsonl`), so it
  produced no disagreement. It is not a G3 result in the registered sense.
- **Also UNRESOLVED: which consequence governs a G3 disagreement.** G3's own sentence says "stops the use of capped F4"; the
  §4 heading says "stops the study". The draft does not resolve this.
- **Also UNRESOLVED: whether G3 is one gate for the study or one per stage.** Amendment 3 recorded one pass, and `pfd-a`
  passed it again. The per-job `g3` is a design choice in `run.sh`/`entry.sh`, not registered text.

## 4. Proposed rule (to freeze)

**4.1 Scope by capacity.** G3 applies, as registered, to every closure-measured random-target cell (prefix `A-`, `B-`, `C-`,
not naive, `N = m·t ≤ 20` as in Amendment 3 and `entry.sh`) with `m − 2 ≤ 4`, that is `m ≤ 6`. `m − 2` is the F4 engine's
free-variable count (`f4_fp_tower::MAX_FREE = 4`). A cell with `m ≥ 7` is labelled **"G3 not applicable (F4 engine
capacity)"**. The label depends only on `m`, never on an outcome. It applies to every `(kind, p, t, control)` with `m ≥ 7`. In
the registered cells that is stage C at `t = 2, 3` with `m = 7…10`; the `t = 3` ones are also outside `N ≤ 20`.

**4.2 What such rows may and may not be used for.**
- *May:* their closure values (exact `d_F`, or a censored bound as §2 defines it, and `ffd`) enter every §6 rule that uses
  closure values, as registered: H-m on ladder C, and H-LFD on the `(kind, m)` ladders. G3 validates capped F4 against the
  closure. It is not a gate on the closure (G1, G2 and G4 are), so a missing G3 removes no closure value.
- *May not:* no capped-F4 value (`=D (F4)` of Amendment 3, the bracket of Amendment 7 B, a §5 D reach cell) exists or may be
  constructed at these cells by any engine. H-FFD and H-gap need `D`, and "rows without `D` do not enter this rule" (§6 as
  written). No `D` exists here, because `f4_fp_tower` cannot run and `run.sh` has no `d-C` stage.
- *Disclosure, required in RESULTS:* at `m ≥ 7` the closure runs with 5–8 free variables. No gate checks the closure at that
  width: the G1 hand cases use `n_free ∈ {0, 1, 3}` (`examples/hky_fp_tower_check.rs`, measured), G3 reaches at most 4, and
  G4 runs at `m = 2, 3`. Rows at `m ≥ 7` therefore rest on the closure alone. They are reported in their own block with
  the label. Separately, Amendment 3's sentence that `m ≥ 7` "lies beyond … the closure's limits" no longer holds for the
  closure in the frozen image (`hky_fp_tower::MAX_FREE = 8`, and the hky-C stages exit 0). That is a fact about the
  instrument, recorded here.

**4.3 Options considered.**
1. **Keep the rows with the label (recommended).** This matches the registered definition of G3 ("where both run") and
   Amendment 3, which already names `m ≤ 6` as the F4 capacity. It keeps §5 C ("`m = 2 … 10`") and H-m as registered. It
   changes no hypothesis, rule, cell, seed or budget.
2. **Exclude `m ≥ 7` cells from every rule that needs G3.** This is the same as option 1 in effect, because no rule needs G3
   for closure values. Excluding them from *every* rule would cut ladder C to `m ≤ 6`, against §5 C, and would remove the
   cells H-m exists for. Not recommended.
3. **Substitute another engine for G3 at `m ≥ 7`** (raw `--engine f4 --cap c`, which runs there). Not recommended: its
   truncation is by degree in the polynomial ring with the tower equations as input, not by normal-form degree in the
   quotient, so "refutes at cap `c`" is not the §3 statement. It is a new instrument and would need its own derivation and
   amendment.

**4.4 Mechanism: frozen image, no new image (derived; development-checked).** A one-off G3-only command job on the frozen
image `…/isolab/tasks@sha256:0d6b2ecd…3ae8`. It copies the 19 within-capacity stage C run files of `pfd-c-20261108`, chosen
by file name, that is by `(m, t, p)` only, and runs the frozen `prime_fall_degree_tools gate3` with the registered flags
(`--max-n 20 --jobs 2 --budget 900`). The pass test is the one in `entry.sh` (`', 0 disagreements'` and exit 0).

Why it works with what exists:
- The command-request validator (`taskq/deploy/eks-karpenter/remote/run-command`, jq; sha256 `1cdaa744…`) accepts any command
  array of newline-free strings of at most 8192 characters.
- The image runs `/bin/bash`, `aws`, `sha256sum`, `grep`, `nproc` and `mkdir`. `entry.sh` uses all of them, and `pfd-c`
  wrote `binary.sha256`, `host.txt` and synced its rows (measured from its outputs). No `jq` is needed.
- The job role can read the earlier job's rows. Live IAM policy `isolab-job-artifacts` v1, attached to role `isolab-job`,
  allows `s3:GetObject` on `runs/*` and `s3:ListBucket` with prefix `runs/*` (measured with `aws iam get-policy-version`). I
  did not check that the job pods assume `isolab-job` (heuristic from the name).
- The `//` in the source key is literal. The local aws-cli 1.46 fetched `gates/g3.log` through it (measured). The pod's aws-cli
  v2 wrote those keys (heuristic: it reads them the same way).

Proposed `job.command` (frozen verbatim with the amendment):

```
["/bin/bash","-c","set -u; : \"${ISOLAB_RESULTS_URI:?}\"; S=/work/g3c; mkdir -p \"$S/runs\" \"$S/gates\" || exit 2; SRC='s3://isolab-results-104186811093-us-east-2/runs/pfd-c-20261108/3eeef6a6-9565-415c-918c-b3f92801aafc//runs'; for f in C-t2-m2 C-t2-m3 C-t2-m4 C-t2-m5 C-t2-m6 C-t2-m2-p0 C-t2-m3-p0 C-t2-m4-p0 C-t2-m5-p0 C-t2-m6-p0 C-t3-m2 C-t3-m3 C-t3-m4 C-t3-m5 C-t3-m6 C-t3-m2-p0 C-t3-m3-p0 C-t3-m4-p0 C-t3-m5-p0; do aws s3 cp \"$SRC/$f.jsonl\" \"$S/runs/$f.jsonl\" --only-show-errors || exit 3; done; export RAYON_NUM_THREADS=${RAYON_NUM_THREADS:-12}; (cd \"$S/runs\" && sha256sum *.jsonl) > \"$S/gates/g3-inputs.sha256\"; sha256sum /usr/local/bin/pkm_tower_pilot /usr/local/bin/prime_fall_degree_tools > \"$S/binary.sha256\"; { echo \"nproc: $(nproc)\"; echo \"rayon: $RAYON_NUM_THREADS\"; grep -m1 'model name' /proc/cpuinfo; } > \"$S/host.txt\"; /usr/local/bin/prime_fall_degree_tools gate3 \"$S\" /usr/local/bin/pkm_tower_pilot --max-n 20 --jobs 2 --budget 900 > \"$S/gates/g3.log\" 2>&1; rc=$?; grep -q ', 0 disagreements' \"$S/gates/g3.log\" || { [ \"$rc\" = 0 ] && rc=1; }; echo \"g3 exit $rc\" > \"$S/gates/g3.exit\"; aws s3 cp \"$S/binary.sha256\" \"$ISOLAB_RESULTS_URI/binary.sha256\" --only-show-errors; aws s3 cp \"$S/host.txt\" \"$ISOLAB_RESULTS_URI/host.txt\" --only-show-errors; aws s3 sync \"$S/gates\" \"$ISOLAB_RESULTS_URI/gates\" --only-show-errors; exit $rc"]
```

Resources (proposal):
- `cpu 12`, `memory 56Gi`, as `pfd-c`. Amendment 9 sets 12 vCPU for every relaunch job. The thread count matters here,
  because `gate3` counts a capped run stopped by its 900 s budget as a disagreement (`agree = seed_match && !timed_out && …`,
  `prime_fall_degree_tools.rs:533`).
- `seconds 57600`, above the 50,400 s upper bound. That is at most 90,000 s, so there is no instance pin under Amendment 9.

After the run, the main session compares `gates/g3-inputs.sha256` with the sha256 of the same 19 objects in `pfd-c`'s prefix.
A mismatch voids the job.

Development checks (measured, local, frozen binaries; not EKS):
- The command passes the validator, in a scratch copy only (not written as a request).
- It passes `bash -n`.
- A dry run with a fake `aws` and a plumbing fixture (stage A rows under a `C-*` name; no stage C data) wrote `g3.log`,
  `g3.exit`, `g3-inputs.sha256`, `binary.sha256` and `host.txt`, and exited 0.
- The frozen `gate3` on the copied `A-m2-tower*.jsonl` (`--max-n 8`) gave "48 capped runs, 0 disagreements". All 48 lines
  match `pfd-a`'s `g3.jsonl` on refuted, timed-out, `D`, agree and closure fields.

Not chosen: running `gate3` locally on the frozen static binaries. The registered host and thread count are EKS with 12 vCPU,
and timeouts count as disagreements.

**Fallback, a new image (only if the command job is refused).** The smallest change is in `gate3`
(`examples/prime_fall_degree_tools.rs`, after the `max_n` filter):
`if u(&r, "m") > 6 { eprintln!("G3 n/a (F4 engine capacity) {:?}", key(&r)); continue; }`. Optionally, call `ring.check()` in
`tower_ring` so that the pilot fails with its own message. That means a rebuild with `build_layer_r2.sh`, one new digest
and new requests. Not recommended while 4.4 works.

**4.5 `pfd-b`.** No change is needed for this limit. Every cell of `hky-B`, `d-B` (tower engine, `m ≤ 6`) and its `g3`
(`m ≤ 6`) is within capacity (derived), and the boundary shapes run with the frozen binary (measured, §1). Freeze the
sentence "`pfd-b` runs `g1 hky-B d-B g3` as requested; all its G3 cells are within capacity (rule 4.1)" with this amendment, so
that the scope is fixed before `pfd-b` is submitted. A `g3` failure at the end of `pfd-b` does not remove its `hky-B` and `d-B`
rows, which are synced as they finish (`entry.sh`). This amendment does not address any other `g3` failure mode at stage B,
such as budget timeouts at `m = 5, 6`, which no gate run has exercised yet.

**4.6 `pfd-c`'s `hky-C` rows stand.** They were produced before `g3` and do not depend on it. The `g3` exit 101 and `pfd-c`'s
non-zero job exit are recorded as "G3 not run on stage C (tool scope defect)", not as a G3 result. Stage C's G3 is the job in
4.4.

## 5. Question for the main session (not decided here)

**May the `pfd-x` capped runs (Kummer `m = 2`, P1) proceed before stage C's G3 (4.4) has run?**

The registered text:
- §3: "Only after G3 passes may capped F4 extend reach where the closure does not fit."
- Amendment 7 B: "the procedure §3 and §5 D allow once G3 has passed".
- §4: "One disagreement stops the use of capped F4 as a `d_F` instrument" and "any failure stops the study".
- Amendment 3 recorded "G3 passed" once (`m = 2…4`, 70 runs). `pfd-a`'s G3 passed with 144 runs and 0 disagreements, all at
  `m = 2` (cited).

The plan text (`README.md` l.106) names `pfd-a`'s G3 as the gate for `pfd-x`. The registered text does not say whether G3 is
per stage, or whether `pfd-c`'s crash is a "failure" (§3 UNRESOLVED items).

Points for the decision (derived):
- If the crash is not a failure, then by the plan text `pfd-x` may proceed on `pfd-a`'s pass.
- Even so, a later disagreement in the stage C G3, at any `m ≤ 6`, would by G3's own sentence stop capped F4 as a `d_F`
  instrument study-wide, `m = 2` included. `pfd-x` rows produced earlier would then be reported but not used.
- Running 4.4 first costs at most about 14 h of wall time (heuristic). `pfd-a`'s 144-run G3 took 5 min (measured,
  `progress.txt`), and that removes the ordering question.
- The `pfd-m4-*` jobs are closures (`--engine hky`), not capped F4. Amendment 8's "after G3" applies to its capped F4 runs,
  not to the `m4:` closures (cited). Their "after G3" placement in `R2_READY.md` is plan text.

## 6. Unchanged

No hypothesis, §6 rule, cell, seed, budget, memory limit, image or existing request changes. The new element is one G3-only
command job (4.4), with a scope that depends on `m` only.

## 7. Requests and checks after the freeze (2026-11-09)

- `make_requests_r2e.sh image/image.json` writes `requests_r2e/` (18 requests). It is a copy of `make_requests_r2d.sh` with only
  three changes: the output directory; one sentence for Amendment 11 appended to `provenance.note`; and the added `pfd-g3c` request
  (`pfd-g3c-20261109`, cpu 12, 56Gi, 57600 s, no pin, command = §4.4 verbatim). All 18 are VALID (`r2e_validate_output.txt`). The 17
  existing requests differ from `requests_r2d/` only in `provenance.note` (the appended sentence) and `provenance.protocol_sha256`
  (`ef5e18ea…`). `requests_r2c/`, `requests_r2d/` and `requests_r2c_pending_amendment/` are byte-identical before and after (measured).
- Dry run (measured, local, frozen binaries fetched from the image layer): `.job.command[2]` of `requests_r2e/pfd-g3c.json` was run
  with only the `/usr/local/bin/` and `/work/g3c` paths substituted, a fake `aws`, and a plumbing fixture (stage A rows `A-m2-tower*`
  under `C-t2-m2*` names, the other 17 files empty; no stage C data). It exited 0 with "G3: 104 capped runs, 0 disagreements" in
  4 min 28 s on the 4-core development host, and uploaded `binary.sha256`, `host.txt` and `gates/{g3.log,g3.jsonl,g3.exit,g3-inputs.sha256}`.
