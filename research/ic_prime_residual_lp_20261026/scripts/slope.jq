# Least-squares slope of ln(mean total GAE) on ln r, per arm, at w = 0, over the
# six band JSONs (slurped).  GAE = S * sqrt(r).
[ .[] | .rows[] | {arm, lr: (.log2_r * (2|log)), lg: ((.mean_s[0] | log) + (.log2_r * (2|log)) / 2)} ]
| group_by(.arm) | map(. as $p | ($p | map(.lr) | add / length) as $mx | ($p | map(.lg) | add / length) as $my
  | { arm: $p[0].arm, points: ($p|length),
      slope: (($p | map((.lr - $mx) * (.lg - $my)) | add) / ($p | map((.lr - $mx) * (.lr - $mx)) | add)) })
