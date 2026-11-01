#set page(paper: "a4", margin: (x: 18mm, y: 17mm), numbering: "1")
#set text(font: "DejaVu Sans", size: 9.4pt)
#set par(leading: 0.55em)
#set heading(numbering: none)
#show link: underline
= Complete degree and closure comparison
Fixed p37 seed0 diagnostic · 2026-10-25

Terminal exit 0 at 07:20:28 UTC. All 24 producers, twelve first audits,
twelve A/A parities, the native reducer and final source/input/execution
seals pass. Exactly the registered systems 0,1,2 from each phi/doubling
arm are retained. Source/input freeze c30a3dc6068b precedes the managed
04:08:23 UTC launch. No collector, oracle or target solve was repeated.

== Every native result
#set text(size: 8.6pt)
#table(
  columns: (1fr, 1fr, 0.6fr, 1fr, 1.6fr, 1.2fr), inset: 4pt,
  [Arm / index], [Method], [Power], [Working / product degree], [Search Fp37 multiplications], [Sparse-check multiplications],
  [phi / 0], [raw16], [2], [12 / 12], [33,609,631], [15,211],
  [phi / 0], [closure8], [2], [7 / 12], [916,980], [15,983],
  [phi / 1], [raw16], [none], [16 / —], [142,494,025], [0],
  [phi / 1], [closure8], [none], [8 / —], [569,173], [0],
  [phi / 2], [raw16], [1], [12 / 12], [33,569,618], [14,675],
  [phi / 2], [closure8], [1], [7 / 12], [926,804], [16,686],
  [double / 0], [raw16], [1], [12 / 12], [33,603,044], [14,614],
  [double / 0], [closure8], [1], [7 / 12], [949,579], [16,857],
  [double / 1], [raw16], [none], [16 / —], [142,494,025], [0],
  [double / 1], [closure8], [none], [8 / —], [569,173], [0],
  [double / 2], [raw16], [1], [12 / 12], [33,716,337], [15,142],
  [double / 2], [closure8], [1], [7 / 12], [956,093], [17,595],
)
#set text(size: 9.4pt)

All rows are verified-first-aa. Inversion exponentiation products are
included in multiplication counts. The native table retains additions,
inversion calls and every input/candidate/reference/audit/parity hash.
Zero check-count means no certificate was produced; it is not a cost-free
rejection. All six raw16 results exactly reproduce prior raw degrees 4..8,
including ranks and counters. That requirement does not apply to closure8.

== Working degree and original degree are different
Systems 0 and 2 produce independently checked certificates in both arms
and both methods. Closure finds them at working degree 7, but their actual
original-product degree is 12. Sparse verification is slightly more costly
for its different multipliers and remains counted. No degree-7 original
certificate or equality of certificate bytes is claimed.

Both index-1 systems find no certificate for powers 1,2,3 within the bounds.
Raw16 raises the original Macaulay bound through 16. Closure8 caps working
degree at 8 and tracks original products through 16. These are different
bounded searches; closure failure does not exclude every original-degree-16
representation, and neither failure proves nonmembership at every degree
or power. An explicit q-nonzero common-root witness or separate higher-power
control is a future discriminator, not a result here.

#pagebreak()
= Counts and attribution

#figure(image("figures/full-panel.svg", width: 100%), caption: [
  All twelve configurations; literal recorded native counts. Both methods
  pass their first/audit/A/A/parity checks. Negative labels retain the full
  registered search bounds; they do not mean universal nonexistence.
])

The construction-count reduction occurs for generic doubling as well as
phi. This diagnostic does not establish a twist-specific construction
advantage. Within each arm/index, both methods consume identical equations;
the two maps produce different residual systems. The six systems are from
one synthetic p37 seed0 fixture and are not independent secret samples.

The old phi system-0 F4 reference records 125,327 row-reduction
multiplications for finding all roots. Even closure's 916,980 search
products exceed that partial count. Finding all roots and constructing a
rejection certificate are different tasks. No actual fallback/filter is
executed here. These counts do not price root extraction, the remaining
F4 work, construction, memory or the full pipeline.

The atlas already distinguishes raw Macaulay spans from degree-fall
closure in another setting. No novelty, global atlas saturation,
fastest-tool or asymptotic claim follows. Shared-host clocks remain
unadmitted diagnostics. All complete-price fields are null.

#pagebreak()
= Verification, scope and reproduction

The producer receives equation-only input and peeled abscissa. Prior roots,
physical outcomes and old costs belong to the auditor. Successful
certificates undergo independent sparse expansion against the original
equations and checks of retained roots. The generic conditional rejection
theorem is separately checked. The concrete phi-0 integer-lift proof has
two retained memory failures and a distinct v3 attempt; its outcome is not
inferred here. Full parser, field/group/decoder bridges remain open.

Native v2 gate passes five tests, including the prelaunch cap-versus-actual-
degree regression. V1's four-test source is preserved. Launcher controls
refuse nested outer busy (64) and an existing phase command (65) without
science output. The old v1 queued control was quiesced before admission.
No terminal or interrupted producer is silently retried.

Each phase keeps one worker, common reservation, 8 GiB, 1,200-second outer
and 1,180-second cooperative producer limits. Queue wait is separate.
The original overflowing layout, corrected layout, native gates and partial
historical progress snapshot remain unchanged in the versioned archive.

#table(
  columns: (1.4fr, 2.6fr), inset: 5pt,
  [Requirement], [Actual status],
  [Twelve-configuration fixed comparison], [Complete, all first/audit/A/A/parity and terminal seals pass.],
  [Successful finite certificates], [Sparse identity and retained-root checks pass.],
  [Arbitrary-degree/power nonmembership for failures], [Not established by bounded searches.],
  [New unique relations, matrix rank or recovery], [Not collected by this diagnostic.],
  [Complete cost attributable to twist], [Open. Counted arithmetic excludes other work, memory, I/O, calibration and matched strongest IC/rho/external construction controls.],
  [Original 189 + 21], [Complete and not repeated; no consistent twist-specific full cold gain.],
  [85 target / 42 certificate panels], [Original managed owners, inputs and full workloads retained.],
)

The separate all-42 finite-residual census passes both processes and exact
parity, retaining physical sets and independent ranks. Its 300 phi versus
150 doubling eligible systems are a counterfactual skip count; no filter
is deployed. It is not combined with this construction diagnostic to invent
a measured pipeline result. Multisets require their own replay and scope.

Study: zero_fiber_collector/degree_closure_probe/. See PROTOCOL.md,
FREEZE.json, native sources, run-panel.sh, run-phase.sh and workload-v1.
Raw final table: parent runs/degree-closure-probe-panel-v1/analysis/
summary.json. Compact final receipts are under results/degree-closure-probe-v1/final/.
Extract the complete versioned raw archive under the parent collector;
README.md gives checksum and reproduction commands. The prior reference
corpus remains a prerequisite.

Workload SHA-256:
#text(size: 7pt)[254581dfebfcb992e8ce0c6b7896cbd7cb402259b0a5b72e34fd8c711cc0c88a]
Runner SHA-256:
#text(size: 7pt)[d6e4f2f6d562f0a67bd5735123d63a6d744c9e685939f4c73d9c427ad0c7a92b]
The prelaunch control figure/PDF remain historical. Every actual final
PDF page is inspected; unlisted benchmark copies preserve its bytes and
the SVG/150 dpi PNG. Canonical IC/rho graphs, leaderboard and benchmark
index are unchanged because full-price, reference and VALIDATION gates
remain unmet.
