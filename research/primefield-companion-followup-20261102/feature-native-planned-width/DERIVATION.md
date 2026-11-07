# Planning a bounded interval width before possible doubling

T-1884. Derived width equivalence and producer checks; independent signed verdict pending. [pcc:unverified]

## Public plan

The previous borrowed engine chooses w0=min(ceil(sqrt(m)),B), where B is the existing coefficient block cap. The new optional policy chooses w1=min(ceil(sqrt(2m)),B) before the first coefficient stream. Eligibility is public: m>=3 odd, prime scalar field r under the existing range contract, 8m dividing r-1, and the first two prescribed radii opposite. Unsupported schedules use the existing adapter. No observed signs, recovered scalars or measured results select the width.

RecoveryProducts retains the stream identity (r,m,B); only the interval engine's constructor planning length changes from m to 2m. The doubled wrapper still has identity (r,2m,B) and borrows the same engine. Width is chosen before a polynomial or progression table exists. No cache is resized, rebuilt or invalidated after use. The 2B request cap and at-most-B evaluation-point buffer bound are unchanged.

## Exact arithmetic

For either width w and any admitted start a and length L<r, write L=q*w+t. Then

    product_(j=0)^(L-1)(a+j)
      = product_(i=0)^(q-1) P_w(a+i*w)
        * product_(j=q*w)^(L-1)(a+j),
    P_w(X)=product_(j=0)^(w-1)(X+j).

The existing scalar tail and multipoint/progression evaluators implement this identity in F_r, including wrap and zero factors. Therefore the width policy leaves every interval product, signed-binomial coefficient and mode constraint unchanged. The borrowed lifetime and cache-identity properties from the predecessor remain. The recovery helper, degree caps, source queries and final group verification are identical after the constructor.

## Cost controls

Compare three separate arms on the same complete e=0,1,2 streams:

1. Shared engine at w0, first m then 2m.
2. Planned shared engine at w1, first m then 2m.
3. Separate engines at native widths w0 and w1.

Record a snapshot after the first m stream as the explicit no-rescue control; do not use only the combined result. The snapshot includes polynomial setups, progression setups, evaluation batches, convolution calls, query-tree calls, live coefficient slots and capacity slots in the retained polynomial/weights/samples vectors. Every array and call is still charged; these selected counters are not total arithmetic operations. Larger batches and convolution degrees may cost more even if the count falls.

Live/capacity slot sums across independent contexts are accounting totals, not simultaneous peak memory. Public output separately records the maximum final retained capacity of any one context, which still excludes temporaries and is not a peak-process measurement. A larger initial width can increase retained storage even if the target recovers without rescue. Lazy polynomial/progression creation remains, and small intervals below the inherited threshold may use no retained cache at all.

## Correctness panels

The same four larger scalar-field/stride configurations and five block caps compare all delivered coefficients and the three arms. The public F_257 curve, order-281 subgroup and all targets retain the six m=5 degree caps; m=35 is added at caps 64 and 128 to exercise actual interval caching. Each target compares the same source schedule, call cap and recovery adapter under both context constructors. The wrapper's planned and fallback schedules are also checked. The original 4563-case cohort is untouched.

No width is declared optimal, no timing advantage is inferred, and this policy does not replace the default. Its purpose is to expose the setup/storage versus batch-work tradeoff before an operation-level choice.

## Sources and next step

Sources checked: feature-native-borrowed-rescue/{interval.rs,progression.rs,context.rs,shared-rescue.rs,shared-rescue-checks.rs}; FLINT nmod_poly documentation https://flintlib.org/doc/nmod_poly/index.html for distinct polynomial length/allocation and evaluation conventions. The field-product identity is derived above and checked by the retained native engine, not inferred from external library timing. KB MCP absent and wrapper unavailable.

Next account primitive field work and temporary allocations in the interval engine at both widths, with separate no-rescue and doubled sequences. Batch counts alone do not price their different sizes; a width policy should be selected from charged work rather than inferred from fewer calls.
