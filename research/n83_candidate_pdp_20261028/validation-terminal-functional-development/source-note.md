# Exact donor provenance

The native test executable was built using the exact donor snapshot inside
the development archive (SHA25627bc41e8a266a845262840838b98c3e28e533fa01d76d222d809f91dd09d0df1).
Running rustfmt on the new binary followed its module path and reformatted
seven public adapter functions in the shared donor. The retained patch
`donor-formatting.patch` shows the complete change: whitespace and line
breaks only. The donor in the repository was restored to its original bytes;
the archived executable and its tested source snapshot remain unchanged.
Use the archived source and executable when replaying this development run.
No field arithmetic, source validation, or test behavior was changed.
