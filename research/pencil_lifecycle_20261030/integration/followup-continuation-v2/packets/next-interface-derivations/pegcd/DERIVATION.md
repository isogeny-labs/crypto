# Next source composition: extended GCD into exceptional Cantor addition

The next smallest substantive construction is a six-register word-level pegcd loop with a normalized Bezout endpoint, followed immediately by a call-site lemma making both exact divisions in the Cantor fallback succeed. The nonmonic division certificate supplies each Euclidean step; it does not yet track the four Bezout registers or connect either native pegcd call to the two exact-division assertions.

This is a read-only source derivation and implementation plan. Proposed theorem statements and controls below are not newly checked theorems or executed tests. Native source is snapshotted and hashed. The nonmonic report is read from published commit5cd0a45205e0896a5987364ccde900ec0813f477, not inferred from its title.

## Exact source paths

All native pins refer to /home/ec2-user/crypto-pencil-signed-graph-20261029/src/cryptanalysis/three_leads.rs: Field::new71–81 sets tables=None; base arithmetic195–209, multiplication dispatch210–226, inverse307–330; polynomial helpers796–848; pdiv849–880; pexact885–889; pmonic891–899; pegcd900–928; Divisor::from948–956; reduce990–1002; fast-branch guard1024 and fallback1045–1060. The prime-field Jacobian is constructed at1144–1161 with Field::new(p,1,0); the separate j2 uses the extension field at1163. Keep the reviewed odd-prime n1/no-table,p<2^31 boundary for this work.

The native generic fallback is reached when either divisor degree is not2, or the degree2 pair has norm=0 or s1=0. The new proof must admit shared support and inverse pairs; assuming gcd(ua,ub)=1 would remove precisely the exceptional cases that need it. Degree0 input short-circuits at1006–1011 and requires a separate trivial returned-input-valid lemma.

## Six-register invariant and literal transition

Use original decoded inputs A,B and state (r0,r1,s0,s1,t0,t1), each a canonical normalized word list. Initialize (a,b,[1],[],[],[1]) exactly. Require Normal a and Normal b and not(a=[] and b=[]); native pegcd copies without initial trim. Maintain:

1. decode(r0)=decode(s0)A+decode(t0)B and decode(r1)=decode(s1)A+decode(t1)B.
2. For every H, (H divides decode(r0) and H divides decode(r1)) iff (H divides A and H divides B).
3. Both remainder lists are normal/canonical; r1!=[] implies its actual last word is canonical and nonzero. Both remainders are not simultaneously empty.

For an active iteration use the actual last word of r1 in the published NonmonicStorage.wordDiv. Instantiate sufficient inner fuel with r0.length. Its result Q,R obeys decode(r0)=Q decode(r1)+R and R=[] or R.length<r1.length. Prove the next state equals:

(r1,R,s1,psub(s0,pmul(Q,s1)),t1,psub(t0,pmul(Q,t1))).

This ordering matters: native lines911–912 overwrite remainder registers before lines913–918 update Bezout registers, but s0/s1 and t0/t1 still denote their old values at their respective subtraction. A model that reuses overwritten s0 in s1 would be wrong. Expand (s0-Qs1)A+(t0-Qt1)B=r0-Qr1=R. The common-divisor equivalence follows both ways from the same division equality; no coprimality assumption is needed.

Outer fuel b.length suffices: in every active iteration the next r1 length strictly decreases, including an initially shorter r0 or empty r0. This measure differs from inner division fuel. Native has neither fuel counter, so prove existence of a model trace, not a claim about native timing. After at least one iteration final r0 is the preceding nonempty r1; if no iteration, the non-both-empty premise supplies initial a!=[]. Thus final normalization never unwraps an empty r0 or inverts zero under admitted inputs.

## Normalized endpoint

Native line920 computes inverse(last(r0)), even when the leading word is1, then scales all three outputs with exactly that scalar. Prove a general-length pscale map/trim lemma: canonical input and scalar yield canonical normal output and decode(pscale(xs,c))=C(c)*decode(xs). Do not reuse an affine-transform theorem limited to capacity6 as a general EEA certificate; Bezout-register lengths need their own bounds.

The returned (D,S,T) must satisfy D monic, D!=0, D=SA+TB, D divides A and B, and every common divisor of A,B divides D. Scaling by a nonzero unit preserves all divisor statements and the Bezout equality. Avoid assuming final D is already the gcd. Existing ProductPolynomial.product_correct and ResidualWord.add_correct and ResidualWord.sub_correct decode helpers cover register arithmetic, subject to their exact source/length contracts; a pscale map bridge and nested-loop state theorem remain new work.

## Direct payoff at the two fallback pegcd calls

At1045 obtain D1=E1 Ua+E2 Ub with D1 dividing Ua,Ub and D1 monic nonzero. Ua,Ub come from nonzero monic Mumford u polynomials, so the first call is admitted. At1046 set W=Va+Vb, which can be zero; the second call is admitted because D1!=0 even when W=[] (inverse-pair case). Obtain D=C1 D1+C2 W, with D dividing D1,W and D monic nonzero. Then D divides Ua,Ub,W.

Define S1=C1E1, S2=C1E2 exactly as1047–1048. The two native pexact obligations now follow without assuming their desired quotients:

