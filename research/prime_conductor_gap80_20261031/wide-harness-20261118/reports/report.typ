#set page(paper: "a4", margin: (x: 15mm, y: 14mm), footer: context align(right)[#counter(page).display()])
#set text(size: 10pt, font: "DejaVu Sans")
#set par(leading: 0.5em)
#set heading(numbering: none)
= Exact wide-field ecbench inputs
Implementation and public controls · 18 November 2026 · PR #4723

*Publication pending:* trusted S3 migration received HeadObject 403; a public
read at the correct regional endpoint returned AllAccessDisabled. The archive
was not uploaded or replayed, so this PR remains draft. See publication/status.json.

The new prime_mont4_explicit path carries large prime-field coordinates through
the native ecbench harness. It reuses the repository's Mont〈4〉 arithmetic and
CountedGroup ledger. Subgroup orders remain u64; field coordinates and supplied
ambient orders are exact values of at most 256 bits.

#image("flow.svg", width: 100%)
*Implementation flow only.* These arrows are input/solver routes, not
isogenies, conductor strata or an evaluated 80-bit crossing.

== Representation and checks
Montgomery residues represent xR and yR modulo p, with R=2²⁵⁶. The adapter
stores two four-word residues and an explicit infinity flag. Inputs require
canonical decimal strings and lower-case hexadecimal point coordinates.
Nonsingularity, Hasse eligibility, membership, prime r and [r]G=O are checked.
External prime-field and ambient-order certificates remain required.

The current adapter restricts a,b to nonzero values. Special j=0/1728 models
need their own automorphism accounting. Public target hashing and wide-field
index calculus are explicitly unavailable. The two frozen source fields fit
the representation; their original source evidence remains separately pinned.

== Solver and cost contract
Existing rho.negation uses the same generic counted algorithm and retains
its complete candidate check. Its 64-bit key is a walk hash. The distinct
bsgs.prime_mont4_v1 stores complete point keys in a deterministic B-tree,
avoiding a u64 hash as point equality. The earlier word-key BSGS route refuses
the new instance type.

Group additions/doublings and semantic field multiplication, squaring and
inversion counts are recorded. An inversion represents its native Fermat
exponentiation without also counting it as curve-formula multiplications.
B-tree inserts/lookups and stored point-key bytes are recorded; node overhead,
allocation and exact comparisons remain unpriced. BSGS final verification is
performed by the runner outside its solve totals. No fully priced comparison
or runtime advantage is asserted.

#pagebreak()
= Qualification and original obligations
#image("vectors.svg", width: 100%)
Each source oracle input produces 32 shared-law scalar products. These are
correctness vectors with no timing uncertainty interval. They do not evaluate
the target isogeny. PCC-20261118-224ceb is an unsigned producer attestation,
verdict not_covered and standing unverified. [pcc:unverified]

== Current qualification status
The main library compiles. The full release library test build failed on a
pre-existing duplicate test name in ic_framework/plugins.rs (E0428), alongside
initial omitted fixture includes. Those include subjects were restored and the
exact failure retained. That module was not changed. [pcc:unverified]

The corrected-source focused release target passes all five listed tests:
source arithmetic against 64 PARI vectors and explicit BigUint affine formulas,
tiny known-answer BSGS, rho/BSGS dispatch/replay and canonical guards. Root
library clippy passes with warnings denied. Its separate output directory and
manifests preserve the earlier source/executables. This is not a passing full
legacy test suite. [pcc:unverified]

== Original requirement status
#table(columns: (2fr, 3fr), inset: 4pt, stroke: 0.4pt,
  [*Requested deliverable*], [*Status*],
  [Prime conductor gap at least 80 bits], [Frozen candidate retained; full quotient/images and endpoint evidence open],
  [Exact wide-field inputs], [Checked on degree-2/5 source, destination and coordinate models],
  [Dozens of points in several fields], [32 distinct transported scalars per field; no paired wide-field timings],
  [Scalar and ECDLP measurements], [Canonical scalar timing, pairing/seeds, strongest reference and admitted execution open],
  [Index-calculus comparison], [Wide-field adapter unavailable],
  [Cloud / S3], [No target job; source/image identity and live admission still required],
)

The focused lock retains every parent dependency version and adds only its
test package. An initial offline lock-generation drift is preserved separately.
Official Rust 1.100.0 components were matched to the dated stable manifest.
No source-oracle or test ceiling is reported as elapsed runtime.

*Sources:* ecc/mont_vt.rs, ic_boundary.rs, ecbench generic/workload/method
contracts and docs/ecbench/README.md; PARI ellmul/ellisoncurve/isprime reference;
Rust BTreeMap and Cargo target documentation. Full links, exact source pins,
failed attempts and reproducible commands accompany this report. KB wrapper
unavailable, exit 127. No new formula, speed claim or world record is asserted.

#pagebreak()
= Actual small-degree transfers
#image("transfers.svg", width: 100%)
Both edges are elliptic-curve group morphisms over their respective prime
fields, dimension one. They do not connect different fields or assign any
conductor strata. The original 80-bit target kernel remains unevaluated.

The degree-2 control on the frozen 202-bit source has quotient
y²=x³−2380x+44688 and map x'=x+448/(x+14),
y'=y·(1−448/(x+14)²), with kernel generated by (−14,0).
The degree-5 control uses the unchanged complete native polynomial evaluator
over the frozen 96-bit pilot field and a separately obtained order-5 kernel.
It does not use the pilot's 47-bit kernel. Both conductor-gap fields are null.

The two controls check 70 native/PARI input records: generators, 64 targets,
two kernel points and two infinities. Every transported generator is nonzero
of prime order r; each kernel degree is coprime to r. Coordinate controls use
u=2 with explicit inverse checks. Their supplied ambient orders are inherited
by the rational isogenies and were not recounted. [pcc:unverified]

#image("paired-rows.svg", width: 90%)
The original CLI source and native GMP comparator match 192 point/scalar
rows. All workloads are unregistered. A five-round alternating-order warning
is retained; the successful plan uses six rounds and one warmup. Endpoint
order and algorithm-seed pairing remain unresolved. Off-curve generators,
tampered coordinates and unsupported IC reject. [pcc:unverified]

*Cost table:* construction, conversion, transfer, destination solve, recovery,
verification, peak RSS and uncertainty are all unmeasured. No zero, amortized
time or extrapolation is supplied. Companion transfer assessments separate
supported bounded map/subgroup checks from unknown advantage and scaling.

PCC-20261118-a0f9ac and -f0f516 bind the map controls. Final
PCC-20261118-f8faf8 binds corrected-source tests and input comparisons.
Their standing is unverified, verdict not_covered. Raw failures, version-1
source overlay, corrected sources, pre-call hashes and complete outputs are
preserved. No measured session or record promotion is asserted.
