# Full original rational-group Kummer/lift gate

Prospective assertions [pcc:unverified]. Fixed original y²=x⁵+7 and recovered
candidate y²=495x⁵+539x⁴+47x³+378x²+127x+209 overF601. Read frozen standard
quintic vectors from coordinate-identity/CONTROL.txt; do not alter map.
Original rational order411491 prime, qualify primality by trialdivision
and source firstcanonical catalogue point D nonzero with[N]D=0. Lift its
Kummer image to one canonical candidate divisor D'; require nonzero,[N]D'=0.

Walk all411491scalarpositions k=0..N-1 using native group arithmetic on
both curves. Require source/candidate divisors valid; no earlyidentity;
nonzero map vector; map(K(kD))=K(kD') projectively at everyposition.
Independently lift everymapvector to a valid candidate Mumford divisor
and require it equals kD' or its negative. Includeidentity/degree1 and
repeated-support cases; reportcounts. Bothwalks returnidentity atN.
No input positions discarded, subsampled or new target substituted.

Lift formula for monic u=x²-sx+p: b1²=k4+f2+f3*s+f4*s²+f5*s(s²-p),
then use fmodu to recover b0; b1=0 has separate square-root case.
For degree1, k4=f5*x². GeneralKummer convention Eq2.2-2.4 in primary
Corte-RealSantos/Flynn paper as coordinate-identity protocol. Negation
ambiguity remains explicit. This is exhaustive finite rational-group
compatibility, not global geometric degree/basepoint proof, independent
verification or DLP performance. Scalar indices are qualification controls,
not unknown-target solutions or a benchmark. No secret target input.

Freeze source/import/native/priorcertificates/protocol/binary and push
Git before one120CPU-second invocation. Preserve failed/censored outcomes;
no scientific retry/S3/cloud. Next original-target IC transport/cost case.
