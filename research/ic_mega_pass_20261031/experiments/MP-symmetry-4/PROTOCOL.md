# MP-symmetry-4 protocol (pre-registered 2026-10-31, before any run)
Hypothesis: for E: y^2+xy=x^3+ax^2+b over F_{2^n}, n prime, x!=0 lifts iff Tr(x)+Tr(beta/x)=Tr(a), beta=sqrt(b).
Z = lifting x-set (union of {x,beta/x} orbits). Question: do all-lift F_2-subspaces V (V\0 in Z) exceed a random null?
Fixes adopted from skeptics: sigma condition is Tr(beta*sigma_V)=Tr(a) (bug check only); sigma_V=0 is NOT a falsifier; no closure-by-citation.
## Parameters / seeds
- Fields: n=7 x^7+x+1; n=11 x^11+x^2+1; n=13 x^13+x^4+x^3+x+1; n=17 x^17+x^3+1; n=19 x^19+x^5+x^2+x+1.
- Curves per n: 9 = (a random, b=1) + 8 random (a,b); numpy seed 20261031+n.
- Arm A identity: all x in F*, vs direct solvability of z^2+z=(x^3+ax^2+b)/x^2 (image table of z^2+z).
- Arm B exact counts of all-lift l-subspaces (greedy-min echelon DFS, each V counted once): n=11 l=2,3,4; n=13 l=2,3. n=13 l=4: sequential importance sampling (1e5 chains, seed 7), SIS validated vs exact at n=11 l=4.
- Null 1 (exact): E = Gauss[n,l]_2 * prod_{i<2^l-1} (|Z|-i)/(2^n-1-i).
- Null 2 (MC): orbit-preserving shuffle (permute {x,beta/x} pairs among pairs, fixed point fixed), 3 shuffles per curve at n=11 l=3,4; seed 99.
- Positive control: plant a random 4-dim subspace into a shuffled Z; counter must report >= +1 at l=4.
- Arm C: for every all-lift V (n=11, l=2,3, all curves): check Tr(beta*sigma_V)=Tr(a) (expect 100%), frac sigma_V=0 (~2^-n), count V with beta/V u {0} a subspace (expect ~0).
- Arm D: l*(n) n=7..19: randomized DFS (node budget 2e5 per dim target) on true Z and on one orbit-shuffled Z; null l*_formula = max l with E>=1.
## Predictions
TRUE (no structure): identity 100%; pooled obs/null ratio within Poisson 2-sigma CI of 1 at each (n,l) (or matching 0.92x prior census within CI); l* within +-1 of shuffled/formula null; sigma check 100%; beta/V-closed count ~0.
FALSE (structure): pooled ratio >=1.5 at l=4 with CI excluding 1 and growing n=11->13; or l* exceeds null by >=2 at n>=13; or multiple beta/V-closed all-lift V beyond chance.
Class: relabelling. Budget <20 CPU-min.
# PROTOCOL AMENDMENT (after first n=11 curve timed at 163 s, before seeing other results): budget cut — n=11 shuffles 2 per l on curves 0-2 only, SIS-vs-exact l4 validation on curves 0-2; n=13 exact l=3 on curves 0-1 only, SIS l=3 (1e4 chains) and l=4 (2e4 chains) on all 9. Predictions unchanged.
