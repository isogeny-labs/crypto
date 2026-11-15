# Output-only correction before a degree-four replay

15 November 2026. The frozen `09018f379296` source completed its 405-case loop but failed while serializing the final JSON: Sage `Integer` values in the count dictionary are not accepted by Python's default `json.dumps`. Its `__file__` also pointed at Sage's generated `.sage.py` file rather than the source `.sage` when computing the source digest. Standard output was empty, and this failed attempt is not a result receipt. [pcc:unverified]

The only changes to `degree4_screen.sage` are `default=int` in the JSON serializer and removal of the generated `.py` suffix when hashing source bytes. The field, 405 assignments, CRT tests, polynomial identity, hit predicate and protocol remain byte-identical to the frozen version. Rerun only after this amendment and corrected source are committed. [pcc:unverified]
