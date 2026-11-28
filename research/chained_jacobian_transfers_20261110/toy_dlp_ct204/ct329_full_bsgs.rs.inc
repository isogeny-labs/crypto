// CT329: standalone complete *executed* BSGS group-add accounting.
// Original F103 source helpers, direct_bsgs, and CT327 rho tests are extracted
// byte-for-byte by ct329_compose.py. No original source is changed.
mod ct329 {
    use super::*;

    #[derive(Clone, Copy, Default, Debug)]
    struct AddCounts {
        baby: u64,
        step: u64,
        giant: u64,
        verification: u64,
    }
    impl AddCounts {
        fn total(&self) -> u64 {
            self.baby + self.step + self.giant + self.verification
        }
    }

    fn counted_add(count: &mut u64, a: Ec, b: Ec) -> Ec {
        *count += 1;
        ec_add(a,b)
    }

    // This has exactly the control flow of the original ec_mul, with all
    // executed ec_add calls counted; no omitted doubling of the final bit.
    fn counted_mul(mut k: i64, mut point: Ec, count: &mut u64) -> Ec {
        let mut result = None;
        while k > 0 {
            if k & 1 != 0 { result = counted_add(count, result, point); }
            point = counted_add(count, point, point);
            k >>= 1;
        }
        result
    }

    fn accounted_bsgs(g: Ec, target: Ec) -> (i64, AddCounts) {
        let n = 11i64;
        let mut counts = AddCounts::default();
        let mut table = Vec::with_capacity(n as usize);
        let mut current = None;
        for k in 0..n {
            table.push((current,k));
            current = counted_add(&mut counts.baby,current,g);
        }
        let neg_g = g.map(|(x,y)| (x,m(-y,P)));
        let step = counted_mul(n,neg_g,&mut counts.step);
        let mut giant = target;
        for i in 0..=n {
            if let Some((_,j)) = table.iter().find(|(p,_)| *p==giant) {
                let scalar = m(i*n+*j,R);
                let verified = counted_mul(scalar,g,&mut counts.verification);
                assert_eq!(verified,target,"counted BSGS wrong log");
                return (scalar,counts);
            }
            giant = counted_add(&mut counts.giant,giant,step);
        }
        panic!("counted BSGS did not find log");
    }

    fn on_curve(pt: Ec) -> bool {
        match pt {
            None => true,
            Some((x,y)) => (0..P).contains(&x) && (0..P).contains(&y)
                && m(y*y-x*x*x-x-24,P)==0,
        }
    }

    #[test]
    fn ct329_original_bsgs_executed_additions() {
        let g: Ec=Some((1,51));
        assert!(on_curve(g));
        assert!(!on_curve(Some((0,0))));
        assert_eq!(ec_mul(R,g),None);
        assert_eq!(ec_mul(73,g),Some((42,71)));
        let mut targets=0;
        let mut min_total=u64::MAX;
        let mut max_total=0u64;
        for k in 0..R {
            let q=ec_mul(k,g); // fixture, excluded from either solver
            assert!(on_curve(q));
            let (original_log, original_partial)=direct_bsgs(g,q);
            let (instrumented_log, cnt)=accounted_bsgs(g,q);
            assert_eq!(original_log,k,"original BSGS mismatch");
            assert_eq!(instrumented_log,k,"instrumented BSGS mismatch");
            assert_eq!(ec_mul(instrumented_log,g),q,"external scalar check");
            assert_ne!(ec_mul(m(k+1,R),g),q,"wrong scalar should fail");
            assert_eq!(cnt.baby+cnt.giant,original_partial,
                "partial counter does not match original source loop");
            assert_eq!(cnt.total(),original_partial+cnt.step+cnt.verification);
            assert!(cnt.total()>=original_partial);
            if k==73 {
                assert_eq!(original_partial,17);
                assert_eq!((cnt.baby,cnt.step,cnt.giant,cnt.verification,cnt.total()),
                    (11,7,6,10,34));
                eprintln!("CT329_Q73_COUNTS original=17 baby=11 step=7 giant=6 verify=10 total=34 [pcc:unverified]");
            }
            min_total=min_total.min(cnt.total());
            max_total=max_total.max(cnt.total());
            targets+=1;
        }
        assert_eq!(targets,109);
        assert_eq!((min_total,max_total),(18,39));
        eprintln!("CT329_CORRECTNESS_PASS logs=109 min_total=18 max_total=39 [pcc:unverified]");
    }
}
