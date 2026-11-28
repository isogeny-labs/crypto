# Canonical small-order root replacement qualification

Standard finite-group engineering, not a novel IC method. For primeq,n|(q-1),
w=a^((q-1)/n) has order dividingn; testing w^(n/l)!=1forallprime l|n proves
exactordern. If w is primitive, its powers with gcd(e,n)=1are exactly all
primitive nroots, so their numeric minimum equals old ascending search result.
Existing fullsetup already finds generator g and computes w=g^((q-1)/n);
reuse that w and choose canonical minimum without the old linear search.
Standalone geometricsetup tries a=2,3,..., retaining every trial. No expected
constant trialbound is claimed for deterministicascendinga.

Exhaustively compare standalone helper to old reference on everyprimeq<=1000,
orders4/5/7/9dividingq-1. Prime qualification via exact trialdivision outside
field counts. Then primitive-stage count comparison on everyeligible order
forfixedq101,113,251,421,601,1009,4001,10009,allcheckedprime. Two repeats,
alternating olddirectsearch/powermap, exactroot/countrepeat/REDC identity.

Separate fullsetup comparison on (q,c,g):101/2/2,251/18/2,601/7/2,
113/3/3,421/2/3,19/2/4,37/2/4,73/2/4. Compare complete(FB,z,character)
tuple to unchanged oldhelper, not just root. Newcopy changes only zdiscovery,
reuses existinggenerator,w; allotherloopsidentical. Same two repeats/order
and exact count identity. G4rows are setup qualification, not prime-order
Jacobian, full DLP or genericcurvecoverage. Order4primitive test likewise
does not qualify a Gaussian C4solver integration.

Oldsources/importfreezes preserved. Newhelper must later be applied to both
candidate and control for fair pipeline costs. Rootstage gain cannot be
promoted to fullsetup/fullsolve10xgain; noCPU/wall or noveltyclaim. KBsearch
forfinitefield power-root construction returned no useful priority evidence;
the displayed cyclic-group proof is elementary, not a search-absence claim.

Freeze source/protocol/native/import/binary hashes,push before one120CPU-
second invocation. Git raw/exit/hashes/receipt,no overwrite/scientificretry,
S3/cloud. Retain any regressions or equality failures.
