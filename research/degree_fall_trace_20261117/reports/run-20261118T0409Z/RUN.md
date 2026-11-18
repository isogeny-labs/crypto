# Conditional survivor tracking in the toy relation flow

Run 20261118T0409Z keeps the [previous toy fixture](../run-20261118T0159Z/RUN.md)
fixed. It uses the three five-element candidate bases over F₁₀₁ and the
shared target ((t,s)=(3,2)). Membership is followed by the sum relation
(x+y+z=t) and the mixed relation (y(x+z)=s). This run checks every source
variable and every pin in the intersection of the three candidate bases.

At each stage, enumerate the unpinned and pinned ordered triples. Project
both sets onto the other two variables, keeping the exact ordered pair
counts and the number of distinct values in each coordinate. Verify the
unpinned stage totals against the earlier Singular run. Check that each
nonempty pin has a subset of the unpinned support and that every empty pin
is explicit in the summary. There are no timing measurements.
