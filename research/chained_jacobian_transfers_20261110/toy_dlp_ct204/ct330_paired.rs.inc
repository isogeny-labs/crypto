    // CT330 appendix inserted inside the original source-derived ct327 module.
    // Do not substitute/reimplement original rho or BSGS for a timed pair.
    #[test]
    fn ct330_paired_original_source_rho_bsgs_q73() {
        let g: Ec = Some((1, 51));
        let q: Ec = Some((42, 71));
        assert!(on_curve(g) && on_curve(q));
        assert_eq!(ec_mul(R, g), None);
        assert_eq!(ec_mul(73, g), q);
        for round in 0..3 {
            let original_rho = || {
                let s = rho_public(g, q).expect("original fixed-capped rho failed Q73");
                assert_eq!(s.log, 73);
                assert_eq!(ec_mul(s.log, g), q);
                s
            };
            let original_bsgs = || {
                let timer = Instant::now();
                let (log, partial_adds) = direct_bsgs(g, q);
                let ns = timer.elapsed().as_nanos();
                assert_eq!(log, 73);
                assert_eq!(ec_mul(log, g), q);
                (ns, partial_adds)
            };
            let (rho, bs) = if round == 1 {
                let bs = original_bsgs();
                let rho = original_rho();
                (rho, bs)
            } else {
                let rho = original_rho();
                let bs = original_bsgs();
                (rho, bs)
            };
            assert!(rho.total_ns > 0 && bs.0 > 0);
            assert_eq!(bs.1, 17); // CT329: original BSGS partial Q73 count.
            assert!(rho.tally.ec_adds > 0 && rho.restarts <= MAX_RESTARTS);
            let classified = rho.setup_ns + rho.walk_ns + rho.verification_ns;
            assert!(classified <= rho.total_ns, "original rho phase timers overlapped");
            let unknown = rho.total_ns - classified;
            let vmhwm_kb = std::fs::read_to_string("/proc/self/status").ok()
                .and_then(|s| s.lines().find(|l| l.starts_with("VmHWM:"))
                    .and_then(|l| l.split_whitespace().nth(1))
                    .and_then(|num| num.parse::<u64>().ok())).unwrap_or(0);
            let order = if round == 1 { "BSGS_RHO" } else { "RHO_BSGS" };
            eprintln!(
                "CT330_PAIRED_JSON {{\"round\":{},\"order\":\"{}\",\"p\":103,\"subgroup_order\":109,\"target_x\":42,\"target_y\":71,\"rho_log\":{},\"bsgs_log\":73,\"rho_total_ns\":{},\"bsgs_total_ns\":{},\"rho_setup_ns\":{},\"rho_walk_ns\":{},\"rho_verify_ns\":{},\"rho_unclassified_ns\":{},\"rho_ec_adds\":{},\"rho_walk_steps\":{},\"rho_restarts\":{},\"rho_canonicalisations\":{},\"rho_negations\":{},\"rho_collisions\":{},\"rho_useless_collisions\":{},\"rho_table_entries\":{},\"rho_table_capacity\":{},\"bsgs_original_partial_adds\":{},\"bsgs_ct329_full_adds\":34,\"vmhwm_kb_processwide\":{},\"pcc\":\"unverified\"}}",
                round, order, rho.log, rho.total_ns, bs.0,
                rho.setup_ns, rho.walk_ns, rho.verification_ns, unknown,
                rho.tally.ec_adds, rho.tally.walk_steps, rho.restarts,
                rho.tally.canonicalisations, rho.tally.negations,
                rho.tally.collisions, rho.tally.useless_collisions,
                rho.tally.max_table_entries, rho.tally.max_table_capacity,
                bs.1, vmhwm_kb
            );
        }
        eprintln!("CT330_THREE_PAIRED_Q73_COMPLETE pcc=unverified");
    }
