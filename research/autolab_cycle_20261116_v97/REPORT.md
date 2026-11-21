# C97: verified oblique cells, structured ranks and an explicit-cover barrier

Three immutable experiments passed on one core with4GiB per-process caps. This cycle tested the C96whole-cell idea rather than infer global pruning from a large cardinality. It produced exact certificates, a lower-rank structured family,and a method-specific obstruction to explicitly enumerating a whole-field cover. The main algorithmic goal remains active and unpassed.

## Hypothesis and contract

The pair coefficient relaxation is M(w)z=B,with53columns at n83,d18. Every matrix entry is affine-linear in83separator bits. A dual lambda withlambda(B)=1 defines the entire affine set C_lambda onwhichlambda(M(w))=0. Every separator inthat set has no coefficient solution and therefore no original source pair. Empty affine sets certify nothing. A surviving value remains unresolved until exact product support. No original source tuple or original full-chain root is emitted by this component.

The implementation forms equations by transposition; an independent checker reconstructs coefficients from original M(0),M(e_i),uses different pivots,and checks affine rank,consistency,base and full direction basis. Exhaustive tinyGF4/8/16/32controls test60duals,1,360separator memberships and740original source pairs in admitted cells. Forty-two cells are nonempty; every admitted original pair test proves empty as required. These are independently checked WHOLE-cell claims,not sampled rejection extrapolation.

## ActualGF83profile and changed hypothesis

The frozen profile has0/1positive controls plus126source-independent random separators. Each rejected query supplies the first32deterministic dual choices. All4,032certificates have rank53,dimension30: each excludes2^30values, but the sum-of-sizes union upper bound is only4,329,327,034,368values,at most4.477e-13ofthewholefield. The126query rejections include their own generating queries by construction; that is not held-out generalization. The two positive controls remain unexcluded.

A changed structured family uses lambda(v)=Tr(g^2v),g=T^r/Q,r1/2,Qmonicodddegree3..6,and admits onlyTr(g)=1. For this modulus,trace restricted toD18 is constant-coefficient extraction. Polynomial intersection and trace consistency prove cellrank35+r. Of120fixedrational candidates,62qualify:32cellsrank36,dimension47,and30cellsrank37,dimension46. All pass the independent checker. The explicitg=T/(T^3+T+1)is a verifiedrank36example. These cells are up to131,072times larger than a rank53cell,while still a small fraction ofGF83.

The union ofall62structured cells has a proven lower bound2^47and upper bound6,614,661,952,700,416values: fractions1.456e-11to6.840e-10. Their exact mutual overlap is not computed. None ofthe128frozen queries is covered. Consequently this bounded family is refuted as a broad standalone pruning method,despite the valid larger cells. No performance gain or global rejection rate is claimed.

## A stronger method-specific bound

COVER_BARRIER.md classifies every nonempty lower-rank dual cell through a unique bounded rationalg=P/Q. Trace consistency forcesP(0)=0,Q(0)=1. Counting all admissible numerator/denominator degree pairs gives a union upper fraction strictly below3/4forALLrank<53cells,notjusttheprofiledfamily. At most2^54separators can pass the coefficientrelaxation,by counting53-bit(s,t)states and atmost2wroots perstate. Explicitly covering all remaining rejected separators therefore needs at least2^51-2^24rank53cells,each ofsize2^30. This is an exponential lower bound for explicit WHOLE-field enumeration inthiscertificateclass,notan impossibility result forrootfinding orfor the actual outer-image domain. The exact rational sum is checked by portable replay.

The resulting next hypothesis is alignment: the original five-source chain generates a much smaller structured outer-separator image. Structuredcells might intersect that image unusually well. Freeze exact tiny full-chain covers and actualsource-generatedsamples before measuring;retainbothfinitewbranches,zero/infinityexceptions,andalloriginalintermediatecoordinates. Implicit family constraints and stronger product-support certificates also lie outside the cover barrier.

## Resources and provenance

Observed worker walls were0.045775s(tinyoracle),13.014769s(queryprofile),0.695928s(structuredprofile);process high-waterRSS126,120/125,112/143,724KiB. These are outer process/inherited high-water records,not isolated kernel allocation or summed descendants. All workers had90sfull-process budgets and ran sequentially; no qualified hostisolation/A/A exists. Source/input hashes,exactduals,rankrecords,proofs and allattemptstreams are retained. Ledger integrity passes.

Primary review: Bachoc,Serra,Zemor,An analogue of Vosper's Theorem for Extension Fields,arXiv1501.00602v2,https://arxiv.org/abs/1501.00602 (abstract reviewed),studies low-dimensional products of vector subspaces in finite-field extensions. It provides relevant prior context,not this implementation's correctness or a novelty clearance. The structured-cell argument here is proved directly; no claim that trace-duality or polynomial rational reconstruction is new is made.

The C96conditional611.81xsource-scan comparison remains unchanged and is not multiplied into a global gain. Whole339-variable completion,fullmatching genuineF4/F5/F6,20preregistered hardinstances,allrequestedfields,qualifiedisolation/A/A andthe50xend-to-endstretch remain open. The frozen2x floor isunchanged.

Replay: PYTHONDONTWRITEBYTECODE=1 python research/autolab_cycle_20261116_v97/REPLAY.py
