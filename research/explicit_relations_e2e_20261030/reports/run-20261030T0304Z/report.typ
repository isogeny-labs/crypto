#set page(paper: "a4", margin: 17mm)
#set text(size: 9.5pt)
#set heading(numbering: none)
= Full recovery on one RTX PRO 6000

This is an observation report for a bounded engineering study. It reports
complete public-input recoveries, failures and explicit limits. It does not
certify a speedup, establish a largest possible ECDLP size, promote a research
hypothesis, or implement the paper's conditional tensor contraction algorithm.

== Implementation and accounting
The standalone reduction now certifies a prime group order using a nonzero
prime-order point and a unique multiple in the exact integer Hasse interval.
Tonelli–Shanks replaces the exhaustive root table. Modular products use u128
when required. Deferred dependency reconstruction removes repeated dense
provenance propagation; certificate replay uses sparse relation entries.
The original tiny constructor and default 512-column cap remain. Explicit
limits admit up to 16384 columns; the driver accepts `p < 2^60` and `r < 2^61`.

GPU inverse-hash base construction and generic meet-in-the-middle relation
finding feed CPU elimination and scalar extraction. Public Q is obtained by
coordinate lifting; its scalar is neither generated nor supplied. Every
success is serialized, replayed in a fresh CPU verifier, compared byte-for-byte
across arms where prescribed, and subjected to a scalar-plus-one rejection check.

Each cold solve includes target-coordinate construction, process/GPU startup,
curve validation, all base trials, relation collection, elimination, extraction,
in-process replay, serialization and teardown. Total audit time also includes
fresh replay, corruption rejection and certificate comparison. Fixture
construction, compilation and preflight are recorded separately. Shared
RunPod is L0; every exported wall_valid flag is false.

== Initial ladder: all 44 prescribed executions
Source: `614791252805f64a618a8ce0080e8f4f040c3a8f`.
Pod: `ao7xe0bzb6wgz9`, started 2026-10-30T03:19:20UTC, one RTX PRO6000 Blackwell
Server Edition. Frozen protocol-v3; seeds 101/202, eight slots,
tau=2ceil(r^(1/8))+2, max_rows=2^29, chunk=2^20, max_work=2^30,
max_columns=16384, max_trials=16. Solve deadline 300 s; replay 60 s.

There were 40 independently replayed recoveries through 52-bit fields and
four 56-bit timeouts. Both GPU invocations and both public targets passed at
52 bits. CPU reference ran only at 20/24 bits and matched exact certificates.
The native analyzer passed; controller and worker exited 0; pod deletion was
verified. All 13 Rust tests, host/CUDA differentials, reduction/replay,
Clippy, memcheck, racecheck and synccheck passed.

#table(columns: (1fr,1fr,1fr,1fr,1fr), inset: 4pt,
 [Field bits], [Cold solve(s)], [Oracle(s)], [Linear(s)], [Relations/attempts],
 [20], [0.434163], [0.010898], [0.000162], [57/57],
 [24], [0.374702], [0.013362], [0.000211], [71/86],
 [28], [0.348764], [0.011190], [0.000457], [105/106],
 [32], [0.369914], [0.022081], [0.000781], [128/215],
 [36], [0.541954], [0.118107], [0.002500], [183/300],
 [40], [0.729476], [0.290260], [0.006634], [267/380],
 [44], [1.172951], [0.661162], [0.016964], [384/505],
 [48], [9.034036], [8.175220], [0.047612], [495/954],
 [52], [64.277743], [63.106494], [0.143088], [743/1050],
 [56], [300.009571 timeout], [Incomplete], [Incomplete], [Incomplete],
)
Recorded values for arm u / seed 101, rounded for display; no averaging or pooling.
All other executions are retained in rows.csv and the graph. At 52 bits, the
four cold solves were 64.059441,64.277743,71.206916,71.240142 s. Oracle time
was 62.884962–70.030751 s; linear time was 0.117555–0.143088 s. These stage
observations motivated a lookup index. They do not identify hardware counters
or establish which kernel instruction dominates.

#pagebreak()
== Every initial-ladder execution
#image("scaling-v3.svg", width: 95%)

Presentation rendering of recorded rows; log seconds, all 44 executions,
including all four timeouts. Run ao7xe0bzb6wgz9; two fixed targets per field,
GPU u/x on 20–56 bits, CPU h on 20/24 bits. A timeout is a limit observation,
not a proof of nonexistence or hardware impossibility.

