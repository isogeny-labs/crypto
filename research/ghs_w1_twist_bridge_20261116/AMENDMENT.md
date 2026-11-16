# Output serialization correction

The first `verify_prime23.sage` run passed all field, curve, point-count, irreducibility, discriminant and modulo-23 assertions but stopped while serializing a Sage `Integer` in the point-count list. Its empty stdout and full traceback are retained as `prime23_inputs_first.json` and `prime23_inputs_first.stderr`.

The only source change before the replay was to add `default=int` to `json.dumps`. The replay is a separate run with a new source SHA256. The failed first run is not counted as a completed finite receipt.
