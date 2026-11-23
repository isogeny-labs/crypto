#[path = "curve_b.rs"]
mod curve;
#[path = "field_b.rs"]
mod field;
mod inherited;

use curve::{key, Chart, Divisor, ORIGINAL};
use field::{Field, Q};
use inherited::peval;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

fn apply(matrix: &[[u8; 4]; 4], id: usize) -> usize {
    let mut result = 0;
    for (i, row) in matrix.iter().enumerate() {
        let value: usize = row
            .iter()
            .enumerate()
            .map(|(j, &entry)| usize::from(entry) * ((id >> (3 * j)) & 7))
            .sum();
        result |= (value & 7) << (3 * i);
    }
    result
}

fn binary_rank(mut rows: [u8; 4]) -> usize {
    let mut rank = 0;
    for bit in 0..4 {
        if let Some(pivot) = (rank..4).find(|&i| rows[i] & (1 << bit) != 0) {
            rows.swap(rank, pivot);
            for i in 0..4 {
                if i != rank && rows[i] & (1 << bit) != 0 {
                    rows[i] ^= rows[rank];
                }
            }
            rank += 1;
        }
    }
    rank
}

fn main() {
    let table_path = std::env::args()
        .nth(1)
        .expect("fresh full-coordinate TSV path");
    let output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(table_path)
        .unwrap();
    let mut output = BufWriter::new(output);
    let f = Field::new();
    println!(
        "FIELD q={Q} modulus_low12={:?} generator={} all_nonzero_inverse_and_Fq=pass",
        f.modulus, f.generator
    );
    let chart = Chart::new(&f);
    let h = &chart.h;
    println!("CURVE original={ORIGINAL:?} roots={:?} r={} scale={} leading={} monic={:?} delta={} pole={}", chart.roots, chart.r, chart.scale, chart.leading, h.h, chart.delta, chart.pole);
    assert_eq!(chart.frobenius(&chart.pre), chart.post);
    let naive = chart.push_regular(&chart.post);
    let correct = chart.frobenius(&chart.post);
    assert_ne!(naive, correct, "omitted moving-basepoint control must fail");
    let next_root = f.pow(f.pow(chart.r, 3), 3);
    let direct = h.sub(&chart.point(next_root, 0), &chart.post);
    assert_eq!(correct, direct);
    println!(
        "BASEPOINT_CONTROL naive={:?} corrected={:?} direct={:?} restored=pass",
        key(&naive),
        key(&correct),
        key(&direct)
    );

    let mut pair_masks = BTreeMap::new();
    for i in 0..6 {
        for j in i + 1..6 {
            let d = chart.pair(i, j);
            assert!(!h.is_identity(&d) && h.is_identity(&h.add(&d, &d)));
            assert!(pair_masks
                .insert(key(&d), (1_u8 << i) | (1_u8 << j))
                .is_none());
            let ii = chart
                .roots
                .iter()
                .position(|&x| x == f.pow(chart.roots[i], 3))
                .unwrap();
            let jj = chart
                .roots
                .iter()
                .position(|&x| x == f.pow(chart.roots[j], 3))
                .unwrap();
            assert_eq!(chart.frobenius(&d), chart.pair(ii, jj));
            println!(
                "PAIR mask={} d={:?} Fmask={}",
                (1_u8 << i) | (1_u8 << j),
                key(&d),
                (1_u8 << ii) | (1_u8 << jj)
            );
        }
    }
    assert_eq!(pair_masks.len(), 15);
    let order = 282_375_475_200_u128;
    let cofactor = order / 4096;
    assert_eq!(order % 4096, 0);
    assert_eq!(cofactor & 1, 1);
    let mut seed = 0x4a61_636f_6269_616e_u64;
    let mut basis = Vec::<Divisor>::new();
    let mut span = vec![h.identity()];
    let mut draws = 0;
    for draw in 0..256 {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        let x = (seed % u64::from(Q)) as u32;
        draws += 1;
        let Some(y) = f.sqrt(peval(&f, &ORIGINAL, &x)) else {
            println!("DRAW id={draw} x={x} square=0");
            continue;
        };
        let point = chart.point(x, y);
        let conjugate = chart.point(f.pow(x, 3), f.pow(y, 3));
        assert_eq!(chart.frobenius(&point), h.sub(&conjugate, &chart.post));
        let d = h.mul(&point, cofactor);
        assert!(h.is_identity(&h.mul(&d, 8)));
        let top = h.mul(&d, 4);
        let independent = !span.contains(&top);
        println!(
            "DRAW id={draw} x={x} y={y} D={:?} top={:?} independent={}",
            key(&d),
            key(&top),
            u8::from(independent)
        );
        if independent {
            let extra: Vec<_> = span.iter().map(|a| h.add(a, &top)).collect();
            span.extend(extra);
            basis.push(d);
            if basis.len() == 4 {
                break;
            }
        }
    }
    assert_eq!(basis.len(), 4, "fixed256-draw basis budget exhausted");
    assert_eq!(span.len(), 16);
    assert_eq!(
        span.iter()
            .filter(|d| !h.is_identity(d))
            .map(key)
            .collect::<BTreeSet<_>>(),
        pair_masks.keys().cloned().collect()
    );
    for a in &basis {
        assert!(h.is_valid(a));
        assert_eq!(h.add(a, &h.neg(a)), h.identity());
        for b in &basis {
            assert_eq!(h.add(a, b), h.add(b, a));
            for c in &basis {
                assert_eq!(h.add(&h.add(a, b), c), h.add(a, &h.add(b, c)));
            }
        }
    }
    let multiples: Vec<Vec<_>> = basis
        .iter()
        .map(|b| (0..8).map(|n| h.mul(b, n)).collect())
        .collect();
    let mut table = Vec::new();
    let mut lookup = BTreeMap::new();
    for id in 0..4096 {
        let mut d = h.identity();
        for (j, values) in multiples.iter().enumerate() {
            d = h.add(&d, &values[(id >> (3 * j)) & 7]);
        }
        assert!(h.is_valid(&d) && d.u.len() <= 3);
        assert!(h.is_identity(&h.mul(&d, 8)));
        assert_eq!(h.add(&d, &h.neg(&d)), h.identity());
        assert!(lookup.insert(key(&d), id).is_none());
        table.push(d);
    }
    assert_eq!(lookup.len(), 4096);
    let mut matrix = [[0_u8; 4]; 4];
    for (j, b) in basis.iter().enumerate() {
        let image = chart.frobenius(b);
        let code = *lookup.get(&key(&image)).unwrap();
        for (i, row) in matrix.iter_mut().enumerate() {
            row[j] = ((code >> (3 * i)) & 7) as u8;
        }
        println!(
            "BASIS j={j} D={:?} F={:?} Fcode={code}",
            key(b),
            key(&image)
        );
        let mut p = h.identity();
        let mut iterate = b.clone();
        for coefficient in [9, 6, 3, 2, 1] {
            p = h.add(&p, &h.mul(&iterate, coefficient));
            iterate = chart.frobenius(&iterate);
        }
        assert!(
            h.is_identity(&p),
            "actual Weil polynomial annihilation failed"
        );
        let mut iterate = b.clone();
        for _ in 0..12 {
            iterate = chart.frobenius(&iterate);
        }
        assert_eq!(&iterate, b);
    }
    println!("FROBENIUS_Z8 rows={matrix:?}");
    let masks: Vec<_> = basis
        .iter()
        .map(|b| pair_masks[&key(&h.mul(b, 4))])
        .collect();
    let mut omega = [[0_u8; 4]; 4];
    for (i, row) in omega.iter_mut().enumerate() {
        for (j, value) in row.iter_mut().enumerate() {
            *value = ((masks[i] & masks[j]).count_ones() & 1) as u8;
        }
    }
    let row_bits = omega.map(|row| row.iter().enumerate().fold(0, |r, (j, v)| r | (v << j)));
    assert_eq!(binary_rank(row_bits), 4);
    for i in 0..4 {
        for j in 0..4 {
            let mut pairing = 0;
            for a in 0..4 {
                for b in 0..4 {
                    pairing ^= (matrix[a][i] & 1) & omega[a][b] & (matrix[b][j] & 1);
                }
            }
            assert_eq!(pairing, omega[i][j]);
        }
    }
    println!("WEIL2 basis_pair_masks={masks:?} omega={omega:?} nondegenerate=pass Fsimilitude_mod2=pass actual_weil8=not_computed");
    let mut images = Vec::new();
    let mut unique_images = BTreeSet::new();
    writeln!(output, "id\tu\tv\tactual_frobenius_id").unwrap();
    for (id, d) in table.iter().enumerate() {
        let image = chart.frobenius(d);
        assert!(h.is_valid(&image));
        let image_id = *lookup.get(&key(&image)).unwrap();
        assert_eq!(
            image_id,
            apply(&matrix, id),
            "full actual coordinate/Frobenius mismatch"
        );
        unique_images.insert(image_id);
        images.push(image_id);
        writeln!(output, "{id}\t{:?}\t{:?}\t{image_id}", d.u, d.v).unwrap();
    }
    output.flush().unwrap();
    assert_eq!(unique_images.len(), 4096);
    for (id, d) in table.iter().enumerate() {
        for (j, b) in basis.iter().enumerate() {
            let shift = 3 * j;
            let digit = (id >> shift) & 7;
            let target = (id & !(7 << shift)) | (((digit + 1) & 7) << shift);
            assert_eq!(h.add(d, b), table[target]);
            assert_eq!(
                h.add(&table[images[id]], &table[images[1 << shift]]),
                table[images[target]]
            );
        }
    }
    let mut fixed_counts = Vec::new();
    for (n, expected) in [(1, 1), (3, 16), (6, 256), (12, 4096)] {
        let fixed = (0..4096)
            .filter(|&id| {
                let mut image = id;
                for _ in 0..n {
                    image = apply(&matrix, image);
                }
                image == id
            })
            .count();
        assert_eq!(fixed, expected);
        fixed_counts.push((n, fixed));
    }
    println!("SUMMARY actual_classes=4096 actual_F_images=4096 generator_edges=16384 J2_pairs=15 basis_draws={draws} fixed={fixed_counts:?} Weil_polynomial=pass F12=pass basepoint_mutant_rejected=pass restoration=pass full_table_private_custody_pending=1");
}
