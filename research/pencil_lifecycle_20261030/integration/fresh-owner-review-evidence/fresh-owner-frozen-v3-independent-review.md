# Independent source review: fresh owner native V3 and wiring V2

Read-only review; no source changes, builds, cloud calls or scientific execution. Reviewed exact native/package patch f1f53433860cadd9e412e7cadc30b368065c48bde0f678d3e04c6830d38f1788 and incremental campaign-V2 wiring patch 890bf747a00d03af14508dda25bea72f5296a09c41fa50ff0f831e84738d40bf. Separate schema proposal 6d15a2cac085587c5d68354a7a7db18f436d26c1c8a0fc3164b9cc8a93932ea3 remains subject to root reservation authority; this review does not authorize its application. All proposed files match source-manifest-v2.json. Packaging V3 publisher correction is separately recorded; original three-field image receipt preserved.

## Review observations

The earlier cutoff mismatch is corrected: campaign reservation.deadline_unix_seconds caps requested fresh-owner stop before intended manifest persistence. Wrapper consumes this exact env and controller independently reconstructs expected config using its immutable stored manifest stop. Exact request/spec/launch/binary/image/Job/Pod identities and argv match actual wrapper commands. New option absent retains legacy request serialization/direct invocation. Source provenance and trusted same-UID instrumentation remain necessary; hashes do not establish root-private authenticity.

Wrapper now launches native owner asynchronously, forwards TERM/INT and repeats wait after interruption. Native owner owns a separate child process group and preserves terminal status. Distinct fixtures now exercise fake child self-signal, direct native owner TERM and literal wrapper forwarding sections. Original nonzero child exit is preserved on artifact hash/upload failure, with the secondary collection error reported. A successful child with failed collection remains failure90/91.

Failed/succeeded collection precedes result interpretation. Expiry retains existing Job, obtains genuine Pod identities with matching Job ownerReference, then invokes bounded evidence-only collection. Calls require185 seconds remaining at the frozen absolute campaign collection deadline. Missing bytes/custody and ambiguous downloads remain findings; no replay or verifier is launched. Four exact owner config/receipt files are retained with SHA-addressed filenames. Collector explicitly does not authenticate receipt semantics or accept costs.

Native and controller consumers deliberately keep accepted_external_closure and total_active_worker_ns null. Controller compares expected config and retained receipt/checker/build/journal identities. This is not the full native journal semantic validator. No unsupported promotion to full cost, allocation release or mathematical correctness was found in the reviewed scope.

## Validation evidence inspected

Read existing logs only: native test-v7 has4unit+2integration passes; native clippy-v7 completed. Controller-wiring-test-v7 has39passes (nested1test subprocess reports are subsets); controller-clippy-v6 completed. Earlier failed controller compile logs remain retained. No independent rerun performed. These software observations do not prove real Pod mounting, IAM permissions, image packaging, full wrapper AWS upload integration, infrastructure teardown or worker allocation measurement.

## Verdict and limits

No remaining concrete defect found within the reviewed fresh observation/wiring slice after the listed corrections. This is a scoped source review, not runtime admission. Computation supervisor absolute reservation enforcement remains a separate unresolved gate (delayed-start and blocking command paths); fresh setup, hashing, publication and cleanup are outside the native child polling interval and cannot establish allocation release. Full journal closure acceptance, actual image/package qualification and campaign admission remain separate. Budget remains25200seconds and absolute expiry2026-10-30T22:08:40Z; this review does not renew either.
