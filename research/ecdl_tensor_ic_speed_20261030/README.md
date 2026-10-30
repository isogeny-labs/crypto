# Speed of the tensor-contraction index calculus (2026-10-30)

**Status: instrument ready; the measurement has not run yet.** The L2
wall-time session runs on Iso Lab EKS once the `ic.tensor_contraction_v1`
arm is on `main` (the `isolab-experiment` workflow builds `main` commits
only). No number in this directory is a result yet.

## Question

What does the toy tensor-contraction index calculus of
[`crates/ecdl-tensor-ic`](../../crates/ecdl-tensor-ic/README.md) (Rust port
of the Go `ecdltensor` prototype) cost in counted operations S and in L2
wall time on small prime-order curves, against the matched `rho.negation`
reference and the existing prime-curve `ic.pipeline` baseline, and how does
that cost scale with r? The owner chose the full ecbench route, wall time
included (2026-10-30).

This is not a speedup lead. Every counter in the crate is exponential, and
the [Ope26] contraction algorithm is not implemented: one relation costs
about |F_1||F_2| ≈ 2r point additions and about 2(2r)^(1/2) relations are
needed, so S is expected to grow roughly like r (predictions P1, P2 in the
protocol).

## Instrument

- Arm `ic.tensor_contraction_v1` (`src/cryptanalysis/ecbench/methods.rs`,
  `solve_tensor_contraction`), charge documented in
  [`docs/ecbench/README.md`](../../docs/ecbench/README.md) §7. It charges every
  non-trivial point addition and doubling the crate performs, from its
  per-phase tally (`crates/ecdl-tensor-ic/src/opcount.rs`). Contraction rows,
  the factor-base listing and elimination multiplications are counted
  `_uncharged` for the §5a band.
- Specs:
  - [`tensor-ic-vs-rho.json`](../../docs/ecbench/specs/tensor-ic-vs-rho.json):
    the registered roster curves `icv1-fp10-t5-192cb216`,
    `icv1-fp12-t19-d4a315df` and `icv1-fp14-tm43-158b6914`; 8 targets each,
    6 rounds, L2 required.
  - [`tensor-ic-vs-rho-16.json`](../../docs/ecbench/specs/tensor-ic-vs-rho-16.json):
    `icv1-fp16-t351-d433f24e`, counted S only, 1 round. One 16-bit tensor
    solve took 417 s in a descriptive sizing run.
- Arms in both specs:
  - `rho-neg` (reference)
  - `ic-mitm` (`ic.pipeline`, `prime-abscissa:size=32` with
    `mitm:negation_folded=1`; baseline)
  - `tensor-m2` (candidate, m = 2)
  - `rho-neg-aa` (A/A control)
- Preregistration: [`protocol.json`](protocol.json) (`experiment.protocol/v1`,
  validated with `crypto-autoresearcher/tools/experiment_protocol.py`).

## Development checks so far (not results)

A local smoke session (L0, not committed) ran `rho-neg`, `ic-mitm` and
`tensor-m2` on the 10- and 12-bit roster curves with 2 targets each:

- all 24 rows verified;
- `ecbench verify --replay-all` reproduced all 12 measured runs, 4 per arm,
  every one `identical`.

Its S values are recorded in the protocol's provenance as data seen after
the predictions were written. They are not reported here as results.

## Next

1. Merge the arm.
2. An `experiment-director` card freezes the EKS dispatch of both specs
   from the merged commit, and `executor-mechanical` runs it.
3. Audit with `--replay-all`, then run `compare` for `rho-neg` against
   `tensor-m2`, `ic-mitm` against `tensor-m2`, and the A/A pair.
4. Build the per-curve table with the §5a band, then the report, graph and
   PDF.
5. Add the `benchmarks/` row with `VALIDATION.json`.
