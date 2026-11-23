# Separate-role replay of the fixed-d6 identity record

Reviewed 2026-11-23 UTC (2026-11-22 Pacific) by `map_cost_audit`, a separate agent in the producing team on the same host. Standing: unsigned source review and evaluation replay. This is neither a Lean proof nor a signed execution certificate. [pcc:unverified]

## Exact object and statement

Read the complete `coupled/native/certificate/main.rs`, its manifest, producer build and pre-call bindings, the certificate metadata, and the unchanged native checker's statement validation, expression evaluation, digest checking and extra-point path.

The input is the already-frozen record `IDC1h099b8aa03eee75f6`, at `../certificate-attempt1/cusp-d6.identity.json`, SHA-256 `6939fba6d17836eddee3f83a85a55627e09e0e0509ac558ff742e6d3f253fc20`. Its `proof` is null. It evaluates the fixed d=6 univariate recurrence identity over the field of modulus 2305843009213693951 = 2^61-1. This is a different field from the Prime31 coefficient-rank fixtures.

With alpha=(1-T)^2, beta=-2*T*(1+T), gamma=T^2, the code constructs U_0=2, U_1=-beta and U_j=-beta*U_(j-1)-alpha*gamma*U_(j-2). It compares alpha^6+gamma^6-U_6 with (T-1)^12+T^12-2*T^6*sum_(j=0)^6 binom(12,2j)*T^j. Inspection found the constructed recurrence and signs consistent with the independently reviewed manuscript formula. Empty expression products at exponent zero correctly represent one.

The executable first requires exact equality between the certificate statement and this fixed expected statement. The reused checker then validates the syntactic degree bound, recomputes the statement identifier, replays the original points and digest, and evaluates a domain-separated stream using the supplied checker seed.

## Fresh-seed replay

After the certificate was fixed, this reviewing agent drew an eight-byte seed from the operating system random source. The retained unsigned decimal seed is:

`17665607814493179865`

The invocation used the immutable producer-built executable in check mode, with a 60-second timeout and a 2 GiB virtual-address-space ceiling (`ulimit -v 2097152`). It returned exit zero and accepted the 32 recorded certificate points plus 32 new checker-seeded points. Both expression syntactic degrees were 12. `proof` remained null. All five source/dependency/manifest hashes, the executable hash and the certificate hash matched before and after the invocation. No source, producer evidence, or certificate was modified. [pcc:unverified]

The checker seed is independent of the producer's chosen seeds in the role/provenance sense. The point sequence is a deterministic expansion of that seed; this review does not assert 32 statistically independent uniform samples or adopt the checker's advertised numerical false-accept bound.

## Limits

This checks the named d=6 expression over one finite field. It does not establish the identity over the integers, for every d or every field, the generic coefficient-rank theorem, the d=1302 fixed-curve rank, or a solver advantage. Those claims require their own mathematical arguments and evidence.

The producer's issue-mode output reports rejection of an added-one false statement and a changed digest. This reviewer inspected that control code but did not rerun issue mode or claim those controls as fresh replay outcomes; check mode emits null for those fields. The source-to-binary association is inherited from the producer build manifest, with fresh hash checks and execution, not an independent rebuild. This is same-host, same-team validation by a separate role and remains PCC-unverified.

## Bindings and receipts

- Certificate driver source SHA-256: `83eb7414aedc7bf4f9b970dd5750236ee3af04d116e30e481865c15b30fcb923`.
- Immutable executable SHA-256: `23d45f31d9e6e06f225bc774812b5700ce05e5ac6efb8ef089b884ec0ecbd682`.
- Original identity checker SHA-256: `d81ee915ad769a87d7aac5d9664631239607f4a0deb27850cdb52cea84e34270`.
- Original SHA-256 module SHA-256: `2bafe271f1b2f6e6eb1a100f672f81e2b16bbe30b6db29ac579c0740dfe9eddf`.
- `certificate.receipt.json`: machine-readable outcome and source binding.
- `certificate.pre-call.txt`, `certificate.pre-call.sha256`, `certificate.post-call.sha256`: separately retained pre-call intent and unchanged bindings.
- `certificate.stdout.jsonl`, `certificate.stderr.log`, `certificate.exit-status`: raw outcome.
- `certificate.checker-seed.txt`: exact decimal seed, retained without floating-point conversion.
- `certificate.source-binding-check.txt`, `certificate.source-binding-check.exit-status`, `certificate.source-stability.exit-status`: source-to-manifest and stability observations.
