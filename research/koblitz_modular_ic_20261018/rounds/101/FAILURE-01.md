# R101 attempt 01: constant-term suffix parser failure

The frozen `fiber101-01` preparation exited 1 with
`ValueError: missing G51 monomial separator`. The last rational coefficient
in `G51` is its constant term and has no `*l` or `*y` suffix; the parser
required a suffix for every coefficient. No Singular process was launched
and no fiber result was produced. The saved `prepare.log`, exit code, donor,
and frozen source remain in `runs/fiber101-01`. Amendment 02 permits the
empty suffix as exponent pair `(0,0)` and keeps all other source checks.
