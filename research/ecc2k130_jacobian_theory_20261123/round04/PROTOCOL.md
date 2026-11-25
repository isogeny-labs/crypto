# Explicit integral-basis certificate and bounded successor

2026-11-23. Continue the same theory goal and draft PR5079. No curve,
Jacobian realization or executable DLP correspondence has been found.

The previous bnfinit(H,1) timed out before returning. Do not repeat it.
Construct the explicit basis 1,s1,...,s64,u,u*s1,...,u*s64, with
sj=zeta^j+zeta^-j. Verify its identities in Q[u]/H and compute its trace
Gram determinant independently with native GMP. The expected discriminant
is 7^65*131^129. Establish maximality in the written argument before
passing the basis to PARI as known maximal. [pcc:unverified]

First check PARI's initialized-field bnfinit interface on Q(sqrt21).
Then retain a bounded successor using the explicit basis. Each bounded
fixed-input certificate process has a30-second wall cap,512MiB PARI
stack ceiling and1GiB process cap. No new paid resource, throughput
measurement or larger class-group experiment is authorized by this note.
Retain timeouts and all errors; they imply no principality result.

The specified ideal is p263=(263,u+1) in the maximal real field. A positive
result needs exact ideal equality and total positivity. A negative class
result needs unconditional certification and the correct narrow-class
interpretation. Even a totally positive generator leaves gluing,
Jacobian realization and efficient correspondence open. [pcc:unverified]

KB unavailable: the maintained wrapper has no executable and no KB MCP
is loaded. Official PARI nfinit source documentation accepts[pol,B,P];
buch2.c Buchall_param calls get_nfpol(P,&nf) and reuses initialized nf.
See https://pari.math.u-bordeaux.fr/dochtml/ref/General_number_fields.html.

Follow-up after explicit-basis initialization succeeds but the bounded
degree130 bnfinit again does not return: test necessary norm conditions
in the unique real subfields of degrees2,5,10,13,26. They are selected
from polsubcyclo(917,d) by real signature. The even cyclotomic quotient
is cyclic of order390, so each is contained in the degree130 field.
Prime263 splits completely there. A totally positive generator upstairs
would norm to one below; any certified nonzero narrow class below would
exclude this candidate. Passing these smaller tests does not lift a
generator. Retain full polynomials, norms, ordinary/narrow class vectors
and bnfcertify status. Use the same bounded fixed-input limits.
