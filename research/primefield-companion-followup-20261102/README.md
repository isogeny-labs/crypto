# Companion inverse constructions and frozen binary validation

This follow-up preserves the work after PR #3266, from standalone source commit
`f208412`. The earlier `primefield-companion-schur-20261101` snapshot is unchanged.
The source elliptic-curve scope remains prime fields, with the original method
exclusions. No numerical experiment is launched by this publication.

## Construction and evidence

| Component | Delivered scope |
|---|---|
| Recursive inverse | Two-query leading generators of width s+2; explicit recursive inverse preparation and reusable width-s inverse generators. Derived and syntax checked; numerical validation pending. |
| Pivot transport | Same-field row swap with two transpose queries and width s+2; first-column pivot and right-null certificate primitives. Derived and syntax checked; numerical validation pending. |
| Binary trace codec | Canonical streaming fingerprint, independent decoder and eight software tests. Serialization cost has not been benchmarked. |
| Binary validation | Frozen full 1,698-row / 34,248-product contract, 24 source locks, two input locks, all seeds, exact launch command and single-owner launcher. Thirteen software tests; zero numerical trials. |
| Original fused run | Terminal exit 124 at its original deadline. All 877 completed rows retained: complete 849-context candidate arm and 28 paired A/A contexts. Full-panel acceptance remains pending. |
| Schur/lift integration | Full 642-context checker preparation and locks; zero numerical trials. |

The recursive inverse replaces supplied solves with explicit preparation, under
the stated nonzero recursive-pivot condition. The pivot primitive is not an
automatic balanced-tree fallback; repeated scalar pivots can increase width.
See the original derivations and their Bostan et al. 2017 Section 6.2 reference.

The original fused raw archive contains 431,414,955 bytes, SHA-256
`db199b944109851a17bbd77b3e0d547310d62b89bcaf538804288d06357ce4a2`.
The complete candidate arm records 17,124 products. All 28 completed A/A payload
pairs match; 821 planned rows lack completion receipts. These are the saved
custody results, not a fresh numerical replay. The original invocation must not
be restarted or its cap extended.

## Restore and inspect

From the repository root:

```sh
python3 tools/artifacts/artifacts.py pull research/primefield-companion-followup-20261102
python3 research/primefield-companion-followup-20261102/verify_publication.py
```

Native adjacent artifact manifests retain the exact original paths, hashes and
byte counts. `publication-manifest.json` binds all 110 imported files to their
standalone source revision. The verifier checks every imported file and all
three included source-lock sets without invoking a numerical kernel.

The two archived numerical inputs remain in the predecessor publication. Restore
them using that snapshot's README and `verify_bundle.py --hydrate`; their exact
hashes are retained in this snapshot's input locks. This follow-up is an overlay:
for a separate replay workspace, first restore the predecessor, then overlay the
imported follow-up files. Do not overwrite an existing immutable evidence tree.
All other historical path and host assumptions remain visible in the source.

The planned experiment command targets the original standalone workspace.
`launch.py` requires exact local source/input hashes, a clean committed source,
and a receipt binding every source to an actual immutable GitHub revision.
Publication does not itself create that receipt or authorize a rerun of old
invocations. A fresh invocation retains one CPU, 8 GiB per-process address space,
4 GiB output, a shared 3,600-second deadline and one attempt with no retries.

## Reports and validation

- [Recursive inverse derivation](companion-recursive-inverse/DERIVATION.md)
- [Pivot transport derivation](companion-pivot-transport/DERIVATION.md)
- [Codec design](companion-trace-codec/DESIGN.md)
- [Capped fused report](companion-fused-validation/reports/capped/report.pdf)
- [Frozen validation contract](companion-fused-binary-validation/CONTRACT.md)

The copied tree passes all 21 software tests (13 instrument/admission/resource
tests and eight codec tests). The preparation AST check confirms unchanged
numerical stages, verification body before fingerprinting, and main schedule
after the declared guard/output-routing substitutions. These checks do not
execute the research kernels or establish runtime improvement. Repository-wide
checks and artifact publication status are recorded separately as they finish.
