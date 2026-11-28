# R35 larger closed-point algebra and Picard inputs

Observation report, not an independent scientific review or an IC performance
claim. This round executes the construction proposed in R34 rather than
substituting the smaller L2D kernels for large-model inputs.

| Run | Exact observed output | Controls and retained certificates |
|---|---|---|
| algebra35-01 | F2 algebra dimension770;154local coefficients over F32 | All770basis/inverse recombinations; actual coefficient-field embedding; ring identities; corrupt inverse/field controls |
| jets35-01 | Precision154; section ranks68,196,324 | All5source equations at everyNewtonstage;8units/discriminant;5coefficient conjugates;68literal descent identities; complete old9term regression; corrupt source rejected |
| multiply35-01 | All52650unordered products; outputrank708 | Everyproduct agrees between GF32convolution and bitplane convolution; all1024field products; allproduct/pivot recombinations |
| inflate35-01 | Ten spaces, eachrank196 insideV324;46240products | Original128constraint/68kernel recombinations; everyproduct independently checked; allambient and newbasis coordinates; corruptkernel rejected |

The algebra is A=F2[t]/(t5+t3+1)^154. Its actual coefficient-field embedding
is recovered from the fixed Q, not picked as an arbitrary isomorphic F32.
The fullforward/inverse basis between powers of t and a^i z^j is retained.
The coefficient lift t1024 and z=t+t1024 satisfy the required ring identities.

The new jets use explicit reduction after each product. A Singular quotient
ring alone does not truncate polynomial representatives. Eight Newtonstages
reach154; fullsource checks follow each correction. The comparison with R34
uses actual series moduloz9, not incompatible packed trace/polynomial bits.
Native selected-basis ranks come with original-column recombinations.

Both multiplication implementations use the same serialized F32 inputs but
different arithmetic: scalar field convolution versus binary bitplane
convolution followed by descending reduction by X5+X2+1. Their agreement is
an internal independent implementation check, not an independent external
scientific replay. Everyunordered324pair was processed, including products
after rank708 was reached. All52650coordinates and selected708pairs retained.

Each new vanishing space comes from all68sections times all68oldkernel
sections. Membership and coordinates in V324 are checked for all4624products
perdivisor. Given the retained proper normalization/local and literal pole
bounds, these are complete L(3D0-D_i)spaces: degree256,dimension196. Native
rank alone does not establish that geometric bridge. D_i=P_i+9Q has degree128;
none of these ranknumbers is an IC relation rank or a targetfactor image.

The Lean source ClosedPointAlgebra35 proves the coefficient-lift identity
under explicit characteristic2,residueperiod32 and nilpotence hypotheses.
The terminalcompile/audit receipts and exact auditcount are in results.json;
no nativearrays,normalization or Riemann--Roch theorem is imported as a proof.

Allnative invocations retain4GiBvirtual/1200slimits; Lean4.34.1/mathlibd13f23b
uses-j1-M4096/1200s. Frozen sources preceded affected native runs; the machine
protocolindex is explicitly retrospective. Every attempt, including failures
if any, remains in the archive. No statistical/timing measurements or speed
comparison; CPU/RSS/performance not claimed. The rootRustsuite is not newly
passed; prior cap/missingfixtures remain documented in R32/R34.

Next is exact subspacedivision,flip/addflip/equality and the actual signed
sum using certified deckordering. R36 begins that operation with zero and
same-sectiondoubleflip regressions. The intended F2factor map and nonzero11
image remain separate. Fullquotient saturation,polarized embeddings,theta,
node187 and arbitrary/larger collector supports remain active obligations.
No keyrecovery,asymptotic or end-to-endIC result follows from this setup.

Sources read: Khuri-Makdisi,
[Asymptotically fast group operations on Jacobians of general curves](https://arxiv.org/pdf/math/0409209),
RepresentationB and large-model/flip algorithms; R34 LOCAL_GEOMETRY.md,
COMPLETE_DIVISORS.md,NEXT_CONSTRUCTION.md and accepted source/pole notes.
This applies established arithmetic rather than claiming a new Jacobian method.
