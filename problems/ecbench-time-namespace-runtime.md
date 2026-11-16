# Ecbench clock controls require unavailable Linux time-namespace interfaces

- **Status:** open
- **Kind:** software-validation runtime capability
- **Last updated:**2026-11-05

On pinned bbf/d30 source, two diagnostic_reap_publication_regression_tests fail
at DiagnosticSessionClock::begin before their sidecar assertions. Same-binary
focused reproduction exits101 (2passed/2failed). `/proc/self/ns/time` and
`/proc/self/timens_offsets` return ENOENT; boot_id is readable. The runner blob
d9f17458168bfaa615802394740034435224b281 exactly equals main d30.
[Receipt and exact focused capture](../research/prime_ecdlp_three_leads_20261029/continuation-20261030/validation/main-d30b9e0db-20261105/library-final-20261105T1134Z/receipt.json).
These are unsigned producer observations. [pcc:unverified]

Linux time_namespaces(7) documents time namespace support and proc interfaces
(<https://man7.org/linux/man-pages/man7/time_namespaces.7.html>). No kernel
configuration or precise absence cause has been established here. The actual
unavailable interfaces are sufficient to explain the observed initialization
errors. No sparse fixture restoration, fabricated proc data, assertion weakening,
ignore or gate waiver is appropriate.

Next: qualify a runtime exposing the required interfaces, or have the clock
owner specify and test a fail-closed unsupported-runtime diagnostic contract.
Then run the original sidecar controls and full release library on final source.
The two failures and6168-case cap remain recorded; no execution clock admission
or scientific panel is authorized by this problem.
