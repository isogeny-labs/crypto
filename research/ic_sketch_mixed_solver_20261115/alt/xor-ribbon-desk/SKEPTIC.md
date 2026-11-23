# Skeptic review: xor-ribbon-desk
Reproduced: model.py rerun gives byte-identical raw_output.txt and results.json. Headline numbers (Kmax, +13..21%, kappa 1.34/1.44, cold 663->850-951 s, 284x->364-407x) match raw output and recompute by hand. Lever label memory_only is honest; no counted-GAE claim.
Issues:
1. README construction-memory figures at K4317 (17.9 GB ribbon, 15.7 GB XOR) disagree with raw_output section 7 (19.49 GB ribbon, 17.12 GB XOR). K3596 figures match. README typo, not in headline.
2. Baseline choice: the "measured frontier K=3596" is root4 with a pow2 50%-load table. root11 (dense 5 B slots, 41.06 bits/entry at K3550, 12.7 GiB) is a cheaper memory fix already in the repo and would also raise Kmax under 15.25 GiB; the +13..21% is gain over the weakest layout, not over the best existing one. Not modelled.
3. All above-K3596 online/cold values are extrapolated (K^-1.985, linear setup); kappa unmeasured; construction memory exceeds guard unsharded. All labelled in caveats.
