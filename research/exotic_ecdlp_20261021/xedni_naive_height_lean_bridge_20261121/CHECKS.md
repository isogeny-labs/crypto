# Lean verification receipt

Date: 2026-11-21. Run from `/Volumes/SSD990/cryptanalysis/experiments/xedni-public-first-large-holdout-20261115/formal` using its pinned Lean 4 and Mathlib installation. The packet's `formal/lean-toolchain` and `formal/lake-manifest.json` are byte-for-byte copies of that installation's pins; `formal/build.sh` compares them before building.

The predecessor `FixedCurveSquareDenominator.lean` compiled with `lake env lean -R <predecessor-formal-dir> -o <packet-formal-dir>/FixedCurveSquareDenominator.olean <predecessor-formal-dir>/FixedCurveSquareDenominator.lean`. The current module compiled with:

```sh
lake env sh -c 'LEAN_PATH="$1:$LEAN_PATH" lean -R "$1" "$1/NaiveHeight.lean"' sh /Volumes/SSD990-2/crypto-worktrees/exotic-ecdlp-recovery-20261119/research/exotic_ecdlp_20261021/xedni_naive_height_lean_bridge_20261121/formal
```

Exit status: `0`.

```text
'XedniNaiveHeightBridge.exists_coordinate_height' depends on axioms: [propext, Classical.choice, Quot.sound]
'XedniNaiveHeightBridge.naiveHeight_eq_log_coordinate' depends on axioms: [propext, Classical.choice, Quot.sound]
'XedniNaiveHeightBridge.coordinate_cap_implies_naiveHeight_bound' depends on axioms: [propext,
 Classical.choice,
 Quot.sound]
```

`lean4checker` was not installed in this environment. The module contains no `sorry`, `admit`, custom axiom, or native evaluation in its proofs.
