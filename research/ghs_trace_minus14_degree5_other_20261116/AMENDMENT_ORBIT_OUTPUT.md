# PGL2 orbit receipt output correction

The first `carrier_orbits.sage` run completed its orbit and polynomial
identity assertions but failed to serialize the `maps` dictionary because
Sage's preparser made its target mask keys Sage Integer objects. No
`CARRIER_ORBITS.json` was written. Before rerunning, the source converted
the already computed target mask to a Python `int`; the field, matrix,
polynomial and orbit assertions are unchanged. The failed producer is
retained in this amendment rather than counted as a receipt.

The second run wrote a complete JSON receipt and passed all orbit assertions,
then failed in the final console summary because its literal `12` was also
preparsed as a Sage Integer. That receipt is preserved as
`CARRIER_ORBITS_PRE_PRINT_FIX.json`. The next output-only amendment adds
`default=int` to the final `json.dumps` call. The mathematical assertions
are still unchanged. Only an exit-zero rerun will be the final orbit receipt.
