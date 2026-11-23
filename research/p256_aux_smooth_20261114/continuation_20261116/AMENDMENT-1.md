# Calculator compatibility amendment, before request002

Request001 is retained as a failed qualification: the public service
forbids SetNthreads(1), reports success=false, yet returns exit_code0 and
continues to the later probe. Strict response admission correctly rejects
it. Evidence: PCC-20261116-3df4fa, unsigned [pcc:unverified].

For development units on this endpoint only, remove exactly the single
SetNthreads(1) directive from each frozen exported program. No thread-count
or performance qualification follows. Preserve the frozen original files.
The standalone/EKS comparison protocol's thread/isolation requirements are
unchanged.

To remain within nine total requests, combine the corrected field probe
(without its final quit) with the dense m2/d1 program in request002. Require
both the probe success marker and the full relation solution block. The
other three frozen arms follow only after strict response admission and
exact native replay. Four optional preconditioner units remain within the
original total allowance. No automatic retries or larger resource caps.
