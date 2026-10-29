# Lean proof gate audit — 2026-10-21

**Scope.** This is a validation and CI repair for the `crypto` repository at main
`db1be50b0d4d3f9123ac1ebe9cc68538331e7bd9`. It checks the pinned SEA
proof project and the repository proof gate. It does not certify every
historical experiment in `crypto`, `cryptanalysis`, or `crypto-autoresearcher`.
No scientific trial or performance measurement was run.

## Finding and correction

The [Lean proof gate run 39459394168](https://github.com/aburan28/crypto/actions/runs/39459394168)
at `892ef408e98f190ef19c0ab5fee8030cb5d91c6d` had 37 successful jobs,
two failed jobs, and one cancelled job. These are **job outcomes**, not counts
of verified theorems or experiments.

* `research/sea_point_count_20261016/lean` built `SeaProofs.lean` successfully,
  including its `#print axioms` output, then failed its separate axiom audit.
  Lean 4.19 parsed the gate's `--trusted` program argument as a Lean CLI option
  and rejected it. The [archived audit log](evidence/ci-sea-audit-failure.log)
  shows the parser error. This was a gate implementation failure, not a
  counterexample to a theorem.
* `research/n83_indexed5_load75_20261013/DenseCapacity.lean` failed while
  downloading Lean 4.34.1 because of a TLS connection error. The
  [archived build log](evidence/ci-dense-build-failure.log) supports an
  infrastructure classification; that CI run made no theorem verdict. A
  subsequent local run of the exact repository gate passed all five theorems
  on pinned Lean 4.34.1, with no axioms or trusted declarations.
* The cancelled `research/ic_tree_split_structured_v_20261011/lean` job failed
  during Git checkout (`curl 56` connection reset and `index-pack` failure),
  before installing Lean or checking a theorem. The [checkout log](evidence/ci-tree-checkout-cancelled.log)
  records that infrastructure event. A later local run of the exact gate on
  pinned Lean 4.34.1 passed 24 declarations with one manifest-registered
  `native_decide` proof. That proof uses compiler trust; the gate reports it
  separately from kernel-checked declarations.

The gate now passes its existing trust manifest through the
`LEAN_AUDIT_TRUSTED_FILE` environment variable, leaving module names as the
only program arguments. `AuditAxioms.lean` still accepts the old explicit
`--trusted` form for callers that can pass it. Its whitespace normalization
uses `String.join` and `Char.toString`, which compile under Lean 4.19 and
4.34.1. The workflow installs each pinned toolchain before the proof step and
retries a failed download at most twice. A build or axiom failure is never
retried or converted to success. [The diagram](diagram.svg) shows the gate
boundary and both observed failure modes.

## Validation and proof boundary

| Check | Observed result | Meaning and limit |
|:--|:--|:--|
| `lean tools/lean/AuditAxioms.lean` | Compiled on pinned Lean 4.19.0 and 4.34.1 | Auditor source is accepted by both versions; this alone does not audit a theorem. |
| `tools/lean/proof-gate.sh --out ... research/sea_point_count_20261016/lean` | **PASS**, 20 theorem declarations, 0 failed, 0 compiler-trusted | The [build log](evidence/local-sea-build.log) and [axiom log](evidence/local-sea-audit.log) are retained. The theorem count includes generated `GroupOps` declarations. |
| `tools/lean/proof-gate.sh --out ... research/n83_indexed5_load75_20261013/DenseCapacity.lean` | **PASS**, 5 theorem declarations, 0 failed, 0 compiler-trusted | The [build log](evidence/local-dense-build.log), [axiom log](evidence/local-dense-audit.log), and [gate summary](evidence/local-dense-gate.log) are retained. These are arithmetic bounds, not a benchmark replay. |
| `tools/lean/proof-gate.sh --out ... research/ic_tree_split_structured_v_20261011/lean` | **PASS**, 24 theorem declarations, 0 failed, **1 compiler-trusted** | The [build log](evidence/local-tree-build.log), [axiom log](evidence/local-tree-audit.log), [gate summary](evidence/local-tree-gate.log), and [status](evidence/local-tree-status.txt) are retained. `TreeSplit.Replay.replay_ok` uses manifest-registered `native_decide`, so this is not a wholly kernel-only proof set. |
| `ELAN_TOOLCHAIN=leanprover/lean4:v4.34.1 tools/lean/selftest/run.sh` | Seven of seven expected controls passed | The [self-test log](evidence/selftest-v434.log) records plain proofs and the reviewed `native_decide` fixture passing; custom axiom, `sorry`, unlisted native proof, empty audit, and build-error fixtures fail for their expected reasons. |
| Lean 4.19 negative/positive controls | Custom axiom rejected; registered native proof reported `1 trusted` | [Negative](evidence/v419-negative.stdout) and [trusted](evidence/v419-trusted.stdout) logs show the forbidden-axiom check and trust manifest were retained. |
| `cargo test --release --lib` | **PASS:** 4,464 passed, 0 failed, 119 ignored | This validates the library test suite in this checkout, not historical experiment outputs. The [compressed raw log](evidence/cargo-test-release-lib.log.gz) is retained. |

The checked SEA declarations include `SeaProofs.bsgs_complete`, which proves
search completeness for an explicitly supplied additive group, and
`SeaProofs.direct_equals_full_intersection`, which proves a predicate
identity. Their exact hypotheses are in
[`SeaProofs.lean`](../../research/sea_point_count_20261016/lean/SeaProofs.lean).
The audit found only standard Lean foundations (`propext` and `Quot.sound`)
for the SEA declarations. It does **not** prove the Rust point-counting
implementation conforms to those abstract predicates, nor a new SEA speed
or ECDLP claim. The older CI run's successful jobs are not a fresh build of
the current main tree; the new PR's full proof matrix must still pass.

The Rust test ran in the repository's minimal sparse profile with the SEA
fixture and local `isogeny_algos` dependency included. Earlier attempts in
incomplete sparse profiles failed during compilation because those tracked
inputs were missing; their [fixture](evidence/cargo-sparse-fixtures.log) and
[dependency](evidence/cargo-sparse-dependency.log) logs are retained. The
successful raw log has SHA-256
`de7281263c07499c30f671e7e0821010f60a16f918095b18c2ff6a37ebf95271`;
its committed gzip has SHA-256
`117b4cc687ba06834fbac29e66f45e971885395e55742b995941da6799c91985`.

## Requested scope and graph status

| User requirement | Status at this change |
|:--|:--|
| Retrofit all prior experiments across three repositories with faithful Lean proofs | **Partial.** This repair makes one existing proof project's axiom gate usable again. A source-to-claim review and missing lemmas remain for the rest of the corpus. |
| Validate code and archived numerical results | **Partial.** The gate implementation, SEA model, DenseCapacity arithmetic bounds, tree-split Lean target, and Rust library suite were tested; historical numerical results were not replayed here. |
| Refresh graphs and performance dashboards | **Not attempted in this change.** There was no new performance measurement or corrected performance datum. No canonical performance graph should move on the basis of this CI repair. |
| All results good to go | **Not established.** The full crypto proof matrix and cross-repository scientific claims still require their own evidence and semantic review. The companion autoresearcher workspace was built and its 62 present targets machine-verified separately; semantic review remains pending. |

The only new visual is the gate diagram. It depicts observed CI states and
the validated local gate transition; it is not a performance graph. The
[PDF](report.pdf) contains this report's findings and the same boundary.

## Reproduction

```sh
elan run leanprover/lean4:v4.19.0 lean tools/lean/AuditAxioms.lean
elan run leanprover/lean4:v4.34.1 lean tools/lean/AuditAxioms.lean
tools/lean/proof-gate.sh --out /tmp/crypto-sea-gate research/sea_point_count_20261016/lean
tools/lean/proof-gate.sh --out /tmp/crypto-dense-gate research/n83_indexed5_load75_20261013/DenseCapacity.lean
tools/lean/proof-gate.sh --out /tmp/crypto-tree-gate research/ic_tree_split_structured_v_20261011/lean
ELAN_TOOLCHAIN=leanprover/lean4:v4.34.1 tools/lean/selftest/run.sh
```

The audited `SeaProofs.lean` SHA-256 is
`0c18e419b033bb577d9afea5c1cc18b77f28c427ed1fbed2afa89dcb9ecf6daf`.
The audited `DenseCapacity.lean` SHA-256 is
`dc848680be90d5f173f7b5e4a88c243826f3badd95c735333d212e01cb44fe18`.
The audited `TreeSplit/Replay.lean` SHA-256 is
`d2393d666481f24276b9a3cb0a5f2f324bdcf4320c442a53207514292e55f054`.
The repaired auditor and gate script SHA-256 values at this report revision
are `7bbe3fdd3adff99f13d0bc17c9e26b242fe93a08698a37d4c164ab08a0cda1eb`
and `b5f0a43c6e35ad736ef2c9933f5c303d509883dd81cfb467d55b5a1214f125cc`.
