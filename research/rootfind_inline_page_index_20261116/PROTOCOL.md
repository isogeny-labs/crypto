# Inline-singleton binary pair index: correctness and storage screen

Frozen before implementation and execution on 2026-11-16. Ancestor commit:
`235001fbb834bdd7d10237ebb5410f481f9ccaf1`. This is a bounded
representation screen for the existing torsion-harvest pair stream, not a
new relation construction or a complete-call timing experiment.
[pcc:unverified]

## Sources and reason for this screen

The primary [rootfinding paper](../../papers/rootfinding-decomposition/rootfinding_decomposition.tex)
derives the binary pair table and charges its one-time construction in a
single solve. The current [builder](../../src/cryptanalysis/ic_boundary.rs)
keeps the first pair per sum abscissa, and the previous
[paged-index parity record](../rootfind_paged_curve_parity_20261116/REPORT.md)
establishes exact first-witness and collision-vector equivalence on the
three cells. K13/K19/K23 retained 261/4,800/18,842 keys in
204/4,473/18,498 occupied 16-key pages. Thus 57/327/344 keys are beyond
one per occupied page. An inline directory entry for the first key of each
page, with a spill page only when another slot becomes occupied, may reduce
memory traffic and allocation. Speed is unmeasured. [pcc:unverified]

Shared KB revision `9141e386254340a9bdba3036f2bb706a` was ready but
last refreshed 2026-11-14T21:44:48Z, before the current prototype.
Sources checked (KB): `paper:743d3d3e7e3aab6f980813c0944c1d2cbacc906e075e7283de0abc7b923b7c01`
at `research-pdfs:files/rootfinding_decomposition.pdf` supplies the
pair-table method; `source:e9db8cc2be204429e8fb1daa12a3324c` at
`crypto:papers/ecdlp-index-calculus-program/sections/rootfinding.tex`
describes prior dense-index engineering; `source:91b6bbe9fb73267bbeb5d7c97f493b26`
at `crypto:docs/ecbench/README.md` supplies the complete measurement
boundary. Searches for inline singleton pages and packed witnesses did not
identify an exact current implementation; the local source is authoritative
for this build. No novelty claim follows. [pcc:unverified]

## Fixed design and inputs

- Domain: binary sum keys `1..=2^n`, `n=1..=23`, with pair indices below
  512. A four-byte directory word covers 16 keys. Empty is zero. A singleton
  word stores the four-bit slot and two nine-bit indices, with a nonzero
  witness code. The high bit identifies a spill-page pointer. A spill page
  contains 16 four-byte witness codes. The first witness for every key wins.
- A second distinct slot in one page promotes the singleton to a spill
  page. The original witness must survive exactly. A repeated key returns
  the original pair without promotion or overwrite. Explicit degree, key,
  witness and spill-limit refusals leave existing entries intact.
- Use the same `koblitz_instance(0,n)`, standard-subspace dimensions 5/7/8,
  signed-Frobenius-orbit bases and ordered torsion pair stream as the
  committed [actual-curve parity test](../../tests/rootfind_paged_curve_parity.rs).
  Pin and recompute the full ICV1 IDs and base SHA-256 digests from that test.
- Fixed expected counts: base points 33/139/275; insertion attempts
  272/4,830/18,906; retained keys 261/4,800/18,842; duplicate collision
  rows 11/30/64. Every collision gets a full signed group-equation check.
  Compare the complete collision vector with the production torsion builder
  and every insertion with both a standard hash map and the earlier paged
  prototype. Check all retained keys, boundary/absent keys, and every K13
  key. Record actual singleton/spill pages and backing capacity.
- Synthetic controls fill all 16 slots of one page and force promotion,
  duplicate insertion, index boundary, invalid keys and a zero spill limit.
  The deliberate failures must preserve earlier entries. [pcc:unverified]

## Execution and decision boundary

Commit the design, source and test before executing. Run a dependency-free
unit test and the actual-curve integration test against the exact library
source, with a local Cargo harness if the root package's Linux-only binary
blocks macOS compilation. Retain exact commands, source/binary hashes,
toolchain, host, UTC times, stdout/stderr, exits and resource data where
available. Stop on any mismatch or failed group equation. No timing ratio
is estimated from test-runner wall time. [pcc:unverified]

Passing makes this representation eligible for production integration once
Conductor T-6708 releases `src/cryptanalysis/ic_boundary.rs` and
`src/bin/ecbench/pdp3_koblitz.rs`. It does not rank hash, flat, paged and
inline layouts by complete-call time. The frozen EKS L2 F4/F5/F6 panel in
`../rootfind_f456_batch_combined_20261115/` remains a separate experiment.
The live 2026-11-16 monthly Iso Lab readback was USD 2,094.914 against a
USD 1,300 limit, so this study launches no EKS job. A report, editable
diagram, quantitative graph and PDF must accompany every result, including
a failed one. [pcc:unverified]
