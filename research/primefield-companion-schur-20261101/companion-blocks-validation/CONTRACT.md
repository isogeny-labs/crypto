# Full arbitrary-dimension companion block interface panel

The immutable plan.json lists369contexts perarm:81retained public generator
contexts and288synthetic contexts. Bothcandidate andtuple-vectorAA use exactly
that order and the full expanded369entry seedtable; it repeats once perarm.
Predata seed-table verification must check every export cell against plan.json.
No producer invocation is permitted before that check and the predata commit.

Sourcecontexts retain all81candidate records from the earlier generator panel.
Each tests full multiplication in bothdirections onzero/ones/seeded vectors,
everybasis vector fordimension<=19,otherwise bothendpoints. Fourprincipal cuts
{1,2,floor(r/2),r-1}, fournewwraps(2,1),(1,2),(0,1),(1,0): all16displacement
identities are checked entrywise, withchangedboundary controls. Their products
are checked on everybasis vector forsmall dimensions and aseededvector otherwise.
Boundary acquisition uses theproductionoperator; all64requests,10queries,54hits
and10rretainedfieldslots are charged once percontext. Thedensechecker separately
verifies allcachedvectors. Correction costs are separate fromacquisition costs.

Synthetic contexts crossr=7,13,19,43; n=1,2,3,r-1,r,r+1; fourwraps above;
unituppertriangular,rankdeficientdiagonal,andzeromatrices. Dense displacement
G withK=I is areference input; allbasis/zero/ones/seeded orendpoints aretested.
Foruppertriangular matrices, independentlyconstructan inverse,verifybothinverse
identities,thenuseitonlyasthereferenceoracles forinverse-generator transport.
Checkthe swapped-wrapidentity, changedgenerator control andbothinverseproducts.
Singularcases carryanonzeronullwitness andtheoraclefailuremustpropagate.
Foralln>1,checkthehalf-sizeprincipalblockwithswappednewwraps,allfourcharged
boundaryqueries,corruptboundarycontrolandbothproductdirections. No fastinverse
orfastlinearsolver is implementedorclaimed by theseoracletransport checks.

Everyproductoutput andfieldcount has acorruptioncontrol. Everycontext rejects
threeequal-wrapinputs; sourcecontextsalsorejectequalprincipalwraps. Native
operanddescriptors are retained and bounded; fixedcountformulas areindependently
checked. Allprimitiveproducts reuse thepreviouslychecked packingkernel; no
newprimitiveaccuracyexperiment isclaimed. Allcasecounts arefixed in
expected-summary.json byenumeratingthisplan beforedata. Fullrows includeall
vectors,results,generators,querycounts,blocks,controlsandtimestamps.

One900secondinvocation,no restart. Allrows/failuresretained. Publicinputs only;
primefieldarithmetic throughout; no excludedECDLPmethods orsource substitution.
No timing/RAMcomparison. No completecompilerrecoveryornewscalarrecovery.
Allsource/checker/analyzer/contracts/inputhashes/plans/seeds frozen andcommitted
beforedata. Localcommitsonly,nopush/PR. Previousseed-exportfailure staysimmutable.
