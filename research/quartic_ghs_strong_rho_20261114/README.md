# Strong-rho control for the GHS-weak plane-quartic frontier

Status: Round A implementation and protocol frozen; measurement pending. The
[round-A amendment](ROUND_A.md) changes only the high/top active deadlines and
was frozen before primary output. [Round B](ROUND_B.md) prospectively replaces
the generic six-product cubic square with a five-product specialization. Its
profile and benchmark are checkpoint-blocked in the current runtime, so it has
no measured throughput result, frontier change, or leaderboard row. The live
execution-admission gap is tracked in
[the problem record](../../problems/ghs-strong-rho-measurement-admission.md).

The reviewed 2026-11-18 engineering composition layers reusable hot-loop
scratch storage and hash caching, reciprocal base-field reduction, direct
cubic inversion, corrected jump-only setup batching, and an optional
field-counter-free build. Deterministic controls and their source-bound counts
are in [`round-c-integration/`](round-c-integration/RESULTS.md). They establish
the combined implementation and count identities only; checkpoint admission
remains unavailable, so no throughput, end-to-end, frontier, or leaderboard
claim is made `[pcc:unverified]`.

This study reopens the restricted `E0/F_{q^3}` plane-quartic program for one
specific obligation: compare its end-to-end index-calculus result with a rho
reference that combines the negation map, lockstep walks, Montgomery batch
inversion, and six-product cubic-field multiplication. It preserves the frozen
N4 and N4-v2 sources and runs their historical rho in the same driver as a
control.

The preregistered primary is `strong-b8`. A development-only width pilot fixed
that choice before paid data: wider batches saved inversions but increased
collision work, so batch 64 was not retained as the reference
`[pcc:unverified]`. The walk also applies the frozen rho-neg look-ahead rule and
escapes residual cycles by doubling the cycle's canonical minimum.

The scope remains public synthetic known-answer instances in the special GHS
family. It says nothing about generic, standard, or deployed curves. Live
requests, benchmark records, a validation receipt, and any dashboard update
will be added only after their gates pass and before a performance conclusion
is made [pcc:unverified].

## Package map

- [`PROTOCOL.md`](PROTOCOL.md) freezes the scientific question, grid, arms,
  estimator, cost model, admission interval, and publication gates.
- [`ROUND_A.md`](ROUND_A.md) is the prospective deadline-only amendment.
- [`ROUND_B.md`](ROUND_B.md) freezes the five-product squaring lead;
  [`round-b/RESULTS.md`](round-b/RESULTS.md) retains its partial
  correctness/count result and checkpoint blocker.
- [`HOTLOOP_PROTOCOL.md`](HOTLOOP_PROTOCOL.md) and
  [`HOTLOOP_RESULTS.md`](HOTLOOP_RESULTS.md) record the reusable scratch,
  cached-hash, partition-mask, and table-reservation controls.
- [`ROUND_C.md`](ROUND_C.md) and
  [`ROUND_C_DEVIATION.md`](ROUND_C_DEVIATION.md) specify reciprocal reduction,
  its explicit cold setup-division diagnostic, and the unmeasured width screen;
  [`round-c-reduce-retune/`](round-c-reduce-retune/RESULTS.md) retains the
  branch-local evidence.
- [`ROUND_C_INV.md`](ROUND_C_INV.md) specifies direct-adjugate inversion;
  [`round-c-inv/`](round-c-inv/RESULTS.md) retains its branch-local controls.
- [`ROUND_C_SETUP.md`](ROUND_C_SETUP.md) and
  [`round-c-setup/`](round-c-setup/RESULTS.md) retain the rejected first setup
  attempt for audit. [`ROUND_C1_SETUP.md`](ROUND_C1_SETUP.md) and
  [`round-c1-setup/`](round-c1-setup/RESULTS.md) contain the corrected
  jump-only candidate used by the combined implementation.
- [`ROUND_C_RETRY.md`](ROUND_C_RETRY.md) and
  [`round-c-retry/`](round-c-retry/RESULTS.md) retain the exact negative retry2
  screen. Its production source is intentionally excluded from the combined
  default path.
