# Levelwise binomial family: checked parity core

This project proves the central **signed-integer binomial parity lemma** in
the proposed levelwise interpolation counterfamily. It also ties the actual
rational binomial polynomial to its integer values, proves origin value one,
and proves the required divisibility of the level-dependent polynomial's
node value. The complete coefficientwise-chart counterexample remains only
partially formalized; see [PROOF_MAP.md](PROOF_MAP.md).

The written construction under review is
`/workspace/audit/levelwise_binomial_counterexample.md`. Its scope is a
generic interpolation-data bridge for different levelwise polynomials.
Realizability by actual packet networks is not claimed, and this is not an
ECDLP impossibility result or a hardness statement for algorithms reading
explicit polynomial coefficients.

## What is proved

Mathlib's signed generalized binomial is `Ring.choose t n`, rather than an
`Int.choose` API. The strongest checked identity is, for **every** signed
integer `t` and every natural `i`,

```
(Ring.choose t (2^i) : ZMod 2) = ((t / (2:Int)^i : Int) : ZMod 2).
```

The division on the right is actual Euclidean integer division. Negative
integers are covered by a proved negative-binomial formula and the exact
negative-division identity; they are not omitted or treated as natural
absolute values without a bridge. It follows that for `t≠0`,

```
Odd (Ring.choose t (2^(padicValInt 2 t))).
```

The polynomial `binomialPolynomial n : Q[X]` is the actual generalized
`Ring.choose X n`. Ring-homomorphism lemmas prove that its value at every
signed integer `t` is the rational cast of `Ring.choose t n`. Define the
actual family `H_(i,p)=(1-binomialPolynomial (2^i))^p`. The checked consequences
are `H_(i,p)(0)=1` and, when `t≠0` and `padicValInt 2 t=i`,

```
2^p divides (1-Ring.choose t (2^i))^p
```

in the integers, with a proved equality between the cast of that integer
and the rational polynomial value. This is a statement about node values,
equivalently the chart's constant coefficient. Full coefficientwise scaled
chart integrality/divisibility, including all higher coefficients, remains
an explicit formal obligation. Node-value divisibility is not presented
as a proof of that stronger statement.

[MATHEMATICAL_PROOF.md](MATHEMATICAL_PROOF.md) gives the readable proof.

## Pins and reproduction

| Dependency | Exact pin |
| --- | --- |
| Lean | `leanprover/lean4:v4.35.0` |
| Shared library/auditor/gate | `aburan28/crypto-lean` revision `148f5dc389b341401e06d76abe7783c4ccf42f2e` |
| Mathlib | `cfc55fc58714a59dc06e6e79edc6d1d95140d3c2` |
| Transitive packages | Exact revisions in `lake-manifest.json` |

This session uses the previously checksum-verified official toolchain at
`/workspace/audit/toolchains/lean-4.35.0-linux/bin`. The local audit dependency
symlink is not part of the package; the manifest uses pinned Git dependencies.

```sh
MATHLIB_NO_CACHE_ON_UPDATE=1 lake update
lake exe cache get Mathlib.Data.Nat.Choose.Lucas Mathlib.NumberTheory.Padics.PadicVal.Basic Mathlib.RingTheory.Binomial
lake build
lake build +LevelwiseBinomial
lake env lean --run .lake/packages/CryptoLean/tools/AuditAxioms.lean LevelwiseBinomial
lake env leanchecker --verbose LevelwiseBinomial
lake env leanchecker --fresh --verbose LevelwiseBinomial
```

The unmodified pinned shared gate can check this project in its containing
Git repository. The exact commands, failed and successful source snapshots,
full logs, axiom output and actual verification statuses are retained in
`receipts/`. Nine reported theorems have explicit `#print axioms` commands.
No custom research axiom, `sorry`, `admit`, `native_decide` or kernel bypass
is used in the successful source. Ordinary `decide` proves only numeral facts.

Three main builds failed during cast normalization and matching `/` against
the exact `Int.ediv` head. Their sources and logs are retained; the corrected
fourth build proved the general signed identity, and the fifth build proved
the final nine-theorem module. An independent natural Lucas probe and signed
extension probe passed; their exact sources are preserved as `.lean.txt`
receipts so they are not extra proof targets.

The final build, explicit source build, transitive axiom audit, module replay
and literal shared proof gate passed. The audit checked nine theorems, zero
failing and zero native-trusted; only `propext`, `Classical.choice`, and
`Quot.sound` occur. Fresh import-inclusive replay also passed, exit 0, with
receipt `replaying LevelwiseBinomial with --fresh`. All exact outcomes are
recorded in `receipts/VERIFICATION.json`.

Neither frozen ternary package was modified. Their package SHA256 checks are
retained separately in the receipts.
