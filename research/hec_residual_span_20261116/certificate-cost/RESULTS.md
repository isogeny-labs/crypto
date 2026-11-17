# First scalar-certificate implementation cost

Producer assertions [pcc:unverified]. Freeze18d7f02a248 pushed before
single invocation, hashesmatch, exit0. All24calls/12fixedtargets complete;
certificateacceptance, sourcebasefieldreplay, exactphase sums/repeats pass.
Candidate=producer-nativeexternalcontrol+fullcertificateverifier; witness
construction counted, no freecertificate or removedverification.

|p|candidate/source basefield REDC|candidate/source subgroup REDC|
|---|---|---|
|101|2.233-2.502|5.105-6.268|
|251|3.900-6.158|17.707-22.831|
|601|10.164-11.362|28.502-42.298|

Everycase loses inbothmoduli. Retain firstcertificate as a soundness
prototype under explicitinstancepremises, not an acceleration. Example
p601,sm1,case0: oldbase13940, nativefinalcontrol4798, newverifier149250,
derivedcandidate158392. Verifier binding10296,anchors142,relations138812,
basefieldwitness0. Subgroupwitness/assembly costs retained separately.
The nativebinding checks alone exceed removedscalarcheck in thisexample;
perfectly optimizing relationlookup is insufficient byitself.

Next costjustified changes are two separate obligations: (1) preindex
verified orbitpoint/action weights once instead of repeated allorbit
search perusedpoint, and (2) replace nativegenerator/action scalarchecks
with an explicitly proved/checked action qualification, not merely trust
producerlambda. Possible mathematics: independently qualify pi=P(sigma),
then Bezout of Phi5 and P-1 on rational elltorsion yields sigma=lambda.
This is a proposed route requiring a sound Frobenius-identity premise and
fullcost, not an implemented verification or permission to omit checks.
Even a fastverifier leaves unchanged scan/prepare costs; the full10x
objective also needs a changedproducer. No generalimpossibility/novelty
claim or speedup is inferred from this fixedimplementation loss.
