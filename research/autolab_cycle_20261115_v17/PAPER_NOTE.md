# Exact coefficient-fiber compression for five-source binary norm charts

**Preparation research note, not a generalized all-roots speedup paper.**

**Lemma.** In a characteristic-two affine quintic family with fixed first elementary coefficient a, fixing the second coefficient b makes the Gold3/Gold5 Newton-image constraints affine overF2. Their complete affine solution reduces the Gold9 condition to a quadratic Boolean system. Proof: use g3=ab+c, g5=b(a3+g3)+ca2+da+e and g9=g3^3+(a4+b2+d)g5+b(d(a3+g3)+ea2); squares areF2linear and b isconstant. Kernel parameterization and complete affine Gaussian solving preserve every original family coordinate, including all repeated-root cases; no division by a derivative or squarefree assumption occurs.

**Certificate.** If1 is a Boolean polynomial combination of the resulting necessary coordinate equations, the whole coefficient fiber contains no supported quintic. Degree-bounded Macaulay/Nullstellensatz coefficient elimination is existing machinery; a certificate is an explicit identity, not a novelty assertion or speed ratio.

**Evidence.** On eight exposedGF(2^83)cells,69parameters split34outer/35inner; linear constraints leave12bits. Each4096point conditional table is empty, and each has a degree3identity. Full enumeration and certificate workers respectively took0.548s/0.559s and25.7/27.5MiB peakRSS. These single traces establish conditional cell rejection, not comparative acceleration. Only8of2^34outercells were tested.

**Open problem.** Derive tractable guarded outer-cell certificates and full source/auxiliary/ghost coverage, establish primary-source novelty of any claimed new composition, then pass the20hardmatching-instance all-roots timing/memory gate across all requested field families.
