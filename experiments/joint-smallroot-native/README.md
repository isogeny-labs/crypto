# Native weighted joint-smallroot construction — prospective source

TASK-20261026-589ad5, BATCH-ee839d. **review_required; approved_by=null.**
No symbolic bound cell, sign/tree control, ideal/closure construction, interpolation,
calibration, lattice reduction or root extraction has run. The original reach-cost
tradeoff and verified improvement over a named frozen current prime-field IC
baseline remain unresolved. Rho is a separate reference.

The C++17 source implements the restricted QQ/formal-p construction with GMP,
native Singular4.4.1 and a separate rational cddlib/GMP helper. Native function
bodies are compiled. Compilation and eight sealed structural fixture groups do
not establish cuso equivalence, theorem applicability or scientific readiness.

```
make -C experiments/joint-smallroot-native -j2 all
/workspace/native-smallroot-build/structural \
  /workspace/skeleton_sage-runtime/bin/Singular \
  /workspace/native-smallroot-build/cdd-helper /workspace/FRESH-structural-output
```

Expected values were sealed in `spec/structural-expected.txt` before the first
invocation. `spec/admission-expected.txt` seals two missing/unapproved envelope
codec rejection fixtures. Exact successful and unsuccessful build logs and the
final structural Singular/cdd inputs, outputs and pivot trace are in `spec/checks/`.
The raw records stay loose because the manifest binds their individual paths.
The native source uses no Python runtime or research wrapper. Native clang-format
was provisioned as a software-formatting tool; document rendering is separate.

`src/model.hpp` fixes all24 cells and exact input serialization, including active
coordinate projections, identical A/A algebra bytes and nearby-v1 SHA256
coefficients. `src/exact.hpp` implements rational sparse polynomials, signed Sage
comparison, first stripped leading coefficient, FIFO closure, constant-pivot
intersection and rational interpolation/limit. `src/backend.hpp` emits and retains
native Singular scripts. Sage10.9 explicitly maps wdeglex to Singular Wp; the
structural comparator fixtures test that representation. `src/polytope.hpp`
enumerates exact facets/equalities and discrete augmented sums. No floating hull
or fraction-field matrix elimination is used.

`src/cdd_helper.cpp` is an independently written GPL-2.0-only generic vertex-to-H
converter. The cuso-derived LGPL-3.0-or-later core exchanges decimal rational text
with this separate executable and does not link cddlib. Installed cddlib metadata
says GPL-2.0-only, GMP offers GPL-2.0-or-later OR LGPL-3.0-or-later, and OpenSSL
says Apache-2.0. The process boundary is recorded for review; it is not a legal
compatibility certification. No binaries are distributed. License texts and
actual dependency/binary hashes are retained.

The prospective production interface is `--approved-panel ADMISSION`, which
requires source/protocol/runbook/trialplan/dependency/binary digests and external
approval/lint/review receipt fields. Workers also require their actual supervisor
cgroup and8GiB memory.max. This local envelope is additional to programme
preapproval/Director/Coordinator gates; authoritative receipt interpretation
and the final exact packet must still be independently reviewed. No valid
admission envelope exists here, and this interface has never been invoked with
one. The supplied template deliberately rejects.

The supervisor source requires at least12GiB MemAvailable, a writable delegated
cgroup-v2 with memory.max, memory.peak, memory.events and cgroup.kill; launches one
cold worker at a time after attaching its stopped process; applies a monotonic
1800-second watchdog; and retains process-tree kill/status/CPU/memory records.
Current host cgroupfs is read-only: **infrastructure-not-started**. No RSS-polling
or unguarded fallback is provided. Actual delegation, watchdog/tree-kill behavior
and native admission integration remain unverified.

A proposed control expansion prepares four fixed(r,mu) planted-tree
ideal/domain/closure constructions with exact denominator-cleared101^mu shift
divisibility before the24 bound calls. It computes no bound. This makes the
original control's shift requirement concrete but adds explicit prospective
control workload: Coordinator amendment/adoption and an exact reviewed control
budget are required before running. The envelope requires
`control_amendment_approval`; source does not grant that approval.

Source-semantic gaps remain: basis/normal-form cross-version comparison has not
run; Sage integral-point iteration order is replaced with a deterministic lex
list before the reference M sort; augmented None valuation and zero-leading
failure timing are classified rather than fully reproducing Sage exceptions;
per-phase CPU receipts separate worker-self and waited-child user/system
microseconds; individual wait4 receipts are diagnostics, never extra addends.
Full phase process-tree CPU remains unknown; formal specialization/height/theorem
and extraction obligations remain open. `spec/director-request.md` names the smallest
repair checks and forbids treating them as authorized now.

Literature: pinned cuso fb2747d2105a8e48499bf10a3a717cf80c14d079 source and license,
Sage10.9 comparator/matrix/order/ideal source, frozen BATCH-3cfef2 design and the
current lead were read. Exact Ryan eprint2024/1577 PDF returned403. Ryan's2025
UC San Diego dissertation, chapter3, was identified as an alternative primary
source; this session's fulltext retrieval returned429, so the full chapter and
the Algorithm3/Lemma7 assumptions remain unread. No byte equivalence between
paper versions, novelty or theorem applicability is asserted.

Guard qualification continuation TASK-20261026-6374dd adds only controlled native
test drivers/workers and a compile-only `guard-qualification` Makefile target.
Production source remains pinned to86ddd19f67e08160c22a0d8ba9a32a92321d18ad. The
sealed five-class plan uses the unchanged production1800-second watchdog and
exact8GiB worker cap; it never invokes a scientific envelope or algorithm. Raw
records under `spec/checks/TASK-20261026-6374dd/` are individually content-bound
by the programme owner-source manifest, including failures and not_started cases.
An exact parent-authorized immutable cache subset permits at most one read-only
fadvise pass; advice is not reclamation proof. Q4 still requires measured12GiB
visible enclosing headroom, excluding the worker cap, and effective zero swap.
The final programme qualification-results/report state actual class outcomes;
source presence or a partial run is not full guard or scientific readiness.

Initial qualification outcomes: Q1/Q2/Q5 passed; Q3 failed because the new fixture's
generic orphan cleanup consumed the competitor status before its dedicated wait4
receipt, despite retained genuine1800.071s watchdog/lock/tree-cleanup observations.
Q4 was infrastructure_not_started: measured visible enclosing headroom was
12439695360bytes, below12884901888 required, after the single authorized cache pass;
no OOM worker was launched. All five original rows/raw records and initial source/
runtime/seal are frozen under the task check directory. No full guard qualification
or scientific readiness is established; any repair/replay requires separate admission.

## Panel exception-accounting repair (2026-10-30)

The supervisor now reports process creation to the panel before any later
startup, accounting or receipt operation can fail. A post-launch exception
therefore records `failed` with explicitly incomplete supervisor accounting;
`not_started` is reserved for failures before the worker process was created.
If the supervisor returned an outcome and the subsequent health check throws,
the panel retains `complete`, `failed` or `censored` and discloses the failed
post-run accounting check. Every exception stops admission of later cells.

`make panel-status` runs eight injected supervisor/health-failure software
fixtures, including all three terminal outcomes after a health-check error.
The existing 17 CPU-accounting fixtures also pass, and the production runner
and guard-qualification drivers compile with strict warnings. These software
checks do not validate real delegated-cgroup behavior, launch scientific cells,
or extend historical source-bound qualification receipts to the repaired code.