== Preserved launch failures and export boundary
Version 1, pod liii98dyd3b7to, failed before compilation because wrapper commands
were joined. Version 2, pod t9kc1zn12mr2tc, passed 13 Rust tests and device/preflight
checks, then failed to compile the native chart analyzer. Both produced zero
ladder observations. Their full archives and deletion receipts are retained.
Version 3 corrected the analyzer literal and made instrument compilation use
Rust 2021 with warnings denied; the original frozen protocols are unchanged.

The v3 export passes canonical schema, flow and file-hash validation. The
protocol-aware checker has three retained coverage findings: it assumes
3 arms×10 fields×2 seeds=60 rows, whereas the frozen nonrectangular plan explicitly
contains 44 rows because CPU h runs on two fields only. The native frozen
analyzer verifies the actual 44-cell contract. This report does not claim a
full canonical protocol-checker pass or scientific validation-gate approval.

Raw archive SHA-256:
#text(size: 8pt)[`b59d596b382be86e1734c8d09eafa70f11e69baa5b08fdb73bc5b67962128dfb`].
Archive: `s3://setec-astronomy-artifacts/validation/ecbench-explicit-x/e2e-20261030-v3/results.tgz`.
`results-v3/archive-manifest.json` hashes every raw member;
`certificate-audit.json` retains each public input, certificate digest,
positive replay output and negative-control output. The manifests supplement
the executed verifier, not replace it.

== Scope and unresolved comparisons
The relation oracle is generic decomposition, bounded by the square-root
barrier. No matched rho or fastest external-tool comparison was run here.
This is not an implementation of the paper's conditional subexponential
tensor backend. The root crypto/ecbench package was not compiled locally
because of the user's disk constraint; standalone cloud validation and
current-head repository CI are distinct checks.

Primary mathematical sources: the supplied paper §1.2/§3; the repository's
order-witness arithmetic; Hasse/point-order notes at
https://www.hyperelliptic.org/tanja/teaching/isogeny-school21/csidh-sidh-week-3.pdf;
PARI ellcard documentation at
https://pari.math.u-bordeaux.fr/dochtml/html-stable/Elliptic_curves.html.

#pagebreak()
= Indexed lookup: complete 56-bit rung

Frozen protocol-v4, source
`320694bfff1eaed8e2471e9335d9e7cfa94e8ad4`, pod as4eurugkqwpy4,
started 2026-10-30T04:20:00UTC. One RTX PRO6000 Blackwell Server Edition,
97,887 MiB, driver 595.91.07, CUDA 13.2.51. The source adds at most 65,537 u64
prefix offsets to narrow each full-key binary search. Infinity uses the full
table; duplicates preserve the lowest original tuple id. Setup and index
storage are included in cold timing and GPU admission.

All 20 prescribed executions through 58 bits ran: 16 verified recoveries through
56 bits and four 58-bit work-limit failures. The frozen stopping rule left
four potential 60-bit executions unattempted. All successful certificates
matched across arms and passed fresh replay and scalar-corruption rejection.
The native analyzer, 13 Rust tests, host/CUDA/reduction checks, Clippy and
index-specific memcheck/racecheck/synccheck passed. Controller and worker
exited 0, and pod deletion was verified.

#table(columns: (1fr,1fr,1fr,1fr,1fr), inset: 4pt,
 [Bits], [Seed], [Binary b(s)], [Indexed u(s)], [Identical x(s)],
 [48], [101], [8.096602], [4.833402], [4.762925],
 [48], [202], [7.799079], [5.051183], [5.073777],
 [52], [101], [59.337260], [38.096282], [36.137007],
 [52], [202], [65.950374], [40.096043], [40.053185],
 [56], [101], [Not prescribed], [211.902261], [212.031528],
 [56], [202], [Not prescribed], [231.374448], [230.939895],
 [58], [101/202], [Not prescribed], [Work limit], [Work limit],
)
Recorded cold solve seconds, rounded; each successful cell is one complete
recovery plus in-process replay. Additional independent replay/audit costs
are separately recorded in rows.csv. No values are pooled across hosts or
versions. The seed 101 52-bit u/x cold values differ by 1.959275 s despite
almost identical oracle times; this variation is retained, not removed.

At 58 bits, seed 101's planning cap selected 4,033,512 left rows and
87,620,130,000 right rows; that exceeded max_work=2^30. All four 58-bit failures
were explicit admission/work failures, not exhaustive unsuccessful searches.
Version 5 separately raises the explicit row/work caps and tests bounded
partial-sum reuse; v4 is unchanged.

