# Independent finite specification review

A separate mathematical reader checked the fixed specialization before
execution without examining this implementation or its outputs.

The binary matrix with columns1,2,3 has kernel{000,111}. Hence every x
has exactly partners y=x and y=x xor111, giving16slots and two copies
of each first orientation. Fixing first-register bit0 leaves8slots.
U0 toggles that bit; U1,U2 preserve it. Opposite row/column orientation
conventions therefore make U0 local, with I,U1,U2 in the crossing block.

At t=1 and alpha=16, the selector f*=ell0 has order4. Each other
crossing selector sums an even number of normalized odd leading units,
so f1,f2 have order at least5. The exceptional node-8 belongs to f0
and is local. Thus the expected finite certificate is B/256 modulo2=I8.

The review found no orientation or valuation discrepancy. It did not
execute a program, examine the native run, prove a global mode bound or
provide an independent native replay. Raw outputs and the compiled code
still require their own verification, retained separately here.
