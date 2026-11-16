# Bounded inverse solve and exact parametrization verification

G1source6221010218c0/headc3268afa412f/checkpoint214444 passed once; original
systems and raw output immutable. New collector/verifier source freezes before
any solver cell. Same equations and publictarget, no expected divisor/scalar.

Private msolve0.11/FLINT3.5fingerprints checked immediately before launch.
Flags -t2 -v1 -c0 with explicit fixed parameter t in generated equations.
One cell positive then one target ONLY after positive collector and independent
verifier succeed. No retries or genericity reselection. Maximum300seconds and
2097152KiBRSS per cell; retain censored/failure output and status. Source and
checkpointgate before each launch. Timing/RSS here are resource receipts, not
an admitted comparison or speed claim. No other scientific cell launched.

verify.rs parses bracketed prime-field parametrization, requires field127,
8unchanged variable names, quotient degree6or12, squarefree elimination and
invertible denominator. Normalize the two possible numerator sign conventions
by testing exact modular substitution; exactlyone must satisfy all12original
input equations identically modulo the elimination polynomial. These are
independent operations on emitted input text, not a solver exit trust check.
Known positive point must be recovered at its fixed primitive parameter value.
A field root or finite ideal alone does not pass.

Actual class reconstruction is still required after target parametrization.
It must consider finite-algebra factors without field/group enumeration,
reconstruct a closed divisor/class and compare its projective coordinates.
A separately frozen recovery verifier may follow; do not call parametrization
checking a completed Jacobian map.

PUBLIC-NEXT-KUMMER.txt is M*source(B+2D1), exported from existing atlasholdout,
for orientation after lifting B+D1. It is a second public coordinate input,
not an expected divisor or DLP scalar. If candidateX is +/-Y and G is known
nonzero odd-prime-subgroup generator image, exactlyone of kappa(X+G) and
kappa(-X+G) can match kappa(Y+G) when Y is nonzero. Otherwise wrongsign gives
-Y+G=+/-(Y+G), forcing2Y=0or2G=0. This elementary sign argument must be
implemented/checked before subtracting G to recover B. Prior geometric Bimage
may verify the final answer but must not choose its sign.

No full10xclaim, cold-cost substitution or genus2/4closure. Preserve every
failure, positive/control/source provenance and candidate/chart exclusions.
