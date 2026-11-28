# Replay ring correction

The first independent replay script loaded the R91 certificate into R50's
polynomial ring `F2[u]` and rejected a full product. Some exported power
vectors have rational coefficients (for example a denominator `u+1`),
which that ring cannot represent as field elements. Rewriting printed
`u155` notation as `u^155` did not fix the ring mismatch; that attempt
also rejected. The two failure logs are retained as
`replay-polynomial-ring91.log` and `replay-portable91.log`.

`REPLAY_CERT91.sing` now maps the R50 PID matrix into a new, ideal-free
ring over `F2(u)` before loading the untouched export. Both 32-by-32
products and all 64 columns pass there. This ring has no relative curve
ideal, so the product equality cannot be supplied by a quotient relation.
The replay checks polynomiality of `C`, while allowing rational `W` as
required by the quotient-field power basis.
