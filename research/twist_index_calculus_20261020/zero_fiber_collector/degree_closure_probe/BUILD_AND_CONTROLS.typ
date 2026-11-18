#set page(paper: "a4", margin: (x: 17mm, y: 16mm))
#set text(font: "DejaVu Sans", size: 9pt)
#set par(justify: false, leading: 0.55em)
#set heading(numbering: none)
= Raw degree versus degree-fall closure
*Native control checkpoint, 25 October 2026. Comparison cases pending.*

The fixed diagnostic supplements the full mixed-component extension-field
study. It uses p37 seed0, phi and doubling, the first three stored polynomial
systems per arm. Both raw16 and closure8 run on all six systems: twelve
configurations, twenty-four producers, twelve first audits and twelve A/A
parities. These are public synthetic systems, not independent secrets.

#image("figures/flow.svg", width: 100%)

== Observed implementation controls
Native gate v2 passes five Rust tests with zero failures. Format, tests,
release build and final source-seal checks exit 0.

#table(columns: (2.1fr, 1fr), inset: 5pt,
  [Control], [Observed outcome],
  [Degree fall: working degree 2; original product degree 3], [Certificate expanded exactly],
  [Nonradical system and altered multiplier], [Power 2; tampering rejected],
  [Common nonrepeated root], [No bounded certificate],
  [Insufficient original product cap], [Sparse verifier rejects],
  [Raw cap 4 versus actual product degree 1], [Distinct fields, exact identity],
)

Executable SHA-256:\
#text(size: 7pt)[d6e4f2f6d562f0a67bd5735123d63a6d744c9e685939f4c73d9c427ad0c7a92b]

Gate parent: `6b16155b7ae747d2f00f74857b3d5b032f5a58d3`. The gate retains
the then-uncommitted source snapshot. Commit the source/input freeze before
any comparison execution. Input preparation and launch controls retain
separate receipts; no production result is implied here.

Prelaunch review caught a raw-cap/actual-degree reporting error. V2 derives
the actual product degree from multipliers and checks it separately. V1's
four-test build/report remain preserved. Only its queued infrastructure
control was quiesced before admission; it is not a passing launch gate.
No comparison case or original science service was stopped or repeated.

== Exact boundary
raw16 reproduces the previous raw Macaulay degrees 4–8, then extends through
16. The auditor requires exact rank and arithmetic-counter equality for
the retained prefix. closure8 reduces at intermediate degrees 4–8 and
multiplies newly found lower-degree rows, tracking every representation
over the original equations. Its original-product cap is 16. A certificate
at working degree 8 may require a larger original-product degree; report
both. Count representation-cap refusals. Failure at either bound proves
no nonexistence statement.

#pagebreak()
= Verification, cost and remaining scope
Both methods search powers 1,2,3 of the repeated-factor polynomial
$q_a=a^3-a^2 e_1+a e_2-e_3$. The sufficient identity is
$q_a^k=sum_(i=0)^3 h_i f_i$. Sparse coefficient expansion over the actual
prime field checks every returned identity independently of the dense
construction. The later auditor checks the sealed original roots and
requires no accepted physical relation in a certified system. The producer
receives no prior roots, relation outcomes or costs.

The existing conditional rejection proof and cubic IDC
`IDC1h10810f17e0f6c8f6` are reused. The native parser, field, polynomial and
group/decoder bridges remain open. No new general theorem is claimed.

== Accounting and host
Count search and sparse-check modular multiplications, additions and
inversion calls, including failed degrees. Inversion exponentiation
products are already counted as multiplications. RSS, memory operations
and I/O remain unpriced. No fallback, relation collection or target recovery
runs in this diagnostic. Complete end-to-end cost remains null.

Linux x86-64; Intel Xeon 6975P-C; 32 logical CPUs; Rust 1.99.1
(`58cf8f9c3`, 2026-10-13). One build worker, separate Cargo target. Science
phases share the busy reservation, one worker, 8 GiB address-space cap,
1,200-second outer watchdog and 1,180-second cooperative producer deadline,
starting after the queue wait. Shared-host clocks are diagnostic only.

== Evidence and original requirements
Source-linked report: `BUILD_AND_CONTROLS.md`; prospective specification:
`PROTOCOL.md`; native source: `native/src/probe.rs` and `algebra.rs`.
Raw gate: `../runs/degree-closure-probe-native-gate-v2/metadata/`.
Exact public fixture SHA-256:\
#text(size: 7pt)[5a55715c3d8f532634722ff6f58562e8f7e9f7d339f529cda724677da0f9b393]

The historical 189+21 comparison is complete and is not repeated. The
full recovery and certificate panels continue under their existing owners.
Unique physical relations, factor-log rank and complete cold cost remain
the original comparison. These polynomial ranks are a different quantity.
The new diagnostic has not yet run its twelve comparison configurations.
No full-price, calibrated GAE, matched strongest IC/rho, external-tool,
scale or breakthrough result is established by these controls.

Prior-art match is partial: the atlas lead
`LEAD-ICE-descent-closure-degree-at-scale` and `KR-IC-fbdb61` distinguish
raw spans from closure in binary descent. Their numerical bounds are not
transferred to this extension-field restriction. No novelty or atlas
saturation is asserted. Canonical scoreboards and benchmark index receive
no row; any copied control PDF and figures stay explicitly unlisted.
