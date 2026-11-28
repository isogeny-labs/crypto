# R90 bound-61 transport: one column checked, vector-dimension failure

The frozen R90 source was pushed at `156f7cbe80488153878fb2a96f711df18faedcf2`
before its one attempt, `transport90-01`. In the forward direction, column 1
first reached polynomial R50 module coordinates at overlap power **61**.
The script checked its 32-coordinate reconstruction, the complete
`B*C=F*W` product, and one corrupted-coordinate rejection before printing
the column line. This is a checked prefix of a rejected attempt, not a
64-column transport certificate.

On forward column 2, Singular reported `matrix size not compatible
(32x32, 31x1)` at `S90*matrix(powerVector90)`. A trailing zero in the
32-component power vector was trimmed by dynamic `matrix(vector)`
conversion. The procedure then emitted `RR90_FATAL` and stopped before
the completion marker. The driver rejected the attempt (`native=0`),
and there is no reverse-direction output. The exported `transport90.sing`
contains only forward column 1. The next source revision must construct an
explicit 32-by-1 power matrix before multiplying by `S` and `F`.

The 18 raw members were uploaded as deterministic archive SHA-256
`e0f9c30552acfe5534b65352d71970515f7eb46b1a47cbc2fc33244ae0239932`
to `s3://setec-astronomy-artifacts/store/sha256/e0/e0f9c30552acfe5534b65352d71970515f7eb46b1a47cbc2fc33244ae0239932`.
Checksum-protected PUT, fresh GET, archive digest and all member hashes
passed. The source, log, partial export, and driver are retained in that
archive. No mutation of the R90 protocol or raw attempt is needed.
