# Mixed degree shared-support cancellation: field and word proofs

The source-derived field and UInt64 theorems now exclude the mixed-degree coincident-y mismatch from valid inputs and preserve canonical normalized validity on the cancellation return. Exact prelude/p4 expressions are modeled; no output identities or validity conclusions are premises.

At field scope, Kraw=Z⁴ Zb⁴ W*u(x). Nonzero weights and Kraw=0 give u(x)=0. The proof factors u=(U-x)(U-(-q-x)), derives validity for both extracted supports using polynomial divisibility, then derives y²=v(x)² from the original degree-one validity. A constant divisible by a degree-one polynomial must vanish; this supplies the ordinate dichotomy without assuming it. Exact scaled comparisons are L=D*y and R=D*v(x), D=Z⁵ Zb⁵ W Wb. Therefore cancellation is tested first and the later mismatch is excluded.

The literal p4 normalizes to x4=-q-x,y4=v(x4), with weightsZ*Zb,Wb nonzero. Factorization and divisibility also cover repeated support and zero ordinate; no x-x4 or y denominator is introduced. The field theorem is polynomial algebra over an arbitrary field, with nonzero representation weights.

WordMixed models the literal source multiplication/square/add/subtract/negation tree, including parentheses. FieldExpr composition proves canonical prelude/p4 words and field decoding under prime5<=p<2³¹ and10canonical input coordinates. Canonical equality/negation reflection proves exact guard correspondence. mixed_word_branch_invariant then starts with the original two normalized validity predicates and four nonzero word weights; it excludes mismatch and, on cancellation, proves canonical degree-one output, nonzero output words and normalized validity.

Controls: a full nonunit rational cancellation uses a=(9,-486,3,2), b=(-4,0,24,0,2,3), f=X², normalizing to point(1,-1), supportX(X-1), ordinateX. Input divisibilities are constructed and the full field theorem applied. The full word theorem is instantiated atF7 with a=(0,0,3,2),b=(0,0,0,0,2,3),f=0, checking repeated-support zero-ordinate cancellation with nonunit weights. Additional controls cover invalid ordinate mismatch, p5 cancellation, zero-weight output failure and a nonzero-input literal payload. These are algebraic fixtures, not smooth genus-two claims.

Final field module, word module, controls and separate axiom readback exit0; only propext, Classical.choice and Quot.sound occur. Initial field attempts failed for missing noncomputable annotations/unavailable linear_combination, overbroad polynomial simplification and a simp-list syntax error; rewritten elementary identities succeeded. Initial controls required norm_num and qualified support name. Every failed source/receipt remains. Each compile was bounded120seconds with pinned existing dependencies.

The pure mixedWords bundle computes p4 values even when common dispatch would return early; only cancellation uses them as returned output. It models values, not operation counts or side effects.

Scope: source n1/tablesNone and ordinary execution/counter headroom; no compiler or counter theorem. common and positiveCoincidence are prelude-tail markers only: generic mixed, tripling and secondary positive-coincidence outputs are not asserted. This completes the requested cancellation slice, not the whole mixed dispatcher or a divisor-class theorem. Independent source review is pending. No scientific trials or cost measurements.

Literature/source references and exact hashes: PLAN.md,sources.json. Original plan and prior proof packets remain unchanged.
