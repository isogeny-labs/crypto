. as $records
| def rho($arm):
    $records | map(select(.rec == "rho" and .arm == $arm)) | first;
  def field_zero($row):
    $row
    | to_entries
    | map(select(.key | test("^(setup|walk|verify)\\.(fq_|f3_|batch_inv|group_add)")))
    | all(.[]; .value == 0);
  def same_execution($a; $b):
    [
      $a.batch,
      $a.setup_mode,
      $a.q,
      $a.seed,
      $a.i,
      $a.target_kind,
      $a.ell,
      $a.k,
      $a.k_found,
      $a.ok,
      $a.steps,
      $a.dps,
      $a.restarts,
      $a.lookahead,
      $a.cycles,
      $a.cycle_adds,
      $a["setup.reciprocal_div"],
      $a["setup_native.rng"],
      $a["setup_native.hash"],
      $a["setup_native.canon"],
      $a["setup_native.scalar_add"],
      $a["setup_native.band_ops"],
      $a["walk_native.rng"],
      $a["walk_native.hash"],
      $a["walk_native.canon"],
      $a["walk_native.lookahead_test"],
      $a["walk_native.dp"],
      $a["walk_native.table"],
      $a["walk_native.cycle_cmp"],
      $a["walk_native.scalar_add"],
      $a["walk_native.scalar_mul"],
      $a["walk_native.scalar_inv"],
      $a["walk_native.band_ops"]
    ]
    ==
    [
      $b.batch,
      $b.setup_mode,
      $b.q,
      $b.seed,
      $b.i,
      $b.target_kind,
      $b.ell,
      $b.k,
      $b.k_found,
      $b.ok,
      $b.steps,
      $b.dps,
      $b.restarts,
      $b.lookahead,
      $b.cycles,
      $b.cycle_adds,
      $b["setup.reciprocal_div"],
      $b["setup_native.rng"],
      $b["setup_native.hash"],
      $b["setup_native.canon"],
      $b["setup_native.scalar_add"],
      $b["setup_native.band_ops"],
      $b["walk_native.rng"],
      $b["walk_native.hash"],
      $b["walk_native.canon"],
      $b["walk_native.lookahead_test"],
      $b["walk_native.dp"],
      $b["walk_native.table"],
      $b["walk_native.cycle_cmp"],
      $b["walk_native.scalar_add"],
      $b["walk_native.scalar_mul"],
      $b["walk_native.scalar_inv"],
      $b["walk_native.band_ops"]
    ];
  def paired_width($width):
    rho("strong-b" + ($width | tostring)) as $strong
    | rho("fast-b" + ($width | tostring)) as $fast
    | ($strong.count_authoritative == true)
      and ($fast.count_authoritative == false)
      and ($strong.ok == true)
      and ($strong.k_found == $strong.k)
      and field_zero($fast)
      and same_execution($strong; $fast);
  (map(select(.rec == "instance")) | length == 1)
  and (map(select(.rec == "instance"))[0].source_revision == "d85752a9a0")
  and (map(select(.rec == "rho")) | length == 12)
  and all(map(select(.rec == "rho"))[]; .["setup.reciprocal_div"] == 1)
  and ([1, 4, 8, 16, 32] | all(.[]; paired_width(.)))
  and (rho("strong-b8")
    | [
        .["setup.fq_mul"], .["setup.fq_inv"], .["setup.fq_add"],
        .["setup.f3_mul"], .["setup.f3_sq"], .["setup.f3_inv"],
        .["setup.batch_inv"], .["setup.group_add"],
        .["walk.fq_mul"], .["walk.fq_inv"], .["walk.fq_add"],
        .["walk.f3_mul"], .["walk.f3_sq"], .["walk.f3_inv"],
        .["walk.batch_inv"], .["walk.group_add"],
        .["verify.fq_mul"], .["verify.fq_inv"], .["verify.fq_add"],
        .["verify.f3_mul"], .["verify.f3_sq"], .["verify.f3_inv"],
        .["verify.batch_inv"], .["verify.group_add"]
      ]
      == [
        56484, 511, 204142, 6437, 2346, 511, 33, 1455,
        52861, 241, 192668, 7034, 1553, 241, 231, 1551,
        1090, 30, 3466, 80, 50, 30, 0, 31
      ])
  and (rho("strong-b8-legacy-setup") as $legacy
    | rho("fast-b8-legacy-setup") as $fast_legacy
    | ($legacy.count_authoritative == true)
      and ($fast_legacy.count_authoritative == false)
      and same_execution($legacy; $fast_legacy)
      and field_zero($fast_legacy)
      and ([
        $legacy["setup.fq_mul"], $legacy["setup.fq_inv"],
        $legacy["setup.fq_add"], $legacy["setup.f3_mul"],
        $legacy["setup.f3_sq"], $legacy["setup.f3_inv"],
        $legacy["setup.batch_inv"], $legacy["setup.group_add"]
      ] == [85131, 2348, 271096, 6245, 3897, 2348, 0, 2429]))
