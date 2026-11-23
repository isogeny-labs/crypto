# E18/E20 native gluing certificates, version 1

Date: 2026-10-23. Commit this protocol and native source before executing.
This starts the second execution wave; all E18–E29 designs and all original
E00–E17 requirements are retained. A finite filter or torsion certificate is
not a polarized quotient, subgroup map, complete IC comparison or novelty claim.

## Inputs and controls

Read the existing `execution-v1/fixtures.json`, SHA-256
`8d490a36d65ebf906b93e04eeeb21ac5802f2a9382ef03271a4648fd4b78a4ed`,
and the committed canonical registry. The ten ordinary toy fixtures keep
their exact field/model/order/generator/twist identities. Public NIST
P-192/P-224/P-256/P-384/P-521 are parameter-only inputs; their published
whole-group orders are not newly proved prime here. No public NIST target
logarithm is used. Preserve registry, fixture, source, lockfile, binary and
tool-version hashes in each receipt.

E20 crosses all fifteen sources with 5,7,11,13,17: 75 cells. Repeat the
fifteen original 3-torsion module cases as regression controls. Enumerate
all determinant-minus-one matrices M satisfying M*A=(-A)*M for the exact
companion matrix A over F_ell. Exclude characteristic/subgroup-prime cells
explicitly. Save every matrix and every zero row. Native Rust emits a
separate GP replay using independent modular-matrix arithmetic. A successful
filter is necessary-module evidence only. Actual higher-prime torsion and
quotient construction remain separate requirements of E20/E19.

E18 processes the three already-positive toy fixtures (p=211,751,10111) and
public P-521. Each curve has a separate fresh directory and process cap.
Construct the actual division polynomial, split it into quadratic factors
using at most 128 deterministic equal-degree trials, and construct all eight
nonzero 3-torsion points. Use Fp4 with a fully specified irreducible tower:
w^2=d, v^2=e+w, where e is the first integer in 0..127 whose tower norm is
a nonsquare. The flat modulus is v^4-2e*v^2+e^2-d. Arbitrary-size prime
arithmetic uses native Rust BigUint, including the public P-521 case.

The specialized square-root adapter requires p=3 mod 4; this holds for all
four prior-positive inputs and is checked, not assumed for other curves.
Membership at extension degrees 1,2,3,4 is checked in increasing order by
actual Frobenius fixed points in the constructed algebraic closure. Finding
full torsion at degree four stops the original degree-at-most-eight search;
no separate smaller-field polynomial construction is needed to reject a
nonfixed point. Failed degree/factor/square-root checks remain failures.

Give a basis, its nontrivial Weil pairing, both actual Frobenius matrices,
all four anti-isometries and all nine points in each Galois-stable graph.
Check all 81 basis-pair bilinearity cases. Reject dependent bases, the
identity matrix's wrong pairing direction and the wrong twist Frobenius
sign. The old seven negative toy/four negative NIST cases retain their
matrix-filter rejection; no vacuous positive torsion certificate is issued
for them. GP independently checks field irreducibility, curve membership,
order, pairing, Frobenius and each graph. GP completion markers and an empty
error stream are required; GP exit code alone is insufficient.

## Commands and bounds

From repository root, Rust 1.90.0:

```sh
cargo build --release --locked --manifest-path research/twist_routes_1_3_20261021/runner/Cargo.toml
research/twist_routes_1_3_20261021/runner/target/release/twist-route-experiments followup filters research/twist_routes_1_3_20261021/execution-v1/fixtures.json FRESH_FILTER_DIR
research/twist_routes_1_3_20261021/runner/target/release/twist-route-experiments followup torsion research/twist_routes_1_3_20261021/execution-v1/fixtures.json FRESH_TORSION_DIR SOURCE_NAME
gp -f -q FRESH_DIR/verify.gp
```

Use one thread/CPU and 4 GiB address space per process. Cap the 90-cell
native filter batch and its GP replay at 1,400 seconds each (15 seconds
per configured cell plus startup allowance); cap each of the four torsion
constructors and its GP replay at 1,800 seconds. Checkpoint JSON after each
cell/curve. Store stdout, stderr and exit status outside the raw output
directory. A timeout, panic, invalid certificate or missing independent
marker leaves the cell unresolved; retain it and version corrections.

These runs are L0 correctness/applicability work. No wall-time ranking,
complete operation pricing, source-group DLP, geometric quotient existence
certificate or speedup is admitted. Field/torsion certificate coordinates
retain the registered source identity and exact extension presentation;
canonical extension registration is an explicit gate before a new extension
slug is cited or a cross-method comparison is attempted.

## Next obligations

E19 uses the explicit genus-2 family and degree-three maps in Broeker–Howe–
Lauter–Stevenhagen, arXiv:1403.6911, Algorithm 5.4 and Appendix A. Actual
polarization, curve smoothness, map identities, subgroup injectivity and
scalar recovery must be checked separately. E20's selected actual torsion
at larger primes, and all E21–E29 work, are still owed after these panels.
