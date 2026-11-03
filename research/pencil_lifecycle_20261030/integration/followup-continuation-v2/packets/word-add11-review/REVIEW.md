# Independent word Add11 correspondence review

No blocking discrepancy was found for the stated literal-word-model correspondence. The sealed producer manifest and all source pins matched. Fresh private WordAdd11, Controls and separate Readback compilations returned exit0 against retained dependencies. Readback lists only propext, Classical.choice and Quot.sound for the five headline declarations. Dependencies were reused, not rebuilt; failures from producer attempts remain retained.

## Pow and primitive source fidelity

Actual Field::pow starts r=1, tests the low exponent bit, shifts exponent, then squares only if the shifted exponent remains positive. For exponent5 the path is5,2,1,0: multiply1*a, square a to a², square a² to a⁴, multiply accumulator by a⁴. There is no final unused square. wordPowLoop matches this ordering and pow5_unroll is reflexive. This clarifies the prospective plan's check for an unused square: the source excludes it.

With Field.n=1,tables=None and odd prime path, bm/bs return modular machine multiplication/square; add returns (a+b)%p; neg's one iteration returns(p-a%p)%p; sub calls add(a,neg(b)). The Lean primitives match these value expressions. The bound p<2^31 and canonical operands prevent the modeled product/addition intermediates from overflowing. Counter updates and residual loop bookkeeping are outside the value model and require the stated normal-execution/counter-headroom condition.

## Payload and dispatch

payloadExpr and add11WordPayload preserve the six payload coordinate associations in gate add11. Structural induction proves every evaluated subexpression canonical and decodes it to field operations; payload_decode establishes canonicality of all six output words and equality to the reviewed WeightedAdd11.output. pow5Expr repeats the square subtree as a value expression: this is not a sharing or operation-count proof.

The dispatch model includes inverse first, doubling second, then the source assertion as an explicit assertionFailure outcome. Nonzero decoded z and normalized x inequality yield unequal field cross coordinates. Equal machine cross words would decode equally, contradicting that fact, so the modeled source reaches distinct. This argument does not require proving all possible y-word branch correspondences because both preceding guards require the already-excluded cross equality.

The payload theorem describes the body expression even on inputs that would not reach it. Combining that expression theorem with distinct_word_guard justifies its source branch scope under the stated additional hypotheses; it is not an executable Rust compiler theorem. The packet correctly leaves composition with normalized word Mumford validity and geometric group law separate.

## Controls and scope

Controls compile for pow5 residues0,3,6 modulo7; nonunit payload/dispatch; zero-ordinate inverse precedence; doubling; same-cross invalid assertion; equal normalized x with distinct raw x; a noncanonical input; and UInt64 overflow outside the envelope. These are kernel-evaluated fixed controls, not native benchmark measurements. They complement but do not replace the universal conditional Lean propositions.

The source model has an explicit prime fact,5<=p<2^31,eight canonical input words, and distinct-branch assumptions where needed. It does not prove actual Field object validation, count headroom, compiler behavior, inverse/doubling payload correctness, class-sum or whole-walk invariance. No scientific execution or cost ratio was produced. The reviewed field formula and producer packet remain unchanged.
