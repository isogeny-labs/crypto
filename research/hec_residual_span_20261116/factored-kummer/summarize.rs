use std::{collections::BTreeMap, fs};
fn main() {
    let root = "research/hec_residual_span_20261116/factored-kummer";
    let old=fs::read_to_string("research/hec_residual_span_20261116/generic-kummer/CONTROL.tsv").unwrap();
    let new=fs::read_to_string(format!("{root}/CONTROL.tsv")).unwrap();
    assert_eq!(old.lines().count(),new.lines().count());
    let mut fixed=0;
    for (a,b) in old.lines().skip(1).zip(new.lines().skip(1)) {
        let x:Vec<_>=a.split('\t').collect();let y:Vec<_>=b.split('\t').collect();
        assert_eq!(&x[..8],&y[..8]);
        if x[1]=="17"||x[5]=="0" {assert_eq!(x[8],y[8]);fixed+=1;}
    }
    eprintln!("retained_labels_equal=true unchanged_control_records={fixed}");
    let mut rows = BTreeMap::new();
    let mut sum = BTreeMap::new();
    for l in fs::read_to_string(format!("{root}/CONTROL.tsv"))
        .unwrap()
        .lines()
        .skip(1)
    {
        let v: Vec<_> = l.split('\t').collect();
        assert_eq!(v.len(), 9);
        let n = |j: usize| v[j].parse::<u64>().unwrap();
        let key = (n(0), n(1), n(2), n(3), n(4), n(5));
        let cs: Vec<u64> = v[8].split(',').map(|x| x.parse().unwrap()).collect();
        assert_eq!(cs.len(), 16);
        assert_eq!(cs[8], 2 * cs[7] + cs[5] + cs[6]);
        if key.3 == 1 {
            assert_eq!(rows[&(key.0, key.1, key.2, 0, key.4, key.5)], cs)
        }
        assert!(rows.insert(key, cs.clone()).is_none());
        if key.2 < 2 && key.3 == 0 {
            let a = sum.entry((key.0, key.1, key.5)).or_insert(vec![0u64; 16]);
            for j in 0..16 {
                a[j] += cs[j]
            }
        }
    }
    assert_eq!(rows.len(), 9504);
    for (&(p, sm, case, rep, step, arm), v) in &rows {
        if arm != 1 {
            continue;
        }
        let kernel = &rows[&(p, sm, case, rep, step, 2)];
        assert!(v.iter().zip(kernel).all(|(a, b)| a >= b))
    }
    println!("p\tsm\tnative_REDC\tfull_candidate_REDC\tkernel_REDC\tnative_over_full_REDC\tnative_over_kernel_REDC\tnative_over_free_kernel_remaining_REDC\tnative_inv\tfull_inv");
    for (&(p, sm, arm), a) in &sum {
        if arm != 0 {
            continue;
        }
        let b = &sum[&(p, sm, 1)];
        let k = &sum[&(p, sm, 2)];
        println!(
            "{p}\t{sm}\t{}\t{}\t{}\t{:.6}\t{:.6}\t{:.6}\t{}\t{}",
            a[8],
            b[8],
            k[8],
            a[8] as f64 / b[8] as f64,
            a[8] as f64 / k[8] as f64,
            a[8] as f64 / (b[8] - k[8]) as f64,
            a[9],
            b[9]
        );
    }
    let names = [
        "new",
        "add",
        "sub",
        "neg",
        "mul",
        "pre",
        "mul_pre",
        "reduce128",
        "redc",
        "inv",
        "inv_steps",
        "poly_products",
        "poly_divrem",
        "group_add",
        "group_add_place",
        "group_mul",
    ];
    let mut bounds =
        String::from("p\tsm\tmax_recorded_component_ratio\tlimiting_component\tzero_denominator\n");
    for (&(p, sm, arm), a) in &sum {
        if arm != 0 {
            continue;
        }
        let b = &sum[&(p, sm, 1)];
        let mut limit = 0f64;
        let mut at = 0;
        let mut zero = false;
        for j in 0..16 {
            if b[j] == 0 {
                if a[j] > 0 {
                    zero = true;
                    at = j;
                    break;
                }
            } else {
                let r = a[j] as f64 / b[j] as f64;
                if r > limit {
                    limit = r;
                    at = j
                }
            }
        }
        bounds.push_str(&format!("{p}\t{sm}\t{limit:.6}\t{}\t{zero}\n", names[at]));
    }
    fs::write(format!("{root}/VECTOR-BOUNDS.tsv"), bounds).unwrap();
    let mut setup = BTreeMap::new();

    for l in fs::read_to_string(format!("{root}/SETUP.tsv"))
        .unwrap()
        .lines()
        .skip(1)
    {
        let v: Vec<_> = l.split('\t').collect();
        let n = |j: usize| v[j].parse::<u64>().unwrap();
        let cs: Vec<u64> = v[4].split(',').map(|x| x.parse().unwrap()).collect();
        let key = (n(0), n(1), n(2), n(3));
        if key.3 == 1 {
            assert_eq!(setup[&(key.0, key.1, key.2, 0)], cs)
        }
        setup.insert(key, cs);
    }
    assert_eq!(setup.len(), 36);
    eprintln!("primitive_records=9504 checked_steps=3168 setup_records=36 repeats_and_REDC_identity=true ordinary_chains_only_in_ratio=true nested_kernel_not_added=true");
}
