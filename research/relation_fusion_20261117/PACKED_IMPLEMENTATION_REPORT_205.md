# Complete packed constructor: correctness qualification205

The packed polynomial/provenance constructor is implemented and qualified on the six fixed inputs. All36 sparse/DAG ×kernel combinations produce byte-identical complete proofs and equal metadata to the legacy constructor. The actual Rust adapter separately passes36new-backend cases and retains36passing old-backend cases. These are correctness results; no affected200 timing data has been collected. [pcc:unverified]

Private monomials occupy one u128:16seven-bit exponent lanes, with total degree and dimension in separate high bytes. Operations enforce dimension agreement, checked packing and degree≤64 before addition, preventing lane carries. Typed range refusals become explicit errors; unrelated panics propagate and kernel selection restores during unwinding. Input decode and wire export are boundary conversions; intermediate polynomials and provenance retain packed keys through projection, coordinate propagation, change of order, internal checks and proof completion. Exact coefficient arithmetic remains the existing implementation. Unsupported shapes are refused, with no hidden solver oracle or precomputed answer.

Maintained production kernels are shared includes instantiated with the original Vec representation and private packed keys. Every original legacy test module is byte-preserved; vector operations retain their old semantics. This avoids a copied mathematical constructor drifting from the control. The separately maintained checker does not supply construction arithmetic.

|Qualification|Observed result|Scope|
|---|---:|---|
|Default producer release195|68passed|65original +3packed arithmetic/range tests|
|Counter-enabled producer204|69passed|Includes original conditional counter test|
|Separate checker release197|14passed|Whole-proof checking and original mutation canaries|
|Fixed matrix196|36cases|72complete replays; equal old/new proof bytes and metadata;126mutations rejected|
|Actual-root198 / old-path199|36 +36cases|2 +2test functions,7public mutation classes and refusals|
|Saved-root audit203|36proofs|Native/control bases and replay stats bound;2hash-consistent sabotage views rejected|

All tests above have0failures/ignored. Actual-root198/199 use an explicitly opt0 release profile for correctness; build/test durations provide no performance comparison. Saved-artifact203audit does not independently re-execute native operation counts. Full ordinary root unit qualification remains incomplete after the interrupted165run and its46observed failures, retained in185/187. No global-runtime or clean-lint claim is made. [pcc:unverified]

Evidence: PACKED_QUALIFICATION_197.json, ROOT_PACKED_OUTCOME_198.json, ROOT_PACKED_SOURCE_PINS_198.json, ROOT_PACKED_AUDIT_203.json and PACKED_CHECKS_204.json. All six input hashes bind to published169/96 originals; no seed selection. Exact source, raw failures193, full successful logs, both proof matrices and immutable small worker/auditor tools are packaged for replay.

Next is prospective1260-call protocol200:12strong controls including frozen and rebuilt matched forms,6candidate ablations, five repeats, neutral A/A for every control and unchanged full parent boundary. Primary is packed-dag487; allsix simultaneous lower bounds must exceed15, allcalls/proofs must pass and monitor contention must be zero. Source/protocol/tool/input pins are published before data. This is a planned experiment, not an admitted improvement. Original15x goal remains active. [pcc:unverified]

Sources and limitations: hypothesis182 lists primary maintained projection/quotient/compact/replay, separate checker and full profiles179. KB/conductor unavailable as recorded192; no novelty claim. Packing does not measure solving degree or intrinsic regularity; observed solving degree remains null. All new evidence is unsigned producer data under the localCPU0/one-thread/zero-cloud and checkpoint/S3 waiver.
