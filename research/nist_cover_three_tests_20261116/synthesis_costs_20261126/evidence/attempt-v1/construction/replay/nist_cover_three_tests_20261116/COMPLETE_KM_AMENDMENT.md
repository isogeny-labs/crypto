# Retain the vertical control and extend rational-point encoding

2026-11-17,after retained complete-km-v2 failure,before v3. The final expected
value uses actual rational point classes on the first pole fiber. The older
point_class constructs an effective divisor with multiplicity4 at one such
point,then calls an encoder limited to3;the control therefore fails before
comparing values. Preserve the original fiber and all v2 output.

Add a rational-point wrapper that uses the same base divisor,RR sections and
unchanged KM arithmetic,but obtains enough exact local curve jets for every
requested multiplicity (up to4 for this point-class interface). Check agreement
with the old wrapper at an existing nonpole reference point. Use the extended
wrapper for the unchanged three-point collision expected value.

No source/target input, acceptance equation or degree/rank check changes.
This also implements the rational-point interface at the pole-fiber inputs
that the older encoder rejected. Record v3 independently;v2 stays failed.
