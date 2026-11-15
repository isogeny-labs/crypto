# CRT reciprocity and the general polynomial block radical

2026-11-13. Component proof report; unsigned producer standing [pcc:unverified].

This round replaces the earlier constant-B control with general polynomial
reciprocity and radical statements. It also supplies the abstract condition
under which a branch hyperplane introduces no extra radical vectors. These
results are not yet assembled into the actual finite coefficient matrix or a
public-input certificate finder [pcc:unverified].

## Objects and exact scope

Let K be any field, F,H monic, b=natDegree(H)>0, a=natDegree(F), and b+1<a.
Suppose explicit polynomials U,V,f satisfy

    U F + V (f H) = 1.

Write mu(Z)=coeff_(a-1)((Z V) mod F). For deg A<=a, deg B<=b define

    B_mu((A,B),(Q,T)) = mu(A Q - f B T),  deg Q<=a, deg T<=b.

The general theorem characterizes its radical exactly:

    B_mu((A,B),(Q,T))=0 for every bounded Q,T
      iff A=c F and B=d H for some c,d in K.

This uses the actual remainder-defined pairing, not a supplied rank
assumption. No separability or characteristic restriction is required.
The even norm chart a=b+2 satisfies the gap; constant H is excluded here and
was handled by the earlier planted control. No curve/subgroup is silently
introduced by these polynomial statements [pcc:unverified].

## Proof and declaration map

1. `NormCRTReciprocity.reconstruct`: for coprime monic F,H and bounded S,T,P
   satisfying F|(S H-P), H|(T F-P), the difference S H+T F-P is divisible
   by F H and has degree below its degree. Hence S H+T F=P.
2. `coefficientReciprocity`: if additionally natDegree P<a+b-1, compare
   coefficient a+b-1 to obtain coeff_(a-1)(S)+coeff_(b-1)(T)=0.
3. `remainderReciprocity`: construct S=(P V) mod F, T=(P U) mod H from
   U F+V H=1, deriving both congruences from ordinary monic division.
4. `weightedReciprocity`: U F+V(f H)=1 gives, for natDegree R<a+b-1,

       mu(f R) + coeff_(b-1)((R U) mod H) = 0.

   This is the missing nonconstant lower-block relation, with monic H.
   Nonmonic normalization constants still need an explicit assembly step.
5. `NormPairingRadical.annihilatorIff`: the unit-twisted coefficient pairing
   annihilator is F|P. If P mod F is nonzero, the earlier generic coefficient
   detector supplies a contradictory low-degree test. Congruence preserves
   all relevant remainders. The converse is direct divisibility.
6. `boundedDivisibility`, `boundedRadical`, `fullBoundedRadical`: when
   natDegree P<=a, F|P is equivalent to P=c F. Allowing tests of degree a
   retains this exact radical.
7. `lowerAnnihilator`, `lowerBoundedRadical`, `lowerFullRadical`: CRT transfers
   the lower pairing to the quotient modulo H. Bezout implies H is coprime
   with U; the same detector applies. For full tests through degree b,
   deg(P Q)<=2b<a+b-1 explains the strict gap assumption.
8. `normBlockRadical`: test with (Q,0) and (0,T); the preceding two radical
   characterizations give precisely (c F,d H). Conversely both directions
   annihilate the complete block pairing.
9. `branchTestsDetectFull`: for any bilinear pairing and branch functional l,
   if a right-radical vector k satisfies l(k)!=0, every v can be adjusted to
   v-l(v)/l(k)*k in ker(l). Thus vanishing against branch-zero tests is
   equivalent to vanishing against all tests. This does not itself identify
   the actual branch or its scalar intersection in the norm coordinates.

All listed statements are Lean-backed components with explicit hypotheses,
not an assembled rank/completeness claim [pcc:unverified]. Exact source:
[CRT module](../lean/NormCRTReciprocity.lean),
[radical module](../lean/NormPairingRadical.lean). All fourteen producer claim
bindings are in [evidence](../crt-evidence/); standing is unsigned.

