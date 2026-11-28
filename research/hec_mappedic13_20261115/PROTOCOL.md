# Frozen mapped plane-quartic IC correctness gate

Source exactly previous F127 genus3 fixture; D1 from fixture and B twice the
second canonical rational place, here(3,21). No scalar relative to D1 is given
to the mapper or destination solver. Same kernel certificate, alpha values,
Q1 and fixed differential lambda10. Re-execute both public input maps with
original8192trial limits. Do not load their previous output encodings.

Enumerate all16257 projective F127 lines in fixed normalized order [1,a,b],
[0,1,b],[0,0,1]. Keep lines meeting the129point list in four distinct points;
Bezout certifies their full canonical hyperplane divisor. First such line H
is reference. Each row is sum(points on line)-sum(points on H) mod ell1028893.
Anchor first point log0; eliminate128columns and require rank127/nullity1.
Retain all row residual checks, all129kernel coefficients and rank/counts.
A deliberately changed coefficient at a nonzero kernel coordinate must fail.
Verify first8split-line principal-divisor identities using actual target classes.
This is full-base line enumeration/dense elimination, not an optimized asymptotic
Diem implementation or a performance claim.

Descend each mapped class by adding multiples a=0..64 of the first nonzero-log
point difference. Extract a degree3rational divisor by testing canonical evaluation
rows against its3x10annihilator. Require three distinct points and exact class
reconstruction; other cases excluded and logged. Subtract base divisor logs and
known translate contribution. Require nonzero logs and recover k=log(B)/log(D1).
Verify k independently by scalar multiplication on destination and source.
Reject k+1 on both. Then run existing hyperelliptic IC on exactly the same
source pair (default parameters/seed20260916, max200000trials), require same k;
print only trials/relations, no timings. This is a different relation algorithm,
not an independent arithmetic implementation.

Commit source+lock+protocol and S3checkpoint before one release execution.
600second aggregate cap. Any rank/descent/map/native-control failure is terminal
and retained; no input substitution, adaptive patch or scientific rerun. Record
stdout/stderr, all exclusions, encodings and scalar. No closed-record/default/
leaderboard changes. Input kernel/curve construction was previously executed and
is loaded here; a later complete-cost evaluation must charge that setup, calibration,
all maps, relations, descent, linear algebra and verification. No10x claim.

Prior art: repo native tian_quartic_km.rs line_sieve_rank/line_sieve_descent,
sourcejet13/publicmap13 proofs and receipts; Tian arxiv:2007.03172 §1/4.2 and
its Diem references. Relations follow div(L/H) on a canonical plane quartic.
KB query 'plane quartic line sieve relation descent tian_quartic_km' revision
9141e386254340a9bdba3036f2bb706a returned older pipeline notes; these predate
this explicit transport and are not used as present validation. No novelty claim.
