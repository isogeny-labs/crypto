# Independent repeated-support tripling field review

The field-level construction matches the retained Rust tripling branch at the reviewed scope. Exact original valid point/divisor premises, four nonzero weights, outer K=0, cancellation false, and repeated cross-scaled support yield valid normalized degree-two output with nonzero output weights. No output-validity or nonzero-derivative premise was inserted.

## Source fidelity

Reviewed source projective_law_gate.rs285-345 and mixed dispatcher828-878, SHA5976e7f30b3e9c5c49a5fc0f4095404b7322832e4b02f467bc3cb7e0d9dcc8c0. The literal triple has no internal conditional. The review plan's possible exceptional-output question is resolved by that source inspection. Cancellation is checked before the ordinate mismatch and repeated-support branch. Validity supplies the positive ordinate alternative when cancellation fails; positive_nonzero derives 2!=0 and original y!=0. Hence literal k=8*y^3 and z_out=k*z are nonzero. The repeated cross-scaled support equality yields normalized q=-2x,r=x².

rawOutput reproduces the source x/z powers, fp/fpp, k, aa, bb, cc and all six returned fields [q3,r3,s3,t3,z3,w]. Field-expression comparison includes multiplication nesting but does not infer machine-word overflow or counter semantics. raw_normalizes proves four normalized coefficients plus two nonzero weights. The source branch theorem uses original normalized input divisibility, not a proposition about the desired output as an assumption.

## Independent algebra check

Write m=f'(x)/(2y), A=(10x³+3a3*x+a2-m²)/(2y), h=A(T-x)²+m(T-x)+y. Expanding f-h² about x gives zero constant, linear and quadratic terms from the point equation and definitions. The remaining monic quotient by (T-x)³ is u=T²+qT+r with q=3x-A² and r=a3-2A(m-2Ax)+3x(q-x). Reducing -h modulo u gives v=(Aq-m+2Ax)T+Ar-y+mx-Ax². Thus h+v=A*u and u divides f-v². This uses neither derivative nonvanishing nor nonsingularity. The exact Lean identity includes the three residual terms before their elimination.

## Fresh verification and controls

All63 manifest/source binding entries matched. Fresh isolated checks of Tripling, producer Controls and six-theorem Readback passed. Named axioms are propext, Classical.choice, Quot.sound only. Existing pinned Lean and dependency objects were reused; there was no independent binary/kernel rechecker or whole imported-library rebuild. Source and value fidelity is established for the retained lineage, not current Cargo linkage.

Producer controls include full original-input nonunit rational fixtures for T5-T+1 and T5+1, zero-y cancellation and an invalid original point. Reviewer fixed controls add characteristic3 full repeated-branch theorem, characteristic5 zero-derivative weighted validity, and characteristic2 cancellation precedence. The characteristic5 curve may be singular; this deliberately checks the polynomial-invariant theorem's stated scope, not smooth Jacobian arithmetic.

Reviewer preparation initially misread the nested manifest shape twice; no Lean invocation occurred then. Reviewer control attempts1-3 exposed missing ZMod and field-instance imports, attempt4 left concrete finite-field inequalities unsolved by norm_num. These receipts and exact source versions are retained. They are reviewer fixture preparation failures, not failures of the producer theorem.

## Independence and scope

Reviewer authored the imported positive-distinct construction and previously reviewed mixed cancellation/generic dependencies, but did not author this tripling candidate. That dependency role is disclosed; this is an independent tripling source/proposition review, not fully independent derivation of the whole library. Producer files remain unchanged. No scientific or native cryptographic run occurred.

The accepted component is weighted field-value Mumford divisibility and nonzero output weights. The source classifier result is positiveCoincidence, combined with the repeated-support premise and source-order inspection to identify the tripling call; it is not a whole dispatcher/compiler theorem. UInt64 literal8/10 handling, operation counters and divisor-class addition remain separate obligations. Whole-cost ratios are null.
