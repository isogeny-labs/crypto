# Reproduce the formal degree proof

Start with [FORMALIZATION.md](FORMALIZATION.md). Headline files:
[DegreeBridge.lean](DegreeBridge.lean), canonical highest forms, and
[CanonicalFixture.lean](CanonicalFixture.lean), the exact F17 degree 5→3 result.

Use the pinned toolchain and locked dependencies:

```sh
cd research/prime_factor_base_degree_20261031/proof
lake exe cache get
lake --wfail build
lake env lean --run ../../../tools/lean/AuditAxioms.lean -- \
  CanonicalFixture DegreeBridge SecantFixture RelationDegree \
  SecantTopForms DenseTopForms SecantQuadratic SecantIdeal \
  SecantIdentity SecantGraphTop TopDegree
```

The audit must say `0 failing, 0 trusted (native_decide)`. Generated equation
lemmas are included in its 121 declarations; this is not a claim of 121 distinct
mathematical results. All transitive axioms must lie in `propext`,
`Classical.choice`, `Quot.sound`. The existing proof gate discovers the Lake
project and its tracked modules. No gate or trust-manifest change is made.
Keep the committed dependency manifest; the commands materialize its pinned
Git revisions rather than updating dependency branches.

If the cache is unavailable, build the pinned dependency from source and run
the same final build/auditor. Cache download here returned proxy CONNECT 403.
Mathlib's supported `MATHLIB_NO_CACHE_ON_UPDATE=1` suppressed only its
post-update download hook; compilation/axiom checks remained. The final
command was `lake --no-cache --wfail build`. `XDG_CACHE_HOME` used writable
workspace storage, without changing `HOME`. `lean4checker` was unavailable.

The recorded compiler was `/workspace/prime-degree-lean/bin/lean`, Lean 4.19.0.
Local `.lake/packages` symlinks shared the source-built dependency; they are
ignored. The committed manifest pins normal Git dependencies, with no local
path override.

Final receipts: [verification/](verification/). `runs.tar.xz` preserves
dependency-build logs, failed development attempts, initial source snapshots,
cache errors and Lake-update logs:

```sh
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
```

Extracted `runs/proof-development-20261031/development/` maps to the original
working `verification/development/`. Failed logs with Lean's error-generated
`sorryAx` are retained failures; only the final successful build and transitive
audit establish the proof. Published Lean source has no such placeholder.

Repack the extracted tree deterministically:

```sh
tar --sort=name --mtime='2026-10-31 00:00:00Z' --owner=0 --group=0 \
  --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs \
  | xz -6 -T1 > FRESH-runs.tar.xz
sha256sum FRESH-runs.tar.xz
```

Compile `reports/formalization.typ` with Typst to reproduce the illustrated
report. The SVG is editable source; rendering performs no research arithmetic.
