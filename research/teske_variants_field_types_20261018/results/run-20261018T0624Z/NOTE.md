# Aborted run 20261018T0624Z

Launched with tool sha256 `1566fa0a…` right after the first smoke run. The
smoke run's Part C output (not part of any recorded run) showed
`neighbours_missing > 0`: the `normalize` routine returned twist-class
models with leading coefficient `nu` while the enumeration stores every
model as `y^2 = c f` with `f` monic and the twist in a flag, so a twist
model's neighbours were never found and classes were split. Under the
protocol ("a failed check stops the run, the tool is fixed, and the failure
is reported") this run was stopped during Part B; the tool was fixed
(`normalize` now returns `f` monic and the twist flag; a zero count of
missing neighbours and the absence of duplicate models are asserted) and
the recorded run is the next directory. Part A's output here is identical
to the recorded run's (same code path); the partial Part B files are left
as written.
