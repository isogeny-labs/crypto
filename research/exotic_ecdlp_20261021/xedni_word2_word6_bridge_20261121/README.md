# Six coefficient-two bridges complete the coefficient-six word system

The complete coefficient-six relation envelope on the order-43 subgroup of
`E/F_47: y² = x³ − x − 2` has rank 14 over 21 sign-folded columns. Adding six
selected coefficient-two words raises the rank to 20, the maximum after the
one-dimensional scalar-log solution is retained. This is a minimal row witness:
the initial rank deficit is six, each added row increases rank by one, and
deleting any selected row lowers rank to 19. An independent Sage reconstruction
of the curve, word sets, ranks, and deletion checks passed.

| System | Rows | Folded columns | Rank | Nullity |
| --- | ---: | ---: | ---: | ---: |
| Complete coefficient-six words | 42 | 21 | 14 | 7 |
| Complete coefficient-two words | 42 | 21 | 18 | 3 |
| Union of complete envelopes | 84 | 21 | 20 | 1 |
| Coefficient-six words plus selected bridges | 48 | 21 | 20 | 1 |

The bridge source indices, in insertion order, are `425, 2, 85, 201, 353,
511`. The first has a measured rational-lift relation in the earlier
coefficient-two packet. The remaining five are finite-field word construction
targets; this packet does not attach measured rational lifts to them. Exact
point coordinates, insertion ranks, and support flags are in [`result.json`](result.json).
The search was frozen in [`protocol.json`](protocol.json) before running.
[`sage-audit-result.json`](sage-audit-result.json) records the independent
reconstruction; [`sage-runtime-info.json`](sage-runtime-info.json) records
the checked Sage build used for it.

## Lean result and proof map

The pinned Lean 4 and Mathlib module [`formal/Bridge.lean`](formal/Bridge.lean)
proves `complete_words_determine_nonzero_logs`: for every function
`f : ZMod 43 → ZMod 43` satisfying **both complete word systems**
`a + 2b = 0 ⇒ f(a) + 2f(b) = 0` and
`a + 6b = 0 ⇒ f(a) + 6f(b) = 0`, with `f(1)=1`, one has `f(x)=x`
for every nonzero `x`. The supporting `combined_multipliers_cover` theorem
shows that `−2` and `−6` together reach every nonzero residue. The theorem
does not assume the desired conclusion: the word equations propagate the
anchor through the multiplicative group. `#print axioms` reports only Lean's
standard `propext`, `Quot.sound`, and `Classical.choice` foundations, with no
research-specific axioms.

The rank witness and Lean theorem concern the complete finite-field word
envelopes. The next experiment is to construct and cost rational lifts for
source indices `2, 85, 201, 353, 511`, then measure their rank increment and
full target-dependent costs under the candidate measurement contract. This
is the exact bridge from the algebraic witness to an implemented relation
pipeline.

## Reproduction

From the repository root:

```sh
python3 research/exotic_ecdlp_20261021/xedni_word2_word6_bridge_20261121/run.py
SAGE_AUDIT="$(pwd)/research/exotic_ecdlp_20261021/xedni_word2_word6_bridge_20261121/sage_audit.py"
/Volumes/SSD990/cryptanalysis/sage -python "$SAGE_AUDIT"
sh research/exotic_ecdlp_20261021/xedni_word2_word6_bridge_20261121/formal/build.sh
```

The checked launcher was used for the recorded run. The formal build uses the pinned toolchain and Mathlib
manifest in `formal/`.
