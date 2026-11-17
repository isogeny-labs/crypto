//! Producer witnesses only;the checker must independently evaluate maps.
use super::{key, SevenChain, SuccessorChain};
use crate::balanced::{Balanced, Class};
use num_traits::ToPrimitive;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

const MODELS: [[u64; 7]; 3] = [
    [31, 75, 74, 54, 71, 55, 1],
    [95, 26, 19, 27, 94, 91, 1],
    [95, 93, 43, 39, 92, 17, 1],
];

fn classes(model: usize, all: &[Class]) -> Vec<u8> {
    let mut out = b"CT27CLS1".to_vec();
    out.push(101);
    out.extend(MODELS[model].map(|x| u8::try_from(x).unwrap()));
    out.extend(u16::try_from(all.len()).unwrap().to_le_bytes());
    let mut degrees = [0_usize; 3];
    for d in all {
        let (u, v, plus) = d.components();
        let degree = u.degree().unwrap();
        assert!(degree <= 2 && usize::from(plus) <= 2 - degree);
        degrees[degree] += 1;
        let coefficient =
            |p: &crate::reference::FpPoly, i| u8::try_from(p.coeff(i).to_u64().unwrap()).unwrap();
        out.extend([
            u8::try_from(3 * degree).unwrap() + plus,
            if degree > 0 { coefficient(u, 0) } else { 0 },
            if degree == 2 { coefficient(u, 1) } else { 0 },
            coefficient(v, 0),
            coefficient(v, 1),
        ]);
    }
    assert_eq!(degrees, [3, 236, 12078]);
    assert_eq!(out.len(), 18 + 5 * 12317);
    out
}

const NAMES: [&str; 6] = [
    "first-forward",
    "first-adjoint",
    "second-forward",
    "second-adjoint",
    "two-forward",
    "two-adjoint",
];
const ENDPOINTS: [(usize, usize); 6] = [(0, 1), (1, 0), (1, 2), (2, 1), (0, 2), (2, 0)];
fn read_map(out: &std::path::Path, case: usize) -> Vec<usize> {
    let data = std::fs::read(out.join(format!("{}.bin", NAMES[case]))).unwrap();
    let (source, target) = ENDPOINTS[case];
    assert_eq!(data.len(), 14 + 2 * 12317);
    assert_eq!(&data[..8], b"CT35MAP1");
    assert_eq!(
        data[8..12],
        [
            101,
            u8::try_from(source).unwrap(),
            u8::try_from(target).unwrap(),
            u8::try_from(case).unwrap()
        ]
    );
    assert_eq!(data[12..14], 12317_u16.to_le_bytes());
    let indices = data[14..]
        .as_chunks::<2>()
        .0
        .iter()
        .map(|r| usize::from(u16::from_le_bytes(*r)))
        .collect::<Vec<_>>();
    assert!(indices.iter().all(|&i| i < 12317));
    assert_eq!(indices.iter().collect::<HashSet<_>>().len(), 12317);
    indices
}
#[test]
fn export_one_retained_map() {
    let out = PathBuf::from(
        std::env::var_os("CHAINED_CT35_OUT").expect("existing frozen complete corpora"),
    );
    let existing = PathBuf::from(std::env::var_os("CHAINED_CT35_EXISTING").expect("frozenCT27"));
    let case = std::env::var("CHAINED_CT35_CASE")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    assert!(case < 6);
    assert!(out.is_dir());
    assert!(!out.join(format!("{}.bin", NAMES[case])).exists());
    let models = MODELS.map(|equation| Balanced::new(101, &equation).unwrap());
    let all = models.each_ref().map(|m| m.classes().unwrap());
    assert!(all.iter().all(|a| a.len() == 12317));
    let indices = all.each_ref().map(|a| {
        a.iter()
            .enumerate()
            .map(|(i, d)| (key(d), u16::try_from(i).unwrap()))
            .collect::<HashMap<_, _>>()
    });
    for i in 0..3 {
        assert_eq!(indices[i].len(), 12317);
        let data = classes(i, &all[i]);
        if i < 2 {
            assert_eq!(
                data,
                std::fs::read(existing.join(format!("t{i}-classes.bin"))).unwrap()
            );
        }
        assert_eq!(
            data,
            std::fs::read(out.join(format!("t{i}-classes.bin"))).unwrap()
        );
    }
    let (source, target) = ENDPOINTS[case];
    let mut data = b"CT35MAP1".to_vec();
    data.extend([
        101,
        u8::try_from(source).unwrap(),
        u8::try_from(target).unwrap(),
        u8::try_from(case).unwrap(),
    ]);
    data.extend(12317_u16.to_le_bytes());
    let mut distinct = HashSet::new();
    if case >= 4 {
        let (left, right) = if case == 4 { (0, 2) } else { (3, 1) };
        assert_eq!(ENDPOINTS[left].0, source);
        assert_eq!(ENDPOINTS[left].1, ENDPOINTS[right].0);
        assert_eq!(ENDPOINTS[right].1, target);
        let first = read_map(&out, left);
        let second = read_map(&out, right);
        for &middle in &first {
            let index = second[middle];
            assert!(distinct.insert(index));
            data.extend(u16::try_from(index).unwrap().to_le_bytes());
        }
    } else {
        let first = SevenChain::new();
        let second = SuccessorChain::new();
        for d in &all[source] {
            let image = match case {
                0 => first.forward(d).unwrap(),
                1 => first.dual(d).unwrap(),
                2 => second.forward(d).unwrap(),
                3 => second.dual(d).unwrap(),
                _ => unreachable!(),
            };
            let index = indices[target][&key(&image)];
            assert!(distinct.insert(usize::from(index)));
            data.extend(index.to_le_bytes());
        }
    }
    assert_eq!(distinct.len(), 12317);
    assert_eq!(data.len(), 14 + 2 * 12317);
    std::fs::write(out.join(format!("{}.bin", NAMES[case])), data).unwrap();
    println!("PRODUCER V2 case{case} {} all12317 retained old outputs/bijection T{source}→T{target},all3corpora byte-identical to01 andT0/T1frozenCT27;composites use exact memoized old index composition;no pure producer evaluator or logarithm labels. Othercases/full independent acceptance separate,no timing claim",NAMES[case]);
}
