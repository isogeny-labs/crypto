# Adapted two-large-prime relation and graph diagnostic

[Report](report.pdf) · [Complete Rust summary](summary.json) · [Flow](flow.jsonl)
· [Frozen protocol](../AMENDMENT_23_ADAPTED_TWO_LP.md) · [Custody](local-evidence.json)
· [Library API](../../../src/cryptanalysis/jv_sieve_two_large_prime.rs)

On the same 300,000 distinct B values and 148,948 ordered lines, the
full-smooth reference produces 29 independent equations over the 375-point
small base. The adapted two-large-prime finder plus graph produces 114,
with all 114 output certificates replayed. The observed rank increase is
85 equations. The full factor base has 500 points. [pcc:unverified]

| Observed component | Full-smooth small base | Adapted two-LP + graph |
|---|---:|---:|
| Jacobian-verified original relations |29|229|
| Zero / one / two large primes |29 / 0 / 0|29 / 93 / 107|
| Complete rows / certificate replays |29 / 29|114 / 114|
| Exact modular rank |29|114|
| Output nonzeros |261|2183|
| Observed primitive subtotal, bitops |1729950841332|1743212955422|
| Complete operation cost |unknown|unknown|

The candidate graph has 116 large-prime vertices, peels 31 relations,
uses 84 pivots/168 merges, closes 85 nontrivial cycles and records no fill
refusal. Every original source is verified in the Jacobian; the copied Rust
qualifier independently checks coefficient/source binding, modular witness
replay, exact rank, phase repricing, sampler counts and process flow. These
are producer checks without an independent signed group verdict.
[pcc:unverified]

Both arms use the same reduced factor base, incidence/shared recipe and
separate enumeration/extraction RNGs. Their ordered cohort digest is
f5c0015aebd3ab5482ff104ddb5eafef003fab91450b94ece745b1f49dc60300.
The boundary includes cold cover/full-base/table setup, source finding and
verification, duplicate removal, graph filtering and certificate replay.
Public instance generation and the qualifier are outside this component.
[pcc:unverified]

Source d2c2e7f947ce1ea549c74532424207e7c8c8879d was published before the sole
call. Exit 0 in 46.497 seconds, two starts/two completions, one validation
round/zero warmups, CPU0/8GiB/one claimed thread, 900 seconds per arm/1900
whole and no retries. All 145 source hashes and pre/post receipts pass.
The complete original 12,146-byte summary and exact 17,548-byte smooth raw
are in Git. The original 112,999-byte two-LP raw exceeds 64KiB and remains
exact with its hash under /workspace/ic-local-runs/local-cover-two-lp-component-20261116-28,
alongside immutable binaries/qualifier/wrapper/source and host facts.
[pcc:unverified]

Native/container and generator work remain incomplete. Known selected
sampler integer prices and explicit unpriced events are separate from the
primitive subtotal. No speed/cost ratio, A/A, interval, ranking, original-DLP
recovery or rho comparison belongs to this protocol. Rank 114 is still
short of a complete small-base relation basis. Verified descent/target
integration and actual scalar recovery are the next pipeline obligations.
[pcc:unverified]

The single final PDF page was visually inspected. Exact PDF/SVG/150dpiPNG
benchmark copies are hash-checked and original copies remain local. No
scientific, qualifier or report failure occurred, and no retry or budget
change was made. No successful S3 checkpoint upload, canonical encrypted
schema, strongest-external or admitted-ranking assertion. [pcc:unverified]
