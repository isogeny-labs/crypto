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

## What the pins reveal

- Membership provides a control. Each pin reduces 125 triples to 25, and
  all 25 pairs of the other variables remain feasible. [pcc:unverified]
- For the centered base at the sum stage, pinning `x=2` retains all four
  individual values of each of `y` and `z` while retaining only four of
  their ten jointly feasible pairs. [pcc:unverified]
- After the mixed stage, `x=2` leaves `(y,z)=(1,0)` in all three bases.
  The centered base also leaves `(2,-1)`. [pcc:unverified]
- Pinning `y=0` leaves two sum survivors for the centered and spread bases
  and four for the consecutive base. The mixed stage rejects them all
  because (y(x+z)=0) when (y=0). [pcc:unverified]

The figure shows distinct ordered pairs retained over all pairs feasible
with the same base and stage before the pin. The summary also gives
ordered triple totals and the number of distinct values for each of the
other two variables. The support CSV gives each feasible pair and its
multiplicity. A blank pin labels the unpinned projection. A pinned group
with no support rows is represented by an explicit zero in the summary.
Marginal multiplicities can be read by summing over the other member of
each pair. These are exact finite counts at one shared target.

[Figure](survivor_retention.svg) · [PDF](report.pdf) ·
[Summary CSV](survivor_summary.csv) · [Exact support CSV](survivor_support.csv)

## Replay

g++ 14.2 generated the records on Linux x86-64. The unpinned stage
totals match the earlier Singular run. An independent check solved the
sum for the omitted coordinate for each possible pair and used
(y(t-y)=s) at the mixed stage. It matched every summary and support row.
A second run reproduced both CSVs byte for byte. [pcc:unverified]

```sh
build_dir=$(mktemp -d)
g++ -std=c++20 -O2 -Wall -Wextra -Wpedantic -Werror \
    measure_survivors.cpp -o "$build_dir/measure_survivors"
"$build_dir/measure_survivors" "$build_dir/summary.csv" "$build_dir/support.csv"
diff -u survivor_summary.csv "$build_dir/summary.csv"
diff -u survivor_support.csv "$build_dir/support.csv"
python3 plot_survivors.py
typst compile report.typ
```
