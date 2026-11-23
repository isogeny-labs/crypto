# Genuine genus-two J[8] and Frobenius: first complete native qualification

Unsigned claim PCC-20261123-5bd9de records native profile D at source
0eac0c327a2e4446a988bf59885d839a590694a6. All strict formatting, release
compilation, Clippy, actual-coordinate and source/tool/qualifier guards pass
under unchanged2GiB/120s/one-worker caps with overflow/debug assertions.
Earlier A format1, B compile1 and C inherited-type Clippy1 remain intact.
[pcc:unverified]

The actual curve is the predetermined census fixture
C/F3:y^2=x^6+x^5+x^2+1, with its canonical genus-two Jacobian. The computation
constructs F3^12 using X^12+X^2+2 and primitive encoded element14. All6 roots
lie in the constructed F27. Four order8 divisor classes are selected after14
fixed-seed draws; their quadruples span precisely the16 genuine J[2] classes.
Every one of the4096 combinations has a distinct valid canonical reduced
Mumford representative. [pcc:unverified]

In that actual basis, the Frobenius matrix on J[8], acting on columns, is

```text
5 3 3 5
2 4 6 3
5 7 4 2
4 5 0 1
```

All4096 actual chart-Frobenius images match this matrix and are distinct.
All16384 basis-generator edges satisfy addition and Frobenius equivariance.
The actual divisor computation checks P(F)=F^4+2F^3+3F^2+6F+9=0 and F^12=1
on the basis. Fixed counts at powers1,3,6,12 are1,16,256,4096, agreeing with
the independently computed curve/Weil extension orders. [pcc:unverified]

The selected chart has r=14013,r^3-r=1, pole code2 and monic scaling s=7371.
Omitting the moving-basepoint term gives a different valid divisor, so a
validity-only check would miss the error. The explicit copied-rule mutant
fails against cubing original branch roots; the corrected rule passes all15
branch-pair images, both chart poles and all sampled original points.
[pcc:unverified]

The genuine mod2 pairing on the quadrupled basis, from branch-pair masks
3,20,5,24, is nondegenerate and Frobenius-invariant:

```text
0 0 1 0
0 0 1 1
1 1 0 0
0 1 0 0
```

This is **not** the actual mod8 Weil pairing. A compatible alternating matrix
lift must not be substituted for a computed geometric pairing. Actual mod8
pairing, isotropic mod8 kernels, explicit elliptic/chain morphisms and their
prime-field descent/cost remain open. No complete ECDLP speedup is claimed.
[pcc:unverified]

Whole public native stdout is2753B,
SHA256d68b2673acd34c1593cdca9a1089d1a83ec818e387b399c40d6e5f8f3465bb7f,
including every draw, field/chart,15 branch pairs,4 basis classes, matrix,
pairing and coverage results. The whole185920B coordinate TSV containing all
4096 Mumford pairs and actual images is retained at
/dev/shm/ct23-jacobian8-first-D/coordinates.tsv,
SHA256ac9671d926c8dd1a0ed9870d72673859e7a62886203ad4b551c666fe0530c6ae.
Its remote custody is pending, as are whole4730376B ELF copies with
SHA256a591eadac60956e222e22baca9bb489fad90344a7b35c05bde1beb15d55bf027.
The private paths are temporary; no remote mirror, external replay, Lean
proof, signed verdict or hermetic execution proof is asserted. [pcc:unverified]

Next unblocked work is the actual rational Richelot codomain/map boundary
for this curve, reusing audited field/Cantor inputs without rerunning this
unchanged coordinate population. The absent ordinary nonsplit minimal fixture
from the F3 census remains a negative result. [pcc:unverified]
