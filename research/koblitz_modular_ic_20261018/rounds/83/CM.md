# R83: strict gate rejected a ring-scope error

The frozen direct-reduction run passed its Git source and nine donor
hash checks. Singular then reported that `module50` was undefined
after the script switched from the selected source ring to the
candidate normalization ring. It continued with a printed result and
terminal marker despite the native errors; the strict `?`-error gate
rejected the attempt. No valid fraction identity was checked. Its raw
log and checkpoint are retained.

A syntax-only two-ring Singular probe showed that strings made from
source polynomials remain accessible after the ring switch. A fresh
frozen run can serialize all 27 module entries to strings before
loading the candidate ring, then reconstruct their polynomial images
there. It must keep nonzero direct remainders epistemically
inconclusive, as in R83's protocol.
