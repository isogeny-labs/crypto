# Completed genus-three prime-field descent

This fixture adapter maps encoded points of E(Fq^3) into the canonical
Picard group of an exact historical plane quartic over Fq, for q=5,7,11.
It constructs the completed degree-two fibre ideal, multiplies its three
Frobenius conjugates, descends by coefficient traces and subtracts the actual
origin fibre. The runtime takes coordinates, field and model tags; no scalar
label or support-root oracle is an input. [pcc:unverified, c162f3]

The original `diem_quartic` field/polynomial/core and `plane_quartic_pic`
completion/canonical reduction/group operations are normal dependency modules.
The separate native test oracle uses original `isogeny_algos` FpR3/FpR6,
explicit embeddings, geometric points and local jets. The only changes to
existing engines are recorded visibility and thin original-operation accessors.

`G3_DESCENT_PROTOCOL.md` preceded implementation. The model and ramification
addenda retain the failed assumptions and freeze their corrections before
the next attempts. Unit03c162f3 checks every source point (148/356/1324),
all generating edges, every odd-subgroup scalar (37/89/331), complete fibre
schemes and all twelve geometric branch points. The observed full-source
kernel has size2 in each case. [pcc:unverified]

From the repository root, with Rust1.99.1 and Lean4.35 on PATH:

```sh
cargo test --release --offline --locked --manifest-path research/chained_jacobian_transfers_20261110/genus3-prime-descent/Cargo.toml --lib -- --nocapture --test-threads=1
bash research/chained_jacobian_transfers_20261110/genus3-prime-descent/check.sh /absolute/fresh/output-directory
```

Set `CARGO_TARGET_DIR` to a fresh cache outside the output directory for a
recorded replay. The focused driver records native and tee exits separately
and runs final input guards on either success or failure. It retains all
four native tests, bounded CLI fixtures, both original proof gates and all
eleven exact original PCC declaration bindings.

The CLI binary is `ghs-prime-descent INPUT.json OUTPUT.json`; name it explicitly
with `--bin` when using Cargo, because the package also has an observational
ramification-panel binary. The input schema is `ghs-prime-descent/v1` with
`q` and `points`. Each point has exact `field`, `model`, and `coordinates`
(null for O or two canonical triples). It rejects unknown fields, foreign
models, off-curve/noncanonical coordinates, more than256 points and more
than65536 bytes. Output is created exclusively and contains canonical Picard
keys. See the bounded literal fixtures and the process-control receipts.

`G3_PROOF_MAP.md` maps the eleven local formal statements to their written
geometric bridges. Actual curves, Cartier ideals, Picard functoriality,
Galois descent and reducer correctness are not formalized by the conditional
group lemmas. An arbitrary-class norm and a destination index-calculus solver
are separate implementation tasks. This package reports correctness only.