#image("scaling-v4.svg", width: 95%)

All 20 recorded executions, rendered from unchanged rows. Red 58-bit markers are quick
admission failures, not fast recoveries. A/A repeats are the exact same
indexed service/driver. These L0 timing observations do not certify a speedup.

The export passes schema/flow/file hashes. Its five retained protocol-aware
coverage findings arise from the checker's rectangular 30-cell assumption;
the frozen contract has at most 24 cells and explicitly stops after the first
incomplete rung. Canonical flow calls failed processes completed outcomes:
20 process outcomes comprise 16 recoveries and 4 failures, as coverage.json states.

Raw archive SHA-256:
#text(size: 8pt)[`b93292c25ee0759e94a8f1bfeb89a342a0fd5a4b2701420a760b1e7c948960d1`].
Archive and complete certificate/log manifest are linked in results-v4.

#pagebreak()
= Partial-sum reuse: one 58-bit recovery

Frozen protocol-v5, measured source
`f6338418572c72e1bf97ad24903cd5efd17cf7da`, pod `9fil2lhphdymz0`,
started 2026-10-30T04:51:28 UTC. Two contiguous right-side partial sums were
cached only when each fit the explicit chunk bound. Their construction and
storage were included in setup and admission. The uncached indexed search
was the matched baseline; both arms used max_rows=2^30 and max_work=2^31.

Ten executions ran: nine independently replayed recoveries and one timeout.
Both targets completed at 56 bits. At 58 bits, seed 101 completed and seed
202 reached its 600-second deadline. The stopping rule left both 60-bit
cases unattempted. For v5, the largest complete two-target rung is 56 bits, while the largest
individual verified recovery is 58 bits; later panels are separate below.

#table(columns: (1fr,1fr,1fr,1fr,1fr), inset: 4pt,
 [Bits], [Seed], [Uncached b (s)], [Cached u (s)], [Identical x (s)],
 [52], [101], [38.074392], [39.173036], [38.890065],
 [52], [202], [40.590157], [42.322563], [42.127127],
 [56], [101], [Not prescribed], [217.426942], [Not prescribed],
 [56], [202], [Not prescribed], [240.146670], [Not prescribed],
 [58], [101], [Not prescribed], [595.862060], [Not prescribed],
 [58], [202], [Not prescribed], [600.006124 timeout], [Not prescribed],
)
Recorded cold solve seconds, rounded. Cached and A/A executions were slower
than their matched uncached baseline at both 52-bit targets. Consequently
ordinary builds retain uncached indexed search; caching is explicitly
available with `make cuda-cached`. The running panel was never changed.
No larger-field matched timing comparison was prescribed.

The successful 58-bit recovery used 1,251 relations from 1,714 attempts.
Its recorded oracle time was 593.926630 seconds and linear time 0.725390
seconds; complete execution with additional audit took 596.139162 seconds.
The retained table had 605,026,800 left rows and 584,134,200 right tuples,
with conservative admission 58,084,287,312 bytes. The other target admitted
583,697,295 left rows and 535,791,186 right tuples, then timed out. Neither
58-bit attempt was refused by memory admission. Admission is not peak VRAM.

#image("scaling-v5.svg", width: 95%)

All ten executions are shown, including the timeout; two higher cells were
unattempted. All 13 Rust tests, host/device/reduction checks, Clippy and
partial-cache boundary checks passed, with zero memcheck/synccheck errors
and zero racecheck hazards. Both cached and uncached service variants passed
preflight. Controller and worker exited zero; pod deletion was verified.

Schema/flow/file verification passes. The five retained protocol-aware
coverage findings reflect its rectangular 24-cell assumption versus the
explicit 12-cell maximum and stopping rule. The native analyzer verifies
the actual ten cells. Raw archive SHA-256:
#text(size: 8pt)[`e5677cb35cc3a34421126fe7d5aad1cac041b1bf8c7a678fd4eca0a3b3196e9b`].
Complete custody and replay outputs are in `results-v5` and its linked S3 archive.

#pagebreak()
= Exact compact tables: retained launch failure

The report graphs use the adjacent `render.rs` presentation source with a
fixed 0.01–10,000-second logarithmic axis, keeping large results clear of the
heading. Original frozen analyzer graphs remain unchanged in each result
directory. No measurements are pooled, fitted, omitted or recomputed.


