# H116 curve-coordinate protocol

Protocol: `H116-CURVE-COORDINATE-001`. Frozen before native execution.
[pcc:unverified]

The panel fixes the excluded p31 fixture and the four completed different-j OOS
models. For 24 coefficients, the native producer must construct the source and
scaled-target coordinate units independently and check:

1. `q=1+A z^4q^2+B z^6q^3` and its cleared inverse form;
2. `q_target(z)=q_source(a z)` for target `(a^4A,a^6B)`;
3. `q_target(zNormalized)=q_source(zScalar)` when
   `zScalar=a*zNormalized`;
4. equality of the two resulting complete normalized x-ratio expressions.

The wrong target scaling `(a^2A,a^4B)` and wrong scalar parameter
`(a+1)zNormalized` must each produce a retained first difference in every
cell. No cell may be replaced. Exact field-operation diagnostics are recorded
without timing or speed-ratio interpretation. [pcc:unverified]

The producer uses fixed-point construction; the independent verifier uses a
coefficient-by-coefficient recurrence and parses only the retained TSV. Both
are native Rust. This control tests curve-coordinate algebra, not `V-1`
semantics or ECDLP recovery. [pcc:unverified]
