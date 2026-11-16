# Curve-free null summary: per (curve, B) the mean group operations per solve on
# the curve and on Z/r (random-bijection key), their ratio, and a bootstrap 95%
# interval (2000 resamples; MINSTD generator, exact in jq's doubles) of the ratio of means.
def mean: add / length;
def resample($xs; $s): reduce range(0; $xs|length) as $i ({s: $s, out: []};
  .s |= ((. * 48271) % 2147483647) | .out += [$xs[.s % ($xs|length)]]);
. as $d
| ($d.curve.group_ops) as $c | ($d.curve_free.group_ops) as $f
| [range(0; 2000) as $k | (resample($c; 7919 * $k + 1).out | mean) as $mc
   | (resample($f; 104729 * $k + 3).out | mean) as $mf | $mf / $mc] | sort as $boot
| { r: $d.r, base: $d.base, runs: $d.runs,
    verified: [$d.curve.verified, $d.curve_free.verified],
    curve_mean_s: (($c|mean) / ($d.r|sqrt)), free_mean_s: (($f|mean) / ($d.r|sqrt)),
    ratio: (($f|mean) / ($c|mean)), ci95: [$boot[49], $boot[1949]] }
