# Joint C: literal batch inversion and generated inverse

Independent reviewer `/root/strict_storage_reviewer`, 2026-11-01. Producer input commit `5937d99b29e5027d99b7fda1be35569e44f01836`; prospective plan commit `6d7317d5eab6f3a24847353d1ec5b7089cdf2ea2`. Machine status **passed**, fidelity **faithful_scoped**, outcome **validated_scoped** for the three batch declarations. No sibling new report was read. I authored none of the reviewed components; inherited conversation context precludes a blind-rederivation claim.

## Three reviewed statements

`FastBatch.guarded_product` proves canonicality, nonzero product and decoded multiplication for literal `(n*s)%p`. Assumptions are UInt64 p,n,s, prime p.toNat, `2<p.toNat<2^31`, canonical n,s and nonzero n,s. Canonical word nonzeroness transfers to nonzero residues; primality makes the residue ring a field, so the decoded product cannot vanish. This discharges the generated inverse's nonzero argument.

`FastBatch.field_batch` is a separate field identity: for nonzero n,s, `(n*s)^-1*s=n^-1` and `(n*s)^-1*n=s^-1`. Its orientation is important and correct.

`FastBatch.batch_decode` applies the actual `inverseGenerator` to that product, derives a canonical inverse word, then decodes the two literal UInt64 multiplications. It exports product nonzero, canonical both/ni/si, both=(N*S1)^-1, ni=N^-1, si=S1^-1. No inverse witness or desired inverse value is assumed by the caller.

## Native order and dispatch

The extracted native file is byte-equal to the source inspected during review. In Jacobian::add, after the nonzero norm/s1 guard, native code computes both=inv(mul(norm,s1)), ni=mul(both,s1), si=mul(both,norm). These are the exact model argument order and reduction placement. With tables absent, p odd, and n=1, Field::mul dispatches to `(a*b)%p`; Field::inv dispatches to bi, whose body asserts nonzero then calls modpow(a,p-2,p). Tables and binary/extension dispatches are outside this correspondence. The theorem assumes canonical norm/s1 words; it does not derive their coefficient prelude.

UInt64 multiplication has a proved no-overflow bound: canonical a,b below p<2^31 imply a*b<2^64. Thus the word product agrees with natural multiplication, and the claim does not depend on wrapping behavior or a Rust debug/release overflow choice in its stated domain. The 2^31 threshold is a sufficient bound, not asserted optimal.

`inverseGenerator p a = modpow p a (p-2)`. The helper proves UInt64 subtraction has natural exponent p-2. The modeled wide multiplication casts to 128 bits before multiplication/modulus, then back to UInt64, matching native modpow's u128 arithmetic. Two UInt64 operands have product strictly below 2^128, so the widened product is exact; the remainder fits UInt64. The loop uses the original exponent's low bit for r, shifts e by one, and squares a only when the shifted exponent is nonzero, matching native order. The model has fuel 64; every UInt64 exponent is below 2^64, and loop_decode establishes that fuel suffices. Fermat in the prime residue field derives the inverse product, then generator_decode derives the inverse equality. I freshly rebuilt Wide and Inversion as well as the batch modules.

## Checked boundaries

Seven pinned producer controls cover literal p17 outputs (product6,both3,ni6,si9), a theorem-instantiated si decoding, n=s=-1, noncanonical n=17 at p17, composite p15 with n3,s5, an overflow discrepancy and the zero guard. These expose the exact domain dependencies rather than claiming raw nonzero guards alone suffice.

Three reviewer controls were newly compiled and axiom-audited:

- p3,n=s=2 gives product1,both1,ni2,si2, checking the smallest permitted prime path.
- p=2147483647,n=s=2147483646 gives product1 and an explicit natural product below 2^64.
- Canonical n=s=4294967296 under p=4294967297 yields wrapped word product0 while natural modular product is1. This last modulus is not asserted prime; the control isolates loss of the no-overflow premise. It describes the total Lean word model, not a successful native Rust debug execution on overflowing inputs.

The known-false theorem exits1. Producer audit rejects forbidden axioms, empty output and wrong declaration names. All 17 audited readbacks contain only subsets of propext, Classical.choice and Quot.sound, or no axioms. The assumptions are satisfiable, as the p17 producer theorem instantiation demonstrates.

## Machine custody

Pinned target sources were extracted via git show into private input/. Five modules were rebuilt: Wide, Inversion, FastBatchInverse, FastBatchControls and FastBatchReadback. The latter three are the producer modules; the first two are exact copies of their pinned helper sources. There are ten producer readbacks (three main and seven controls), three reviewer controls and four additional freshly rebuilt bridge readbacks. Each invocation's stdout, stderr and exit is retained. The false fixture is additional and is expected to fail.

Non-union helpers were privately copied. Existing deeper helper and library binary caches remain trusted; dependency-union trees are reused and recursively hashed before/after. There is no complete dependency rebuild, independent kernel rechecker or compiled Rust-to-Lean verification. Hashes bind observed bytes; they do not exclude hypothetical transient edits to shared caches. None were observed. The Lean executable is hash-bound; the complete installed toolchain is not rebuilt.

175/176 copied helper source files match archived Git blobs at the producer commit. The unmatched `deps/43/Readback.lean` (Git blob `0052ce362db2bbd98e372d5358228148f637f4cd`) is retained and hash-bound but outside the actual local import closure; it is an unrelated SameSupportClassifier readback. Imported source closure and unresolved external source roots are explicit in local-import-closure.json. No missing source is silently described as freshly rebuilt.

One orchestration failure is retained in checker-premature-invocation.json: the checker was invoked while its rustc session was still completing, yielding shell exit127. Once the same compiler session completed, the checker passed. This did not alter theorem sources or cause a Lean proof failure. No unexpected Lean failure occurred.

Current main was separately inspected through read-only GitHub APIs at `0a82811c9aa816d3287b255a1e20d2c6657f8d7e`; there is no root ledger directory. No producer branch mutation or official ledger transition occurred.

## Missing bridges and next obligations

Canonicality and decoding of norm/s1 from actual input coefficients are assumed at this packet's boundary. S0 and slope assembly, inverse_lead scaling and the quotient/remainder endpoint belong to further components. This review does not substitute their equations as inputs. Native compiler equivalence, counters and timing are not established. No group-law, Kummer, collector or campaign-cost conclusion follows from this packet alone.

## Attestation

Owned joint: C. Verdict: holds at the scoped source path. Semantic sources read: the published plan; previously read review-evidence skill and Lean reference; FastBatchInverse, FastBatchControls, FastBatchReadback and ExpectedFailure; producer REPORT; Wide, Inversion, UInt64Residue; native bm/bi/mul/inv/modpow and batch branch source. Mechanical source binding/import traversal covers the additional files listed in helper-source-bindings.json and local-import-closure.json; it is not a claim that every helper received a new line-by-line semantic review. No new sibling output or review report was accessed. No scientific workload, timing experiment, cloud operation or authorization change occurred. Prior Joint A artifacts remain unchanged. Parent coordinator owns archival publication and PDF/SVG rendering.
