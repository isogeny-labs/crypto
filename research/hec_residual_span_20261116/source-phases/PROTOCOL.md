# Exact source phase-share and verification-floor audit

Prospective producer assertions [pcc:unverified]. Preserve root-pipeline
arm0 source/algorithm/inputs: same12fixedtargets atp101/251/601, sm1/17,
two cases RNG2026113000xor p, twoexactrepeats. Copybaseline.rs unchanged
except zeroarithmetic counter-snapshot taps. No algorithm or input change.
Require all16counterentries in everymodulus match prior raw vector exactly,
and exclusive phasevectors sum to total, including all invocations/restarts.

Phases: prepare (Fp construction/fullFB/root/character/orbitpartition),
source (publicsourceassert/knownsourceequations/initiallinearrows), anchor
(anchor groupupdate), scan (fullpointcatalogue/batchinversion/buckets),
linear (relationassembly/insert/query), verify (finaloriginal kD=T).
Flushlinear before eachnextanchor and beforefinalverification; do not
misattribute previousbucket processing to nextanchor update. Repeated
stageevents accumulate. No timing/counterweighting or new speed claim.

Derive basefield verification fraction and conditional ceiling
total_base/verify_base for changing EVERYTHING except that verification.
Similarly derive perphase perfect-removal ceilings; keep moduli separate.
If ceiling<10, a mechanism retaining that unchanged phase cannot meet a
10xfactor in that modulus on that fixedcase. This is conditional arithmetic,
not a bound on new verification formulas/othermodels/scaling or all IC.

Prior work: source52c9869df49ce86627da51bf4d120164 KN-OPEN-006 points to
structuredmatrix/harvest tradeoff; source7c0fd4de5428c388885c2f42be69cc92
EXP-ICEX-b84478 is a draft costshare proposal, not a measured result.
Use current frozen source evidence instead of importing its predictions.
Primary genericIC/isogeny context Smith intro previously checked;
https://arxiv.org/html/0806.2995 . No novelty statement about phase profiling.

Freeze copies/import/native/priorraw/protocol/binary hashes, pushGit before
one120CPU-second invocation. Preserve failures, no scientificretry/S3/cloud.
Next select mechanisms from actual phase costs, not hypothetical LA dominance.
