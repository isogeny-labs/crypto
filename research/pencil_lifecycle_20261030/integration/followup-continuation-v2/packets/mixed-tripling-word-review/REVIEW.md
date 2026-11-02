# Independent UInt64 tripling review

Finding: validated at the stated retained no-table, degree-one prime-field value scope. Four fresh isolated checks passed: WordTriple, producer Controls, four-theorem Readback, and reviewer Controls. All60 manifest/source-binding entries matched. Producer files were unchanged.

## Literal and source correspondence

The exact Rust triple tree (projective_law_gate.rs285-345) and retained Field arithmetic (three_leads.rs n=1, tables=None) match tripleWords. Source multiplication nesting is retained through fp/fpp/k/aa/bb/cc and the six payload values. Sq is modeled by the same word multiplication operands; no pow loop occurs here. The table backend is excluded: its operand-indexing behavior would not be justified by this modular-product theorem for unreduced constants.

tripleWords retains raw5/8/10. tripleExpr reduces only those literal leaves. small_literal_mul_reduce proves the two modular products equal, using c<=10 and a<p<2^31. Independently, c*a<10*2^31<2^35<2^64; dynamic canonical products satisfy a*b<p²<2^62. The Lean proof explicitly removes UInt64 wrapping before applying modular arithmetic. Constants2/3/4 remain canonical because p>=5. Each raw5/8/10 multiplication has an already-reduced dynamic right operand, permitting the helper without assuming every input word canonical at the literal-tree stage. Full decode subsequently requires canonical input words and coefficients a1/a2/a3 and proves every expression/payload canonical.

## Guard and validity composition

repeated_word_preserves starts from original normalized point/divisor validity, four nonzero word weights, canonical coordinates, prime5<=p<2^31, outer wordK0, cancellationfalse and equal-other support. mixed_decode and canonical equality transport the cancellation inequality correctly; positive_xz_decode transports the repeated support equation. Nonzero canonical weight reflection admits the reviewed field source_repeated_preserves theorem. No output validity, derivative-nonzero or positive-ordinate assumption was inserted. The ordinate equality is derived from original validity and guards. Exact payload decode then transports nonzero output weights and normalized validity back to word output.

tripleWordDispatch is a scoped conditional: it returns Some of the exact triple payload on positiveCoincidence and equal support, and None elsewhere. This is not the complete Rust mixed dispatcher on the None paths. The original a0 word need not be canonical because this triple formula never reads curve[0]; it appears only as its residue in the supplied original curve-validity proposition. That is consistent with the source footprint.

## Evidence and boundaries

Fresh main, producer controls, readback and reviewer controls each exited0 on first invocation. The first two named lemmas use propext/Quot.sound; the decode and validity lemmas additionally use Classical.choice. No added axiom, sorry or native-decision proof escape was found. Existing pinned dependency objects were reused; no independent kernel-binary rechecker or whole-library rebuild was performed.

Producer controls include raw8/10 atp5/p7, max-envelope operand, fullp5 original-input theorem, nonunitp7 decode and rejected branch conditions. Reviewer controls independently check raw5 atp5, raw8 atp5, raw10 atp7, exact 10*2147483646 product without wrap, a noncanonical ordinate rejection, and a completep7 zero-derivative original-input branch fixture. These are fixed formal correctness checks, not scientific sampling or timing.

Reviewer did not author WordTriple but reviewed its tripling field dependency and authored imported positive-distinct field composition; dependency involvement is disclosed. Value-level source correspondence excludes compiler/counter behavior, table backend, extension fields, whole dispatcher/walker closure and divisor-class addition. Cost ratios remain null.
