# Producer proof receipts and exact scope

Pinned Lean4.35.0, CryptoLean148f5dc389b341401e06d76abe7783c4ccf42f2e,
Mathlibcfc55fc58714a59dc06e6e79edc6d1d95140d3c2.

CommonScalar build003 and RationalScalar build006 completed. The actual
expanded repository source gate reports47 declarations,0 trusted,0 failures;
only propext, Classical.choice and Quot.sound are reported in transitive
axioms. Receipts in gate-final/. Earlier gate/ covers the polynomial candidate
before rational additions. This is producer evidence pending independent
trusted standing. [pcc:unverified]

Reproduce with the pinned toolchain bin directory on PATH:

```sh
cd research/prime_common_scalar_20261108/lean
lake build CommonScalar RationalScalar
lake env leanchecker --fresh --verbose RationalScalar
```

From repository root, stage actual sources, then:

```sh
bash tools/lean/proof-gate.sh --out /tmp/common-scalar-gate \
  research/prime_common_scalar_20261108/lean
```

The original CommonScalar import-inclusive fresh replay completed exit0,
retained as fresh-001.log. The final expanded RationalScalar fresh receipt
is rational-fresh-001.log and fresh-status.json. It includes the current
CommonScalar public formula and exposed local kernel body. No parent source,
trust list, checker, axiom policy or kernel was altered. [pcc:unverified]

All failed attempts remain: first polynomial remainder proof; initial hidden
imported-definition unfold; module-export exposure requirement; redundant
unused-tactic warning; wrong-working-directory build. Only the final clean
build and actual source gate are accepted. No inferred sorryAx from a failed
compilation is used as evidence for a theorem. Exact theorem assumptions and
still-open bridges are in PROOF_MAP.md.