The next variant stores an exact point key and original tuple ID in 16 bytes,
instead of the full-coordinate row's 32 bytes. For valid points on a fixed
odd-prime curve, x and y parity identify the full point (SEC 1 v2, section
2.3.3: https://www.secg.org/sec1-v2.pdf). The internal key is
`2*(x+1)+(y&1)`, reserving zero for infinity. Under `p < 2^62`, both keys and
prefix endpoints fit u64. This is exact compression, not a fingerprint.
Duplicate keys retain the lowest original tuple ID. Partial caching is off.

V6 source `5a7bfdb97af8941261a2e66a9008231e52f3f23a` ran on pod
`oazm8b2kvq0wvz`, created 2026-10-30T05:27:29 UTC. Full-coordinate, cached and
compact CUDA builds and recovery checks passed, along with 13 Rust tests,
Clippy and full/compact query memcheck, racecheck and synccheck. The first
52-bit baseline solve then failed: the service header still rejected
max_rows above 2^30, whereas the frozen panel requested 2^31. The ladder
aborted before recording its first row. This was an implementation failure,
not a frontier observation. Worker exit was 101, controller exit 1; artifact
collection succeeded and deletion was verified.

Raw archive SHA-256:
#text(size: 8pt)[`01b72b36f186beacc5383891fa6a6c540806480c7515dd7949d6ac1737f73db2`].
The failure record, first input/error, sanitizer outputs and complete raw
member hashes remain in `failed-launch-v6`. The failed protocol is unchanged.

#table(columns: (0.5fr,1.3fr,1.5fr,0.7fr), inset: 4pt,
 [Launch], [Build / preflight], [Ladder boundary], [Recorded rows],
 [v1], [Wrapper failed before compilation], [Never entered], [0],
 [v2], [Service checks passed; chart analyzer compilation failed], [Never entered], [0],
 [v6], [All service variants and query sanitizers passed], [First service header rejected the requested cap], [0],
)
All failed launches are retained; none contributes a successful recovery or
an inferred timing advantage. No task GPU was launched until the preceding
pod's deletion had been verified.

Frozen protocol-v7 authorizes one new execution after raising the explicit
service cap to 2^31, matching the checked u64 planner/device bounds. The cap
itself allocates no memory. GPU admission still charges three actual row
buffers plus other allocations and preserves 25% free memory. New unit
checks cover the upper cap, zero, overflow and chunk bounds. The new wrapper
also tests the actual maximum-cap header on both measured services before
any ladder data. Inputs, targets, arms, ladder, analyzer and resource limits
are unchanged from v6. The cap-fix source is
`4014622e18caef9a6b6c89d79653e7633f5aaaff`; v7 outcomes follow separately.

#pagebreak()
= Compact tables: both 58-bit targets recovered

Protocol-v7 ran source `4014622e18caef9a6b6c89d79653e7633f5aaaff` on pod
`ex8tm7votw4h4g`, created 2026-10-30T05:37:11 UTC. The device was an RTX PRO
6000 Blackwell Server Edition with 97,887 MiB, driver 595.91.07 and CUDA
13.2.51. All 12 prescribed executions ran: ten independently replayed
recoveries and two 60-bit timeouts. Both 58-bit targets completed. The largest
complete rung in this panel is therefore 58 bits under its fixed bounds.

#table(columns: (1fr,1fr,1fr,1fr,1fr), inset: 4pt,
 [Bits], [Seed], [Full row b (s)], [Compact u (s)], [Identical x (s)],
 [52], [101], [37.660576], [29.427842], [29.424833],
 [52], [202], [40.318152], [31.972281], [31.781334],
 [56], [101], [Not prescribed], [174.600283], [Not prescribed],
 [56], [202], [Not prescribed], [190.823997], [Not prescribed],
 [58], [101], [Not prescribed], [489.753544], [Not prescribed],
 [58], [202], [Not prescribed], [530.119796], [Not prescribed],
 [60], [101], [Not prescribed], [1200.005057 timeout], [Not prescribed],
 [60], [202], [Not prescribed], [1200.006503 timeout], [Not prescribed],
)
Recorded cold solve seconds, rounded. Additional fresh CPU verification,
corrupt-scalar rejection and certificate comparisons bring the 58-bit total
audit times to 490.031165 and 530.390733 seconds. All matched 52-bit
certificates are byte-identical. The compact executions have lower observed
52-bit cold times than the full-row baseline; these L0 clocks do not certify
a speedup. No matched larger-field timing comparison was prescribed.

