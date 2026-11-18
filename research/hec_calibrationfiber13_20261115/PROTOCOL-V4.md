# Correct the erased-Hessian control cardinality

V3source1570b8f582e6/checkpoint224619 ran once, retained exit101. The SAME
restricted polynomial/roots/all124controls passed. Chord factoring gives
rationalhits(9,13),(89,44) at the first zero. Unique Hessian pair and BOTH
endpoint differential-scale checks passed, then the old erased-Hessian
negative control asserted six possible rational pairs. Two rational hits
have only one pair, not six. No final alignment check was reached in V3.

V4changes only this diagnostic cardinality to h*(h-1)/2. Log that erasing
Hessian is non-discriminating when h=2; do not call it a successful corruption
rejection in that case. Original final independent matrix-image check remains
mandatory. Same roots, inputs, charts, decoderselection and exactcontrols.
Retain RUN-V3.*; V4write-once RUN-V4.* after separate freeze/checkpoint/gate.
