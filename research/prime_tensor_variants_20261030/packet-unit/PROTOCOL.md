# Prospective fixed ternary packet DEVELOPMENT unit

Frozen2026-10-30 before implementation/execution. Primary read:
/workspace/audit/tensor-primary/openai-oct25.{pdf,txt}, AppendixD.4.2-D.4.5,
printedpp58-63. This unit tests a supplied concrete ternary replacement gadget;
it does not implement the primary binary gadget as a ternary theorem, a full
contraction,3-adic precision recovery or a cryptanalytic algorithm.

## Field action and exact fixed panel

Only the small prime fields F_2,F_5,F_67 are used. Characteristic3 is unsupported.
For each level i, encode four entries in order(R_i1,R_i2,C_i1,C_i2).
S(a,b)=(-a-b,a), S^-1(a,b)=(b,-a-b). U_i appliesS to(R_i1,C_i1)
andS^-1 to(R_i2,C_i2). The U_i commute at distinct levels.
A slot x in[0,3^a) has base3digits d_i(x); its class is product_i U_i^d_i(x)gamma.

Exhaust every pair class gamma in F_r^(4a), every slot x,y, for exactly:
(r,a)=(2,1),(2,2),(5,1). Expected class counts16,256,625(total897).
The checks include x=y as diagonal coverage but do not assign a finitev3(0).
For t=y-x!=0, compute i=v3(t) and c=(t/3^i)mod3 normalized to1or2;
require rowR_i,c atslotx equalscolumnC_i,c atsloty.
Check U_i^3=I, U_i^-1, commutation, orbit closure/disjointness,
and orbit size3^k where k is the number of nonzero4-entry levels.
Expected orbits6,36,209(total251). All26505 slot pairs are retained
as per-class aggregate counts, with22278 nonzero-difference locality checks.

Separately test every one of4489 ordered pairs(a,b) inF_67^2:
S/S^-1 order3 and inverses; orbit size1 at(0,0), otherwise3;
for allx,y in{0,1,2}, x!=y, selectS when c=1 andS^-1 when c=2
and verify row-atx equalscolumn-aty. This gives26934 locality checks.
This verifies the action at an actual label fieldr=67>48 without enumerating67^4.

## Full finite matching graphs and uniform padding

For each full-class panel put L=3^a, Darr=r^(4a),
H=ceil(2Darr/L)*L, M=H*Darr. Construct all pair classes at loadH,
using one fresh row and one fresh column vertex for every edge(Delta=1).
Expected(H,M,packets) are(33,528,176),(513,131328,14592),
(1251,781875,260625). Total913731 edges and275393 packets.

For each action orbit O, a packet ofL slots lists product_i U_i^d_i(x)gamma;
each class occursL/|O| times. UseH*|O|/L copies of that packet.
Assign a distinct edge copy of each class on every visit; explicitly check
edge-ID coverage/no duplicate, fresh matching endpoints, no unused slot,
exact class loadsH, totalpackets=M/L and original numerical inequalities
M>=2Darr^2*Delta andM>=L*Darr. For every row/column coordinate and field
symbol, verify the actual assigned-port marginal isM/r, not just a predicted
sum. Retain per-class and per-orbit aggregate records instead of millions
of port rows; an explicit bounded visited-edge array checks all edge IDs.

## Source-tile groups and binary-assumption regression

For every full class, each leveli, componentc=1,2 and tileoffset
0<=y0<3^(i+1), enumerate the columns y=y0+3^(i+1)j in[0,L).
Verify their selected column group is constant. Enumerate all row slots
and collect the distinct selected row groups different from the source
column group; require at most2 target groups.

Explicit regression:(r,a)=(5,1),gamma=(R1,R2,C1,C2)=(0,0,1,0),
level0,component1,tileoffset0. Its source group is1; row values are0,4,1,
so the two cross targets are0and4. Reject a binary-style one-cross-target
assumption on this actually generated packet. The corresponding theoretical
mode-count factor must allow two targets(18M/D instead of9M/D in the
parent's proposed ternary adaptation). This unit does not establish the
full mode budget,3-adic valuations, child contraction or asymptotic theorem.

## Fixed negative controls

1.Wrongsecond action: useS instead ofS^-1 on component2 forF_5,a1,
 gamma=(0,1,0,2),x0,y2. Generate both packets and require correct locality
 passes while wrong locality fails(row1,wrongcolumn2).
2.Nonmultiplepadding: rejectH=32 forr2,a1,L3; do not silently omit slots.
3.Unsupportedchar3: field guard rejects3; independently exhibit nonzero
 fixed pair(1,1), for whichS(1,1)=(1,1)mod3, contradicting the assumed
 nonzero orbit size3.
4.The explicit two-cross-target regression above must reject the
 binary-style one-target predicate.

## Build, records and limits

Implement in nativeC++17 exact small-integer field arithmetic only;
strict-O2-Wall-Wextra-Wpedantic-Werror. Preserve protocol/source hashes,
compileridentity, exactcommand, every build/run attempt and exit, all
per-class/orbit/marginal/tile records, explicit witnesses and summary.
Repeat once and require byte-identical retained records. Counters are
correctness diagnostics, not timings/scaling fits. No Python research
calculation, no novelty claim and no sub-exponential-goal claim.
The optional3-adic Cauchy/Hermite unit is not included in this protocol.
