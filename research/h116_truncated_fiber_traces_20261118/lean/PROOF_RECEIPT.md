# H116 Lean identity-layer receipt

Toolchain: Lean 4.34.1 with Mathlib revision
`d13f23b723b8a846827a245b89c10fc7d3f11612`.

Source SHA-256:

```text
9544320fb1f4b8d53aa2013eb7feb1c0503c3ef9e6cc33cf1d008c4181339a90  H116FiberTrace.lean
```

The repository proof gate built both the default target and the explicitly
tracked source, then transitively audited all theorem declarations:

```text
PASS  9 thm  0 trusted  research/h116_truncated_fiber_traces_20261118/lean
```

Receipt SHA-256 values:

```text
7f163677f2ec320b6801597ed9f3dc31f1acfb62089eec00116b492715914a25  receipts/audit.log
424fab7b57eef10ca2e7f0d89f412708ad00bce4cf045f906493cfaf619a4b71  receipts/build.log
29e284dc7d8024aa6a333a92d0b23035028d78ada47b37f7dd77d8e1a0293483  receipts/cache.log
5ae0c3215cad558dac1cea3d739a54a1fcea0582a30e4ead224d811eecd96ffc  receipts/source-build.log
c26de83abdc9496cd1301470918ec39ecca1cf389ef0ae1c6504da1800d1c431  receipts/status.txt
```

Every theorem reports only `propext`, `Classical.choice`, and `Quot.sound`.
There is no `sorry`, custom axiom, `native_decide`, trusted declaration, raised
limit or curve-specific group-law copy.

Scope: the artifact proves the generic product logarithmic derivative,
finite-root reciprocal sum, abstract coefficient-to-power-sum implication,
relative denominator cancellation and simple-root residue cancellation. It
does not construct the Laurent-at-infinity coefficient instance, the
`V-1` formal jet, a scalar predicate or an ECDLP decoder. [pcc:unverified]
