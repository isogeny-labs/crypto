# Verification receipts

All native proof checks ran in the fresh-main worktree at repository base
2741765c9ddc724c6cda7a1bb38b9f900a2e8d31. No cryptographic experiment or
CAS benchmark ran. Pinned dependencies came from the existing source-built
Mathlib checkout; source revisions and all new source bytes were checked.

## Completed commands

The exact published generic project first ran, from its own directory:

```sh
env PATH=/workspace/prime-degree-lean/bin:$PATH LEAN_NUM_THREADS=4 /workspace/prime-degree-lean/bin/lake --no-cache build
env PATH=/workspace/prime-degree-lean/bin:$PATH LEAN_NUM_THREADS=4 /workspace/prime-degree-lean/bin/lake env lean --run /workspace/secant-fixture-proof-20261031/tools/lean/AuditAxioms.lean -- SecantIdentity SecantGroupLaw SplitFactorBase
```

Both exited0. Each of the three source modules was also run through
`lake env lean -DwarningAsError=true <module>.lean`, all exit0. The retained
logs are `published-group-build.log`, `published-group-axioms.log` and
`published-group-warnings.log`. The full generic census reports22 theorems,
zero failing and zero trusted. All prior group SHA256SUMS entries pass.

The new project then ran:

```sh
env PATH=/workspace/prime-degree-lean/bin:$PATH LEAN_NUM_THREADS=4 bash verify.sh verification/final
```

It exited0 with `final/status.json` reporting stage complete. The script
records the exact stages: thirty input hashes; full `lake --no-cache build`;
new source with warnings-as-errors; unchanged repository axiom auditor
covering all fourteen source modules. The census is159 declarations,
zero failing and zero trusted. The auditor includes generated declarations.
Only propext, Classical.choice and Quot.sound appear transitively.

The existing old identity module is present in both inherited packages;
those source bytes are identical. Lake resolves and builds the dependency
graph successfully without source copying or changing either package.
`dependencies.json` records actual native revision matches for all nine
locked Git dependencies; all repository-local source is hash bound.

## Preserved failures and source custody

Build attempts01–07 failed; attempt08 and final verification pass. Failed
source snapshots and complete logs are in `development-records.tar.gz`,
together with recovered prior group-development logs. Final accepted source
is separately hash-bound; failed declarations containing Lean's diagnostic
sorryAx are not accepted proof modules. `recovered-prior-build03.log` is
now the actual recovered original on-disk log, rather than the previous
continuation's labeled tool-output excerpt.

The original pre-outage group-source hashes match the published recovered
copies on recovery. A fresh published-byte build and full axiom audit now
close the earlier gap; the sealed historical receipt is not rewritten.

The metadata helper initially failed spawning git with EPERM; direct native
shell reads supplied the revisions and the metadata comparison passed.
No source or scientific calculation was changed to accommodate this.

## Rendering and inspection

Typst0.15.0 from `/tmp/n83-report-venv` compiled `reports/report.typ` into
a two-page A4 PDF. `pdfinfo` reports two pages and no suspects. `pdftoppm`
rendered both pages at100dpi; both were visually inspected with the image
viewer. Source links, curve/base IDs, theorem name and diagram labels are
legible with no clipping. The PDF source and SVG are retained. This is a
new report, not an old PDF reused as current.

`lean4checker` is not installed. No successful cache-download preflight is
claimed. The full unchanged auditor was run directly; no gate policy,
workflow or trust manifest was modified. CI status is recorded on the PR.
