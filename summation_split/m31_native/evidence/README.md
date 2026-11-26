# Native development evidence

These are producer-local, unsigned development receipts. They are not a
checkpointed scientific execution, signed PCC verdict, performance comparison,
or the required 50 complete projected Gröbner solves per configuration.

## Final qualification

`QUALIFICATION.json` indexes the final immutable source replay, input preparation,
and bounded one-case quotient-F4 observation. The `qualification-02/` manifests
bind the source and copied executable bytes before execution and again after it.
The final source was copied before building; the live crate was compared to that
snapshot after testing. Compiler/test elapsed text is retained as raw logging,
not used as a solver benchmark.

The prepared archive contains 400 input files per witness mode, with four arms,
two arities and 50 fixtures in every cell. The monolithic files occur in both
modes, so 800 files does not mean 800 distinct mathematical systems. Every
configuration has the same four-point section over M31. `PREPARED` establishes
input generation only; no batch solver has been called. Source and manifests
are retained separately from the archive for review.

Geometric decomposed inputs now declare chart variables first, then leaf
squares and `t,u`, exactly as the quotient adapter. The manifest records the
actual variable order. An msolve `-e` block-elimination solve still has different
order semantics from a single grevlex basis and must be reported separately.

## Retained earlier records

- `f4-verifier-step-limit-initial.txt`: the first independent verifier exhausted
  a 32-step cap that had been treated as a matrix-batch cap.
- `f4-verifier-reduction-limit-second.txt`: the next verifier exhausted its
  20,000-reduction cap. Neither failure establishes an incorrect basis.
- `f4-m3-ordinary-verification-success.txt`: the same producer completed and
  the ordinary verifier finished with a separately declared larger budget.
- `qualification-01/`: an earlier immutable replay with 27 library and two CLI
  tests passing. Its source archive is preserved. Review then found that the
  tests permuted eight mixed planes per arity while the preparer did not, and
  that geometric exports used a different variable order from the quotient
  adapter. Those prepared manifests are superseded and must not be used for
  the matched comparison. No batch solve used them. Its source hash list was
  made after compilation but before test execution; it is not a pre-build pin.
- `root-library/`: both broader repository compilation attempts were capped
  before tests ran. Those incomplete gates are separate from this component.

All coupled F4 replays use one unique fixture, `fixture_plane(3,0)` with the
geometric sequential arm. Repeating it during qualification does not increase
the number of distinct completed Semaev instances. The 50 one-chart F4 controls
are algebra unit tests and are not members of the requested Semaev solve panel.

## Reproduction

From the repository root, the final crate is dependency-free:

```sh
cargo test --release --manifest-path summation_split/m31_native/Cargo.toml -- --nocapture --test-threads=1
cargo build --release --manifest-path summation_split/m31_native/Cargo.toml
```

Use the preparation commands in `../README.md` in new output directories.
Preserve source, binary and input hashes and the full command/exit record.
The existing shared execution requirements apply to the scientific comparison.

[pcc:unverified]