The two 58-bit successes used 1,251/1,232 relations from 1,714/1,877 attempts.
Their oracle times were 487.892647/528.341356 seconds and linear times
0.727965/0.653090 seconds. The 60-bit tables admitted 1,202,274,400 and
1,025,758,188 left rows, with conservative allocation bounds 57,709,730,784
and 49,236,951,816 bytes. Both later reached the 1,200-second solve limit.
Neither was a memory-admission refusal, an exhausted search or a proof that
60-bit recovery is impossible. Allocation bounds are not measured peak VRAM.

#image("scaling-v7.svg", width: 90%)

Every v7 execution, including both timeouts; no higher cells were omitted.
All 13 Rust tests, host/full/cached/compact CUDA checks, Clippy, maximum-cap
service headers and full/compact query memcheck, racecheck and synccheck
passed. Worker and controller exited zero. Archive collection and pod
deletion were verified. Archive SHA-256:
#text(size: 8pt)[`01cd78dbd5cb64410d0c02ba12473964683299f42702a7c5cb46a8f4b7c37891`].
The source, raw rows, exact public inputs, complete archive-member hashes,
replay outputs and deletion receipt are retained in `results-v7`.

Schema, flow and file hashes pass. The canonical protocol checker retains
four rectangular-coverage findings (24 assumed cells versus the frozen 12).
The frozen native analyzer and separate strict frontier audit pass the
actual contract. The strict audit rejects eight coverage/input/status
mutations, including a truncated successful prefix. This does not imply
independent scientific validation or erase the canonical findings.

#pagebreak()
= Wider index: both 60-bit targets recovered

Frozen protocol-v8 ran source `bbf0f2f51e73afd7e41174094feb2c56a72d3d9d`
on pod `royvvmmehidtwt`, created 2026-10-30T06:50:25 UTC. The RTX PRO 6000
Blackwell Server Edition completed all 12 prescribed executions. Every
certificate passed fresh CPU replay and scalar-plus-one rejection; matched
52-bit arms produced identical certificate bytes. The complete frontier of
this fixed panel is 60 field bits, with no failed or omitted executions.

Both arms use exact 16-byte rows. The baseline permits up to 2^16 index
buckets; the candidate permits 2^24, with at most one bucket per 32 left rows.
The largest offsets array is 128 MiB plus eight bytes, included in setup and
memory admission. The CPU driver is identical across arms. It includes
separately merged arithmetic improvements, so cross-v7/v8 comparisons do
not isolate this index change. The wider variant remains explicit opt-in.

#table(columns: (1fr,1fr,1fr,1fr,1fr), inset: 4pt,
 [Bits], [Seed], [Index16 b (s)], [Wide u (s)], [Identical x (s)],
 [52], [101], [29.344188], [17.810696], [17.813335],
 [52], [202], [31.542124], [22.262391], [22.299082],
 [56], [101], [Not prescribed], [99.731624], [Not prescribed],
 [56], [202], [Not prescribed], [107.053471], [Not prescribed],
 [58], [101], [Not prescribed], [252.753463], [Not prescribed],
 [58], [202], [Not prescribed], [273.055707], [Not prescribed],
 [60], [101], [Not prescribed], [770.270416], [Not prescribed],
 [60], [202], [Not prescribed], [845.179937], [Not prescribed],
)
Recorded cold solve seconds; all clocks remain L0 observations. Complete
60-bit audit totals are 770.303987 and 845.214673 seconds. The two solves use
1,471/1,438 relations from 2,208/2,380 attempts. Oracle time is
768.772528/843.662947 seconds; elimination is 1.108359/1.127755 seconds.

#image("scaling-v8.svg", width: 90%)

All 17 Rust tests, host/full/cached/compact device differentials, Clippy,
maximum-cap headers and full/compact/wide query memcheck, racecheck and
synccheck passed. The wide test checks 2^24 offsets against exact CPU lower
bounds, duplicate/parity/infinity cases, and the valid-curve fixtures.
Worker/controller exited zero; archive hash and pod deletion were verified.
Archive SHA-256:
#text(size: 8pt)[`f8924a3418fd268f0f8492f260b8e7fda66616017e1b60fc84488342a98e7d10`].

Schema/flow/file hashes and strict cell audit pass. Four canonical rectangular
coverage findings remain, as for v7; they are not erased by native checks.
The CPU driver currently accepts only p < 2^60, so this result is the largest
verified rung within that implementation, not the GPU's absolute capacity.
A 61/62-bit extension requires a reviewed wider arithmetic/hash path; the
next extension is tracked separately under T-906. There is no matched rho
or fastest-external-solver result, independent scientific promotion, or GPU
implementation of the paper's conditional tensor backend.
