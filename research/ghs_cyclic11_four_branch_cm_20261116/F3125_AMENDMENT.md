# Post-run output amendment

The frozen `a267af6790c9` F3125 run produced non-descended ordinary
quotient trace candidates `t=2` and `t=24`, but its report grouped rows
by sorted inertia multiset. That does not identify one exact ordered
curve for each trace. Version 2 adds the first ordered `(a,b,c,d)` row
for every `(lambda, sorted pattern, N, t)` class. Field arithmetic,
population, character calculation, integrality and Hasse checks are
unchanged. The original output remains retained as `F3125_RUN.stdout`.
Freeze this amendment and corrected source before running version 2.