- D^2 divides Ua*Ub, because D divides each factor. Nonzero Ua,Ub,D imply the quotient U is nonzero. This discharges the denominator and empty-remainder assertion at1049.
- Let N=S1 Ua Vb+S2 Ub Va+C2(Va Vb+F), exactly the nested padd/pmul expression1050–1058. The first two terms are divisible by D. From input validity Ua divides F-Va^2 and D divides Ua, obtain D divides F-Va^2. Since D divides Va+Vb,
  F+Va Vb=(F-Va^2)+Va(Va+Vb)
  is also divisible by D. Hence D divides N. This discharges pexact(N,D) at1059. The argument tolerates shared support and W=0.

Use NonmonicStorage.word_exact to turn both derived divisibilities into successful word-level exact-division traces. A divisibility witness can be chosen as wanted; do not postulate a specific quotient without a separately derived identity. Then prem modulo nonzero U produces the v degree bound. This is the first useful collector-facing milestone: admission and exact divisions for the native complete fallback, not another isolated Bezout identity.

## Next bounded reduction after that milestone

For input divisors of degree<=2, deg U<=4. After reducing v modulo U, deg v<deg U. If deg U is3 or4 and F is monic degree5, deg(F-v^2)<=max(5,2deg v), so the quotient by U has degree<=2. Also F-v^2!=0 because an odd-degree polynomial cannot equal a square. Therefore reduce990–1002 needs at most one iteration on this admitted fallback shape, provided the composition lemma first establishes U divides F-v^2. The latter Mumford-validity composition identity is still an explicit new obligation; the two exact-division lemmas above alone do not establish it. Once proved, reduction preserves divisibility because new v is congruent to -old v modulo new u, and final Divisor::from bounds follow. A divisor-class/group-law interpretation is a separate geometric interface.

## Malformed-input behavior to model explicitly

- pdiv([], divisor[]) asserts at851 before checking the numerator; zero-leading nonempty divisor invokes inv0 even for empty numerator. Current raw word model may return on empty input; extend a guarded Result type if proving whole-entry behavior.
- pexact propagates division failure and additionally asserts on nonempty remainder at888.
- pmonic([]) returns[]; pmonic([0]) invokes inv0, since it does not trim input first.
- pegcd([],[]) skips its loop then unwraps empty r0 at920. pegcd([0],[]) reaches inv0. pegcd(a,[0]) enters pdiv and invokes inv0. A normalized nonzero-input contract excludes these; do not label them native rejections by a checked model that lacks these guards.
- reduce([],v) first pmonic returns[], then prem uses empty divisor and asserts. Divisor::from also requires nonempty u,length<=3 and v.length<u.length.
- Noncanonical words, composite modulus, tables and extension arithmetic are not covered by the reviewed native correspondence. Existing proof hypotheses are not runtime validation tests.

## Falsifiable controls proposed, not executed

Over F17: EEA(X^2+1,X+1) should return D=1,S=9,T=8X+9 (check 9A+(8X+9)B=1). This catches wrong update sign and final-only-D scaling. EEA(2X+2,4X+4) should return D=X+1,S=0,T=13; EEA(2X+2,0) returns D=X+1,S=9,T=0; EEA(0,2X+2) returns D=X+1,S=0,T=9. EEA(1,X^2+1) exercises an initially shorter first remainder. At each call compare decoded Bezout identity and actual word outputs, not just gcd.

For F=X^5+1 over F17 and point x=0,y=1: add (u=X,v=1) to its inverse (X,-1) must take the degree1 fallback, feed an empty Va+Vb into the second EEA, obtain D=X, exact U=1 and final identity representation. Doubling (X,1) exercises D=1 and expected algebraic output (X^2,1). These are prospective fixture expectations derived here, not executed results or a geometric proof.

Negative fixtures: both-empty EEA, zero-padded last words, wrong supplied leading coefficient, insufficient inner and outer fuel separately, altered s-update using new s0, omitted final S/T scaling, Vsum sign switched, or numerator term F replaced by -F. The last mutations must fail the appropriate invariant/divisibility control instead of being accepted by assumed quotient witnesses.

## Implementation sequence and exact interfaces

1. NonmonicStoredResult: combine NonmonicDivision.divide_total with NonmonicStorage.word_projection/word_good and QuotientStorage.finish_normal into a single existential word-result theorem containing equality, canonical/normal outputs and strict remainder shortness. The published packet has these components, but no single generic stored total-result interface; word_exact alone cannot service arbitrary EEA remainders.
2. PolynomialScaleWord: general-length native pscale map/trim and decoded unit scaling. Reuse source-level scalar mul and trim, not a capacity6 transform theorem.
3. PegcdState: actual six-word-list transition, two Bezout invariants, common-divisor equivalence, normalization/nonzero remainder invariant. Obtain word q/r from step1 without postulating outputs.
4. PegcdTotal: nested sufficient-fuel trace and final common inverse applied to all three output registers; return a structured monic Bezout/divisibility certificate, with explicit guarded failures.
5. CantorFallbackExact: instantiate PegcdTotal twice and derive D^2|UaUb and D|N; invoke stored word_exact twice. This is the recommended first complete milestone for a review packet, even if steps1–4 are split into smaller published components.