- [`round-c-counterfree/`](round-c-counterfree/RESULTS.md) records the optional
  `fast-b*` field-counter-free mode and the analyzer's fail-closed authority
  gate. Fast records are diagnostics and cannot enter modeled-cost analysis.
- [`../quartic_ghs_strong_rho_parallel_20261118/IMPLEMENTATION_SPEC.md`](../quartic_ghs_strong_rho_parallel_20261118/IMPLEMENTATION_SPEC.md)
  defines the source changes, counters, controls, matched measurement grid,
  comprehensive ECBench rerun, publication gates, and follow-on PR sequence.
- [`../quartic_ghs_r0_r99_20261119/`](../quartic_ghs_r0_r99_20261119/)
  expands that contract into one hundred explicit R00–R99 lanes, dispatched
  in six disjoint parallel batches with serialized composition and promotion
  gates.
- `main.rs` and the Round A revision of `strong_rho.rs` are the
  source-hash-pinned Round A measurement code. The current `strong_rho.rs` is
  the combined prospective engineering candidate; Round A remains bound by
  the source hash in [`ROUND_A.md`](ROUND_A.md).
  [`pod_run.sh`](pod_run.sh) compiles it for the worker and seals each split.
- [`build_context.sh`](build_context.sh) and [`Dockerfile`](Dockerfile) create
  the exact-revision private image; [`LAUNCH.md`](LAUNCH.md) specifies the
  operator receipts and two-wave submission sequence.
- [`make_requests.py`](make_requests.py) refuses incomplete or stale live
  admission evidence and emits no request until budget arithmetic, all-tenant
  capacity, checkpoint, Git custody, image digest, and source bindings pass.
- [`analyze.py`](analyze.py) audits the historical archive and, once all four
  raw split manifests exist, recomputes the frozen producer table. Its output
  remains unverified until the separate validation/PCC chain admits it.

Focused development checks are in `test_make_requests.py` and
`test_analyze.py`. The exact historical input can be checked without making a
performance claim:

```sh
python3 analyze.py audit-historical \
  --archive ../quartic_n4v2_20261020/results/grid-20261025/units.tar.gz
```

The 2026-11-14 local audit read 63 archive members, 261 successful arm units,
and 33 common targets; q = 4,194,271 intentionally has no B0 unit
`[pcc:unverified]`. This is an input-census check, not the pending strong-rho
measurement.

Development checks:

```sh
cd research/quartic_ghs_strong_rho_20261114
rustc --edition 2021 -O main.rs -o /tmp/ghs-strong-rho
/tmp/ghs-strong-rho selftest
/tmp/ghs-strong-rho run 223 1 0 2 ic-repeat old-neg,strong-b4,strong-b8,strong-b16
```

`ic-repeat` is the primary policy: every independent rho walk uses the exact
target consumed by the frozen N4-v2 IC run for the same `(q, seed)` pair.
`sequence` retains that target at index zero and emits deterministic holdouts
after it for correctness-only sensitivity checks.

Sources checked before implementation:

- `research/quartic_n4_20261019/{PROTOCOL.md,README.md}` — frozen historical
  pipeline and serial rho.
- `research/quartic_n4v2_20261020/{PROTOCOL.md,README.md}` — negation-map rho,
  exact family, and unbatched-inversion limitation.
- `research/ghs3_vs_rho_20261023/{PROTOCOL.md,TABLES.md}` and
  `src/cryptanalysis/jv_rho_strong.rs` — existing same-repository strong-rho
  control design.
- Diem (ANTS VII, 2006), Diem–Thomé (J. Cryptology 21, 2008), and
  Gaudry–Hess–Smart (J. Cryptology 15, 2002), bibliographic records in
  `papers/ecdlp-index-calculus-program/refs.bib`.

KB receipt: the research-kb MCP was not exposed and the wrapper found no
executable. The relevant lead-atlas YAMLs were visible but vault-encrypted in
this runtime. No absence or novelty claim is based on that failure.
