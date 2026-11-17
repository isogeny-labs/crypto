# Boolean bounded-Macaulay certificate audit

Build and test this small native crate independently of the main library:

```sh
cargo test --locked --manifest-path research/n83_factor_base_designs_20261031/native/Cargo.toml
cargo build --release --locked --manifest-path research/n83_factor_base_designs_20261031/native/Cargo.toml
```

Input JSON schema (all fields mandatory; unknown fields rejected):

```json
{"variables":3,"max_degree":4,"polynomials":[[0,3],[0,6],[5]]}
```

Each polynomial is a sorted duplicate-free list of squarefree monomial masks.
Mask 0 denotes 1, mask 3 denotes x0*x1; the empty list is the zero polynomial.
The example encodes x0*x1+1, x1*x2+1 and x0*x2.

After the repository's experiment checkpoint and execution gates pass:

```sh
degree-certificate certify system.json certificate.json
degree-certificate verify system.json certificate.json
```

The tool writes a fresh JSON file, refuses overwrite, binds the exact input
bytes by SHA-256 and self-audits the result before writing. For a refutation
at D it emits generator/multiplier recipes summing to 1 and a dual vector
annihilating every bounded row at D−1 while pairing to 1 with the constant.
The verifier checks those polynomial identities using set-valued multiplication,
checks the truth table, and recomputes profile metadata. For no refutation
through the cap, it emits only the last-degree dual; SAT and an unresolved
UNSAT input retain distinct truth-table counts. Neither becomes an F4-degree
or homogeneous-regularity conclusion.

Caps: 18 Boolean variables, 64 generators, 4,096 rows and 4,096 columns per
matrix. Exceeding a matrix cap returns an explicit UNKNOWN error. The caps
limit this tool to small certificate controls; N83 production systems require
a scalable exporter/certificate producer and a separate complete verification
path. The generator-plus-multiplier degree is measured before Boolean reduction;
reduced products may have smaller degree. Every generator remains in the input,
including ones too large to contribute at a particular degree.

Unit tests include exact-degree fixtures, corrupt certificates, incorrect input
bindings, SAT versus capped-UNSAT cases, constant contradictions, resource caps
and all 256 pairs of two-variable Boolean generators. These validate the audit
tool; they are not factor-base experiment results.