## Evidence and controls

Pinned Lean4.35.0, CryptoLean148f5dc389b341401e06d76abe7783c4ccf42f2e,
Mathlibcfc55fc58714a59dc06e6e79edc6d1d95140d3c2. Frozen CRT source commit
3487f067d; frozen radical source commit0d1208d8b. Build001/002 for CRT and
radical builds002–006 succeed; radical build001 failed because rewriting a
polynomial also rewrote its coefficient expression. All raw logs, including
that failure, are retained. Subsequent normalization edits remove unnecessary
tactics; no failed source is described as proved [pcc:unverified].

The first tracked-source gate covers95 declarations, before the new radical
module was tracked. Final gate and per-module fresh status receipts identify
their exact scope. Final gate002 covers104 declarations with zero failing and
zero additional trusted axioms. Both fresh replays exited0 including imports.
Only propext, Classical.choice and Quot.sound are admitted;
no native_decide, extra axioms or holes are admitted by this study's gate
[pcc:unverified].

Native record **IDC1h4b52a3733d292783** checks the universal degree3 residual

    (P V-F Q)H+(P U-H T)F-P
      = P(U F+V H-1)-F H(Q+T).

It uses the unchanged maintained issuer/checker with32 statement-derived and32
extra-stream points, and pins the exact universal CommRing Lean source.
This native checker does not execute Lean, check degree hypotheses, or prove
radical/rank conclusions. Its adapter's22 inherited tests pass, including
statement-swapping and tamper rejection. These are correctness checks, not
performance measurements [pcc:unverified].

## Relation to the earlier rank-directed completion proposal

The earlier annotation proposed replacing the identity contribution and
choosing H in W_new=W_base+C(T)H(T) to lower crossing rank while preserving
node values. This round develops a concrete quotient-moment alternative for
producing the intended radical. It does not establish that the form is
reachable through that jet-completion family, or give an efficient choice of
its free coefficients. Those must not be inferred from certificate existence
[pcc:unverified].

## Next obligations and empirical boundary

1. Build the bounded polynomial coefficient bases and identify their matrices
   with this pairing, including monic normalization of a supplied norm witness.
2. Derive mu(G)=0 and actual coefficient-pencil membership from A^2-fB^2=cG.
3. Instantiate the branch lemma, compute the witness-line intersection, and
   connect exact corank1 to the earlier guarded adjugate recovery theorem.
4. Cover degree drops, zero/constant charts and their probabilities.
5. Find those moments/pencil coefficients from public data at an accounted
   2^{o(n)} bit cost, n=ceil(log2 r); current Bezout data comes from supplied
   witness polynomials. Certificate existence alone does not perform this search.

No empirical timing/scaling campaign or paid job was run. The monthly isolab
budget remains exhausted; no replacement host or budget override is assumed.
Pure exact identities do not depend on CPU contention. A later runtime claim
would require the recorded isolated-host protocol and end-to-end accounting.
No benchmark/leaderboard/ecdb record changes are appropriate [pcc:unverified].

## Sources and search coverage

KB MCP is absent; `scripts/research-kb status` reports no executable. Fallback
review used the preceding NEXT_GENERAL_PROOF.md and exact pinned Mathlib
polynomial Div, Degree.Operations, Degree.Domain, Monic and Coprime.Basic
sources, plus the already checked NormQuotientMoment detector. No priority or
exhaustive literature claim is made for classical CRT or quotient pairings.
This is a bounded continuation, not a new cross-repository survey.

Diagram source and rendered copies: [DOT](bridge.dot), [SVG](bridge.svg),
[PDF](bridge.pdf). Report [PDF](report.pdf), editable [LaTeX](report.tex).
Canonical scoreboards, progress timeline, curve diagrams and existing study
figures were checked for scope: none gains a measured result or changes its
historical conclusion; frozen older figures remain unchanged.
