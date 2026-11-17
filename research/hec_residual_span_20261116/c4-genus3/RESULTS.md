# Genus3 Gaussian reconstruction: amended coefficient gate passes

Original gate92bd5d0ea64 exits101on its first model. It incorrectly restricts fixed-point y to Fp², although y^p=-i*y gives y^(p²)=-y. Diagnostic a1eeaf8593b identifies the too-small field. Original failure/source are retained. Explicit prospective amendment74cbe3a5458 tests h^((p-1)/2)=-i for h=y² in Fp², without restricting its roots y to that field. No original output is replaced. [pcc:unverified]

Amended once-only run exits0: all6fixed models are smooth; all12orientations pass with exact repeated tuples. Each trace-plus-N candidate set has1element and its polynomial norm equals the independently counted full degree6Weil polynomial. Higher extension counts are used only for qualification and supplied N, not passed as coefficients to candidate enumeration. These are local coefficient checks, not independent certificates.

|p|a5|C1|C2|C3|N|candidates per orientation|
|---|---:|---:|---:|---:|---:|---:|
|13|0|20|112|2180|2920|1|
|13|1|10|168|2098|1576|1|
|17|0|24|260|4920|6656|1|
|17|1|18|266|4800|4660|1|
|29|0|34|800|24106|27200|1|
|29|1|26|818|24170|20866|1|

## Algebraic consequence and scope

For odd degree7 f, sigma(x,y)=(-x,i*y) acts on regular differentials with multiplicities2+1. The Gaussian weight-one factors have rank3. The determinant proof in ../c4-gaussian/DETERMINANT-PROOF.md adapts to Lie dimensions2and1, giving the two split-prime determinant valuations{2,1}. Hence d=p*eta with eta a Gaussian integer of normp (8possibilities including units/conjugation). This does not require ordinarity. Which prime has which valuation depends on conventions; enumerating both avoids an unsupported orientation choice.

Write G=T³+a*T²+b*T+d. Weil reciprocity within the Gaussian realization gives b=(d/p)*bar(a). Ordinary and sigma*pi traces give a; the exact group order imposes norm(G(1))=N. Thus at most8candidate factors require checking. This supplies a finite reconstruction method on the stated C4class; the observed uniqueness is not a proof of universal uniqueness. Opposite finite-field i choices give conjugate output factors on all6models.

No scalar-action operator identity was replay-tested here, no prime-order source/target admitted, no absolute-simplicity claim, and no end-to-end or runtime speedup measured. The extension field arithmetic/counts are standalone, but v2's fixed-point exponent criterion shares the elementary earlier Gaussian trace method; it is not independent Fp4point enumeration. The paper-level determinant derivation still invokes standard realization compatibility and needs independent review. No priority claim.

The retained binomial genus3controls are y²=x7+c, which are not this odd-polynomial C4family. They cannot be replaced to manufacture same-target improvement. Genus2C4geometric splitting remains a separate restriction. This result expands the special-family reconstruction argument; it does not satisfy the mathematical10xgenus2+objective.

Raw failures,diagnostic,amendment,successful data,run commits,exit statuses and source/binary hashes are retained in Git under S3outage direction. No S3polling,paidresource,oldproducer rerun or silent protocol change.
