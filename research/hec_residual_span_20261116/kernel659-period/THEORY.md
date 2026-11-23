# Symplectic normalization, complex periods and claim boundary

[pcc:unverified] Prior source/quotient module interpretation still uses its stated cyclotomic, canonical-lift and polarization premises.

An exact integral unimodular alternating Gram admits a unimodular congruence to standard symplectic pairs. Euclidean column operations and their transpose row operations reduce one pairing to1 and split its orthogonal complement. Each elementary2x2operation has unitdeterminant; exactfinalmatrix identity verifies the result without trusting the algorithm narrative.

For a symplectic lattice basis interleaved(a1,b1,a2,b2,a3,b3), evaluate the known powerbasis at the3CMembeddings toformcomplexcolumns. Put A=(a1,a2,a3), B=(b1,b2,b3), tau=A^-1 B. Riemannrelations imply symmetry and positiveimaginarypart. Algebraic unitrootballs andmatrixballs enclose these values. No f64 estimatesor tolerancezero tests replace these conditions.

The consumer checks symmetries/positiveCholesky, computesanexactsymplecticSiegelreduction and evaluatesalltheta characteristics withFLINT. Theta zeros arepreserved under symplecticchange, buttheirlabels change; evaluating onoriginaltau avoids comparing differentlylabelledreducedvalues acrossprecisions. A ball excluding0 provesnonvanishing for thatcomplexvalue subjecttotheexactperiodidentification/library correctness. A ballcontaining0 isunresolved, notproofzero.

For genus3 thehyperelliptic theta-nullcriterion motivates thetest: a smooth hyperellipticJacobian has a vanishingeven theta constant; thereare36evencharacteristics. Nonvanishingofall36therefore rulesouthyperellipticity of the complex principallypolarized object underthatcriterion. It doesnot itself provide a planequartic equation, prove finitefieldreductionproperties or measure anattack. Decomposability/Schottky/CMidentification arguments mustremainexplicitif claimingtheobjectisaJacobian.

Most importantly, nonzeroalgebraicmodularvalues incharacteristic0 canreduce tozero at421. A complexnonhyperelliptic canonical lift maythereforereduce toahyperelliptic variety. No complexresultis silentlypromotedtoF421, and aperiodmatrixisnota targettransportmap.

Primarysources: Marseglia, https://arxiv.org/pdf/1805.10223 section8; Balakrishnan,Ionica,Lauter,Vincent, Constructing genus3hyperellipticJacobianswithCM, https://www.uvm.edu/~cvincen1/files/preprints/vincent7.pdf vanishingcriterion/uniqueevennull discussion; officialFLINT https://flintlib.org/doc/acb_theta/index.html describesrigorouserrorbounds/charencoding. Webdocsare3.7dev whereasinstalledheaders/librariesare3.6.0;actualcompiledAPIreceipts/hashesenclosed. This isknownnumericalalgebraicgeometry, no novelty or speedclaim.
