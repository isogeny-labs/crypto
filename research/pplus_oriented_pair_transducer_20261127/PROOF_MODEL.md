# Evidence and proof model

## What is public

There is no secret seed or randomized target selection in this study. The
curve, Dickson depths, square-root branch order, signed leaves, planted-target
rule, eight scalar targets, infinity control, cost panels, and promotion
inequalities are fixed in the protocol and amendment 1. Re-running the same
source produces the same mathematical rows; elapsed time and peak memory are
diagnostics only.

The custody chain is:

```text
frozen protocol + amendments
        -> source commit + SHA-256 manifest
        -> checkpoint-gated one-shot runner
        -> candidate transcript
        -> independent verifier transcript
        -> frozen Rust analyzer verdict
        -> evidence SHA256SUMS
```

## Claim-to-evidence map

| Claim | Evidence |
| --- | --- |
| Dickson fibre and signed factor base | Both Rust paths independently replay every tower equation and curve lift. |
| Exact pair support and multiplicity | Candidate quadratic unions are compared with independent direct enumeration at every node pair and level. |
| State/circuit totals | Independent direct states are hash-consed separately; actual/unique states, entries, and union edges are compared. |
| Exact `GF(2)` ranks | Independent sorted-column elimination compares every level, leaf rank, and maximum rank. |
| Hits and recovery | Candidate discloses a canonical four-leaf witness; verifier recomputes the target and replays the group law. |
| Miss completeness | Both paths scan the complete exact pair support for the target complement. |
| Fixed targets | Verifier reconstructs the planted target and `[k]G`, `k=1..8`, instead of trusting transcript coordinates. |
| Growth verdict | Source-bound analyzer applies exact integer cross-products and the frozen slope rule. |
| Cold m=7 projection | Candidate emits the measured depth-six miss vector and no-cross-target projection; analyzer applies literal integer limits. |
| Source/output integrity | Git commit identities and SHA-256 manifests bind source and evidence bytes. |

## Why this is not called a zero-knowledge proof

Nothing in the experiment needs to remain secret. A zero-knowledge proof of a
single decomposition witness would establish neither complete misses nor the
full state ranks, circuit sizes, target-selection rule, or operation-count
semantics. Conversely, hashes and replay do not prove which host executed a
binary. The correct label is **public deterministic replay evidence**.

A future succinct proof-of-execution lane would need a separately specified
arithmetic circuit or trace VM covering tower generation, every group
addition, state interning, rank elimination, complete complement scans, and
the analyzer, plus commitments to the exact source/build and public outputs.
That is a distinct engineering project and is not silently claimed here.

## Residual limitations

- Candidate and verifier share the public curve constants and mathematical
  specification, so a specification error can affect both. Amendment 1 and the
  archived registry/manifest identity address the discovered curve mismatch.
- Field-operation counters describe the instrumented source path. They are not
  an instruction-level host attestation, and additions receive no invented
  field-equivalent price.
- The experiment tests one exact semantic transducer. Failure does not prove a
  lower bound against every implicit resultant, separator coordinate, or
  pair-spectrum data structure. [pcc:unverified]
