#set page(paper:"a4",margin:18mm)
#set text(size:11pt)
= Fused companion validation: launch observation
2026-11-01T22:52:05.867104+00:00 · Live process snapshot; not a terminal report

The single frozen invocation is running with its original 3600-second cap.
Producer PID 2245648 and follower PID 2245741
were verified against their process start ticks at this observation. The
follower never launches trials; it runs analysis and export checking only
after producer exit zero. No restart occurred.

#image("coverage.svg",width:100%)

*Completed prefix.* 18 context rows, 2250 product calls,
9000 rejected output/count/trace/sign mutations,
6750 equal-wrap rejections and
0 radix-guard rejections. These counts describe
only complete rows at this timestamp. Later contexts, carry cohort and A/A
are not inferred from this prefix.

*Construction under test.* Deferred-reduction companion application retains
positive packed coefficients through convolution, extraction, wrap, sum and
inverse scaling, decoding and reducing the final n coefficients. The radix
bound, every carry-fixture intermediate, archived outputs and independent
native bit-volume accounting are checked under the frozen contract.

*Full declared cohort.* All369 archived contexts per arm, plus480 carry
contexts per arm: 1698 context rows and34248 product calls. Every archived
full, inverse and principal-block product is consumed exactly once per arm.
The archive supplies a dense-verified correctness baseline; no timing
comparison is made. Field counts, native arithmetic/bit volumes and bytes
remain separate.

*Predata evidence.* Commit3e77b2c; protocol hash
21e18b0471cccf698b0ffc2f82e02fe3e393f86004813af3d41a145aa1a0c843.
The schedule preflight matched all1698cells before data. Ten source files
and two public archives are hash-locked. Source and analyzer were committed
before the first trial. The primitive actual-overflow certificate is separate
from rejecting a radix that lacks the proof bound.

*Custody.* Producer session 13479; follower session
28565. Native timeout deadline epoch
1793576987.8451536. Follow these handles; never rearm the timeout.
The original capped Schur panel remains unchanged. No external jobs changed.

*Snapshot integrity.* 112503540 complete-prefix bytes, SHA256
2234b227ee779199f64b3dcf614f642b2882136128a7dd6f8c81603c9a0c4c37. snapshot.json retains exact fields. Raw output is mutable
while the producer runs and is not sealed by this launch report. Final
rendering is prepared but requires successful producer, analyzer, export and
follower terminal receipts. Freeze and launch records remain independent of
any later result.
