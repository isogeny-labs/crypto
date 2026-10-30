#set page(paper: "a4", margin: 16mm)
#set text(font: "New Computer Modern", size: 10pt)
#let data = json("construction.json")
#assert(data.status == "PASS" and data.search_performed == false)
= N83 D18 shared-bit encoding
_Construction and semantic validation · frozen protocol dated 2026-10-20_

== Requested scope and result boundary

The user requested completion of PR 2271 and continued D18 shared-bit
encoding optimization. This report covers exact construction counts for
the full retained 16,627-class D18 base at m=2,3,4, Boolean equivalence,
4,478 passing library tests, 29 passing example tests and the implemented
common-public m=4 comparison. Bounded Linux construction and paired solver
execution are pending. The updated-parent validation ran on 2026-10-24 UTC.
The source baseline is `e294f8c927b019f59e6552039d32e6b4ba237f84`.
It is the encoding baseline; a complete verified N83 IC cost is still
an open gate of the broader factor-base study.

#align(center, image("counts.svg", width: 90%))

== Exact transformation

For each bit that agrees across all K representatives, replace K
selector implications by one common-bit unit. At least one selector is
active, so the unit follows from the original implications. Conversely,
that unit satisfies every removed implication. Varying bits retain their
original clauses. The replacement covers shared ones and shared zeros.

Variables, selector order, phase wires and all other CNF/XOR constraints
are preserved. Units are installed after domain construction because the
solver eagerly simplifies later clauses against root assignments. The
default API retains the original encoding; the optimization is explicit.

D18 describes the 18-dimensional F2-linear source space from which the
retained subgroup representatives were selected. The working field stays
F2^83. Shared bits constrain each selected root before Frobenius; all 83
phases and both point signs remain available in the factor-base closure.

For m domains, c common bits and K representatives, the exact removed
clause count is `m*c*(K-1)`. The receipt records actual allocations and checks
that each matches its preflight. This is an encoding-size result.
Runtime improvement and complete IC cost are not measured by this check.

#pagebreak()
== Identity and reproducibility

Curve: `icv1-f2m83-tm6151469093347-debefd74`, a=0, cofactor 4,
subgroup order 2417851639230796216685689, polynomial z^83+z^45+z^2+z+1.
The retained representative CSV is pinned by SHA-256:
#text(size: 8pt, data.representatives_sha256).

Full D object BLAKE3:
#text(size: 8pt, data.object_blake3).
It contains 2,760,082 signed/phase-labeled points. The earlier export and
independent subgroup/label replay remain the point-object evidence; this
construction check reads its selected source-coordinate inventory.

Actual shared bits: #data.shared_bits.len().
Ordered representative BLAKE3:
#text(size: 8pt, data.root_representatives_blake3).

The native tool builds six models with target x fixed to public corpus
index zero. It compares preflight against actual counts and checks that
choice-wire allocation is identical between encodings. It performs no
SAT search. The receipt records SHA-256 of the library module and tool.
Host, commands, exits and validation logs are preserved beside the receipt.
This local arm64 construction is a structural check on a shared host (L0).
Its memory was not capped by a container. The separate 4 GiB Linux
construction gate remains required before the paired card can launch.

== Proof and executable checks

Core Lean 4.34.1 checks three theorems in `lean/SharedRootBits.lean`:
factorization, equivalence within the unchanged context, and the exact
natural-number count identity. The first two have no axioms; the count
theorem uses only `propext`. There is no admitted theorem or native
evaluation. `PROOF_MAP.md` gives each exact statement and the Rust bridge.
The Rust compiler and the full SAT solver are outside that formal proof.
The additional `lean4checker` implementation is unavailable on this host.

The count polynomial also has native certificate #data.count_identity.id,
issued on 32 points and checked on 32 additional points. That fixed-field
certificate has a narrower scope than the Lean natural-number theorem.
Twenty-nine affected-example tests pass, including receipt tampering and
command-card order/failure handling. The implemented library tests cover
exhaustive degree-3 representative/phase/x domains, shared ones and zeros,
atomic caps and invalid domains. The full release-library suite passed with
4,478 passed, zero failed and 119 ignored. Counts, commands and failure receipts are
tracked in `README.md` and the versioned validation archives.
The first example build failed with E0282; an explicit count-tuple type
fixed it before the passing run. Earlier SSD-space and interrupted-build
receipts are preserved in validation v1; the passing gates and compile
failure are in validation v2. The 34-file validation v3 archive retains the
successful updated-parent run and its identical construction receipt. It also
retains an earlier sandbox run's nine local-socket permission failures.
The actual Cargo compiler receipt names Rust 1.98.0 on Apple arm64; the later
interactive shell's Homebrew compiler observation is retained separately.
Validation v4 repeats the full native suites after adding independent
construction-resource checks to the freezer and archive auditor.

== Remaining experiment and original gates

The implemented card compares the same D object under both encodings: five
A/A constructed pairs, five D A/B pairs, one U A/B pair, each with fresh
general-field replay, then eighteen natural m=4 attempts across all eight
public points. It stops before natural work if a control fails. A committed
host card must bind the final source, binary, inputs and wrappers before
launch. Its execution status is recorded separately from this report.

Natural useful rank, m=2/m=3 SAT policies beyond the completed finite events,
complete cold and reused cost,
linear algebra, matched same-point rho, fastest-external comparison, and
best-base selection remain open. Capped queries retain their censored
status. Candidate and selected-best-runtime fields remain null.

Canonical runtime dashboards and benchmark rows are unchanged: this
result records exact clause counts, without a time measurement or pipeline
ranking. The study graph is generated directly from `construction.json`.
Sources: `PROTOCOL.md`, `PROOF_MAP.md`, `construction.json`, `README.md`,
and `problems/n83-prebuilt-base-quality-m2-m4.md`.
