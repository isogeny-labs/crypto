# Independent review: exact fast polynomial calls

Input: calls/ at aa3e94c0e6834aa1b5de2156c7c5128cabd64e23. Reviewer /root/quintic_fidelity_review authored none of these modules. Read prospective review-plan.md before sources. No sibling new reports read. Historical context inherited; not mutually blind derivation. Prior Joint D unchanged.

## Result

Machine status **passed**, source fidelity **faithful_scoped**, outcome **validated_scoped** for five named source-specific list-model declarations. The returned modeled quotient and remainder are connected by explicit successful call equalities, and the returned storage preconditions follow. This result does not assert full Rust compiled equivalence, actual array copies, full Jacobian dispatch or divisor-class interpretation.

## Source-linked findings

1. FastCallInputs.lean inputs_decode: ua=[ra,qa,1] and ub=[rb,qb,1] match monic degree-two native up() outputs. va=trimFuel 2 [ta,sa] matches vp()'s two-coordinate trim. The slope list [slope0,slope1] matches native constant-then-linear coefficient order; the native code does not trim that list at construction, nor does the model. Canonicality and decoding are derived from original canonical inputs and actual word guards, rather than supplied as new correspondence assumptions.

2. prepared_decode: product(ua,ub) and padd(va,product(sl,ua)) match the native expressions U and V. U normality comes from the product model. The theorem only needs canonical V for subsequent residual calculation, so omission of Normal V from its outward statement is harmless. This is a pure value/call composition: native computes V before U whereas proof construction first names U evidence; no global execution scheduling or counter trace is modeled. Interpret report wording 'native order' as operand/nesting order, not temporal statement order.

3. FastCallQuotient.lean scaled_exists: the source payload supplies an algebraic Q and residual divisibility. The actual modeled exactDiv call returns a canonical normalized list q. Its decoding equals Q by right cancellation against decoded U. That cancelled polynomial is nonzero because it is a product of two monic degree-two supports over a field; it is not assumed without proof. The factor orientation is reconciled by commutativity explicitly. Thus the theorem does not alias an arbitrary existential field quotient with a list by assumption.

4. scaled: PegcdWord.scale uses actual decoded inverseLead word on the returned q and trims, matching native pscale's value semantics. The theorem derives canonicality, normality, monicity and degree two. The scaling scalar is nonzero from both guards; degree preservation and the inverse-scalar residual-divisibility witness use that fact. No zero-scale case is silently canceled.

5. FastCallFinish.lean monic_stored_shape: canonicality makes the last word's ZMod decoding injective; Normal and nonzero monic polynomial establish length=degree+1. Degree two therefore gives length3 and last word1, discharging the native explicit monicity assertion. Trailing zero storage would invalidate this argument and is excluded explicitly.

6. fast_output_exists computes remainder(pneg(v),scaled(q)) with the negative sign exactly as native. The congruence argument alone would not distinguish +v from -v because both square equally, but the successful call equality fixes the intended sign. remainder_total supplies canonical normalized storage and strict length. The endpoint exports scaled length3 and r.length<=2; these together imply r.length<scaled.length and nonempty scaled list, so all Divisor::from length assertions follow directly. This endpoint avoids the predecessor issue where only independent upper bounds were exposed.

## Boundary attacks and fixture scope

Zero norm or slope are excluded by explicit word premises. Missing either original residual congruence prevents invoking the source quotient construction; no unconditional exact division is claimed. Malformed trailing-zero [1,0] fails Normal, as freshly checked. Original input supports are constructed with last coefficient1, so malformed external support arrays are not admitted by reinterpretation; binding the named inputs to actual native Divisor objects remains a distinct obligation.

The fixture fwords=[1,0,0,16,16,1] decodes to X5-X4-X3+1 over F17. Inputs are Ua=X², Ub=X²+1, Va=1,Vb=X+1; source lists and negative slope [0,16] are fixed controls. complete_calls supplies actual original residual witnesses X³-X²-X and X³-X²-2X. It then instantiates the full theorem to prove existence of lists satisfying exactDiv and remainder call equalities, canonicality/normality and storage validity. This is a kernel-checked existential modeled output, not a separately executed Rust trial or printed numeric quotient/remainder. No curve smoothness inference is made.

The statement's prime2<p<2^31, canonical coefficient/f words, f degree5 and both guards remain unchanged. Native source interpretation is n=1 with diagnostic tables absent. Counter headroom, allocations and array-copy semantics are outside the source-value model. The report names these boundaries correctly; the operation-order phrasing can be made more precise as noted above.

## Fresh machine evidence

Five modules compiled freshly: FastCallInputs, FastCallQuotient, FastCallFinish, FastCallControls, FastCallReadback. Ten named readbacks cover five main declarations and five fixed controls. All readbacks have only accepted standard axioms. ExpectedFailure was rejected with exit1. A native Rust fail-closed checker verifies exact ordered names, nonempty/complete readback, exact accepted line syntax, module exits and allowed axioms. It rejects an empty audit, forbidden sorryAx substitution and wrong theorem name. acceptance.json records these checks. No unexpected compilation failures occurred in this review.

Sources were extracted by git show from the immutable publication commit into a private first search root. Reviewed module caches were freshly built there; original producer directories were not modified. Imported helper sources/caches and recursive dependency-union contents were hashed before/after and remained unchanged, as did compiler and review input sources. Dependencies were reused, not independently rebuilt or binary-rechecked; their source-to-cache provenance remains trusted. Logs, exits, source hashes, check results, replay script and checker source are retained.

## Current main, sources and next obligations

Fetched main separately at27b3b8403914c2483bfbc0605300c5e68785e91b. No ledger directory is present; its dated session journal was inspected and retained. Producer branch unchanged. No scientific authorization is inferred from this source work.

Sources read: immutable review plan; calls REPORT/status; all FastCallInputs/Quotient/Finish/Controls/Readback source; exactDiv/exact_total_normal and remainder/remainder_total helper source; check.sh and dependency list; native three_leads.rs pinned in metadata, including optimized branch, up/vp/Divisor::from; own fresh logs/checker; current-main tree and dated journal. Existing AGENTS/review-evidence Lean-route requirements retained from this session. No sibling new reports consulted.

Next obligations: connect actual stored input objects and full add dispatch, prove final struct-copy semantics, compare fast and fallback outputs under shared assumptions, and establish divisor-class meaning. No official hypothesis promotion or performance inference. No repository edits, scientific/cloud/timing execution or changes to budgets occurred.
