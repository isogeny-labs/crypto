# Ramification coverage diagnostic, frozen before execution

Original unit02 has three passing tests and one failed counter assertion,
not complete acceptance. All148/356/1324 source coordinates,complete source
generating edges592/1424/5296,all37/89/331 odd scalar images and controls
pass. The Fp6 scheme test matches all148 q5 inputs then fails ramified>0;
q7/q11 scheme loops were not reached. All807 input hashes remain unchanged.
Preserve this exact failed assertion and all output. [pcc:unverified]

Question:do the frozen source groups actually contain ramification images
of pi0? Observe ALL NINE old public cases,without dropping the primary3.
Use existing FpR3 with each exact old modulus,enumerate the two roots of u1
when split,and test whether u0(t)u2(t) has square roots in K. Record t,rhs,
its norm in Fq,Euler power and source-point counts. Irreducible cases may
have zero u1 roots. No predicted count is imposed;this is a complete finite
observation plan,not a new transfer/performance result.

Interpretation:if primary or additional cases have rational branch targets,
use them as ADDITIVE runtime controls while preserving all original points.
If ramification lies only over Fq6,record that domain gap and add full
geometric ramification controls before any completion claim. Do not merely
delete the positive-coverage assertion or call zero coverage complete. A
wrong counter or field interpretation must be corrected with evidence.
No change to the protocol,test assertion or runtime precedes this diagnosis.

Native Rust only,timeout120seconds,fresh diagnostic output boundary. This
post-failure correctness diagnostic may reuse original unit02's unchanged
native dependency cache;record all source hashes and the original cache
provenance. No timing ratio or cold-build claim;no paid experiment. Retain
complete command/source/output/originalexit/guards as a distinct record.
