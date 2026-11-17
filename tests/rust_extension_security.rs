//! Exercise the public API from a downstream consumer's point of view.
use crypto_lib::asymmetric::rsa::{RsaKeyPair, RsaPrivateKey, RsaPublicKey};
use crypto_lib::ecc::curve::CurveParams;
use crypto_lib::ecc::keys::EccKeyPair;
use crypto_lib::keys::{ecc_private_to_hex, KeyBundle};
use crypto_lib::pqc::kyber::kyber_keygen;
use crypto_lib::pqc::mceliece::McElieceKeyPair;
use crypto_lib::utils::random::RandomError;
use num_bigint::BigUint;
use rand::{rngs::StdRng, SeedableRng};

fn toy_rsa() -> RsaKeyPair {
    RsaKeyPair {
        public: RsaPublicKey {
            n: 3233u32.into(),
            e: 17u8.into(),
            bits: 12,
        },
        private: RsaPrivateKey {
            n: 3233u32.into(),
            e: 17u8.into(),
            d: 2753u32.into(),
            p: 61u8.into(),
            q: 53u8.into(),
            dp: 53u8.into(),
            dq: 49u8.into(),
            qinv: 38u8.into(),
            bits: 12,
        },
    }
}

#[test]
fn injected_csprng_reproduces_valid_ecc_keys_and_bounds_fail_without_panic() {
    let curve = CurveParams::p256();
    let mut a = StdRng::seed_from_u64(42);
    let mut b = StdRng::seed_from_u64(42);
    let first = EccKeyPair::try_generate_with_rng(&curve, &mut a).unwrap();
    let second = EccKeyPair::try_generate_with_rng(&curve, &mut b).unwrap();
    assert_eq!(first.private.scalar, second.private.scalar);
    assert_eq!(
        first.public.to_sec1_uncompressed(),
        second.public.to_sec1_uncompressed()
    );
    assert_eq!(
        first.public.to_sec1_uncompressed(),
        EccKeyPair::from_private_checked(first.private.scalar.clone(), &curve)
            .unwrap()
            .public
            .to_sec1_uncompressed()
    );
    let mut invalid = curve;
    invalid.n = BigUint::from(1u8);
    assert!(matches!(
        EccKeyPair::try_generate_with_rng(&invalid, &mut a),
        Err(RandomError::InvalidScalarBound)
    ));
}

#[test]
fn private_key_debug_output_has_no_secret_fields_even_with_alternate_formatting() {
    let ecc = EccKeyPair::from_private(BigUint::from(0xdeadbeef_u64), &CurveParams::p256());
    let rsa = toy_rsa();
    let kyber = kyber_keygen();
    let mceliece = McElieceKeyPair::generate();
    let context =
        crypto_lib::ct_bignum::MontgomeryContext::new(crypto_lib::ct_bignum::Uint([61, 0, 0, 0]))
            .unwrap();
    for debug in [
        format!("{:?}", ecc.private),
        format!("{:#?}", ecc.private),
        format!("{:?}", rsa.private),
        format!("{:#?}", rsa.private),
        format!("{kyber:?}"),
        format!("{kyber:#?}"),
        format!("{context:?}"),
        format!("{context:#?}"),
        format!("{:?}", mceliece.private),
        format!("{:#?}", mceliece.private),
    ] {
        assert!(debug.contains("[REDACTED]"));
        for secret_field in [
            " n:",
            " d:",
            " p:",
            " q:",
            " dp:",
            " dq:",
            " qinv:",
            " s:",
            " s_inv:",
            " support:",
            " g_poly:",
            " perm:",
        ] {
            assert!(!debug.contains(secret_field), "unexpected field in {debug}");
        }
        assert!(!debug.contains("3735928559"));
    }
}

