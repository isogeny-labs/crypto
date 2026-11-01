# Joint D: independent coefficient-prelude and payload review

Input revision 6d7317d5eab6f3a24847353d1ec5b7089cdf2ea2. Reviewer /root/quintic_fidelity_review. Prospective review plan read first. Joint D only; batch inversion is imported and its independent Joint C review is not presumed. No sibling new reports read. This reviewer authored neither component. Historical context inherited; no mutually blind derivation claim.

## Scoped result

Machine status **passed**; source fidelity **faithful_scoped**; technical outcome **validated_scoped** for nine coefficient-prelude and five payload declarations under their stated arithmetic hypotheses and helper-cache trust. The final payload endpoint constructs a field-polynomial quotient and monicity after decoded scalar scaling. It does not claim the native polynomial routines or returned divisor have yet been proved equivalent.

## Source correspondence

Input mapping is qa=a.u[1], ra=a.u[0], sa=a.v[1], ta=a.v[0], qb=b.u[1], rb=b.u[0], sb=b.v[1], tb=b.v[0]. Native optimized dispatch has degree-two input divisors, so these indices exist. The model Inputs structure supplies these words; the act of extracting them from stored Divisor arrays is a remaining bridge.

The expression tree preserves the arithmetic nesting:
- aa=qa-qb and bb=ra-rb; cc=sb-sa and dd=tb-ta, retaining the native reversed ordinate orientation.
- N=(bb*bb - (aa*bb)*qb) + (aa*aa)*rb, with each multiply and add/sub reduced at the same boundary.
- S1=cc*bb-aa*dd.
- S0=dd*(bb-aa*qb)+(aa*cc)*rb.

literal_norm/literal_s1/literal_s0 are definitional equalities between explicit word definitions and trees. Expr.composition inductively proves canonicality and ZMod decoding rather than assuming intermediate results. The square operation is modeled by multiplying identical operands; this agrees with prime-field arithmetic values, not its square-versus-multiply instrumentation counts. Repeated expression evaluation is a pure value model and does not claim a native allocation or event trace.

Native code computes N and S1, tests both for nonzero, then generates the batch inverse before computing S0. The payload/prelude endpoints explicitly require those actual word guards. The standalone total s0Word definition is a value formula for the guarded body, not a claim that native code evaluates S0 on bypassed branches. The report says this explicitly. No invalid branch execution or cost claim is inferred.

Slope words are S0*ni and S1*ni. The imported guarded batch endpoint supplies canonical ni and si and derives their inverse decoding from original canonical inputs. inverseLead applies wordNeg only after the norm*si product and its square; the minus sign is not moved or dropped. slope_polynomial_decode proves the decoded slope polynomial equals the field CRT slope. source_payload_quotient invokes the previously proved source_quotient_exists using derived nonzero field guards and the two input residual hypotheses, then rewrites decodedV via that proved slope correspondence. It assumes neither a quotient nor the desired slope equality.

## Actual fixed fixtures

The sample arithmetic fixture is distinct from the full residual fixture. Do not combine their numerical values into one curve instance.

For sample=(1,2,1,2,0,1,4,6), p=17, native differences are (aa,bb,cc,dd)=(1,1,3,4). Thus N=2, S1=16 and S0=7. Product N*S1 is 15; its inverse is 8; ni=9 and si=16. The slope words are (12,8), leadRatio=15, its square=4 and inverseLead=13. Producer controls check N/S1/S0 and payload; new reviewer controls independently evaluate ni,si,leadRatio from the source definitions. Flipping ordinate differences changes S1 to1, as the replayed orientation control states.

