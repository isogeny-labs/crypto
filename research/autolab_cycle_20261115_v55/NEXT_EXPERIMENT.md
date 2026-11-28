Freeze a fresh fused affine-norm compiler and compare every coefficient column with the generic compiler before timing. For F=X^3+rX^2+sX+t, G=hX+k, t=z^3+rz^2+sz+(hz+k)y, vary r,s at fixed h,k. Since deltaF_r=r(X+z)^2 and deltaF_s=s(X+z), division of deltaN=deltaF^2+XG deltaF by X+z gives exact columns:

deltaH_r=[z^3r^2,z^2r^2+kzr,zr^2+(hz+k)r,r^2+hr,0,0]
deltaH_s=[zs^2,s^2+ks,hs,0,0,0].

Compute one base norm and83 columns for each parameter directly; reuse exact coefficient-span/constant-image elimination. No change to admissible fibers or original-chain output. Prediction: reduce approximately0.50s four-chart non-native norm cost; current original-output compressed median1.9186s needs below approximately1.8505s for8x against observed14.8036s, subject to fresh matched timings and A/A<=10%. This is diagnostic, not a release guarantee.

Next experiment90s/4GiB/onecore: compare every column/base/fiber on existing216 tiny systems and four positiveGF83 instances, including zero h/k branches supported by the generic contract. Only after exact equality, register a new cold comparison with complete original serialization/literal verification. Do not rerun unchanged C55 noisy timing. Keep genuineGB interface engineering and global original-to-norm coverage proof independent.
