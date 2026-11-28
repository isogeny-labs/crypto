// CT230: exact, std-only audit of two native CT229 F101 factor-base panels.
// This audits immutable hosted output, NOT a new ECDLP solve or an independent PCC.
// Reproduce: rustc --edition=2021 --test tests/ct230_reduced_base_cost_audit.rs -o /tmp/ct230 && /tmp/ct230
use std::collections::BTreeSet;
const DATA: &str = include_str!("../research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct230_native_reduced_base_pairs.tsv");
const HEAD: &str = "run\tbasis\trank\tsmooth\tcandidates\toffsets\trank_complete\ttarget_recovered\ttarget_candidates\tsource_scalar\tsetup_ns\trelations_ns\tla_ns\ttarget_ns\ttotal_ns\tbsgs_ns\tcantor_adds\ttarget_adds\tmodel\tpcc";
#[derive(Debug)]
struct Record {
    run: String, basis: usize, rank: usize, smooth: usize, candidates: usize,
    offsets: usize, rank_complete: bool, target_recovered: bool,
    target_candidates: usize, source_scalar: Option<usize>,
    setup_ns: u128, relations_ns: u128, la_ns: u128, target_ns: u128,
    total_ns: u128, bsgs_ns: u128, cantor_adds: usize, target_adds: usize,
}
fn parse(data: &str) -> Vec<Record> {
    let mut lines = data.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty());
    assert_eq!(lines.next(), Some(HEAD), "invalid CT230 schema/header");
    let mut result = Vec::new();
    for line in lines {
        let s: Vec<_> = line.split('\t').collect();
        assert_eq!(s.len(), 20, "wrong field count");
        let number = |j: usize| -> usize { s[j].parse().expect("bad integer field") };
        let nano = |j: usize| -> u128 { s[j].parse().expect("bad nanosecond field") };
        let boolean = |j: usize| -> bool {
            match s[j] { "true" => true, "false" => false, _ => panic!("bad boolean") }
        };
        assert_eq!(s[18], "F101_known_answer_first_b_x_prefix");
        assert_eq!(s[19], "unverified");
        result.push(Record {
            run: s[0].to_owned(), basis: number(1), rank: number(2), smooth: number(3),
            candidates: number(4), offsets: number(5), rank_complete: boolean(6),
            target_recovered: boolean(7), target_candidates: number(8),
            source_scalar: if s[9] == "null" { None } else { Some(number(9)) },
            setup_ns: nano(10), relations_ns: nano(11), la_ns: nano(12),
            target_ns: nano(13), total_ns: nano(14), bsgs_ns: nano(15),
            cantor_adds: number(16), target_adds: number(17),
        });
    }
    result
}
fn audit(data: &str) {
    assert!(data.contains("# GitHub run=44491343842 job=135447446971 head=b7c4be0e0688977cabd9939d69ebcd2158dd17c5"), "source provenance omitted");
    let rows = parse(data);
    assert_eq!(rows.len(), 12, "retain all six arms, twice");
    let expected: [(usize,usize,usize,usize,usize,bool,usize,usize);6] = [
        // basis,rank,smooth,candidates,offsets,success,cantor adds,target candidates
        (12,0,0,6540,60,false,6632,0),
        (18,1,3,6540,60,false,6632,0),
        (24,1,3,6540,60,false,6632,0),
        (36,34,38,6540,60,false,6632,0),
        (48,48,54,2213,21,true,2234,10),
        (54,54,62,1421,14,true,1435,3),
    ];
    let mut pairs = BTreeSet::new();
    for (i, row) in rows.iter().enumerate() {
        assert!(row.run == "A" || row.run == "B");
        assert!(pairs.insert((row.run.clone(),row.basis)), "duplicate run/basis");
        let e = expected[i % 6];
        assert_eq!((row.basis,row.rank,row.smooth,row.candidates,row.offsets,row.target_recovered,row.cantor_adds,row.target_candidates), e, "native witness mismatch");
        assert_eq!(row.run, if i < 6 { "A" } else { "B" });
        assert_eq!(row.rank_complete, row.rank == row.basis);
        assert_eq!(row.rank_complete, row.target_recovered);
        assert!(row.smooth <= row.candidates && row.rank <= row.smooth && row.candidates <= 6540);
        assert!(row.total_ns >= row.setup_ns + row.relations_ns + row.la_ns + row.target_ns);
        assert!(row.total_ns - (row.setup_ns + row.relations_ns + row.la_ns + row.target_ns) < 20_000, "missing cost");
        assert!(row.bsgs_ns > 0 && row.relations_ns > 0 && row.total_ns > row.bsgs_ns);
        if row.target_recovered {
            assert_eq!(row.source_scalar, Some(37));
            assert!(row.target_candidates > 0 && row.target_adds > 0);
            assert!(row.total_ns > row.bsgs_ns * 20_000, "source CI must not be mislabeled a speedup");
        } else {
            assert_eq!(row.source_scalar, None);
            assert_eq!(row.target_candidates, 0);
            assert_eq!(row.target_adds, 0);
        }
    }
    assert_eq!(pairs.len(), 12);
}
#[test]
fn ct230_two_unpruned_native_panels() { audit(DATA); }
#[test]
fn ct230_fail_closed_mutations() {
    let variants = [
        DATA.replacen("A\t12\t0\t0\t6540", "A\t12\t1\t0\t6540", 1),
        DATA.replacen("A\t36\t34\t38\t6540", "A\t36\t36\t38\t6540", 1),
        DATA.replacen("A\t48\t48\t54\t2213", "A\t48\t48\t54\t2212", 1),
        DATA.replacen("A\t54\t54\t62\t1421", "A\t54\t54\t62\t1422", 1),
        DATA.replacen("head=b7c4be0e", "head=00000000", 1),
        DATA.lines().filter(|l| !l.starts_with("B\t54\t")).collect::<Vec<_>>().join("\n"),
        DATA.replacen("F101_known_answer_first_b_x_prefix", "unpriced_oracle", 1),
    ];
    for (i, bad) in variants.iter().enumerate() {
        assert!(std::panic::catch_unwind(|| audit(bad)).is_err(), "mutation {i} accepted");
    }
}
