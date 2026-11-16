# Frozen native K4096 phase diagnostic

This is one bounded observability run for the already retained `public_x_hash`,
seed `2026100801`, 4096 signed Frobenius-orbit base. Its purpose is to locate the
last completed native chained-S3 phase and independently replay the exact public
relation query. It cannot select a factor-base runtime winner.

The compressed base is the 20,621,466-byte S3 object with SHA-256
`ccc1196d75de406b1daba634b8011f08308c37bd1e674758f9efeabb0ff0bcad`
and BLAKE3 `a84572e6ca52efa7867082261bf0543723d52c0d136061a45c970970716aa779`.
Its original custody is
`s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/v2-size-frontier/a0/objects/a84572e6ca52efa7867082261bf0543723d52c0d136061a45c970970716aa779.jsonl.gz`.
The already verified setec copy has the same bytes. The runner reads a verified
copy at `s3://ephemeral-runs-104186811093-us-east-1/n83-frozen-inputs/ccc1196d75de406b1daba634b8011f08308c37bd1e674758f9efeabb0ff0bcad.jsonl.gz`.
The Git-tracked manifest/replay/upload receipt and fixture sidecars are copied
from `verification/k4096-orbit-model-20261018/evidence/`; their SHA-256 values
must match `verification/cloud-matched-k2048-k4096-20261018/input/fresh-inputs.json`.

Freeze the pushed clean source commit and binary SHA-256 in the run receipt.
Use `primary-orbit-chain-cold` with public fixture 0, `m=5`, one trial, seed
`2026100901` for the relation sampler, `max_variables=150000`,
`max_domain_clauses=2000000`, `max_models=32`, and conflict budget `1000`.
Set `N83_CHAIN_PHASE_TRACE=1`. The native x86 worker runs inside Docker with
one CPU, 4 GiB hard memory, zero swap, and no container network. Its internal
wall cap is 600 seconds; the outer cap is 620 seconds. Run on the supported
ephemeral runner in `us-east-1` using the smallest fitting `c7i.xlarge`
(8 GiB host RAM) with a two-hour TTL, then let the runner terminate.

The gate is the committed small-field positive trace and its independent query
check, followed by the exact source and input hash checks. The native audit
must validate one flushed query against the public fixture by separate group
arithmetic and check any completed model counts against the prior K4096
capacity receipt: 679,936 points, 4,096 orbits, 339,968 x coordinates,
1,773,110 domain clauses, and, if installed, 131,958 variables, 2,021,197
clauses, 4,980 XORs. Preserve the raw stream and exit status even for a wall
cap, process failure, or an incomplete phase. Such outcomes remain censored;
they are not zero relation yield or a SAT refutation. `selected_best_total_runtime`
remains null absent a fully charged, replayed end-to-end comparison.
