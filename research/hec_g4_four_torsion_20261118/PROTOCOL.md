# Frozen original genus-four Mumford half gate [pcc:unverified]

OriginalcurveF31:y²=x9+x3+16x+1. Bind the qualified branchlabels/field from hec_g4_theta_admission/CONTROL.txt withoutreordering. BuildF31⁸asF31[z]/q4(z²-c), where c is thefirst0..30withalpha+c nonsquareinF31⁴. Proveirreducibility byexactFrobenius/gcd andverifyembeddingalpha=z²-c/alloriginalroots/product. No assumed primitivegenerator; explicitlycheckz nonsquarebeforeTonelli-Shanks.

For eachof9finiteWeierstrasspointsW, takecanonicallexicographicsquare roots of allnonzero branchdifferences and enumerateALL256independentsignchoices. ZarhinExample3.7constructs(U,V). Require monicdegree4,degV<4,U|(F-V²), noWeierstrasssupport, exactprincipalidentity F-(V+e1U)²=(X-a)U², independentgeneralCantordoubling2H=W, and4H=0. Everybranchmusthave256distinctMumfordpairs in an exactset, nothash-onlydeduplication. Retain signmask/digestforeveryhalf andfullcoordinatesformask0. Checknegationbelongs tothe256set. MutateVconstantby+1andretainnumberinvalid; requireatleastoneinvalidmutationoverall,notallmutationsclaimedinvalid.

Constructkernel0halvesofbasis1,10,20,160frommask0Weierstrasshalves. Requireeachdoubles toitsoriginalcoordinate2class. Enumerateall256Z/4linearcombinations; require256distinctvalidclasses anddoublesexactlythe16kernelclasses. This DOESNOTcheck4Weilpairing,isotropy,theta structure,affineliftsorquotientSchottky. Noisogenyadmissionfromdoublingalone.

Fieldsource is a verbatimarithmeticcopyofpriornativegate withvisibilityexports; Cantor is a translationofqualified nativejac.rs toextensionfieldpolynomialvectors, separatefromtheclosedhalfformula. Bothsharefieldarithmetic,notanindependentfieldimplementation. NoMagma/Sage/Pythonexecution. NoDLP/timingmeasurement orspeedclaim.

CompilewithRustoverflowchecks. Pushsource/protocol/derivation/input/import/binaryhashesbeforeone600CPUprocess. Retainrawstdout/stderr,exit,executioncommit,hashreceipts andfailures; no unchangedrerun. GitcustodywhileS3down; noS3/cloud/merge. Thiscontinuesthefixedgeometricroute,notafullsolvercampaign. Independentreviewandfull10xgoalremainopen.
