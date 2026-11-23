## 2026-11-23T00:24Z / 2026-11-22 16:24 America/Los_Angeles — Prime31 Dickson quotient iterations (PR #4928)

### What was done

- Preserved the owner-requested summation-polynomial lane in draft PR #4928, branch `lane/prime31-dickson-quotient-gate`; source and evidence live under `research/prime31_summation_compression_20261122/` with the example in the existing native crate.
- Derived and implemented a denominator-free homogeneous trace/norm resultant recurrence, full-fibre lift equivalence, and deterministic oriented sign recovery. Four native evaluated identity certificates have `proof:null`, with separate checked-i128 coefficient expansion and a root replay. Claim `PCC-20261123-5f10b5` remains unverified. [pcc:unverified]
- Development receipts report 5 example tests, 25 original-curve fixtures, 41 unchanged library tests and 12 instrumented cases. The final source SHA-256 is `b76cd5a1efeff77bc065554d89acc9b53be051e6b17811cca589c2dc93a018b1`. At N=64, B=31, the recurrence uses 17,923 field multiplications versus 8,816 for sparse product; setup/extraction/group checks are excluded. Claim `PCC-20261123-acf74d` remains unverified. [pcc:unverified]
- A second mathematical iteration establishes exact bidegree (2N,2N) for the full-curve relation between `D_N(x(P))` and `D_N(x(R-P))` for nonzero odd-order R. `NEXT_PROOF.md` includes the argument and its limited scope; claim `PCC-20261123-f61d26` remains unverified. No Lean check exists. [pcc:unverified]

### Decisions made

- Record the loss to sparse product; do not promote the comparison to a benchmark or full ECDLP result. Degree-sensitive schoolbook cost is O(B^2[1+log^+(N/B)]), quadratic when N/B is bounded. A logarithmic number of recurrence layers does not price coefficient work. [pcc:unverified]
- Continue with a bounded, theorem-first root-intersection question instead of repeating dense evaluation at larger sizes. The scalar degree obstruction does not constrain every auxiliary representation or finite-fibre algorithm.
- Keep the original library and lock byte-identical. The separate development adapter has eight explicitly recorded dependency differences. No canonical leaderboard row or paid run is authorized by these diagnostics.
- Reuse the existing hourly automation and this single PR; reconcile the single uncertain Space creation before any additional Space write.

### Open threads

- Establish a complete common-root isolation bound using succinct circuits or auxiliary states, including all rejected branches and witness extraction; otherwise retain the obstruction and pivot.
- Reproduce with the original lock in CI; obtain formal/signed verification rather than treating source review or native replay as signed standing.
- Implement the prospective target generator and complete per-query controls in `PROTOCOL.md`; require repository admission for EKS measurements. An asymptotic claim additionally needs a growing field family and complete index-calculus accounting.
- Publish the report, PDF, claim records and continuation checkpoint; recover the Space destination and update the existing task prompt after publication.

### Errors

- Exact-original-lock offline Cargo attempt exits 101: missing `either 1.19.0`; retained the stderr and used only the explicitly qualified separate cached-dependency adapter for development checks.
- Conductor, research KB, AWS/EKS execution, and Lean tools are unavailable. The Conductor documentation permits isolated edits when its binary is absent; no benchmark admission is bypassed.
- Repeated Space reads timed out. One create request for `Prime Field Summation Polynomial Research` returned no write receipt; outcome uncertain, no duplicate creation attempted.
- Reconstructing a local Git object from GitHub normalized commit metadata did not reproduce its SHA, so no object or false HEAD was installed. PCC claims record zero producer commit for this partial materialization and pin actual source/evidence hashes separately.


## 2026-11-23T00:32Z / 2026-11-22 16:32 America/Los_Angeles — third iteration and publication gate (PR #4928)

### What was done

- Wrote the exact output-sensitive Dickson-tree node count conditional on an emptiness oracle, the persistent scalar-norm degree calculation, and the dimension-charged empty-block obstruction. The frozen note is `NEXT_ROOT_INTERSECTION.md`; claim `PCC-20261123-87f011` is unsigned and unverified. [pcc:unverified]
- Rendered an eight-page PDF, two diagrams and reproducible rendering source. Every PDF page and both figures were visually inspected; rendering receipts are retained. The PDF covers the first two iterations, with the full third note in Markdown. [pcc:unverified]
- Added a narrow original-lock GitHub correctness workflow for the unchanged native library and new example. It performs tests and deterministic diagnostics, not a timing experiment; no result is claimed before the run completes.
- Read the existing hourly task's newer PR #4924 and its monic quotient/factor-tree work. CI metadata reports run 45086381347 success at head aa27edacb477ddc55bccd23b86d90584ce438cd1; its Run74 note names another checkout. Source-provenance reconciliation remains required. [pcc:unverified]

### Decisions made

- Set sublinear exact empty-root certification, including setup, as the next mathematical gate. The branch-count theorem by itself does not price the oracle.
- Preserve both existing lanes and their control implementations. Update the existing hourly task rather than create another schedule.
- Rebase the unpublished local ledger append onto the latest main ledger with the canonical PCC append routine, preserving all main entries as an exact prefix and retaining the original draft ledger in scratch. No published ledger entry is rewritten.

### Open threads

- Collect the new original-lock CI receipt; retain its actual checkout SHA, source hashes and limitations.
- Follow CONTINUATION.md and STATE.json; preserve the negative comparison and all unsigned claim standings.
- Space recovery remains unconfirmed with no observed IDs or content. Reconcile the single uncertain creation before any additional write.

### Errors

- Browser fallback returned no UI/account state and was interrupted after its stalled read. No browser or agent Space writes occurred.
- The frozen third mathematical note has three prose-linter flags on future or conditional sentences; its opening and outcome labels explicitly remain unverified. The attested note is retained byte-for-byte, and no signed proof is asserted.
