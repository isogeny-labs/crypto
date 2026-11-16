# Independent audit attempt 1

The independent Sage replay reached its final JSON write after all transfer,
order and scalar assertions passed, then exited 1 because Sage's preparser
converted the literal `218` in `base_order` to a Sage Integer that Python's
JSON encoder does not accept. No audit receipt was written. The next source
revision casts that value to a Python `int`; it does not alter the algebraic
checks. The producer transfer receipt remains unchanged.
