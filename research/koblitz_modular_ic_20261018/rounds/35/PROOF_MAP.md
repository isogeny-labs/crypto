# R35 proof and computation boundaries

ClosedPointAlgebra35.lean has four named lemmas:
- frobenius_1024_of_32: c32=c impliesc1024=c.
- coefficient_lift: inacharacteristic2commutativering, c32=c andz1024=0
  imply(c+z)1024=c.
- nilpotent_precision: z154=0 impliesz1024=0.
- coefficient_lift_precision154 combines them.

These explain why the native1024power extracts the coefficientconstant in
F32[z]/z154. Hypotheses are explicit. Nativearrays,actualcoefficientembedding,
minimalpolynomial,basisranks andinversecolumns are exactCAScertificates,
notLeanimports. Acceptedcompile/axiomaudit receipts are indexedinresults.json;
onlyterminalauditexit0 proves thisworkflowgatepassed. Standardfoundations
propext,Classical.choice,Quot.sound allowed; no trustedcompiler/sorryaxioms.

The R35complete-space interpretation additionally uses R33literalfunctions/
polebounds andR34proper/local/normalizationbridge. These arenotformalizedhere.
R36's nativeclassoperations have explicitkernel/ranknullity/productchecks,
notLeanformalization ofalgorithmorcurvetheory. No nativecounter is IC rank.