#[test]
fn summaries_support_application_sinks_and_do_not_export_private_scalars() {
    let bundle = KeyBundle {
        ecc: EccKeyPair::from_private(BigUint::from(0xdeadbeef_u64), &CurveParams::p256()),
        rsa: toy_rsa(),
    };
    let mut output = Vec::new();
    bundle.write_summary(&mut output).unwrap();
    let text = String::from_utf8(output).unwrap();
    assert!(text.contains("P-256") && text.contains("RSA-12"));
    assert!(text.contains("Public:"));
    assert!(!text.contains("Private:"));
    assert!(!text.contains(&ecc_private_to_hex(&bundle.ecc.private)));
    struct Broken;
    impl std::io::Write for Broken {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::other("closed sink"))
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    assert!(bundle.write_summary(&mut Broken).is_err());
}

#[test]
fn extension_curve_constructors_keep_distinct_models_and_dispatch_to_generic_solvers() {
    use crypto_lib::cryptanalysis::ecbench::methods::{resolve, solve, MethodSpec};
    use crypto_lib::cryptanalysis::ecbench::workload::CurveSpec;
    let method = resolve(&MethodSpec {
        id: "bsgs.textbook".into(),
        params: Default::default(),
    })
    .unwrap();
    for spec in [
        CurveSpec::Fp4Torsion {
            p: 29,
            seed: 1,
            max_cofactor: 16,
        },
        CurveSpec::F2m4Torsion {
            m: 5,
            seed: 1,
            max_cofactor: 16,
        },
        CurveSpec::Fp4Translation { p: 37, seed: 2 },
    ] {
        let instance = spec.build().unwrap();
        let facts = instance.facts(&spec);
        assert_eq!(facts.r * facts.cofactor, facts.group_order);
        assert_eq!(instance.mul_generator_hex(instance.r()), None);
        let target = instance.mul_generator_hex(7).unwrap();
        let report = solve(&method, &instance, &facts, &target, 7).unwrap();
        assert_eq!(report.recovered, Some(7), "{}", facts.construction);
        assert_eq!(
            instance.mul_generator_hex(report.recovered.unwrap()),
            Some(target)
        );
    }
}

#[test]
fn malformed_plugin_maps_and_invalid_subgroup_bounds_return_errors() {
    use crypto_lib::cryptanalysis::ic_boundary::{
        roster_prime_instance, Calibration, FactorBase, GroupOps, PrimeCurve, PrimeInstance,
    };
    use crypto_lib::cryptanalysis::ic_framework::plugins::{MitmOracle, PrimeAbscissaBase};
    use crypto_lib::cryptanalysis::ic_framework::stages::{FactorBaseBuilder, InstanceCtx, Params};
    use crypto_lib::cryptanalysis::ic_framework::{run_pipeline, PipelineSpec};

    struct BrokenBase<'a>(&'a PrimeInstance);
    impl FactorBaseBuilder<PrimeCurve> for BrokenBase<'_> {
        fn name(&self) -> &str {
            "broken-base"
        }
        fn describe(&self, _: &Params) -> String {
            "invalid plug-in output fixture".into()
        }
        fn build(
            &self,
            ctx: &InstanceCtx<PrimeCurve>,
            params: &Params,
            ops: &mut GroupOps,
        ) -> Result<
            FactorBase<<PrimeCurve as crypto_lib::cryptanalysis::ic_boundary::CountedGroup>::Elt>,
            String,
        > {
            let mut base = PrimeAbscissaBase { instance: self.0 }.build(ctx, params, ops)?;
            base.col_of[0] = base.columns;
            Ok(base)
        }
    }
    let instance = roster_prime_instance(16).unwrap();
    let mut ctx = InstanceCtx {
        group: &instance.curve,
        generator: instance.generator_point(),
        target: instance.generator_point(),
        r: instance.r,
        cofactor: instance.cofactor,
        group_order: instance.group_order,
        name: "invalid extension fixture".into(),
        field_degree: None,
    };
    let mut spec = PipelineSpec::default();
    spec.factor_base_params.set("size", "8");
    let base = BrokenBase(&instance);
    let mut oracle = MitmOracle::new(2);
    let error = run_pipeline(
        &ctx,
        &spec,
        &base,
        &mut oracle,
        1,
        &Calibration::default(),
        None,
    )
    .unwrap_err();
    assert!(error.contains("inconsistent point/column indexes"));
    for r in [0, 1] {
        ctx.r = r;
        assert!(run_pipeline(
            &ctx,
            &spec,
            &base,
            &mut oracle,
            1,
            &Calibration::default(),
            None
        )
        .is_err());
    }
}

