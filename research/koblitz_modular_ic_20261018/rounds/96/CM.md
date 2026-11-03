# R96 rejected marked-23 base discriminant attempt

The single frozen `discriminant96-01` attempt is **rejected**. Its source
was pushed at `919130a93` before execution. The strict driver recorded
native exit zero but found `?` errors beginning with
`deg(poly,poly) failed; expected deg(poly,intvec)` when the source used
`deg(f,y)` instead of an explicit weight vector. Singular later printed
the pass marker, which the driver correctly ignored. The partial export
suggests a degree-50 resultant and three factors, but no factorization
or branch-locus claim is accepted from R96.

The deterministic raw archive SHA-256 is
`214107596ce8aefbc34920e733f55316138e715d2d4167977c95cb7b47c2ff5f`.
Checksum-protected upload to `setec-astronomy-artifacts`, fresh GET,
byte comparison and all 16 member hashes passed. The raw native log,
partial export and source hashes remain inside the archive. A new frozen
round will replace every variable-form degree call with the documented
`deg(poly,intvec)` form and independently replay the result.

This was a calculation on R52's actual marked-23 degree-7 base order,
not the normalized degree-112 source. See R63 `CM.md` and the
[Singular manual](https://www.singular.uni-kl.de/index.php/singular.pdf).
