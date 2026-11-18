# Two-leading-coefficient quadratic extraction on general odd-degree curves

Derived candidate [pcc:unverified]; not implemented/performance-tested in this gate, no noveltyclaim. Same valid monicdegreeg anchoru, degv<g, monicfdegree2g+1, vt=v+tu andH=(f-vt²)/u. Write u_i,v_i,f_i for coefficients. Coefficient comparison gives

    H_(g+1)=1,
    h=H_g=f_(2g)-t²-u_(g-1),
    j=H_(g-1)=f_(2g-1)-2t(v_(g-1)+t u_(g-1))-u_(g-1)h-u_(g-2).

Let known DISTINCT rootsx_i be bucket hits withu(x_i)!=0. Because they are actualcurvepoints withy=vt(x_i), H(x_i)=0, so K=product(X-x_i) dividesH. Define s1=sum x_i, s2=sum_(i<j)x_i x_j. For k=g-1knownroots, monic quotient degree2is

    Q=X²+A X+B, A=h+s1, B=j+A*s1-s2.

Proof: Kleadingterms X^k-s1X^(k-1)+s2X^(k-2); compare the two following leadingcoefficients inH=KQ. For k1interpret s2=0. If k=g, quotient isX+(h+s1); ifk=g+1quotient1. Alloperands overFp, signs asdisplayed. The coefficients h/jcome from exactnormidentity, not a truncated approximation. Computing fullH is unnecessary once K|H is established by the point/bucket premises.

Knownroots are distinct but may be repeatedroots ofH; remainingmultiplicity remains inQ and must be factored/aggregated. A residualroot can overlapanchoru; no quotient-by-u cancellation may be silentlyperformed. Currentfullnormcorrectnessgate exercises both phenomena. Distinct-hit eligibility can miss some repeated-smallhit relations; thisformula doesnot repair/exhaustthat sampler.

Prospective implementation should compare Q with full exactdivision on every queried bucket, then reproduce complete acceptedmultiplicity tapes, weightedLPstates, firstdeterminingoutputs andall22scalars. Diagnostic fullH/nativechecks outside any subsequentproductioncounterinterval; cheap bucketbinding/value extraction andsource/targettracking remainpaid. Genericfactorbase optimization ifused must be sharedwithallcontrols. No inferenceofconstanttotalquerycost, samplingrank, genus4speed orfull10xfrom thisidentity. Existing truncated-quotient/RESULTS.md and primaryknownshortenedsieve recordedinPENCIL-NEXT.md supply prior-art context.
