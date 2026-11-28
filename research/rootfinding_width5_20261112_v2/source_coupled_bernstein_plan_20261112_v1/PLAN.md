# Exact source-coupled range intersection

UNEXECUTED changed hypothesis. This note does not modify or add arms to the frozen width5 experiment. Bernstein convex-hull bounds and unimodular coordinate changes are classical; no novelty or speed gain is claimed.

## Why the current box loses source identities

For a fixed u the existing certificate is

    J = lambda0*S² + B*S + C - gamma*T - delta*U,
    S=x+y+z, T=xy+xz+yz, U=xyz, x,y,z in[0,M].

Current replay bounds S,T,U independently. Substitute the SAME integer sources instead:

    J = C + B*(x+y+z) + lambda0*(x²+y²+z²)
          + (2lambda0-gamma)*(xy+xz+yz) - delta*xyz.

This identity is over integers, so it preserves all repeated sources and permutations. It does not depend on generic curve rank. It is valid for the declared prime-field interval lift, not for scalar integer encodings of arbitrary extension-field labels.

## Ten exact Bernstein coefficients bound all sources

Let c=2lambda0-gamma and scale x=M*a,y=M*b,z=M*d with a,b,d in[0,1]. Express this polynomial in the tensor Bernstein basis of degree(2,2,2). For i,j,k in{0,1,2}, its coefficient beta_ijk satisfies

    8 beta_ijk = 8C + 4BM*(i+j+k)
                + 8lambda0*M²*([i=2]+[j=2]+[k=2])
                + 2c*M²*(ij+ik+jk) - delta*M³*ijk.

This follows from t=(1/2)B_1^2(t)+B_2^2(t), t²=B_2^2(t), and the tensor product. Bernstein basis weights are nonnegative and sum to one on the cube. Therefore every real source value lies between the minimum and maximum coefficient. Every INTEGER source value lies in

    ceil(min_ijk(8 beta_ijk)/8) <= J <= floor(max_ijk(8 beta_ijk)/8).

The formula is symmetric, so only the ten multisets0<=i<=j<=k<=2 are required. Retaining all27coefficients gives the same extrema. When M=0 the formula reduces to C and remains valid without dividing by M.

Intersect this new interval with the old independently proved S/T/U-box interval. Both contain every true source, so the intersection is always sound and never wider than the old interval; the Bernstein interval alone need not be tighter. If the intersection omits pZ, exclude that u. Otherwise recover the entire conditional fiber as before. Never equate the hull with the exact source image, and do not infer completeness from any single surviving residue.

## Explicit useful counterexample to the old independent relaxation

Take M=1,u=0,p=7,lambda0=1,gamma=2,delta=0,B=0,C=1. Then

    J=1+S²-2T=1+x²+y²+z² in[1,4].

The old independent box yields[-5,10], which meets7Z. The ten Bernstein coefficients give[1,4], which misses7Z. This is an abstract empty coefficient chart, not a proved realization of a guarded elliptic-curve chart: choose T=4*S²+4 modulo7 (since2*4=1 modulo7), any U coefficient chart consistent with delta0. All eight ordered sources in{0,1}³ fail that T equation. Thus extra pruning is possible algebraically; no measured pruning rate follows for NIST inputs.

## First bounded experiment

Fresh source IDs and independent review. Keep the current exact25vector5D pool fixed to separate range changes from discovery changes. Implement the ten-coefficient formula and a separately evaluated27coefficient version. Exhaustively enumerate x,y,z for M0..3 and bounded signed lambda0,B,gamma,delta,C, compare all literal certificate values against both bounds, including repeated/empty cases, negative endpoints and p-shifted phases. Use the explicit p7 witness above and corrupt one Bernstein coefficient to ensure the independent literal oracle detects the fault.

Then preregister a NEW complete-conditional comparison of old width5 range versus width5-plus-Bernstein intersection on all five shifted exposed training blocks, three alternating repetitions; freeze source/input hashes before timing. Charge all ten-coefficient construction, intersection/modular checks, extra exact replay, every survivor solve and tuple serialization. At most90seconds,4GiB-per-process,onecore; at most200000declared fresh source/phase predicates and at most100000native nodes per recovered fiber. Report all masks and tuples, including fields with no added pruning. Additional exclusions must compensate for range overhead; otherwise refute this variant and keep the negative result.

The global h domain still has p values; a better local envelope alone supplies no tractable global cover, original auxiliary/ghost completeness, binary/Koblitz chart, novelty or qualified genuine F4/F5/F6 gain.

## Subsequent changed discovery coordinates, not an extra current arm

Only after measuring the fixed-pool intersection, investigate coordinates
(lambda0,lambda1,lambda3,c,delta), c=2lambda0-gamma. This is an integer unimodular change from(lambda0,lambda1,lambda3,gamma,delta), determinant-1, so the three-constraint lattice retains indexp³. An explicit basis can be obtained by this exact coordinate transformation of the old basis; membership/determinant require independent replay. Preserve gamma=2lambda0-c as an exact representative when bounding cancellation; silently centering gamma would change c by a multiple of p and could destroy its small source coefficient. Derive omitted phase coefficients modulo p as before.

A possible metric weights lambda0 by3M²+6MH², lambda1 by3MH,lambda3 by3M,c by3M² anddelta byM³. This targets the source identity directly, but is an untested quality hypothesis. Never add it post-result as the faster arm of the fixed-pool experiment. All new variants still require complete identical conditional roots and a separate global matched baseline gate.
