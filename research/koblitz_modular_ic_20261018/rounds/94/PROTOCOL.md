# R94 corrected principal-open overlap certificate, frozen before execution

The accepted R50 algebra is a rank-32 free F2[u] module in its degree-32
source field. R91 checked all 32 forward and reverse generator images after
clearing bounded powers of h=u+v and k=u+v+u². R88 checked rational inverse
composition. R93 was strictly rejected when its source applied `lift` to
the coefficient `u`; its seven preliminary membership lines were not
accepted. R94 changes that step to direct `1/u` coefficient inversion and
rechecks the full seven-function scope. R94 tests the overlap-unit bridge, without repeating
the R91 generator transport or asserting a proper boundary.

In the polynomial ring, require h²+uH=E, k²+uJ=E, and h+N+1=H for
E=v²+u²v+u+u³, H=1+u+u²+uv, J=H+u³, N=v+uv+u². In the accepted field,
check unit identities and the forward image of k and reverse image of h
equal 1/u. Test 1,u,v,h,k,H,J for polynomial R50 coordinates, full
B*C=F*W products, and a corrupted-coordinate rejection. Export all 224
coordinates and 224 power entries for independent ideal-free replay.

One local pinned Singular process, seed 94, one thread, 4 GiB virtual
memory, 1200 seconds. Source/protocol and donor hashes must be committed
before data. Accept exit zero, no native `?` errors, exact completion
marker, seven membership lines, all identities and negative controls.
Retain a failed attempt and freeze a new ID for any repair. No timing or
comparative claim is made. A custom digest-pinned Singular image remains
necessary for EKS Spot execution of this CAS workload.

Sources checked: R50 `RR-ALGEBRA.md` and `NEXT_SOURCE_NORMALIZATION.md`,
R88 `INVERSE88.sing`, R91 `TRANSPORT91.sing` and replay,
R93 `CM.md` and failed native log, and the
[Stacks Project gluing criterion](https://stacks.math.columbia.edu/tag/01JA)
and [normalization](https://stacks.math.columbia.edu/tag/035E).
The criterion requires actual overlap isomorphisms; algebra closure and
generic rational identities alone do not establish proper boundary or
the modular-factor action.
