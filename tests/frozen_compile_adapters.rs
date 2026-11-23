//! Frozen research inputs remain immutable when maintained build targets move.
use sha2::{Digest, Sha256};

#[test]
fn archived_volcano_source_keeps_its_original_bytes() {
    let bytes =
        include_bytes!("../research/full_volcano_graphwide_ic_20261017/execution/driver.rs");
    assert_eq!(
        format!("{:x}", Sha256::digest(bytes)),
        "292b16a82edbdc59700877354be117ea921dda6f08adf0f4c37fd389bcf89e2e"
    );
}

#[cfg(not(target_arch = "x86_64"))]
#[test]
fn ct_audit_refuses_unsupported_register_architecture_without_measurements() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_ct_dynamic_audit"))
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8(output.stderr)
        .unwrap()
        .contains("requires x86_64"));
}

#[test]
fn archived_example_and_adapter_inputs_keep_their_original_bytes() {
    for (source, expected) in [
        (
            include_bytes!("../examples/dreg_score.rs").as_slice(),
            "a6be1f9f98bf4b4e46278a1ca7d58446051563eb3ab568b253a6b84ac0e767ba",
        ),
        (
            include_bytes!("../examples/nonisomorphic_extension_isogeny.rs").as_slice(),
            "d0770791dc4089ee19c6e75d0b4f975fdd175e3b6c5864f3c7a68f524b7a6864",
        ),
        (
            include_bytes!("../examples/koblitz_n83_factor_base_export.rs").as_slice(),
            "bccf057a41a8f7419809698dcf5dbf717f93045e81a274ba01af89c676ad4146",
        ),
        (
            include_bytes!("../research/koblitz_n83_factor_base_sweep_20261008/primary_adapter.rs")
                .as_slice(),
            "bd422778e1ab673490537800e7ca6a573d93b70f5c0d47f09658ce7750de549b",
        ),
        (
            include_bytes!("../examples/n83_d18_shared_bits.rs").as_slice(),
            "94bb16acf3cbbf050a796878d98085e7493f525fcd306d68349c1914c2987ed3",
        ),
        (
            include_bytes!("../examples/n83_rho_speed_eval.rs").as_slice(),
            "f1f29f784a80e668ad539068e8299377d88f352a84f317903a294deb814d3064",
        ),
        (
            include_bytes!("../examples/ic_campaign_pricing_audit.rs").as_slice(),
            "ad3f20ffbe687814979862eb192e70ab5ab7e5c0085c736457678adf07024d18",
        ),
        (
            include_bytes!("../examples/conductor_gap_replication.rs").as_slice(),
            "b8bf092234788987017f05231462bd36d47ce5257da639028ed42e123506c808",
        ),
        (
            include_bytes!("../examples/trace_incidence_rho.rs").as_slice(),
            "1585fffc39afa845c53f07bf210ca1443bbd5160854147bb439df4c3ad7fef94",
        ),
        (
            include_bytes!("../examples/ic_quartic_core_probe.rs").as_slice(),
            "f5d979e3e134cd6aa27e73aaaf5669010378ff865bbf1acd96912d69a68c23e3",
        ),
        (
            include_bytes!("../examples/koblitz_orbit_dlp_fast.rs").as_slice(),
            "8573d0c89bde4174f6d9634e05de16ec5a18cd067c2a1d7d9ca029b8f7f66eeb",
        ),
        (
            include_bytes!("../examples/r6_1_direct_s5.rs").as_slice(),
            "5afdde3e03b096bda2f1256971f804bff7782ca423414d9787a492a3eb67c06d",
        ),
        (
            include_bytes!("../examples/smith_isogeny.rs").as_slice(),
            "1362387fc1f782f8db0d60460a9bce7dd5cc14ff4a6ffd541516df687031ee8d",
        ),
    ] {
        assert_eq!(format!("{:x}", Sha256::digest(source)), expected);
    }
}

#[test]
fn additional_archived_example_inputs_keep_their_original_bytes() {
    for (source, expected) in [
        (
            include_bytes!("../examples/koblitz_n83_free_selector_export_control.rs").as_slice(),
            "6a830f5a557c8585d42f88505a359e196ab403dca7d97903599bc3c10f872df1",
        ),
        (
            include_bytes!("../examples/fp_backend_bench.rs").as_slice(),
            "13eaf3a66f49027b0747ed754d2842f85c1fa9935c497b095d339ef72c57f4c5",
        ),
        (
            include_bytes!("../examples/chained_richelot_automorphisms.rs").as_slice(),
            "2ff674a4eb81e6939d52fe2ecc1fd46637c53e5e1d333dc98dde752c49fc66d9",
        ),
        (
            include_bytes!("../examples/ic_quartic_wide_audit.rs").as_slice(),
            "f508764d7f62cb21b537cbbc0de51a644864cc5028b737e07a52572f93017ff9",
        ),
        (
            include_bytes!("../examples/rr_quadratic_panel.rs").as_slice(),
            "b82735fb7bc1b3bfbee6bd7df313327fec6efd3cb4b373cc99c56c883de4bfed",
        ),
        (
            include_bytes!("../examples/n83_factor_base_sweep_v5.rs").as_slice(),
            "ad47a396027a1fc655ab01e5502c6330211bf3307a1e87724e04ea5c6cc5d7ce",
        ),
        (
            include_bytes!("../examples/rr_sat_amortise.rs").as_slice(),
            "ed55557c2c49034f46ab534bfba58ba5bf8e41b235d08ea84ab6cdb9c117849b",
        ),
        (
            include_bytes!("../examples/jv_sieve_extension_sparse_compare.rs").as_slice(),
            "905958af0b8bb9fb0f19231b11aefad30b4ac667d590deeae3e30f6a86a182f4",
        ),
        (
            include_bytes!(
                "../research/ml_dataset_20261028/startup-integration-cli-fixture-native/run.rs"
            )
            .as_slice(),
            "554e5bf1f70e051f0c59364305faa069b01ea35fbae0ab8e4f443d5d29ddb2c8",
        ),
    ] {
        assert_eq!(format!("{:x}", Sha256::digest(source)), expected);
    }
}

#[test]
fn retained_run_card_fixture_keeps_exact_historical_source_bytes() {
    let bytes = include_bytes!("../examples/maintained/fixtures/n83_d18_run_card.sh");
    assert_eq!(
        format!("{:x}", Sha256::digest(bytes)),
        "b4eb8bc5cd581c908225e7b94e321c22a4e7f5f280f05a567800e547a6d85bfd"
    );
}

#[test]
fn archived_native_solver_adapter_keeps_original_bytes() {
    assert_eq!(
        format!(
            "{:x}",
            Sha256::digest(include_bytes!("../examples/gc_native_solve.rs"))
        ),
        "01654e667faf14f318681883035d6e95b2dc862190fda500f5ea516b26f2ece8"
    );
}
