# Reuse the field reduction context in curve arithmetic

## What was done

Curve now stores the existing Modulus context, constructed once alongside its
other validated parameters. Membership checks, the curve equation and affine
addition reuse it. Fixed-pair tables reuse the same context as well. Canonical
modular sums use bounded additions instead of remainder division. No factor
base, target, scalar, certificate format or oracle search order changes.
The original relation checker and full verifier control flow are byte-identical
to the previous revision; their checked Curve arithmetic uses the new context.
Both complete certificate replays remain.

35 standalone release tests, strict all-target Clippy, formatting and whitespace checks pass. A new independent reference uses u128
remainder products and its own Fermat exponentiation, bypassing both the
reciprocal reducer and inverse cache. It generates points independently and
compares all pair sums for the complete p=17 and p=211 groups,
plus the first 200 points of the 16-bit and certified 40-bit fixtures.
It also checks curve equations, membership and rejection of noncanonical
coordinates. Existing exhaustive reducer and frozen-certificate tests remain.
The reference executable from round16 is preserved and its hash recorded.

## Decisions and literature

Reviewed the existing reciprocal kernel and its exact bounds in
`../reciprocal-candidate.md`, and
[HAC chapter 14, section 14.3.3](https://cacr.uwaterloo.ca/hac/about/chap14.pdf).
The textbook motivates amortizing reciprocal precomputation over repeated
reductions; this repository's existing ceiling variant retains its own bound.
No new modular reduction algorithm or novelty claim is made.

For p <= 2^32-1, the existing reducer computes products without division using
R=ceil(2^64/p) and a bounded correction. Larger fields retain the independent
wide remainder fallback. Constructor work stays inside setup. The inline
context contains two u64 values (16 bytes); no new heap table is introduced.
It is auxiliary Curve storage, excluded from the retained sum-table metric.

All sums given to addmod have canonical operands. Validated p<2^62 means
2*p<2^63, so those additions cannot overflow. For doubling, modular additions
compute 3*x^2+a and 2*y without forming a noncanonical multiplier operand.
Membership rejects noncanonical coordinates before calling the reducer.

## Open threads

No timing result exists for this candidate. Freeze and run the next full
five-arm comparison against round16's integrated source, retaining original
baseline 2bd4588fba603ba1787e1d8ad82f084ef461d7cf, all three fixtures, both
certificate checks, setup, discarded trials and serialization. The further10x
goal remains unproven. Full root/CUDA builds are not run locally because of
the user disk constraint. Strict isolation, independent scientific review,
external comparisons and larger CPU scale remain open.

## Errors and ownership

T-929 reserved the shared source and this handoff directory before editing.
No compilation or release-test failure occurred. Source-level elimination of
remainder operations is not a measured end-to-end performance result.

Validation-record correction: the first region-hash script selected an earlier
`cfg(test)` marker and hashed an empty slice. Its endpoint now searches after
`check_relation` and asserts that the nonempty region includes `pub fn verify`.
The corrected full-region comparison passes; source code and tests were not
changed by this metadata correction. The original record remains in history.
