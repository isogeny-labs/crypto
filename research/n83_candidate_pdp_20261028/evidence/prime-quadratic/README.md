# Prime quadratic multiplier exclusion

These native JSON copies are byte-identical to the sealed raw archive. Seven
complete projected degree-six maps are shared across 42 complete source checks.
The pair-covering argument in PRIME_QUADRATIC_DESIGN.md gives conditional FFD >=7.
Independent scientific review and formalization remain pending. No exact FFD,
new LFD, solving degree or elite-base improvement follows.

Hydrate prime-quadratic-archives/runs.tar.gz and extract into ROOT. Verify
manifest.sha256. For each m=3..9, set N83_QUADRATIC_FROZEN_PRODUCTION=1 and run:
`ROOT/instrument template-replay M ROOT/template-limits.json ROOT/templates/mM/native`.
Then run `ROOT/instrument assemble-replay ROOT ROOT/protocol-prime-quadratic.json ROOT/results`
with the same environment flag and verify the manifest again. Use the frozen
300s/8GiB per-template and 120s assembly limits.

PRIME_QUADRATIC_FORMAT.json documents the format. Native journals retain every
coefficient column and elimination row. Published template-mM.json wraps attempt
and verified_result; its certificate is verified_result.certificate. Raw native
result.json stores certificate directly. Empty/new bounds are not inferred from
caps or failures. All control failures and infrastructure attempts are retained
in the separate support bundle.
