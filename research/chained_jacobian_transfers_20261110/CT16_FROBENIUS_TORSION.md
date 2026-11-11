# CT16: cubic-extension 2-torsion distinguishes operational Richelot targets

Date: 2026-11-11. Parent PR #3877. Scientific standing: [pcc:unverified].

## Result

For both frozen operational prime-field genus-2 fixtures, F17 E19 and F101 shifted E109, the source and either reflection-equivalent Richelot target have **nonisomorphic Jacobians as unpolarized abelian varieties over the base field**. The obstruction is:

\[
\#J_C[2](\mathbb F_{p^3})=16,\qquad \#J_D[2](\mathbb F_{p^3})=4.
\]

An Fp-defined unpolarized-Jacobian isomorphism must commute with Frobenius and thus preserve both counts. This does not rule out isogenies or isomorphisms after extending the field. The previous PGL2 source/target curve-isomorphism audit did not prove this statement.

### Frozen sextics (ascending coefficients)

| Fixture | p | Source f | Target g, reflected target g(-x), raw |
|---|---:|---|---|
| E19 | 17 | [2,0,2,0,0,0,1] | [10,6,14,2,12,10,9]; [10,11,14,15,12,7,9] |
| shifted E109 | 101 | [56,0,13,0,6,0,1] | [56,41,62,78,24,57,80]; [56,60,62,23,24,44,80] |

The exact source factorizations modulo p are

\[
f_{17}=(x^3+6x^2+x+7)(x^3+11x^2+x+10),
\]
\[
f_{101}=(x^3+25x^2+63x+67)(x^3+76x^2+63x+34).
\]

Each cubic is irreducible. Each of the four target sextics is irreducible.
The Rust test checks polynomial products, degree-three Frobenius orbits and Rabin's irreducibility conditions for each normalized target: x^(p^6)=x mod g, gcd(g,x^(p^3)-x)=1, gcd(g,x^(p^2)-x)=1. The source Frobenius branch-root cycle type is therefore (3)(3), while target type is (6). All sextics are separable.

### 2-torsion proof

For a smooth odd-characteristic genus-2 sextic, the fifteen nonzero geometric 2-torsion classes correspond to unordered pairs of its six finite Weierstrass branch points. Frobenius^n fixes a class iff it fixes the corresponding unordered pair. Counting the fifteen pairs and identity gives:

| Extension degree n | Source (3)(3) | Target (6) |
|---:|---:|---:|
| 1 | 1 | 1 |
| 2 | 1 | 1 |
| 3 | **16** | **4** |
| 6 | 16 | 16 |

At n=3 the source six roots are individually fixed. The target Frobenius^3 is three transpositions, so precisely three nonidentity pairs are fixed. This is a base-field unpolarized-Jacobian isomorphism obstruction, regardless of polarization or the producer's point-count/Cantor claims.

### Nonbacktracking consequence

There are 15 geometric perfect matchings of six branch points. A (3)(3) Frobenius permutation fixes exactly three; a (6) permutation fixes just one, the opposite matching {(0,3),(1,4),(2,5)} in cyclic root labels. The standalone test enumerates every matching.

**Conditional on the existing operational first Richelot isogeny being valid**, its Fp-rational dual uses a stable matching on the target. The unique stable target matching therefore is its dual kernel. Hence these frozen codomains admit **no nonbacktracking second single Fp-rational Richelot step**. This excludes neither an extension-defined pair with rational composite (double Richelot), nor other isogeny degrees/intermediates.

### Reproduction and limitation

Committed standalone std-only Rust integration test:
research/chained_jacobian_transfers_20261110/native/tests/ct16_frobenius_torsion.rs

On a Rust 1.99.1 runner:

    manifest=research/chained_jacobian_transfers_20261110/native/Cargo.toml
    cargo fmt --manifest-path "$manifest" -- --check
    cargo clippy --manifest-path "$manifest" --all-targets --features small-prime --offline --locked -- -D warnings
    cargo test --release --manifest-path "$manifest" --features small-prime ct16_frobenius_and_unpolarized_jacobian_obstruction -- --nocapture

Independent local development arithmetic was run using a separately written Node.js polynomial-quotient oracle and preliminary SymPy factorization. The exact Node output/digests are recorded in evidence/ct16-oracle-01.txt. These are producer development checks, not signed PCC. Rustc/Cargo/rustfmt are absent in this session, so the Rust test is committed **uncompiled**. Exact-head CI, strict lint, and independent signed scientific standing remain outstanding. [pcc:unverified]

### Sources / research obligations

Reviewed canonical QUEUE.md, session logs, NONSPLIT_KERNEL_CHECKPOINT.md, NONSPLIT_MAP_CHECKPOINT.md, SHIFTED109_CHECKPOINT.md and SOURCES_20261111.md at parent head 8fbc0ac385e209483744753caeaf597e14a5a8be. Primary prior art: Magma Richelot handbook (https://magma.maths.usyd.edu.au/magma/handbook/text/1615), including Galois-stable kernels and double Richelot; Castryck–Decru–Smith (2020), DOI 10.1515/jmc-2019-0021, for Weierstrass-pair J[2] and dual Richelot.

KB MCP/CLI, locked encrypted autoresearcher atlas, S3 checkpoint, live budget and unsigned PCC remain blockers as in the parent queue. Do not write plaintext into the atlas, rewrite frozen ledger history, launch paid compute, or promote a solver-cost claim.

**Next:** investigate extension-field double Richelot composites or another isogeny degree, explicitly verifying rational composite kernel, maps/duals, subgroup images and recovery. No general impossibility theorem or DLP speedup has been established. [pcc:unverified]
