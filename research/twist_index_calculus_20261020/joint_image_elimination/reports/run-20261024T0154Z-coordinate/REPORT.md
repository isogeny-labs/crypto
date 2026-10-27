# Split source-block coordinate proofs pass

Gate `coordinate-lean-v4` checks the unchanged four coordinate statements
in [Coordinates.lean](../../proof/orbit-v2/Coordinates.lean).
The metadata preserves the exact command, source hash, source snapshot,
launch and finish times. This separates the coordinate obligations from
the larger quartic expansion without weakening their statements.

![Algebra to algorithm boundary](../../figures/joint-mechanism.svg)

The build and axiom audit pass all four theorems: phi and doubling cross
products, and their respective specializations under U^12=T. They hold
over any `Lean.Grind.CommRing` with the stated orbit hypothesis for the
specializations. The [audit log](../../results/proof-checks/coordinates-v4/audit.log)
records only `propext`, `Classical.choice` and `Quot.sound`, with zero
failing and zero trusted-native theorems.

The native IDC checks separately certify the exact integer cross products.
Field division, nonzero denominators, curve-map interpretation and source
partition membership remain separate bridges; native fixture checks cover
the declared 21 cases. These proofs do not certify elimination-ideal
equality, complete roots, independent rank or speed.

Pinned Lean 4.34.1, one worker, 8 GiB, a 900 s watchdog and the busy
reservation are retained. No comparable performance values are admitted.
