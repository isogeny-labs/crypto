# Amendment 27: price rho's native tower arithmetic

After native source checks and clean source/spec publication, run one fresh
public weak-family fixture p101, seed317. Two separate workers use the
same copied rho binary, public curve/points/order and seed: existing
harness prices, then opt-in native tower prices. Both use negation,
batch16, r256 and the existing bounded DP policy. The generator and held-out
answer stay outside workers; private answer fields, including nested ones,
are rejected and CurveInstance.d=0. [pcc:unverified]

The new calibration uses the native arithmetic's actual source with a
const-generic counted instantiation. Production uses COUNT=false, with
the same integer arithmetic and no new per-product runtime hooks. Deferred
products/sums are charged as prime-field algebra under the existing
bitop/v1 convention; these are model weights, not physical circuit or
instruction measurements. Existing inv_mod emits its inversion once.
Legacy harness vectors and TUNED multiplication credits remain separate.
The selected source-derived calibration criteria are55M/44A for native
multiplication and squaring, and80M/42A/1I for inversion; native hand and
differential controls must pass before the trial. [pcc:unverified]

Require both actual scalars to pass native verification, additional elliptic
replay and held-out agreement. Require exact semantic walk/high-level
operation trace equality, correct native calibration vectors, independently
reconstructed precompute/walk/collision counts, phase sums and existing-unit
repricing. Explicit verification counts are reconstructed separately from
the additional elliptic replay: its residual primitive vector must match
between both modes. Never add the whole process hook snapshot to rho_tally.
All known bookkeeping, RNG, tuned-reference and complete-price gaps remain
explicit. This is a reference arithmetic-coverage correction, without an
algorithm speed improvement, IC comparison, parity claim or repricing of
any earlier recorded experiment. [pcc:unverified]

The user switched the pending measurement to Runpod before any32data.
Use one dedicated task container with4vCPU/8GiB, no GPU, at most0.40USD/hour
and2700seconds lifetime, then terminate it after collection. Pin the first
CPU allowed by the observed affinity and record it before the call.
The source-bound Rust binaries are copied unchanged into a pinned official
Rust container; the Rust capture service gets a separate ephemeral token,
never the Runpod API credential. Both credentials stay outside source and
scientific artifacts. [pcc:unverified]

One validation round, zero warmups,8GiB/one claimed thread,
60seconds fixture/900perworker/1900whole, no retries. A failed/censored call
is retained without an unchanged rerun, larger budget or relaxed criterion.
Keep exact raw/summary/export/flow/argv/status/progress, copied immutable
worker/generator/qualifier/wrapper/source, host facts and source/precall/
preworker/post hashes. Render and inspect every final PDF page and retain
exact PDF/SVG/150dpiPNG browsing copies. Large raws are collected exact and
hash-bound; the native export explicitly claims no unavailable canonical
schema conformance. Complete totals and comparative/rho eligibility remain
null/false. [pcc:unverified]

Sources checked (KB): wrapper status/exact/mismatch rho-pricing queries
unavailable, validation/rho-native-kb-{status,search1,search2}.log. Read the
actual native tower, harness calibration, explicit rho_tally and
docs/units/BITOPS.md. This applies the existing convention to the actual
source; no new cost model, optimized-tower implementation or novelty claim.
[pcc:unverified]

The selected batch16 policy is one configured reference. This trial does
not select a cheapest parameter portfolio or establish strongest-reference
pricing; those remain prerequisites of a later primary comparison.
[pcc:unverified]
