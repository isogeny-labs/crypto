# C116: complete leading-count degree cover, not just the common-prefix branch

For five true prefix roots in L_dwith h4leading bit1, kupper-half roots is1,3or5. Old leading flags distinguish them: (u,v,w)=000for k1,110for k3,001for k5. Hence all three cases form a disjoint exhaustive cover of true prefix-supported roots; none may be discarded.

For coefficient h_jwith m=5-j, each product term selects at most min(m,k)upper-half roots, so deg h_j <=m*(d-2)+min(m,k). The bound remains valid under cancellation and repetitions. Translation by c=X^(d-1) switches the upper count to5-k; impose the analogous bounds on translated coefficients too. All are linear conditions on the original raw HC0+M*qand other coefficient bits, with the same global old q.

At d18, unshifted bounds h0..h4are:
 k1:[81,65,49,33,17];
 k3:[83,67,51,34,17];
 k5:[85,68,51,34,17].
Translated bounds use5-k(and the already known translated h4leading cancellation). The k5case reproduces C115bounds[80,64,48,32,16]. Split old chart0by u=0/1for k1/k3, retain old chart1for k5, and measure actual independent ranks. Do not infer them from degree counts.

Next experiments: independent exhaustive small-dmultisets verifying BOTHunshifted and shifted weighted bounds for every kstratum, including repeated/zero roots; then actual rank/maps for ALLthree strata with source/matrix/qconventions frozen.90s4096MiB1core, no dense enumeration. Advancement criterion: at least4total independent rows beyond the old67-dimensional chart for each previously uncontracted k1/k3stratum, with all three strata retained and complete recovery controls. If ranks disappoint, retain the negative result and derive a separately bounded coprime leading-factor lift, rather than repeat the57-bit timeout.

Further route: at normalized infinity the k1case has one simple high root and four low roots, so its high-root series can be uniquely lifted; k3uses coprime degree3/2factor lifting even with repetitions inside groups. Known power-series rootfinding does not automatically solve an unknown coefficient family or supply original-source/Ucompleteness. Prime-field intervals need separate carry/lift arguments; no generic50xpromise is made.
