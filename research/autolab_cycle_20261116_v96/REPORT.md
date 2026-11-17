# C96: exact binary two-factor block recovery

Three immutable experiments passed. A new rebracketing implementation solves the ORIGINAL two-factor block S3(a,x,z)=S3(z,y,b)=0 with x,y in the prefix source domain and z over the whole field. It combines the exact symmetric pair solver with explicit zero-intermediate and infinity branches, then reconstructs and verifies every original z. This is a concrete sparse-block kernel for the index-calculus chain; it is not the complete339-variable solve or a genuineF4/F5/F6 comparison.

## What changed and why the exceptions matter

The previous strong specialized method enumerated every x, solved for z overK, then solved for y inD. Rebracketing instead solves finite S3(a,b,w)=0 (at most2w), runs the exact pair solver on S3(x,y,w)=0, then reconstructs all z from the original equations. The original z may be0, allowing a Frobenius-sign bridge not represented by a finiteK witness; explicitly recover x=sqrt(B)/a,y=sqrt(B)/b. Equal endpoints additionally permit the endpoint sum at infinity, requiring the x=y branch. Pair/diagonal caps return unresolved with exact countnull. Rational curve-point assumptions are not used to discard polynomial roots.

The proof in the source packet aligns Frobenius signs through the non-two-torsion shared intermediate. It proves a finite rebracketing witness overK exists unless the endpoint sum is infinity. Zero and infinity exceptions cover the remaining cases. Original reconstruction filters every proposed pair, so a resultant-only solution never becomes an original root. All repeated source orders and intermediate alternatives are retained.

## Actual evidence

Independent exhaustive original x,y,z enumeration passed every endpoint pair overGF4/8/16,B1/B2:672cases,834complete roots. Of those,12roots are unique to the explicitzero branch and166unique to infinity; disabling those branches would fail these controls. Both the source-scan and rebracketing algorithms match the literal oracle.

The actualGF(2^83) modulus isX83+X45+X2+X+1. Six exposed supported regular blocks atd8/12/18 complete with2original triples each. The explicit d18zero-bridge case completes with3triples. A d18equal-endpoint control correctly remains incomplete because its diagonal enumeration exceeds the declared12-bit cap; its exact count isnull, neverzero. These are CONDITIONAL middle blocks with fixed endpoints, not the originalfive-source solution set.

Three cold repetitions,alternating arm order,full fieldsetup,paircoefficientwork,originalzrestoration,originalequation verification and complete output are charged:

| Source prefix | Exact source-scan median | Rebracketing median | Scoped component ratio |
| --- | ---: | ---: | ---: |
| d8 |0.110885s|0.034900s|3.18x|
| d12 |1.418071s|0.036211s|39.16x|
| d16,new frozen case |21.654271s|0.035394s|611.81x|

All18cold processes complete and agree on the complete original(x,y,z) tuples. The d16case is a newseed961612supported input frozen before timing, not a longer retry. Its three source times are21.885467,21.011290,21.654271s. Rebracketing times are0.069850,0.034944,0.035394s; the slower first run is preserved and noA/A qualification is claimed. The six output sets agree on both original triples. The increase fromd12 to16 exposes the avoided source-domain scan. These are diagnostic medians against an exact SPECIALIZED scan, not a qualified speedup overgenuineF4/F5/F6.

Cold child wait4RSS is12,800–12,928KiB. Outer worker peaks are114,724KiB(oracle),113,716KiB(initialcold),130,536KiB(scalecontroller),with process walls1.046342,4.878275,64.763740s. These are individual-process high-water/inherited accounting; per-process4GiB limits do not bound aggregate descendants. All failed/slow arms would be preserved beforevalidation. The initial component cap90s was followed by a separately frozen150s scale experiment after completed d12evidence; every actual worker was within budget. No qualified isolation orA/A record exists.

## Remaining bottleneck and fresh next hypothesis

Fixed endpoints make the block cheap, but discovering all endpoint combinations still leaves an enormous outer search. Straight integration with the existing five-source endpoint enumeration remainsO(|D|^3),around2^54outer source states atd18,not a tractable whole solve. Do not multiply the611.81xcomponent ratio into a claimed global gain. The old separator join and2^44residue-cover route remain serious competing representations.

A new whole-cell theorem follows from pair-relaxation duals: M(w) is affine-linear in wbits. One verified dual at an unsupported w defines an ENTIRE affine separator cell on which M(w)z=B is inconsistent. At n83,d18 each nonempty cell has at least2^30values, but possibly tiny global fraction and substantial overlap. These oblique cells exclude the unit direction responsible for previousB1prefix-span blindness. NEXT_AFFINE_DUAL_CELLS.md gives the proof and a frozen-budget next experiment proposal. No such cell experiment ran inC96 and no global pruning bound is claimed.

Primary-source review: Galbraith/Gebregiyorgis, Summation polynomial algorithms for elliptic curves in characteristic two, authorPDF https://www.math.auckland.ac.nz/~sgal018/shishay.pdf, pp1–2,7: prior work explicitly uses symmetric variable choices and group-law/resultant decompositions. IACR directPDFfetches returned403; the authorcopy was read. This is not novelty clearance for the exact finite-field implementation. No claim that rebracketing,symmetric variables or associativity is new is justified.

The requested50x end-to-end target and frozen2x releasefloor remain UNPASSED: full339-variable roots,all requested prime/odd-extension/binary/Koblitzhardcoverage,20preregistered hard matching inputs,genuineF4/F5/F6,qualifiedisolation/A/A and independent whole-input completeness are still required. The earlier prime target-cache measurements remain inC95 and are unaffected.

Replay: PYTHONDONTWRITEBYTECODE=1 python research/autolab_cycle_20261116_v96/REPLAY.py
