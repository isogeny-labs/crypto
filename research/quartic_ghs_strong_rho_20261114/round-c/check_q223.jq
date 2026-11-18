[ .[] | select(.rec == "rho") ] as $records
| $records[0] as $baseline
| $records[1] as $candidate
| def scrub:
    del(
      .wall_s,
      .["setup.fq_mul"], .["walk.fq_mul"], .["verify.fq_mul"],
      .["setup.fq_add"], .["walk.fq_add"], .["verify.fq_add"],
      .["setup.f3_mul"], .["walk.f3_mul"], .["verify.f3_mul"]
    );
  def phase($name):
    {
      phase: $name,
      f3_inv: $baseline[$name + ".f3_inv"],
      fq_mul_baseline: $baseline[$name + ".fq_mul"],
      fq_mul_candidate: $candidate[$name + ".fq_mul"],
      fq_mul_delta: ($candidate[$name + ".fq_mul"] - $baseline[$name + ".fq_mul"]),
      fq_mul_expected: (-4 * $baseline[$name + ".f3_inv"]),
      f3_mul_delta: ($candidate[$name + ".f3_mul"] - $baseline[$name + ".f3_mul"]),
      f3_mul_expected: (-$baseline[$name + ".f3_inv"]),
      fq_add_delta: ($candidate[$name + ".fq_add"] - $baseline[$name + ".fq_add"]),
      fq_add_expected: (-12 * $baseline[$name + ".f3_inv"]),
      pass: (
        ($candidate[$name + ".fq_mul"] == $baseline[$name + ".fq_mul"] - 4 * $baseline[$name + ".f3_inv"])
        and ($candidate[$name + ".f3_mul"] == $baseline[$name + ".f3_mul"] - $baseline[$name + ".f3_inv"])
        and ($candidate[$name + ".fq_add"] == $baseline[$name + ".fq_add"] - 12 * $baseline[$name + ".f3_inv"])
      )
    };
  {
    records: ($records | length),
    identity_after_expected_deltas: (($baseline | scrub) == ($candidate | scrub)),
    phases: [phase("setup"), phase("walk"), phase("verify")]
  }
| .overall_pass = (
    (.records == 2)
    and .identity_after_expected_deltas
    and ([.phases[].pass] | all)
  )
| if .overall_pass then . else error("walk/count identity failed") end
