# A method-specific lower bound for explicit whole-field dual-cell covers

The following is a derivation, not a fourth C97 experiment. It concerns the actual n83,d18,E35,B1 prefix family. It does not rule out implicit symbolic families, product-support certificates, or covers restricted to the original chain's outer image.

Every nonzero binary functional has a unique representation lambda(v)=Tr(g^2 v). A valid rejection dual hasTr(g)=1. Its cell coefficient-row space is gD+g^2E and rankR=53-dim(D intersectgE). SupposeR<53 and the cell is nonempty. A nonzero intersection supplies g=P/Q withdegP<=17,degQ<=34. ReduceP,Qcoprime. This bounded rational representation is unique: cross-products have degree<=51<83,so equality modulo the field polynomial is genuine polynomial equality.

All intersections have the form d=Ph,e=Qh. Nonempty-cell consistency requiresTr(d)=0 on the intersection. Trace onD is constant-coefficient extraction for this modulus (C97 verifies every basis value). In particularTr(P)=0,soP(0)=0. Coprimality then forcesQ(0)=1. Write r=degP in1..17,q=degQ. The q=0 case givesg=P inDandTr(g)=0,not a valid dual; hence1<=q<=34.

The intersection dimension ismin(18-r,35-q), and therefore

    R=max(35+r,18+q).

There are at most2^(r-1) monic degree-r numerators with constant0,and2^(q-1) monic degree-q denominators with constant1. Ignoring coprimality andTrace(g)=1 only OVERCOUNTS possible cells. A rankRcell occupies fraction2^-R ofthefield. Thus the union of ALL possible nonempty lower-rank valid dual cells has fraction at most

    U=sum(r=1..17,q=1..34) 2^(r+q-2-max(35+r,18+q))
     =3/4 - 10/2^18 - 17/2^36 <3/4.

This is a union upper bound; no disjointness assumption is used. It covers every rank<53validcell,not merely the62profiled cells.

At most2^54separators pass the pair COEFFICIENT relaxation: there are2^53(s,t)states and each nonzero polynomial s^2w^2+tw+t^2+B has at most2roots inw. No state makes the polynomial identicallyzero becauseB!=0. Therefore,even after taking EVERY lower-rank cell, more than2^81-2^54 rejected separators remain. Each remaining rank53cell has exactly2^30members. Covering ALL rejected values by explicitly listed dual cells consequently requires at least

    2^51 - 2^24

rank53cells, even before accounting for overlap. Enumerating this cover is not a tractable global replacement for the original search.

The scope is essential. The actual outer source-generated separator image is far smaller than the entire field and may be strongly biased. The next falsifiable hypothesis is alignment: do structured cells prune that SAME outer image, while preserving every zero/infinity exception and both finite endpoint witnesses? Freeze exact tiny full-chain covers and actual source-generated samples before measuring. No uniform-field lower bound alone answers that question. Implicit certificate families and stronger product-consistency constraints are also outside this barrier.
