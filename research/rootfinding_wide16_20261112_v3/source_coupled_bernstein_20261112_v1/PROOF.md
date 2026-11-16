# Exact source-coupled interval controls

This is a new pure-Python correctness component, not an actual elliptic-chart profile, a novelty claim or a global all-roots solver. All old measured source and evidence remains unchanged. The coordinator alone executes the frozen command.

For integer x,y,z in[0,M], let S=x+y+z,T=xy+xz+yz,U=xyz. The fixed-u certificate J=aS²+BS+C−gammaT−deltaU is identically C+B(x+y+z)+a(x²+y²+z²)+(2a−gamma)(xy+xz+yz)−delta xyz. Repeated sources and every ordering are retained; no generic-rank or squarefree premise is used.

Scaling each source by M gives a polynomial of degree at most(2,2,2). Its tensor Bernstein coefficients obey

    8beta_ijk=8C+4BM(i+j+k)+8aM²([i=2]+[j=2]+[k=2])
             +2(2a−gamma)M²(ij+ik+jk)−deltaM³ijk.

The tensor basis is nonnegative and sums to one on the real unit cube. Thus every real source value lies between the minimum and maximum coefficients. Integer source values lie between ceil(min8beta/8) and floor(max8beta/8). Symmetry reduces the extrema to the ten nondecreasing triples i≤j≤k; all27 are separately constructed by the generic power-to-Bernstein formula beta_i=sum_(e≤i) p_e prod_j binom(i_j,e_j)/binom(2,e_j). This independent conversion uses Fraction and never calls the closed ten-coefficient formula. M=0 requires no division and gives constant C.

The old interval independently bounds integer S=0..3M, T=0..3M² and U=0..M³. Both intervals contain every actual triple; their intersection is sound and never wider. If ceil(L/p)>floor(R/p), no integer multiple of p lies in the intersection, so every actual source fails J=0 modulo p. All other fibers stay unresolved until direct full source recovery. The controls compare COMPLETE ordered abstract support sets against separate elementary-symmetric evaluation, not merely known witness membership.

The grid has243 coefficient choices and4 M values:972 cases. There are24300 ordered source values and121500 per-prime/source exclusion checks. Each case additionally compares all27 tensor coefficients, exact interval intersection, five complete modular root sets and20 signed phase shifts. The stated counter counts predicate cases, not each integer/Fraction operation; the fixed loops use fewer than173000 predicates plus13 exceptional cases, below200000. Direct enumeration is bounded before any materialization. The prime2 checks concern integer modular support only; they do not admit a characteristic-two curve chart.

Replacing C by C+kp translates every bound by kp and leaves intersection with pZ invariant. Negative ceil/floor controls cover both sides of divisibility. The abstract p7,M1 fixture J=1+S²−2T=1+x²+y²+z² has old interval[-5,10] and source-coupled interval[1,4], proving extra algebraic exclusion is possible. It is NOT a proved realizable guarded elliptic chart. Corrupting its upper-corner Bernstein coefficient from4 to0 makes the false interval omit the literal source value4 at(1,1,1), so the independent source oracle rejects the corruption.

The worker retains every case's27/10 coefficients, intervals, masks and complete abstract modular support sets in exclusive raw evidence. It serializes the whole raw result before accepting its exact component flag. An80second internal work deadline,84second report demotion and88second final print boundary sit inside the runner's90second complete-process deadline; startup/final stdout are externally charged. Late or capped results are inconclusive, never empty roots. Self RSS is individual peak process memory, not aggregate descendants.

The existing width8 prime panel already has no empty fibers left unresolved by5D, so this new bound cannot improve pruning there. It can only measure overhead on that panel. Any pruning-benefit test requires a fresh frozen panel with empty unresolved fibers and complete independent recovery, charging input selection, bounds, replay, all survivors and serialization. No actual-curve performance experiment is included here; the full F4/F5/F6 release gate remains unchanged.
