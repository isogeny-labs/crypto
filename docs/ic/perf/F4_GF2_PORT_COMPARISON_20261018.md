# f4_gf2 port: same-binary comparison against this crate's Boolean F4 kernel (2026-10-18)

Task T14 (`RESEARCH_IC_LEADS_ASSESSMENT_20261018.md`; port 1 of
`docs/ic/STACK_DIVERGENCE_20261018.md` section 5.2). The sibling suite's
rank-indexed `f4_gf2` kernel is ported as `src/cryptanalysis/f4_gf2.rs`; this
page is the gain over THIS crate's default kernel, which the suite never
measured (the suite's 10.3x PDP and 190-280x degree-sweep figures are against
the suite's own reference kernel).

**Class: engineering** (a kernel-level wall-time improvement at one cell and
one degree, answers equal). No end-to-end `S` claim, no rho comparison, no
default `SolverEngine` change. A faster kernel inside the solver becomes a
pipeline improvement only when it moves end-to-end `S`; that is the follow-up
(wiring the port in as an engine variant).

## Host and build

4-vCPU KVM guest, Intel Xeon 2.80 GHz, 15 GB, no PMU (no cycles or IPC
figures), shared with other worktrees' builds (no isolation tier: L0). Release
profile, `CARGO_PROFILE_RELEASE_DEBUG=0`, one test binary for all arms, so the
allocator, flags and inputs are identical. Wall time is `Instant` around each
call; arms alternate order by draw parity (forward on even draws, reversed on
odd) after one untimed warm-up of each kernel.

## Commands

```text
cd <worktree>
export CARGO_PROFILE_RELEASE_DEBUG=0
# agreement tests (always on, 0.4 s): 5 passed, 1 ignored
cargo test --release --lib -- f4_gf2
# comparison (ignored by default; 20 draws, ~13.5 s for all four arms)
cargo test --release --lib -- f4_gf2_same_binary_comparison --ignored --nocapture --test-threads=1
# other cells: F4_GF2_CMP_N, F4_GF2_CMP_ELL, F4_GF2_CMP_M, F4_GF2_CMP_DEGREE, F4_GF2_CMP_DRAWS
```

`--test-threads=1` because the crate-solver arm reads its XOR count from the
process-wide `f4_profile()` atomics.

## What is compared

Cell: Koblitz Weil-descended chain, `m = 3`, `n = 15`, `ell = 5` (30 Boolean
variables, 30 equations), Macaulay degree 4, 20 draws of `ladder_sample`
(seed `0xC04A5E`), default caps (20 000 rows, 40 000 columns; none hit). Arms:

| arm | call |
|:--|:--|
| crate full | `matrix_f4_f2_counted_with(.., RowCriterion::None)`: all reduced rows |
| crate solver | `matrix_f4_f2_solver_consequences(.., RowCriterion::None)`: what `SolverEngine::MatrixF4` calls (made `pub(crate)`, the only edit outside the new module) |
| port F5 on | `f4_gf2::decide_with(.., f5: true)`: the suite default |
| port F5 off | `f4_gf2::decide_with(.., f5: false)` |

All four arms give the same decision on every draw (asserted per draw). At
this degree no draw is decisive (0 refuted, 0 forced of 20), so the cost is
the cost of a full degree-4 pass that finds nothing; the decisive cases are
covered by the agreement tests, not by this timing.

## Result (run of 2026-10-18, per draw, ms)

```text
draw    crate full crate solver   port F5 on  port F5 off
   0       253.105      384.968       34.529       39.147
   1       232.334      399.305       35.536       40.153
   2       222.075      420.725       36.780       40.792
   3       213.129      403.019       34.455       39.120
   4       205.303      399.754       36.959       54.820
   5       207.606      403.861       55.671       46.800
   6       196.285      391.644       35.844       39.937
   7       175.064      380.631       34.128       47.073
   8       186.595      402.517       36.629       41.204
   9       197.462      404.807       36.089       41.221
  10       199.867      389.626       33.532       37.509
  11       174.973      393.999       34.528       38.270
  12       200.356      368.862       35.598       38.910
  13       196.826      418.336       36.831       40.011
  14       192.101      378.177       34.195       37.544
  15       177.017      386.188       34.495       39.084
  16       182.022      370.073       34.986       85.474
  17       207.869      411.430       35.082       39.201
  18       199.162      388.690       35.978       40.482
  19       200.148      399.322       37.541       40.277
```

| arm | total ms | median ms | min | max | median 64-bit word XORs | median speedup vs crate full |
|:--|--:|--:|--:|--:|--:|--:|
| crate full | 4019.3 | 199.5 | 175.0 | 253.1 | 154 963 841 | 1.00x |
| crate solver | 7895.9 | 396.7 | 368.9 | 420.7 | 64 342 540 | 0.50x |
| port F5 on | 729.4 | 35.6 | 33.5 | 55.7 | 57 472 744 | 5.61x |
| port F5 off | 867.0 | 40.1 | 37.5 | 85.5 | 68 746 716 | 4.98x |

Medians over four full runs (this one and three earlier runs of the same
code): crate full 199-224 ms, crate solver 386-397, port F5 on 35.6-37.3, port
F5 off 40.1-42.2. Port F5 on against the crate solver path: 10.4-11.2x
(this run 11.15x); against the full-readback path: 5.6-6.0x. The max entries
(55.7, 85.5) are single outliers on a shared host.

Degree 3 on the same cell (one run, 20 draws): crate full 1.66 ms, crate
solver 1.00, port F5 on 0.34, port F5 off 0.33 (medians); 4.9x and 5.0x
against crate full, 2.9x against the crate solver path.

## Reading

- The port is faster here by a measured margin, 5.6-6.0x against the crate's
  full-readback kernel and 10.4-11.2x against the call the engine makes, with
  equal decisions. Distance to rho: not measured; this is a kernel result.
- Word XORs: the port's elimination count is 57.5 M (F5 on) or 68.7 M (F5 off)
  against 64.3 M for the crate solver path and 155.0 M for the crate full
  path. The port's counter is elimination only; the F5 plan's sorted-list
  XORs are not in it (the crate's F5 criterion does count its own). The
  fully counted comparison is therefore F5 off, 68.7 M against 64.3 M (the port
  does 1.07x more XORs than the crate solver path) at 9.9x less wall time (median 396.7 ms against 40.1 ms):
  the gain comes from the rank-indexed build (no hash map, no sorts, XOR rows)
  and the absent polynomial readback, not from a cheaper elimination.
- The crate solver path is 2x slower than its full path at 30 variables. It
  switches to active-multiplier, linear-tail and flat-matrix builds at 24 or
  more variables (`KIC_F4_*` overrides). Probes of 3 draws at degree 4: 22-23
  variables equal within noise (0.86x, 1.16x), 27 and 28 variables 1.32x and
  1.05x faster than full, 30 variables 0.50x. Whether that switch is a
  regression in the crate's own default at these widths is a separate finding
  worth a controlled A/B before anyone relies on it (3 draws per probe cell
  here, not a frozen protocol).
- Not measured: other cells and degrees (degree 5 exceeds the default row
  cap at 22 or more variables); a splitting-search run (node counts, end-to-end
  `S`); decisive systems under timing; isolated or repeated placement.

## Agreement (the correctness half)

`cargo test --release --lib -- f4_gf2`: 5 passed, 1 ignored (the timing test).

- Koblitz Weil-descended systems, cells (n, ell) = (6,1), (9,1), (8,2), (7,2),
  `m = 3`, 20 draws each at degree 3 and 4 (160 comparisons): 37 refuted,
  73 forced-variable, 50 undecided.
- Random dense systems (9 variables, 8 equations, each monomial of degree at
  most 2 with probability 1/4), 20 draws at degree 3 and 4: degree 3 all
  undecided, degree 4 19 of 20 decisive.
- For each system: reduced rows bit-identical to `matrix_f4_f2`, decision
  equal with F5 on and off, the crate solver path's decision equal, oversize
  behaviour equal. The F5 on/off test also asserts rows are left out
  (142 over 6 draws at degree 3 and 4).
