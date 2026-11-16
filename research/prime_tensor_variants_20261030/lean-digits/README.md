# Integer digits to ternary packet locality

This versioned addition closes the actual signed-integer valuation-to-digit
bridge left open in the earlier ternary label package. It also proves the
valuation coordinate fits in `Fin a` for distinct packet slots in `[0,3^a)`.
The final theorem invokes the existing structural locality theorem with
proved hypotheses. Full graph packing, the kernel support theorem, coefficient
precision, contraction complexity and ECDLP claims remain unformalized.

The frozen earlier package is unchanged. `TernaryPacket.lean` is an exact copy
of its source, SHA256
`a165aabdc3c5c6586722272b35fab805302ef6178f0be00b1a6b594e0ece114b`.
The new source is `TernaryDigits.lean`, with eight additional theorems.
All 62 files in the frozen package's SHA256 manifest were rechecked unchanged;
the output is retained in `receipts/frozen-package-verification.log`.

## Exact mathematical bridge

For every signed integer `x`, define

```
digit i x = ((x / (3 : Int)^i : Int) : ZMod 3).
```

Division is Lean's Euclidean integer division. It distributes over `x+t`
when `3^i` divides `t`; arbitrary signed inputs are covered. For
`t=y-x≠0` and `i=padicValInt 3 t`, the Mathlib maximal-power divisibility
theorems establish this divisibility and establish that

```
c = ((t / (3 : Int)^i : Int) : ZMod 3)
```

is nonzero. Therefore `digit i y = digit i x + c`. This is precisely the
hypothesis needed by the checked ternary label action. There is no assumption
encoding that conclusion.

`integer_slot_locality` permits arbitrary signed `x,y` and explicitly assumes
`i<a`. `valuation_lt_of_bounded_slots` proves this bound when
`0≤x,y<3^a` and `x≠y`: divisibility by `3^a` would contradict the strictly
smaller absolute difference. `bounded_integer_slot_locality` constructs the
actual `Fin a` coordinate and obtains the selected row/column label equality.
The labels belong to any Mathlib `AddCommGroup`; this arithmetic does not
identify the label field with a coefficient ring of characteristic three.

See [MATHEMATICAL_PROOF.md](MATHEMATICAL_PROOF.md) for the readable proof and
[PROOF_MAP.md](PROOF_MAP.md) for quantifiers, hypotheses and remaining gaps.

## Dependencies and reproduction

The exact pins are inherited unchanged: Lean `leanprover/lean4:v4.35.0`,
shared `aburan28/crypto-lean` revision
`148f5dc389b341401e06d76abe7783c4ccf42f2e`, and Mathlib revision
`cfc55fc58714a59dc06e6e79edc6d1d95140d3c2`. All transitive revisions are in
`lake-manifest.json`. This session used the previously checksum-verified
official toolchain at `/workspace/audit/toolchains/lean-4.35.0-linux/bin`.
Local dependency symlinks are an audit-workspace convenience; the manifest
contains ordinary pinned Git dependencies and no local paths.

```sh
MATHLIB_NO_CACHE_ON_UPDATE=1 lake update
lake exe cache get Mathlib.NumberTheory.Padics.PadicVal.Basic Mathlib.Tactic.Abel Mathlib.Tactic.FinCases Mathlib.Tactic.NormNum
lake build
lake build +TernaryPacket +TernaryDigits
lake env lean --run .lake/packages/CryptoLean/tools/AuditAxioms.lean TernaryPacket TernaryDigits
lake env leanchecker --verbose TernaryDigits
lake env leanchecker --fresh --verbose TernaryDigits
```

The unmodified pinned shared proof gate can check the project in its containing
Git repository. All checks and actual outcomes are retained in `receipts/`;
`VERIFICATION.json` distinguishes completed checks from any running check.
Builds, explicit module builds, module replay, transitive axiom audit and the
literal shared gate have passed. The audit and gate checked all 28 theorems,
with zero failing and zero native-trusted proofs; only the standard foundations
`propext`, `Classical.choice`, and `Quot.sound` occur. Import-inclusive
`leanchecker --fresh` also passed, exit 0, with receipt
`replaying TernaryDigits with --fresh`.
The original 20 theorems plus the eight new theorems form a 28-theorem source
inventory. Ordinary `decide` is used only for the original finite digit/F67
facts and a numeral inequality in the new proof; no `native_decide`, custom
research axiom or kernel bypass is used.

Both main source builds succeeded. An independent valuation probe initially
failed because of a library argument-order error, then its corrected proof
and transitive axiom audit passed. Exact probe sources and logs are retained
as receipts with `.lean.txt` suffixes so they are not additional proof targets.
The probe's checked proof is integrated into the new main module.
