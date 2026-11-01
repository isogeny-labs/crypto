# Separate A1/A2 relation replay

The local arm64 native verifier `examples/n83_selector_receipt_replay.rs` had
SHA-256 `c69042dd9e23ea8f3b47eef9fd3e24a28f180a64ea0d251c08c548073d27fb0f`.
It read the retained compressed object with SHA-256
`2457a977ea9c05a11cc60878bdb3b29f6d905dc8db9aa26e52913a702d308369`
and BLAKE3 `1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d`.
The object is retained at
`s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/v2-size-frontier/a0/objects/1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d.jsonl.gz`.
[A1 input hashes](spot-replay/a1-inputs.sha256) and [A2 input hashes](spot-replay/a2-inputs.sha256)
bind the original local binary, object and respective worker receipt. Replacing the
temporary object path with any byte-identical download gives the same replay.

The positive invocation for each arm was:

```sh
VERIFIER 65392fb874584a0a5845515f0f8de0510b9c702c selector-only OBJECT_JSONL_GZ \
  spot-arms/ARM/worker.json spot-replay/ARM.json
```

Here `ARM` is `a1-selector-only-v2` or `a2-selector-only-v2`, with output
name `a1` or `a2` respectively. The recorded invocation used paths expanded
from the repository root; the relative paths above are from this study
directory. Each output has
`PASS_RELATION_REPLAY`, the same four point indices and row digest, and
`solver_model_bits_replayed: false`.

For each arm, one copied worker receipt set `x_decimal[0]` to `"0"`, and
another set `relation.dense_decimal_blake3` to 64 zeroes. They were produced
with `jq`, passed through the same verifier, and both exited 1 without an
output JSON. The exact mutated receipts, stdout, stderr and exit codes are
under [spot-replay](spot-replay/) and bound by [the raw manifest](spot-raw-files.sha256).
The verifier repeats original-field, point/group and row checks but shares
the repository library's arithmetic and does not reproduce the SAT search.
There is no B relation receipt to replay.
