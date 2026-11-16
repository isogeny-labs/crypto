# Isogeny witness output amendment

The first `isogeny_witness.sage` execution completed the mathematical
isogeny and subgroup assertions, then failed at JSON serialization because
Sage preparsed the literal `Q_scalar:7` as a Sage Integer. It wrote no
`ISOGENY.json` file. Before rerunning, the source changed only the JSON
serializer to use `default=int`; all field, curve, map, and subgroup
assertions are identical. The failed producer is not a successful receipt.
