# Two-primary sign projection

See [DERIVATION.md](DERIVATION.md) for the multiplicative projection, minimal
sign-preserving image, reduced monomial factors and validity/addition boundaries.
The native primitive is [lib.rs](lib.rs); source/compiler/commands/binary hash
are in [receipt.json](receipt.json). The five-method development suite passes.
Claims PCC-20261107-b8a1b7 (execution) and PCC-20261107-874911 (derivation)
remain unsigned producer assertions. [pcc:unverified]

[report.pdf](report.pdf) and its editable Typst/SVG sources summarize the result.
The rendered page was visually inspected. No curve recovery, benchmark or cloud
job was run. This primitive is not integrated into the source cursor.
Prime modulus is a caller precondition; composite inputs are unsupported.
A sign candidate requires an external sign-domain promise or full-value check.

Replay locally as a development unit test:

```sh
rustc --edition=2021 -O -C overflow-checks=yes --test lib.rs -o /tmp/sign-projection-tests
/tmp/sign-projection-tests --test-threads=1
```

Next construction: q=2/q=4 projected monomial cursor against the full-value source,
retaining validity checks and accounting for projection and residual work.
