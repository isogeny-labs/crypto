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

#[test]
fn export_six_retained_whole_maps() {
    let out = PathBuf::from(std::env::var_os("CHAINED_CT35_OUT").expect("fresh explicit output"));
    let existing =
        PathBuf::from(std::env::var_os("CHAINED_CT35_EXISTING").expect("frozen class corpus"));
    assert!(!out.exists());
    std::fs::create_dir_all(&out).unwrap();
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
        std::fs::write(out.join(format!("t{i}-classes.bin")), data).unwrap();
    }
    let first = SevenChain::new();
    let second = SuccessorChain::new();
    let names = [
        "first-forward",
        "first-adjoint",
        "second-forward",
        "second-adjoint",
        "two-forward",
        "two-adjoint",
    ];
    for (case, name) in names.iter().enumerate() {
        let (source, target) = [(0, 1), (1, 0), (1, 2), (2, 1), (0, 2), (2, 0)][case];
        let mut data = b"CT35MAP1".to_vec();
        data.extend([
            101,
            u8::try_from(source).unwrap(),
            u8::try_from(target).unwrap(),
            u8::try_from(case).unwrap(),
        ]);
        data.extend(12317_u16.to_le_bytes());
        let mut distinct = HashSet::new();
        for d in &all[source] {
            let image = match case {
                0 => first.forward(d).unwrap(),
                1 => first.dual(d).unwrap(),
                2 => second.forward(d).unwrap(),
                3 => second.dual(d).unwrap(),
                4 => second.forward(&first.forward(d).unwrap()).unwrap(),
                5 => first.dual(&second.dual(d).unwrap()).unwrap(),
                _ => unreachable!(),
            };
            let index = indices[target][&key(&image)];
            assert!(distinct.insert(index));
            data.extend(index.to_le_bytes());
        }
        assert_eq!(distinct.len(), 12317);
        assert_eq!(data.len(), 14 + 2 * 12317);
        std::fs::write(out.join(format!("{name}.bin")), data).unwrap();
        println!("PRODUCER case{case} {name}:all12317 retained old outputs T{source}→T{target};bijection;arbitrary class indices,no logarithm labels;independent replay pending");
    }
    println!("PRODUCER all3 corpora/all6 old maps exported;T0/T1 classes byte-identical to frozenCT27;independent whole-map/kernel replay remains pending,no timing claim");
}
