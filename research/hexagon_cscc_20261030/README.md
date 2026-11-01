# Hexagon computational-complexity integration — 2026-10-30

This is a literature-integration and software-correctness change. It contains
no new benchmark, asymptotic proof, ECDLP recovery result, or research-state
promotion. The [subject query](https://hexagonmath.org/search?subject=cs.CC)
returned **22 works**, including cross-listings, with no next page. This is a
dated snapshot, not a recurring watcher or a claim to cover future deposits.

## Coverage and provenance

[inventory.json](inventory.json) records all returned versioned identifiers,
contributors, primary URLs and SHA-256 digests of the PDFs actually retrieved.
The review covered statements, constructions and limitations relevant to this
repository; it was **not an independent full-proof audit**. Local formal-proof
verification is `not_run` for every paper. A publisher's Lean claim is recorded
as reported, including distinctions between compiled checks and kernel-only
checks. No proof is inferred from a PDF, abstract, passing unit test or source hash.

[catalog.json](catalog.json) joins the complete inventory to
[learnings.json](learnings.json): every work has an operational learning,
assumptions, integration paths, implementation status and evidence boundary.
The native [retrieval tool](../../tools/complexity-catalog/README.md) checks
coverage and exposes those constraints to future work. Its tests reject missing
versions, duplicate papers, unbound sources and unsupported proof promotion.

## Executable integrations

| Requested scope | Delivered artifact | Qualification |
| --- | --- | --- |
| Factoring algorithm | `factoring::DivisorNetwork`, `smallest_prime_factor`, `factor_integer`; `tensor-factor` CLI | Exact bounded reduction using the existing counter; not a reproduction of the paper's Lean engine or its runtime proof |
| All papers in the subject collection | Version-pinned inventory, searchable native catalog, per-paper applicability and implementation status | All 22 covered; source theorems remain literature claims until independent verification |
| Weighted-counting and learning lessons | `weighted::cost_profile`, with exact coefficient counts and min/max extraction | Exhaustive reference backend; source subexponential weighted contraction is not ported |
| Negative results relevant to factor bases/solvers | Native finite controls for the three-generator Boolean construction and overlapping affine-subspace deletion | Finite witnesses, not a new proof of the unbounded degree or asymptotic statements |
| Other prime-field bases | Factory recipes `prime_multiplicative_coset`, `prime_norm_one_trace`, `prime_dickson_image` | Exact materialization, subgroup filtering and existing archive/replay; no new algebraic PDP solver or performance claim |
| Sibling repositories | Byte-identical new tensor modules in `cryptanalysis`; source notes and retrieval pointers in Crypto Autoresearcher | Pre-existing differences elsewhere in the tensor mirrors are recorded, not overwritten |

### Factoring theorem map

For `n`-bit input, the source's Theorem 1 reports one deterministic machine
with bound `C_k * 2^(n/k)` for every positive `k`. It is reported formalized in
Lean. Theorem 2 obtains complete factorization by repeated smallest-factor
calls and is explicitly not formalized in that note. Its stronger
`SqrtLogLogConjecture` is a proposition, not a proved runtime.

The local implementation realizes the section-4 reduction: encode `d*e=N`
with `d>1`, count fixed-width divisor prefixes, choose each least feasible
bit and divide out the recovered factor. The multiplication gadget has
uniquely determined arithmetic witnesses. `N` itself is included, `1` is
excluded, and leading zeros are part of the interface. Positive integer `e`
is forced by the exact nonzero product. Repetition retains prime powers;
one has an empty factorization and zero is rejected by complete factorization.
The smallest-factor API matches Mathlib's boundary conventions `minFac(0)=2`
and `minFac(1)=1` explicitly; those cases require no divisor network.

The counting backend is the existing OpenAI Appendix-D implementation, not
the Anthropic network engine re-proved in the Hexagon note. It returns exact
integer counts, so this integration does not need to port the note's separate
modular zero-detection argument. The source's Mathlib compilation theorem is
also not a proof of this Rust implementation. Finite-size fallback is
exponential. An exhausted meter remains an error, never a zero count or a
completed partial factorization. The remaining general audit is tracked in
[the existing tensor problem](../../problems/hashed-ecdl-tensor-backend.md).

```sh
cargo test --locked --release --manifest-path crates/hashed-ecdl-tensor/Cargo.toml
cargo run --locked --release --manifest-path crates/hashed-ecdl-tensor/Cargo.toml --bin tensor-factor -- 12 100000000
cargo run --manifest-path tools/complexity-catalog/Cargo.toml -- --check
cargo run --manifest-path tools/complexity-catalog/Cargo.toml -- locality
cargo run --manifest-path tools/complexity-catalog/Cargo.toml -- certificate degree
```

### Prime-field factor-base search

The internal prior-art review included the lead atlas's algebraic-membership,
PKM-tower and SAT-encoding leads; `research/prime_ic_program_20261029/`; the
factor-base encyclopedia; and `research/pkm_dickson_torus_20261015/README.md`.
The main external construction reviewed directly is Petit–Kosters–Messeng,
PKC 2016, [sections 3.1–3.4](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf).
Its subgroup and auxiliary-isogeny bases precede this work. APS
[2017/609](https://eprint.iacr.org/2017/609) supplies a related single-relation
direction already represented in this repository; its abstract was retrieved,
and its full proof was not independently reviewed in this session.

The additions extend the standalone factory beyond its progression/random-x
recipes. A multiplicative coset accepts any bounded exact order dividing
`p-1`, not just powers of two. A norm-one torus trace uses an explicitly
checked generator in `Fp[s]/(s^2-nu)` with nonsquare `nu`, norm one and exact
order dividing `p+1`; inverse elements share traces, so orbit count is not
usable point count. A Dickson image evaluates `D_d(x,a)` on an explicit
progression and deduplicates the output. The degree-times-count work cap is
checked before enumeration. It is an image recipe, not a claim of cheap
membership. Every recipe retains both curve signs only after the existing
`[r]P=O` filter; no silent cofactor projection changes its coordinate set.

Kummer/isogeny towers, CM/GLV orbits, affine-hashed bases and coordinate-window
bases already exist in other native modules. Their existing solver and
general-audit obligations remain visible. A factor base in `Fp*` for ordinary
index calculus or NFS is not an elliptic-curve factor base. Factoring `p-1`
does not by itself solve a discrete logarithm or establish an ECDLP transfer.

## Lessons that change how the existing tools are used

- Keep ETH, fixed-width SETH, unbounded-width CNF-SETH, counting hypotheses
  and Orthogonal Vectors assumptions separate. Measure the actual encoding
  size and locality before importing a counting exponent.
- Weighted interpolation needs the source's coefficient-algebra assumptions.
  The local profile backend keeps every coefficient and reports overflow of
  its declared degree bound; no silent truncation or min-plus substitution.
- Low input degree, a low first-fall degree, small matrix blocks or a compact
  membership circuit do not alone bound solving degree, certificate degree
  or total decomposition cost. The three-generator control preserves that
  distinction in a small exact fixture.
- Per-subspace retention does not imply retention of their union. Use actual
  deduplicated points, coverage, subgroup membership and matrix rank when
  selecting a base; the affine-union control explicitly checks the overlap.
- A restricted-policy lower bound is not a lower bound on all algorithms.
  The Fourier-overlap example constrains that query rule; controller width
  alone does not characterize the tractability of a specified reachable image.
- Distinguish universal guarantees from per-instance certificates and
  conjectures in ensembles. Preserve literal inputs, bit lengths, multiplicity,
  distinct-index requirements, randomness and preprocessing when importing
  numerical-analysis refinements or reductions.
- Model-specific hardness and finite-state upper bounds supply reusable
  encoding lessons, but do not become crypto algorithms without a concrete
  reduction. The autocatalysis, D-stability and Bawo entries retain that scope.

Every paper's more specific assumptions and code touchpoints are in the
catalog. None is silently marked implemented because it has been read.

## Verification and open obligations

[VALIDATION.md](VALIDATION.md) records actual commands, results, errors and
source parity. Unit tests are development correctness checks, not experiments
or qualified timings. No EKS job was requested or launched for these checks.
Existing leaderboard/frontier plots were checked for affected claims; no
measurement, admitted bound or state transition changes, so their data and
renderings remain current. [integration.svg](integration.svg) and
[report.pdf](report.pdf) describe this change instead.

Independent formal checks, porting the source-specific Anthropic counting
engine, general correctness/complexity audit, a subexponential weighted
backend, the fast Subset Sum/3SUM implementations, and performance comparisons
remain **unperformed**, not refuted or declared infeasible. Their source
claims and missing bridges remain in the catalog rather than becoming runtime
guarantees. This implementation work does not close those mathematical tasks.
