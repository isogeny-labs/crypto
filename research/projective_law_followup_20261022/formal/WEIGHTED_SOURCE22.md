# Weighted common 2+2 source prefix

The native `verify-weighted-source22` check matches the literal Rust assignments from input destructuring through the two ordered `B=0` and `C=0` early exits. It rejects a changed cross-scaled input, determinant sign, and branch guard. The Lean file `WeightedSource22.lean` defines the corresponding field-valued coefficient and branch model. Its coefficient theorem gives the exact `Z^8`, `Z^9 W`, and `Z^7 W` scales under the stated weighted numerator equations. Its branch theorem characterizes the model's successful prefix by nonzero `B` and `C`.

These are source-shape and algebraic results. The native check does not translate compiled Rust to Lean, and this packet does not prove that the concrete `Field` methods satisfy the Lean field operations. The model ends before the returned projective coordinates. The principal-divisor and `Pic^0` correspondence remain open. No cost ratio changes follow from this packet.

The [machine-readable source certificate](weighted-source22.json), [obligation table](weighted-source22.csv), and [graph](weighted-source22.svg) give the exact proof boundary. `weighted-source22-build.log` records the Lean kernel axiom audit. `WEIGHTED_SOURCE22.pdf` embeds the graph.
