# Next exact AS stratification

For h!=0 in the certified centroid domain, put a=h^2+h+z and C2=h^2 A+za. The next coefficient requires r^2+hr+C2 in W2. Normalizing by h^2 gives AS(r/h) in (W2+C2)/h^2. Its trace right-hand side is independent of h:

Tr(C2/h^2)=Tr(A+z+z/h+(z/h)^2)=Tr(A+z).

Let mu be the trace functional on W2/h^2. If rank(mu)=1, there are exactly2^dim(W2)r solutions. If mu=0, trace(A+z)=1rejects that whole h-chart, while trace(A+z)=0gives2^(dim(W2)+1)solutions and must retain the extra branch. For h=0use the separate bijective square map. ActualA0andtarget tracez0give constant RHSzero, so degeneration would enlarge fibers rather than reject them.

Fresh falsifiable census: prove/check the trace identity with complete tiny r,h,z oracles, then traverse all262144admissible actualhvalues using the affine basis, compute inverseh and trace masks and count degenerate charts. Predict none; retain witnesses if refuted. Compact full product/inverse-sum identities independently checksum the traversal. Stream values, cap90seconds/4GiB/onecore, and demote incomplete counts. This is at most2^18h preprocessing, not a2^73norm-candidate search or original-root solution. Precompute trace masks so each functional test is bit parity, not repeated83-step traces. Do not assume inversion costs or a speed gain without calibration.
