# R84 corrected direct fraction reductions, frozen before data

R83's source module was ring-local and became unavailable after the
candidate-ring switch. R84 serializes all 27 pinned R50 module
polynomials to strings while in the selected source ring, then parses
the common denominator and each of the 19 relevant numerators in the
candidate ring's literal `x,y,p` coordinates. The two-ring string
transport syntax was checked on a separate tiny Singular example; it
was not tested on the research data before this freeze.

Check the 19 surviving `T(k)` names against R81, then reduce each
complete `denom*T(k)+numerator` polynomial against the saved `norid`
generators without computing `std(norid)`. A zero remainder is a
sound positive ideal-membership certificate. A nonzero remainder is
inconclusive and fails acceptance, but does not disprove the identity.
The earlier R50 selected-source denominator check remains pinned as a
donor. Passing does not certify the candidate ring as a domain or a
normalization, nor assign local branch data.

Run locally in Singular, one Singular/FLINT thread, seed 84, 1200
seconds and 4 GiB virtual. Copy committed source/protocol/driver
bytes and verify donor hashes. Accept only exit 0, exact marker,
zero native whitespace-`?` errors and all 19 zero remainders. Retain
failures and timeouts; freeze any repair before a fresh ID. No timing
comparison.
