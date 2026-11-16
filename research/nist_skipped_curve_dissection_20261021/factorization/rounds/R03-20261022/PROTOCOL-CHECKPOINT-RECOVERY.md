# Recover and validate interrupted native polynomial candidates

Recorded before recovery. R03's attempted `-R` continuation restored bytes
but restarted the polynomial search: the interrupted checkpoint had only
`nfs.dat.0.p`, whereas YAFU's resume path reads master `nfs.dat.p` with
an `n: <input>` header. The first complete candidate and leading-coefficient
start repeated the earlier run. Stop that production attempt as an
infrastructure failure; do not count it as new search coverage.

Recovery is a native exact-validation step, not another search allocation.
Preserve both raw per-thread files. For each complete degree-5 candidate,
check the six integer coefficients, rational pair `(Y0,Y1)`, invertibility
of Y1 modulo the literal input, and the exact common-root condition
`f(-Y0/Y1) = 0 mod N`. Reject a wrong input, malformed integer, incomplete
candidate or failed root check. Preserve a trailing incomplete block
separately; do not claim it as a usable polynomial.

Build the master file from verified complete candidate blocks with the
literal `n:` header. Add no invented elapsed-time record, tested range,
score or coefficient. Native metadata is preserved verbatim. This proves
valid saved candidates, not completion of every lower coefficient range,
an optimal polynomial or satisfactory sieve performance.

Use a correct-target positive validation and a different-input rejection
control. The next execution must demonstrate YAFU actually recognizes the
master file before it is called a polynomial resume. A separate diagnostic
copy may run for at most three seconds, with one second kill grace, solely
to capture native input matching and the last-leading-coefficient message;
any incidental work is setup cost, never new production search coverage.
There is no second
production search in R03; the later backlog job must use the corrected
checkpoint and retain a failed restore as an execution failure.
