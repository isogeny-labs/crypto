use blocked_bsgs_rho_wall::{
    bsgs_fast, build_compact_seed_manifest, canonical_compact_seed_bytes, derive_u64,
    ecc::curve::CurveParams, prime_fast, rho_seed, target_scalar, validate_git_revision,
    verify_compact_seed_artifact_bytes, verify_fixtures, verify_machine_protocol_value,
    verify_source_hashes, FIXTURES,
};
use num_bigint::BigUint;
use sha2::{Digest, Sha256};

fn hex_word(words: &str, index: usize) -> u64 {
    let start = 16 * index;
    u64::from_str_radix(&words[start..start + 16], 16).unwrap()
}

#[test]
fn source_hashes_match_frozen_protocol() {
    verify_source_hashes().unwrap();
}

#[test]
fn machine_protocol_hash_and_freeze_fail_closed() {
    let path = format!("{}/../protocol.json", env!("CARGO_MANIFEST_DIR"));
    let bytes = std::fs::read(path).unwrap();
    let protocol: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    verify_machine_protocol_value(&protocol).unwrap();

    let mut stale = protocol.clone();
    stale["question"] = serde_json::json!("mutated after freeze");
    assert!(verify_machine_protocol_value(&stale).is_err());

    let mut bad_commit = protocol.clone();
    bad_commit["freeze"]["commit"] = serde_json::json!("ABCDEF");
    assert!(verify_machine_protocol_value(&bad_commit).is_err());

    let mut retrospective = protocol;
    retrospective["freeze"]["filed_before_affected_data"] = serde_json::json!(false);
    assert!(verify_machine_protocol_value(&retrospective).is_err());
}

#[test]
fn seed_derivation_is_pinned() {
    assert_eq!(
        derive_u64("icv1-fp24-tm5099-a6f137c6", "cold-paired", 0, 0, "target"),
        0xff54_8ce9_2b1d_e5ae
    );
    assert_ne!(
        derive_u64("icv1-fp24-tm5099-a6f137c6", "cold-paired", 0, 0, "target"),
        derive_u64("icv1-fp24-tm5099-a6f137c6", "cold-paired", 0, 0, "rho-walk")
    );
}

#[test]
fn fixed_small_solver_smoke_recovers() {
    let result = verify_fixtures().unwrap();
    let bsgs = &result["fixed_solver_smoke"]["bsgs"];
    let rho = &result["fixed_solver_smoke"]["rho"];
    assert_eq!(bsgs["correct"], true);
    assert_eq!(bsgs["status"], "ok");
    assert_eq!(bsgs["censored"], false);
    assert!(bsgs["error"].is_null());
    assert_eq!(bsgs["phase_sum_ns"], bsgs["total_ns"]);
    for field in [
        "neg_map",
        "width",
        "x0",
        "m",
        "stride",
        "giant_count",
        "table_bits",
        "workers",
        "chains_per_worker",
        "baby_len",
        "giant_len",
        "kernel",
        "pair_k",
    ] {
        assert!(!bsgs["plan"][field].is_null(), "missing BSGS plan {field}");
    }
    assert_eq!(rho["correct"], true);
    assert_eq!(rho["status"], "ok");
    assert_eq!(rho["censored"], false);
    assert!(rho["error"].is_null());
    assert_eq!(rho["phase_sum_ns"], rho["total_ns"]);
    for field in [
        "threads",
        "walkers",
        "table_bits",
        "dp_bits",
        "negation",
        "seed",
        "max_steps",
        "look_ahead",
        "cycle_window",
        "coeff_bits",
        "stride_starts",
    ] {
        assert!(
            !rho["config"][field].is_null(),
            "missing rho config {field}"
        );
    }
    assert!(rho["result"]["inner_wall_ns"].is_u64());
}

#[test]
fn all_fixed_curves_and_targets_agree() {
    for fixture in FIXTURES {
        let params = fixture.params();
        let bc = bsgs_fast::FastCurve::from_params(&params).unwrap();
        let pc = prime_fast::FastCurve::from_params(&params).unwrap();
        let pg = pc.from_textbook(&params.generator()).unwrap();
        assert!(bc.on_curve(bc.g));
        assert!(pc.is_on_curve(pg));
        assert!(bc.scalar_mul(bc.g, fixture.n).infinity);
        assert!(pc.mul(Some(pg), fixture.n).is_none());
        let k = target_scalar(fixture, "cold-paired", 0, 0);
        let bq = bc.scalar_mul(bc.g, k);
        let pq = pc.mul(Some(pg), k).unwrap();
        assert_eq!(bc.lower(bq), pc.to_textbook(Some(pq)));
        assert_ne!(rho_seed(fixture, "cold-paired", 0, 0), 0);
    }
}

#[test]
fn compatibility_reference_addition_matches_fast_curve() {
    let fixture = FIXTURES[0];
    let params: CurveParams = fixture.params();
    let bc = bsgs_fast::FastCurve::from_params(&params).unwrap();
    let g = params.generator();
    let two_text = g.add(&g, &params.a_fe());
    assert_eq!(two_text, bc.lower(bc.double(bc.g)));
    let n = BigUint::from(fixture.n);
    assert_eq!(params.n, n);
}

