# H116 formal-scalar OOS oracle source gate

Status: bound before the first selector or map execution under protocol
`H116-FORMAL-SCALAR-OOS-001`.

The native oracle source SHA-256 is:

```text
9c5c835bbfde7455b4c3aa1cd514a0393d8deea90e85d3d0274a4bc370b94d9e  h116_formal_scalar_oos_oracle.rs
```

It compiled without warnings using `rustc 1.100.0 (0ef1bc386 2026-11-09)`:

```text
rustc --edition=2021 -O -D warnings h116_formal_scalar_oos_oracle.rs \
  -o /tmp/h116_formal_scalar_oos_oracle
```

The pre-run binary SHA-256 is:

```text
ff9f7e1c2dcdf147d5ae95ac0e66295e9a68f12a29cc5cc206772baf7139ed11
```

The source selects alternative models, constructs full degree-`r` Vélu maps
and emits oracle JSON. It does not import or evaluate the Python formal-scalar
candidate. Candidate comparison and independent verification are separate
post-oracle programs. [pcc:unverified]
