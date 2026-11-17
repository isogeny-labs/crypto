
    #[test]
    fn actual_some_none_known_answer_and_diagnostic_ledger() {
        use crate::cryptanalysis::ecbench::methods::{resolve, MethodSpec};
        // Fixed first execution of the existing prepared p739 gate; no search.
        let method = resolve(&MethodSpec {
            id: "ic.three_leads_genus2_pencil_symbolic_v2".into(),
            params: BTreeMap::from([
                ("small_denominator".into(), "8".into()),
                ("matrix_coefficient_cap".into(), "25000000".into()),
            ]),
        }).unwrap();
        assert_eq!(method.method_sha256,
            "187cc5c6d747a3b25cfd9708e970b511886a46c49b5db9fd3b09ce5b8a7fc983");
        let inst = StudyInstance {
            curve: engine::Elliptic {
                f: engine::Field::new(739, 2, 1),
                c: [193650, 493102, 0, 1], binary: false,
            },
            order: 546352, r: 34147, h: 16,
            generator: Point::affine(433246, 59741),
            family: "scholten".into(), parameter: 0, evidence: Value::Null,
        };
        inst.validate().unwrap();
        let target = Point::affine(0x15ed, 0x5b1d4);
        let known = 31124u64;
        let verifier = inst.fresh();
        assert_eq!(verifier.curve.mul(verifier.generator, known), target);
        assert_ne!(verifier.curve.mul(verifier.generator, known + 1), target);
        #[derive(Default)]
        struct Capture(Vec<(String, String, Option<u64>, u64, Value)>);
        impl progress::Sink for Capture {
            fn emit(&mut self, boundary: &str, epoch: &str, walk: Option<u64>, completed: u64, value: Value) {
                assert!(self.0.len() < progress::MAX_SNAPSHOTS as usize, "bounded software-control capture");
                self.0.push((boundary.into(), epoch.into(), walk, completed, value));
            }
        }
        let seed = 17823097723983109480u64;
        let session = measurement::Session::begin().unwrap();
        let plain = solve_with_progress(&method, &inst.fresh(), target, seed, None).unwrap();
        assert!(session.finish().unwrap().online_wall_ns.is_some());
        let mut capture = Capture::default();
        let session = measurement::Session::begin().unwrap();
        let traced = solve_with_progress(&method, &inst.fresh(), target, seed, Some(&mut capture)).unwrap();
        assert!(session.finish().unwrap().online_wall_ns.is_some());
        for report in [&plain, &traced] {
            assert_eq!(report.recovered, Some(u128::from(known)));
            assert!(!report.exhausted);
            assert_eq!(report.detail["candidate"]["verified"], json!(true));
            assert_eq!(report.detail["candidate"]["failure"], Value::Null);
            assert_eq!(report.detail["candidate_over_rho"], Value::Null);
            assert_eq!(report.detail["allocation_observer_available"], json!(false));
            let fresh = inst.fresh();
            assert_eq!(fresh.curve.mul(fresh.generator, report.recovered.unwrap() as u64), target);
        }
        assert_eq!(plain.counters, traced.counters);
        assert_eq!(replay_ledger(&plain.detail["candidate"]), replay_ledger(&traced.detail["candidate"]));
        assert_eq!(replay_ledger(&plain.detail["cold_setup"]), replay_ledger(&traced.detail["cold_setup"]));
        let boundaries: Vec<_> = capture.0.iter().map(|r|r.0.as_str()).collect();
        assert_eq!(&boundaries[..3], &["rational_table_complete", "cold_setup_complete", "candidate_setup_complete"]);
        assert_eq!(boundaries.last(), Some(&"solver_returning"));
        let walks = traced.detail["candidate"]["walks"].as_u64().unwrap();
        assert!(walks > 0 && walks <= 16);
        let observed: Vec<_> = capture.0.iter().filter(|r|r.0=="walk_generation_complete").map(|r|r.2.unwrap()).collect();
        assert_eq!(observed, (0..walks).collect::<Vec<_>>());
        assert!(boundaries.contains(&"sieve_complete"));
        assert!(boundaries.contains(&"candidate_processing_complete"));
        for (boundary, epoch, walk, completed, value) in &capture.0 {
            assert!(value.get("field").is_some() && value.get("arithmetic").is_some() && value.get("symbolic").is_some());
            match boundary.as_str() {
                "rational_table_complete" | "cold_setup_complete" => {assert_eq!(epoch,"cold_setup");assert_eq!(*walk,None);}
                "candidate_setup_complete" | "solver_returning" => {assert_eq!(epoch,"candidate");assert_eq!(*walk,None);}
                "walk_generation_complete" | "sieve_complete" => {assert_eq!(epoch,"candidate");assert!(walk.unwrap()<walks);}
                "candidate_processing_complete" => {assert_eq!(epoch,"candidate");assert!(walk.unwrap()<walks);assert!(progress::milestone(*completed));}
                _ => panic!("unexpected diagnostic boundary"),
            }
        }
        println!("ROOT_ADAPTER_POSITIVE_CONTROL recovered=31124 actual_routes=2 deterministic_ledgers_equal=true diagnostic_events={}", capture.0.len());
    }
