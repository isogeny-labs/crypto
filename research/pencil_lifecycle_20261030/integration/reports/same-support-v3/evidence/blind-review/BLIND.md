# Independent blind semantic derivation

2026-10-30. Read only prospective PLAN.md and standalone statement definitions supplied by root. No candidate sources or producer logs have been received or read. This record is frozen before source review. No builds or scientific executions performed.

## Review plan

1. Derive normalized remainder equations using common u and f.
2. Derive exact common denominators for the literal cross-scaled expressions.
3. Prove classifier exclusion respecting inverse-first priority.
4. Give explicit omitted-premise and corrupted-weight controls, preserving unsuccessful attempts.
5. After freezing this record, inspect an immutable candidate and map every premise and semantic joint to its theorem statement and proof. Distinguish imported coefficient theorem from separately checked polynomial interpretation and distinguish abstract classifier from compiled Rust semantics.

## Statement

u_i=X^2+(q_i/z_i^2)X+r_i/z_i^4; v_i=(s_i/(z_i^3*w_i))X+t_i/(z_i^5*w_i). Require all z_i,w_i nonzero, common f and u_i divides f-v_i^2. Q1=q1*z2^2,Q2=q2*z1^2 and R1=r1*z2^4,R2=r2*z1^4 agree. S1=s1*z2^3*w2,S2=s2*z1^3*w1,T1=t1*z2^5*w2,T2=t2*z1^5*w1.

Classifier priority: if S1=-S2 and T1=-T2, inverse; else if S1=S2 and T1=T2, double; else if S1-S2=0, sameUMixedZeroSlope; else mixed.

## Independent derivation

Write D=z1*z2 and E=z1^3*z2^3*w1*w2. Both are nonzero. The common normalized u is X^2+qX+r with q=Q1/D^2=Q2/D^2 and r=R1/D^4=R2/D^4. This establishes same u by cancellation, rather than assuming it. Normalized slopes a_i=S_i/E and intercepts b_i=T_i/(E*D^2). In particular the extra two powers in T are essential.

Subtract the two divisibility assertions. The common u divides v1^2-v2^2. Reducing X^2=-qX-r gives
2*(a1*b1-a2*b2)-q*(a1^2-a2^2)=0,
(b1^2-b2^2)-r*(a1^2-a2^2)=0.
These follow because a degree at most one remainder divisible by monic degree two u must vanish. The prior coefficient theorem may supply this exact named dependency.

Clearing nonzero denominators gives literal identities
2*(S1*T1-S2*T2)-Q1*(S1^2-S2^2)=0,
(T1^2-T2^2)-R1*(S1^2-S2^2)=0.

If S1 != S2 the zero-slope branch cannot be reached. If S1=S2=S and T1=T2, either inverse or double is selected, both acceptable. Otherwise T1-T2 !=0, and the first identity reduces to 2*S*(T1-T2)=0. Characteristic other than two implies S=0. The second identity gives (T1-T2)*(T1+T2)=0; hence T1=-T2. S1=S2=0 also gives S1=-S2. Thus inverse is selected FIRST. This ordering is material: equal slopes and unequal intercepts are possible valid inputs and must be absorbed by inverse.

## Boundary controls (exact hand derivations, not executed trials)

A. Valid zero-slope unequal-intercept case: over Q set all weights 1, q1=q2=r1=r2=0, s1=s2=0, t1=1,t2=-1, f=1, u=X^2. Both validity assertions hold identically, and inverse catches the case. A proof claiming equal S always implies equal T would be false. Reordering zero-slope before inverse would also be false.

B. Remove same-f validity: over Q all weights 1, q=r=0, s1=s2=1, t1=0,t2=1. Choose f1=X^2 and f2=(X+1)^2 individually. Each separate validity assertion holds, but the common classifier input (1,0,1,1) selects sameUMixedZeroSlope. With common f=X^2 the second validity assertion fails. This is a successful necessity control for common-f validity.

C. Corrupt S exponent 3 to 4: over Q z1=1,z2=2,w1=w2=1; q1=q2=0,r1=-1,r2=-16,s1=1,s2=16,t1=2,t2=32. Normalized u=X^2-1,v1=X+2,v2=2X+1. Set f=(X+2)^2; f-v2^2=-3*(X^2-1), so both valid. Correct S=(8,16),T=(64,32) yields mixed. Corrupted exponent gives S=(16,16),T=(64,32), neither inverse nor double, hence forbidden branch. Successful control.

D. Corrupt cross w orientation to own w in both S and T: over Q z1=z2=1,w1=1,w2=2; q1=q2=0,r1=r2=-1,s1=4,s2=2,t1=1,t2=8. Normalized u=X^2-1,v1=4X+1,v2=X+4. f=(4X+1)^2, f-v2^2=15*(X^2-1). Correct S=(8,2),T=(2,8) yields mixed. Own-weight S=(4,4),T=(1,16) yields forbidden branch. Successful control.

E. Omit nonzero weights: attempted counterexample cannot work for the literal classifier. If any z1 or w1 is zero then S2=T2=0; if any z2 or w2 is zero then S1=T1=0. If S1=S2 (needed to reach forbidden branch), both S values vanish, but T on the other side can remain nonzero, so this alone does NOT exclude forbidden branch. Indeed z1=0,z2=1,w1=w2=1; q1=q2=r1=r2=s1=s2=0,t1=1,t2=0. Lean-style field division by zero makes both normalized v zero, both u=X^2, f=0; normalized validity holds, Q=R=S=0, T=(1,0). Inverse and double both fail, zero-slope branch is selected. Successful control under totalized field division. With conventional partial division these inputs are undefined instead.

F. Characteristic two: no counterexample from dropping characteristic restriction. If S1=S2, the second coefficient identity gives T1^2=T2^2, hence T1=T2 in a field of characteristic two. Thus the claim likely remains true in characteristic two by a separate argument. The stronger restriction is sound but not necessary for this isolated exclusion theorem.

## Preliminary conclusion

The proposed statement is mathematically sound with the exact classifier priority and literal weights. It needs two coefficient identities, not only the linear coefficient identity. Nonzero weights support real cancellation and are necessary under totalized division. Same-f validity and precise exponents/orientation have explicit failure controls. No conclusion yet concerning any unseen formal candidate.