#[test]
fn application_cipher_trait_objects_preserve_standard_cmac_and_gcm_vectors() {
    use crypto_lib::symmetric::aes::AesKey;
    use crypto_lib::symmetric::cmac::cmac;
    use crypto_lib::symmetric::modes::cipher::{BlockCipher, BlockCipher128};
    use crypto_lib::symmetric::modes::gcm::{gcm_decrypt, gcm_encrypt};
    use std::cell::Cell;
    struct ApplicationCipher<'a> {
        key: AesKey,
        calls: &'a Cell<usize>,
    }
    impl BlockCipher<16> for ApplicationCipher<'_> {
        fn encrypt_block(&self, block: &mut [u8; 16]) {
            self.calls.set(self.calls.get() + 1);
            self.key.encrypt_block(block);
        }
        fn decrypt_block(&self, block: &mut [u8; 16]) {
            self.key.decrypt_block(block);
        }
    }
    let calls = Cell::new(0);
    let cipher: Box<dyn BlockCipher<16> + '_> = Box::new(ApplicationCipher {
        key: AesKey::new(&hex::decode("2b7e151628aed2a6abf7158809cf4f3c").unwrap()).unwrap(),
        calls: &calls,
    });
    let message = hex::decode("6bc1bee22e409f96e93d7e117393172a").unwrap();
    assert_eq!(
        hex::encode(cmac(cipher.as_ref(), &message)),
        "070a16b46b4d4144f79bdd9dd04a287c"
    );
    assert!(calls.get() > 0);

    let cipher: Box<dyn BlockCipher128> = Box::new(AesKey::new(&[0; 16]).unwrap());
    let encrypted = gcm_encrypt(cipher.as_ref(), &[0; 12], &[], &[0; 16]);
    assert_eq!(
        hex::encode(&encrypted),
        "0388dace60b6a392f328c2b971b2fe78ab6e47d42cec13bdf53a67b21257bddf"
    );
    assert_eq!(
        gcm_decrypt(cipher.as_ref(), &[0; 12], &[], &encrypted).unwrap(),
        [0; 16]
    );
    let mut tampered = encrypted;
    *tampered.last_mut().unwrap() ^= 1;
    assert!(gcm_decrypt(cipher.as_ref(), &[0; 12], &[], &tampered).is_err());
}

#[test]
fn gcm_siv_accepts_an_application_cipher_factory_returning_trait_objects() {
    use crypto_lib::symmetric::aes::AesKey;
    use crypto_lib::symmetric::modes::cipher::BlockCipher128;
    use crypto_lib::symmetric::modes::gcm_siv::{gcm_siv_decrypt_with, gcm_siv_encrypt_with};
    let master = AesKey::new(&[1; 16]).unwrap();
    let dynamic: &dyn BlockCipher128 = &master;
    let factory = |key: &[u8]| -> Box<dyn BlockCipher128> { Box::new(AesKey::new(key).unwrap()) };
    let static_result = gcm_siv_encrypt_with(&master, 16, &[2; 12], b"header", b"message", |key| {
        AesKey::new(key).unwrap()
    });
    let dynamic_result =
        gcm_siv_encrypt_with(dynamic, 16, &[2; 12], b"header", b"message", factory);
    assert_eq!(dynamic_result, static_result);
    assert_eq!(
        gcm_siv_decrypt_with(dynamic, 16, &[2; 12], b"header", &dynamic_result, factory).unwrap(),
        b"message"
    );
}
