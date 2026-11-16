# PGL2 orbit receipt output correction

The first `carrier_orbits.sage` run completed its orbit and polynomial
identity assertions but failed to serialize the `maps` dictionary because
Sage's preparser made its target mask keys Sage Integer objects. No
`CARRIER_ORBITS.json` was written. Before rerunning, the source converted
the already computed target mask to a Python `int`; the field, matrix,
polynomial and orbit assertions are unchanged. The failed producer is
retained in this amendment rather than counted as a receipt.
