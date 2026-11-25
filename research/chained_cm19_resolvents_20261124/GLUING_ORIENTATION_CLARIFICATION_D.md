# Ordered gluing graph: preserve the conditional-plan orientation gap

The original conditional plan3328aa correctly printed the matrix
T=[[1,-baromega],[0,2]] and its kernel {(baromega*Q,Q)}. It called
this a Graph(baromega) quotient without fixing factor order, then proposed
the Mobius root map Frobenius49. HLP's convention is Graph(psi)={(P,psi(P))}.
With columns ordered (c,e), the actual graph map is baromega inverse=omega
on E[2], so using Frobenius49 in that same order would identify the wrong
labeled gluing. No D source/native/model assertion has run; retain the
original conditional subject and this explicit correction. [pcc:unverified]

Keep the proposed root map pi2=baromega and SWAP the elliptic factors.
Use the actual columns (e,c), with

    S = [[-baromega,1],[2,0]],
    S^dagger Hpp S=2I,
    ker S={(Q,baromega*Q):Q in E[2]}.

Thus M(alpha)=alpha49 matches HLP's ordered graph. The first genus2
elliptic pullback p1^* corresponds to e, the second p2^* corresponds to c.
The previous beta3 chain d satisfies2d=e+baromega*c, so its image in the
genus2 Jacobian is the genuine integral half of p1^*+baromega*p2^* after
the polarized quotient identification. Division2 exists by the actual
quotient construction; moduloell evaluation is a separate subgroup recipe.
Check BOTH T and S and the exact ordered graph in the forthcoming native
unit. No need to select a different curve or silently change Frobenius.
[pcc:unverified]
