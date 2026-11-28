use std::{collections::BTreeMap, fs};
fn main() {
    let root = "research/hec_residual_span_20261116/c4-solves";
    let mut rows = BTreeMap::new();
    for l in fs::read_to_string(format!("{root}/CONTROL.tsv"))
        .unwrap()
        .lines()
        .skip(1)
    {
        let v: Vec<_> = l.split('\t').collect();
        assert_eq!(v.len(), 15);
        assert_eq!(v[9], "true");
        assert_eq!(v[7], v[8]);
        let nums: Vec<u64> = v[14].split(',').map(|x| x.parse().unwrap()).collect();
        assert_eq!(nums.len(), 16);
        assert_eq!(nums[8], 2 * nums[7] + nums[5] + nums[6]);
        let p: u64 = v[0].parse().unwrap();
        let sm: u64 = v[2].parse().unwrap();
        let case: u64 = v[3].parse().unwrap();
        let rep: u64 = v[4].parse().unwrap();
        let arm: u64 = v[5].parse().unwrap();
        let modulus: u64 = v[6].parse().unwrap();
        assert!(rows
            .insert((p, sm, case, rep, arm, modulus), nums)
            .is_none());
    }
    assert_eq!(rows.len(), 144);
    println!("p\tsm\tcase\tmodulus\tfolded_over_pencil_REDC\texpanded_over_pencil_REDC\tfolded_over_pencil_inv\tpencil_REDC\tfolded_REDC\texpanded_REDC");
    for (&(p, sm, case, rep, arm, modulus), a) in &rows {
        if rep != 0 || arm != 0 {
            continue;
        }
        let b = &rows[&(p, sm, case, 0, 1, modulus)];
        let c = &rows[&(p, sm, case, 0, 2, modulus)];
        for arm in 0..3 {
            assert_eq!(
                rows[&(p, sm, case, 0, arm, modulus)],
                rows[&(p, sm, case, 1, arm, modulus)]
            )
        }
        println!(
            "{p}\t{sm}\t{case}\t{modulus}\t{:.6}\t{:.6}\t{:.6}\t{}\t{}\t{}",
            b[8] as f64 / a[8] as f64,
            c[8] as f64 / a[8] as f64,
            b[9] as f64 / a[9] as f64,
            a[8],
            b[8],
            c[8]
        );
    }
    eprintln!("raw_rows=144 completed_calls=72 fresh_workloads=12 all_scalar_fields_match=true repeat_vectors=true REDC_identity=true");
}
