# Native verification correction readback

Root inspected both replacement Rust verifiers and the graph thin-shell replay runner after the reviewers' additive corrections. The quotient verifier checks exact module order, native exits, before/after source hashes, the 36-name inventory and allowed axiom list; its runner propagates native failure. The graph verifier checks exact module order, source-byte equality, the 34-name inventory, allowed axioms and six rejection controls; its shell runner checks source/dependency hashes and propagates Lean exit status.

Root reran both native verifiers against the fresh successful fixed Lean replay directories: both returned exit 0. Retained outputs are quotient-native-root-readback.json and forest-native-root-readback.json. This is source inspection and independent invocation of the reviewers' built debug binaries, not an independent recompilation of those verifier binaries. Prior root familiarity with earlier proof interfaces is disclosed.

The first root invocation assumed release-profile binaries and returned 127 because those paths did not exist; the corrected invocation used the actual debug binaries. No proof source was changed. The original Python workflow violation remains recorded, and the historical status files remain immutable; this additive note records the native correction readback.