For small=(0,0,0,1,0,1,1,1), p=17, N=1,S1=16,S0=0; slope=(0,16), inverseLead=16. Ua=X², Ub=X²+1, Va=1,Vb=X+1, decoded V=1-X³. On f=X⁵-X⁴-X³+1, the first residual witness is X³-X²-X and the second is X³-X²-2X, exactly as in complete_word_to_monic. The complete residual is
f-(1-X³)²=X²(X²+1)(-X²+X).
The reviewer freshly proved this explicit identity, in addition to replaying the producer's existential quotient theorem. Its quotient has degree2 and leading coefficient16; scaling by16 gives X²-X, monic. No smoothness claim follows from this fixture.

At p=3 with the same small inputs, N=1,S1=2,S0=0, slope=(0,2), inverseLead=2. The theorem hypotheses permit this smallest odd prime. A concrete scope detail: native Field.validate rejects odd p<5 in study admission. Therefore p3 proves the word/field arithmetic boundary, not admission of p3 as a validated benchmark instance. This does not contradict the stated mathematical domain or require weakening it to p>=5.

## Attacks and exclusions

Same support gives N=0, excluding the batch branch. A nonzero norm with zero slope is explicitly checked and rejected by the source-guard premise. Noncanonical qa=17 at p17 violates Inputs.canonical. Omission of wordNeg changes inverseLead13 to4 for sample, verified by the missing-negation control. All endpoint inversions require prime modulus and 2<p<2^31; expression-only decoding needs canonical words and 0<p<2^31, not primality. The bound supplies multiplication headroom; native word/runtime equivalence outside that bound is not claimed.

## Fresh machine evidence

Eight immutable producer modules compiled in this reviewer session: FastPreludeExpr, FastPreludeDecode, FastPreludeControls, FastPreludeReadback, FastPayloadWords, FastPayloadPolynomial, FastPayloadControls, FastPayloadReadback. All 27 named readbacks passed exact ordered fail-closed checking: 14 main declarations and13 fixed controls. ExpectedFailure returned1. Six malformed-output controls rejected empty output, wrong declaration name and forbidden sorryAx for the two main audit sets.

A separate ReviewerFixture module adds three checks described above, with three named axiom readbacks. Combined acceptance-with-fixture.json records nine modules,30 names and9 rejection controls. Its first launch failed before elaboration because the shell cwd caused Lean's module root check to reject the absolute input path; logs/ReviewerFixture-attempt1 preserves exit1/stderr. Relaunch in the private input directory passed. No mathematical source change was required; this infrastructure error is not a failed theorem.

All immutable source, helper/cache, recursive union-cache and compiler hashes passed final checks, including the separately hashed reviewer fixture. The first search root contains fresh reviewed module outputs; original producer directories were not changed. Dependencies, including batch inversion and earlier polynomial lemmas, retain their source/cache provenance trust boundary. No independent fresh dependency build or binary rechecker was run. The readback checker is native Rust, checks exact syntax and allowed axioms, rejects missing/extra/misordered names, verifies exit codes and exercises deliberate corrupted evidence. No native_decide proof was substituted for kernel checking.

## Current-main and remaining obligations

Fetched current origin/main separately, still27b3b8403914c2483bfbc0605300c5e68785e91b. Its root tree has no ledger directory; its dated session journal was inspected and retained. It preserves expired scientific limits and pending continuation. Producer branch unchanged; no official scientific status update.

Next bridges: extract Inputs from actual stored divisors; connect native polynomial multiplication/addition/exact division/scaling/remainder outputs to decoded values; derive actual final storage; establish class interpretation separately. Independent Joint C evidence must remain separately attributed. This review does not certify its sibling work by inference.

## Sources read / attestation

Read: immutable prospective plan; packet REPORT.md; prelude Expr/Decode/Controls/Readback; payload Words/Polynomial/Controls/Readback; producer check.sh/dependencies; native three_leads.rs at the input commit, optimized branch and Field.validate; exact imported helper binding inventory; own fresh raw logs, checker and fixture source; current-main root/session journal. Prior relevant AGENTS and review-evidence Lean instructions were retained from this review session. No sibling new report was read. Prior Joint B artifacts remain unchanged. No repository edits, scientific or timing runs, cloud work or budget changes occurred.
