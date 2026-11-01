#set page(paper: "a4", margin: 16mm)
#set text(font: "New Computer Modern", size: 9.5pt)
#let data = json("root-inventory.json")
#assert(data.status == "PASS_ROOT_INVENTORY")
= D18 input inventory
_N83 shared-bit encoding · native inventory v2 · 2026-10-21 UTC_

== Observed inputs and derived counts

The native inventory checks the complete pinned list of 16,627 selected
source representatives. It observes 66 common zero coordinates: bit 0
and bits 18–82. Coordinates 1–17 vary. This is an input-list invariant;
it does not compute affine rank or establish a general subgroup theorem.

#align(center, image("inventory.svg", width: 94%))

== Reproduction and verification

Curve: `icv1-f2m83-tm6151469093347-debefd74`.
The field is GF(2^83), with polynomial z^83+z^45+z^2+z+1. D18 is the
source space for the frozen representatives. Their full Frobenius phases
and signs remain available; only their pre-phase root coordinates are shared.

Input SHA-256: #text(size: 8pt, data.representatives_sha256).
Source SHA-256: #text(size: 8pt, data.source_sha256).
The native `root_inventory.rs` uses a u128 OR-of-XOR mask independently
of the library-backed construction tool's per-coordinate BigUint check.
It checks input digest, header, count, uniqueness, coordinate bounds and
positive multiplicities. A changed input and an existing output were both
rejected. The wrapper was compiled directly with rustc and already built
sha2/serde_json dependencies on the shared Apple arm64 host (L0).

== Open gates

No SAT model or solver search is executed by this inventory. The chart's
reductions are derived from m × 66 × 16,626; total model counts require
the full construction check. The checked Lean theorem proves the Boolean
replacement and count identity under its stated hypotheses; it does not
verify this parser, compiler or complete solver. Paired controls, natural
query yield/useful rank, cold/reused pipeline cost and matched rho remain
open. There is no runtime comparison or selected candidate in this receipt.

Sources and exact commands: `INVENTORY.md`, `root-inventory.json`,
`root_inventory.rs`, `PROOF_MAP.md`, `lean/SharedRootBits.lean`, and
`PROTOCOL.md`. Canonical performance dashboards are unchanged because
this result supplies no measured time or pipeline ranking.