#[test]
fn compact_seed_encoding_round_trips_every_index() {
    let artifact = build_compact_seed_manifest(2).unwrap();
    assert_eq!(artifact["schema"], "blocked-bsgs-rho-wall/derived-seeds/v2");
    assert_eq!(
        artifact["dimensions"]["logical_counts"]["target_seeds"],
        8_340
    );
    assert_eq!(
        artifact["dimensions"]["logical_counts"]["target_scalars"],
        8_340
    );
    assert_eq!(artifact["dimensions"]["logical_counts"]["rho_seeds"], 8_020);
    assert_eq!(
        artifact["dimensions"]["logical_counts"]["schedule_codes"],
        3_300
    );
    assert_eq!(
        artifact["dimensions"]["logical_counts"]["bootstrap_digests"],
        20
    );
    assert_eq!(
        artifact["dimensions"]["logical_counts"]["superseded_jsonl_rows"],
        19_680
    );
    let encoded = serde_json::to_vec(&artifact).unwrap();
    assert!(encoded.len() < 1 << 20);
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&encoded).unwrap(),
        artifact
    );
    assert_eq!(
        serde_json::to_vec(&build_compact_seed_manifest(2).unwrap()).unwrap(),
        encoded
    );

    let target_words = artifact["data"]["target_derived_u64_hex"].as_str().unwrap();
    let scalar_words = artifact["data"]["target_scalar_u64_hex"].as_str().unwrap();
    let rho_words = artifact["data"]["rho_derived_u64_hex"].as_str().unwrap();
    let schedules = artifact["data"]["schedule_codes"].as_str().unwrap();
    let panels = [
        ("warmup", 4u64, 1u64, true, false),
        ("cold-paired", 512, 1, true, false),
        ("bsgs-aa", 64, 1, false, true),
        ("rho-aa", 64, 1, true, true),
        ("amortized-B64", 16, 64, true, false),
    ];
    let mut target_index = 0usize;
    let mut rho_index = 0usize;
    let mut schedule_index = 0usize;
    for fixture in FIXTURES {
        for (panel, replicates, items, has_rho, aa_schedule) in panels {
            for replicate in 0..replicates {
                for item in 0..items {
                    let target = derive_u64(fixture.slug, panel, replicate, item, "target");
                    assert_eq!(hex_word(target_words, target_index), target);
                    assert_eq!(
                        hex_word(scalar_words, target_index),
                        1 + target % (fixture.n - 1)
                    );
                    target_index += 1;
                    if has_rho {
                        assert_eq!(
                            hex_word(rho_words, rho_index),
                            derive_u64(fixture.slug, panel, replicate, item, "rho-walk")
                        );
                        rho_index += 1;
                    }
                }
                let expected = match (aa_schedule, replicate % 2) {
                    (false, 0) => b'0',
                    (false, _) => b'1',
                    (true, 0) => b'2',
                    (true, _) => b'3',
                };
                assert_eq!(schedules.as_bytes()[schedule_index], expected);
                schedule_index += 1;
            }
        }
    }
    assert_eq!(
        (target_index, rho_index, schedule_index),
        (8_340, 8_020, 3_300)
    );
    assert_eq!(target_words.len(), 16 * target_index);
    assert_eq!(scalar_words.len(), 16 * target_index);
    assert_eq!(rho_words.len(), 16 * rho_index);
    assert_eq!(schedules.len(), schedule_index);

    let digest_words = artifact["data"]["bootstrap_selection_sha256_hex"]
        .as_str()
        .unwrap();
    let streams = [
        ("bootstrap-primary", 512u64),
        ("bootstrap-bsgs-aa", 64),
        ("bootstrap-rho-aa", 64),
        ("bootstrap-amortized", 16),
    ];
    let mut digest_index = 0usize;
    for fixture in FIXTURES {
        for (stream, observations) in streams {
            let mut digest = Sha256::new();
            for replicate in 0..2u64 {
                for item in 0..observations {
                    let selected = derive_u64(fixture.slug, "analysis", replicate, item, stream)
                        % observations;
                    digest.update(selected.to_le_bytes());
                }
            }
            let start = 64 * digest_index;
            assert_eq!(
                &digest_words[start..start + 64],
                hex::encode(digest.finalize())
            );
            digest_index += 1;
        }
    }
    assert_eq!(digest_index, 20);
    assert_eq!(digest_words.len(), 64 * digest_index);
}

#[test]
fn compact_seed_canonical_bytes_fail_closed() {
    let canonical = canonical_compact_seed_bytes(2).unwrap();
    assert_eq!(canonical.last(), Some(&b'\n'));
    assert!(canonical.len() < 1 << 20);
    let expected_sha = hex::encode(Sha256::digest(&canonical));
    assert_eq!(
        verify_compact_seed_artifact_bytes(&canonical, 2).unwrap(),
        expected_sha
    );

    let mut missing_newline = canonical.clone();
    missing_newline.pop();
    assert!(verify_compact_seed_artifact_bytes(&missing_newline, 2).is_err());

    let mut changed = canonical;
    changed[0] ^= 1;
    assert!(verify_compact_seed_artifact_bytes(&changed, 2).is_err());
}

#[test]
fn git_revision_validation_fails_closed() {
    let head = "0123456789abcdef0123456789abcdef01234567";
    assert!(validate_git_revision(head, false, head, "").is_ok());
    assert!(validate_git_revision("invalid", false, head, "").is_err());
    assert!(validate_git_revision(head, true, head, "").is_err());
    assert!(
        validate_git_revision(head, false, "89abcdef0123456789abcdef0123456789abcdef", "").is_err()
    );
    assert!(validate_git_revision(head, false, head, " M src/lib.rs").is_err());
}
