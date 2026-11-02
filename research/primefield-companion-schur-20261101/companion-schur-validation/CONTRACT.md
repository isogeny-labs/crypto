# Complete Schur construction and obstruction panel

321fixedcontexts perarm,642rows:all81retained source generatorcontexts with
balancedcut floor(r/2),plus240synthetic cases. Syntheticcross primes7,13,19,43;
full dimensions2,3,4,7,12; allunique cuts{1,floor(N/2),N-1}; fivekindsregular,
zeroSchur,singularSchur,invertible-full/permuted-leading-block,andzeromatrix.
Distinctwraps rotate deterministically among(2,1),(1,2),(0,1),(1,0) inplan.json.
AAchanges queryinput listtotuple only. The321explicitseeds areexpanded inplan
order andrepeatedperarm; auditall642executioncells beforedata.

Independent exactleading-block elimination returnsan inverseverifiedbothways
or anonzeronullcertificate. Everycase isretained. IfEisinvertible, checkfull
Schurdisplacement entrywise,widths+2,generatorconstructioncounts,sourcecalls,
leadingoraclecalls,reference-dense terms and reductions. Referenceinverse
oracles haveexplicit workcounts; thispaneldoesnotimplementfastEsolving.
Testzero/ones/seeded andallbasis vectors form<=19,otherwisebothendpoints,
inbothdirections throughbothSchur.apply andgeneratedcompanionoperators.
Alloutputs comparedto independentlyformed denseS. Controls: appendanonzero
unitouterproducttogenerators,changegeneratorcallcount,changeeachqueryoutput
andfieldcount. Rejectequalnewwraps oncepercase.

IfEissingular,bothgenerationandapplicationmustraiseLeadingBlockObstruction,
withfullychargedpartialwork; neverassertfullAsingular. Verifythenonzeroleading
nullvectorandrejectthezero-witnesscontrol. Permutationcontrolsindependently
verifyA^2=I despiteleadingobstruction. Zeromatrixcontrolsretainfullsingularity.
Allsynthetictypes havefixedbranchoutcomes:288valid,192obstructedoverbotharms,
including96invertible-fullobstructions. Sourcestatuses areobserved,notselected;
validplusobstructedmustaccountforall162sourcecontexts.

Onefresh3600secondinvocation,no restart. ThisisthenewSchurconstructionpanel;
previouscompletedpanelsandtheirbudgets/failedexportsremainunchanged. Source,
checker,reference,analyzer,contract,plan,inputhashes andseeds frozenbeforedata.
Allrows,failures,partialcosts andnativeblockdescriptorsretained. Publicinputs,
primefields only; noexcludedECDLPmethod,extensionfieldsource,secretlabels,
fastinverseclaim,completecompilertrial,timing/RAMcomparison,pushorPR.
