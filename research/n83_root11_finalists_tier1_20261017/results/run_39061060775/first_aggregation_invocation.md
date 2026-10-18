# First aggregation invocation

The first local invocation used the aggregate script by its filesystem path.
Python placed that script directory at the front of module search, and the
sibling `select.py` shadowed the standard-library `select` module imported by
`subprocess`. The invocation failed before any provider ZIP binding or
candidate selection. Its complete stderr was not retained, so this record
does not assert an exact traceback or exit code. The unchanged aggregate
module was then invoked with `python3 -m` from the repository root; that
invocation completed after four independently replayed strata were supplied
through `--reuse-local-replay`.
