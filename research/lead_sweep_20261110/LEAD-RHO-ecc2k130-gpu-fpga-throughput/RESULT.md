# RESULT: LEAD-RHO-ecc2k130-gpu-fpga-throughput

## Outcome
- status: ran; exit code 0; wall 4 s (run.status: exit=0 wall=4); stderr empty.

## Statistic vs preregistered prediction
- Prediction (PROTOCOL.md s3): 0.90 <= rho <= 1.10 and |d| <= 3 SE, A/A within 3 SE, popc2 mean > 1.15 x hash mean.
- Measured (N=2^20-3, 300 trials/arm, 16 classes):
  - hash mean 3739.8 (SE 473.7); hash_AA 3417.0 (SE 384.4); popc 1332.1 (SE 38.9); popc2 13820.9; low 44054.0.
  - rho = popc/hash = 0.356; d = -2407.7, diff SE 475.3 (|d| = 5.07 SE).
- Verdict: falsified (run.py field). Both conditions fail.
- Controls: A/A ok (aa_ok true); positive control ok (popc2 3.7x hash, low worse). Null control: hash arm ratio_to_sqrt(piN/2) = 2.91 (A/A arm 2.66), outside the protocol's expected 1.0-1.3.

## Reading
- Direction is the opposite of "degraded": popc is about 2.8x shorter than hash, and sits at 1.04 x the ideal random-mapping length.
- The "hash" reference arm is itself poor and heavy-tailed (SE about 10% of the mean versus about 3% for popc; ratio to ideal 2.7-2.9). That fails the protocol's null-control expectation, so the pairing is against a weak reference, not a good hash. Likely a toy multiplicative hash with weak index-bit mixing; I did not inspect or alter run.py beyond running it.
- The run.py verdict matches the numbers. The falsification is formal (popc differs from hash beyond noise), but it says more about the hash arm's quality than about popc selection.
- The toy is Z/N additive walk, not the ECC2K-130 Koblitz group with Frobenius canonicalisation, and says nothing about GPU/FPGA throughput or the 15.88 baseline.
- Single run, no replication; the A/A pair differ by 323 (0.5 SE-diff), fine.

## Bearing on the sub-sqrt(p) question
none: this is a constant-factor walk-selection quality check in a toy group; no sub-sqrt(p) mechanism is touched.

## Standing
[pcc:unverified], single local toy run, no independent replay.
