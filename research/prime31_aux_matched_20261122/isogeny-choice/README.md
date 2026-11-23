# Prime31 isogeny choices

The extension implements the requested auxiliary-kernel, equation-presentation and target-isogeny comparisons in the existing native collector. The current optimized development suite records **54 passing contracts**, zero failed or ignored, and **230 structurally valid prepared inputs**. **No campaign cell has run.** These are unsigned producer records, claim `PCC-20261123-2689b1`, standing unverified [pcc:unverified].

- [Illustrated report](reports/Prime31_Isogeny_Comparison.pdf), [editable source](reports/REPORT.typ), and [graph data](fixtures/kernel_correctness_fixtures.csv).
- [Draft bounded protocol](PROTOCOL.md) and [ordered 230-cell panel](panel/index.json).
- [Development receipt](DEVELOPMENT_RECEIPT.json), [raw test outputs](evidence/tests), [source manifest](evidence/source-manifest.json), and [checkpoint failure](evidence/checkpoint-gate.txt).
- [Exact upstream snapshots and selection limitations](upstream/README.md).
- [Current execution status](RUN_STATUS.json) and [continuation note](RESUME.md).

## What the comparison changes

The four auxiliary choices are the cyclic baseline on `A+: y²=x³+x`, and three order-1024 subgroups on `A−: y²=x³−x`: `H0=<K>`, `H1=<K+U>` and the noncyclic `H2=<2K,U>`. The three A− chains share nine degree-two quotients and differ in their last degree-two kernel. Complete fibers, kernel poles, monic membership polynomials and terminal images are retained and checked.

On each original or native-isogenous target base, the solver can use ten degree-two maps, five degree-four blocks, or the expanded degree-1024 polynomial. At arity three, the finite intermediate-sum chart has 31, 16 or 4 variables respectively. The exact membership set remains the same across those presentations. The standalone fixture verifier also checks projective map evaluation with one final inversion; the campaign does not benchmark those evaluators separately.

The target's degree-three isogeny supports two distinct constructions. The native arm lifts the unchanged auxiliary x set on the destination curve. The transported arm maps the original point base and complete finite image of the candidate x set, recording poles, image collisions and the signed column permutation. Exact signed relation sets correspond under transport; bounded first-witness execution can still find different rows or hit different caps.

### Fixed-coset arithmetic observations

| Auxiliary kernel | Original E columns | Native E1 columns | Transported E1 columns |
| --- | ---: | ---: | ---: |
| A+ cyclic | 518 | 506 | 518 |
| A− H0 | 508 | 508 | 508 |
| A− H1 | 512 | 519 | 512 |
| A− H2 | 515 | 533 | 515 |

Each row uses one fixed coset `S+H` with 1024 candidate x coordinates. Every transported candidate set has 1024 distinct finite images in these four fixtures. These are exact fixture cardinalities, not relation-yield or timing samples. `P` and `−P` share one logarithm column. Raw constants and assertions are in `fixtures/`, `kernel_fiber_contract.rs` and `target_transport_contract.rs` [pcc:unverified].

## Implementation and correctness scope

The existing production F4, field/group arithmetic, collector and incremental elimination remain the computational backends. The comparison adds whole-kernel construction, quartic constraints, target transport, matched plain controls, a bounded driver and independent choice-event replay. The updated root-domain policy and sibling batching are explicit options; older cells retain their default settings.

The independent auditor reconstructs kernel/coset data and target maps, checks the selected point base, natural-query coefficients, signed relations, coefficient and augmented ranks, useful dependencies, and original/destination scalar replay. It rejects tampering and inconsistent event order. Driver errors and incomplete streams cannot become successful complete records. The current development receipt records 54 contracts executed from copied, prehashed binaries; these are not signed PCC execution proofs [pcc:unverified].

F4 and rho sources are pinned to main revision `33b3bc3c7485df0fe80aae41e2093cf940d87ff3`. Four complete upstream files and 39 unchanged dependencies are hash-bound. That revision's raw `ic_boundary.rs` contains three malformed, unused sibling-pair members. Exact byte ranges omit those members, with zero retained callers and no replacement bodies. The scope is the declared source closure; the complete upstream library is not qualified. All omissions and raw snapshot hashes are disclosed in `upstream/`.

The three rho references are the production counted negation reference, v6 negation walk, and tuned `prime_fast::rho_parallel` with one thread. Their small-field contracts pass. This does not establish a Prime31 solve or rank their costs. Fast-rho failed counters remain missing; upstream table-add accounting is a diagnostic convention. Per-chart F4 `visit` records retain domain/batching counters not repeated in the compact aggregate.

## Preparing and qualifying the source

These commands compile and check fixed contracts; they do not launch the comparison:

```sh
CARGO_BUILD_JOBS=2 cargo test --release --offline --locked \
  --manifest-path research/prime31_aux_matched_20261122/native/Cargo.toml \
  --tests --no-run --message-format=json > /absolute/new/build.jsonl

bash research/prime31_aux_matched_20261122/isogeny-choice/qualify-development.sh \
  /absolute/new/build.jsonl /absolute/new/qualifier-directory
```

The committed panel was generated with the copied release binary's `panel` command and all 230 inputs passed `validate-cell`. These subcommands do not construct fibers, collect natural-query relations or execute rho. Structural validation alone does not freeze or admit an experiment.

To render the document, install the `typst` and `matplotlib` document packages and run `reports/build_report.py`. That script only plots retained fixture values and lays out the PDF. It performs no cryptographic arithmetic or research execution. The delivered four-page PDF was rendered to page images and visually inspected.

## Execution remains open

The panel has 140 auxiliary IC, 60 matched plain IC and 30 rho cells. All five rounds repeat the same public target, coset and seeds. Every cell has a 120-second envelope, 4096 MiB cap and one CPU/thread. The inherited aggregate USD 5 ceiling includes earlier lane spend; unknown remaining headroom blocks launch.

`scripts/checkpoint.sh gate` exited 1 because no uploader credential or AWS profile was available. This session has no authenticated IsoLab submission path, live shared-budget/capacity receipt, canonical protocol freeze or admitted timing environment. The wrapper retains those requirements. No alternative backend or direct binary run was used to bypass them.

The parent study's historical 55 planned cells remain a separate unexecuted panel. The current 230-cell extension does not reclassify them as completed or silently append new spending allowance. The earlier full-root qualification and the unchanged historical benchmark-record gate remain open.

The required next action is admission on the existing authenticated runner, followed by the recorded ordered execution, complete independent replay and cold-cost comparison. See the step-by-step checklist on the last page of the PDF.

## Background and evidence

The auxiliary quotient construction follows [Petit, Kosters and Messeng, PKC 2016, §3.4](https://christophe.petit.web.ulb.be/files/16PKC_primeECDLP.pdf). The explicit Prime31 maps, fibers and counts are local native fixtures rather than measurements drawn from that paper. [PR #4923](https://github.com/isogeny-labs/crypto/pull/4923) remains the publication location.

No canonical performance graph or scoreboard changed. The new quantitative graph shows fixed cardinalities; the two new diagrams explain kernel factorization and target-base construction. Earlier parent-study graphs remain historical and unchanged.
