use std::collections::{BTreeMap, BTreeSet};

const F2_ROWS: [u8; 6] = [2, 3, 8, 12, 32, 48];
const F4_ROWS: [[u8; 6]; 6] = [
    [0, 3, 0, 0, 0, 0],
    [1, 3, 0, 0, 0, 0],
    [0, 0, 0, 3, 0, 0],
    [0, 0, 1, 3, 0, 0],
    [0, 0, 0, 0, 0, 3],
    [0, 0, 0, 0, 1, 3],
];

#[derive(Clone)]
struct LRecord {
    image: u64,
    stable: bool,
    first: bool,
}

#[derive(Clone)]
struct KRecord {
    image: u16,
    order: u8,
}

#[derive(Clone)]
struct Data {
    ls: BTreeMap<u64, LRecord>,
    ks: BTreeMap<u16, KRecord>,
    perp: BTreeSet<u16>,
    doubled: BTreeSet<u16>,
}

struct Model {
    isotropic: BTreeSet<u64>,
    kernel: BTreeSet<u16>,
    two_bits: u64,
    doubled_bits: u64,
    doubled_ids: BTreeSet<u16>,
}

fn field<'a>(line: &'a str, key: &str) -> Result<&'a str, String> {
    line.split_whitespace()
        .find_map(|token| token.strip_prefix(key))
        .ok_or_else(|| format!("missing {key}"))
}

fn list_ids(s: &str) -> Result<BTreeSet<u16>, String> {
    let body = s
        .trim()
        .strip_prefix('[')
        .and_then(|x| x.strip_suffix(']'))
        .ok_or("invalid id list")?;
    let mut out = BTreeSet::new();
    for item in body.split(',') {
        let id = item.trim().parse().map_err(|_| "invalid id")?;
        if !out.insert(id) {
            return Err("duplicate list id".into());
        }
    }
    Ok(out)
}

fn parse(text: &str) -> Result<Data, String> {
    let (mut ls, mut ks) = (BTreeMap::new(), BTreeMap::new());
    let (mut perp, mut doubled) = (None, None);
    for line in text.lines() {
        if line.starts_with("L bits=") {
            let key =
                u64::from_str_radix(field(line, "bits=")?, 16).map_err(|_| "invalid L bits")?;
            let record = LRecord {
                image: u64::from_str_radix(field(line, "F=")?, 16)
                    .map_err(|_| "invalid L image")?,
                stable: field(line, "stable=")?
                    .parse()
                    .map_err(|_| "invalid stable flag")?,
                first: field(line, "first=")?
                    .parse()
                    .map_err(|_| "invalid first flag")?,
            };
            if ls.insert(key, record).is_some() {
                return Err("duplicate L row".into());
            }
        } else if line.starts_with("K point=") {
            let key = field(line, "point=")?.parse().map_err(|_| "invalid K id")?;
            let record = KRecord {
                image: field(line, "F=")?.parse().map_err(|_| "invalid K image")?,
                order: field(line, "order=")?
                    .parse()
                    .map_err(|_| "invalid K order")?,
            };
            if ks.insert(key, record).is_some() {
                return Err("duplicate K row".into());
            }
        } else if let Some(s) = line.strip_prefix("K_perp_complete_ids=") {
            if perp.replace(list_ids(s)?).is_some() {
                return Err("duplicate orthogonal list".into());
            }
        } else if let Some(s) = line.strip_prefix("doubled_complete_ids=") {
            if doubled.replace(list_ids(s)?).is_some() {
                return Err("duplicate doubled list".into());
            }
        }
    }
    Ok(Data {
        ls,
        ks,
        perp: perp.ok_or("missing orthogonal list")?,
        doubled: doubled.ok_or("missing doubled list")?,
    })
}

fn f2(v: u8) -> u8 {
    F2_ROWS.iter().enumerate().fold(0, |out, (i, &r)| {
        out | (((v & r).count_ones() as u8 % 2) << i)
    })
}

fn image(span: u64) -> u64 {
    (0..64)
        .filter(|&v| span & (1_u64 << v) != 0)
        .fold(0, |out, v| out | (1_u64 << f2(v)))
}

fn binary_pair(a: u8, b: u8) -> u32 {
    let forward = (a & 0b010101) & ((b & 0b101010) >> 1);
    let backward = ((a & 0b101010) >> 1) & (b & 0b010101);
    (forward.count_ones() + backward.count_ones()) % 2
}

fn rref_subspaces() -> (BTreeSet<u64>, BTreeSet<u64>) {
    let (mut all, mut isotropic) = (BTreeSet::new(), BTreeSet::new());
    for p0 in 0..4 {
        for p1 in (p0 + 1)..5 {
            for p2 in (p1 + 1)..6 {
                let pivots = [p0, p1, p2];
                let mut free = Vec::new();
                for (row, &pivot) in pivots.iter().enumerate() {
                    for col in (pivot + 1)..6 {
                        if !pivots.contains(&col) {
                            free.push((row, col));
                        }
                    }
                }
                for choices in 0..(1_u32 << free.len()) {
                    let mut rows = [1_u8 << p0, 1_u8 << p1, 1_u8 << p2];
                    for (bit, &(row, col)) in free.iter().enumerate() {
                        if choices & (1 << bit) != 0 {
                            rows[row] |= 1 << col;
                        }
                    }
                    let mut span = 0_u64;
                    for coefficients in 0..8 {
                        let mut value = 0;
                        for (i, &row) in rows.iter().enumerate() {
                            if coefficients & (1 << i) != 0 {
                                value ^= row;
                            }
                        }
                        span |= 1_u64 << value;
                    }
                    assert_eq!(span.count_ones(), 8);
                    assert!(all.insert(span), "RREF representation must be unique");
                    if binary_pair(rows[0], rows[1]) == 0
                        && binary_pair(rows[0], rows[2]) == 0
                        && binary_pair(rows[1], rows[2]) == 0
                    {
                        isotropic.insert(span);
                    }
                }
            }
        }
    }
    (all, isotropic)
}

fn digits(code: u16) -> [u8; 6] {
    std::array::from_fn(|i| ((code >> (2 * i)) & 3) as u8)
}

fn encode(v: &[u8; 6]) -> u16 {
    v.iter()
        .enumerate()
        .map(|(i, &x)| u16::from(x) << (2 * i))
        .sum()
}

fn f4(code: u16) -> u16 {
    let x = digits(code);
    let y = std::array::from_fn(|i| {
        F4_ROWS[i]
            .iter()
            .zip(x)
            .map(|(&a, b)| u16::from(a) * u16::from(b))
            .sum::<u16>() as u8
            % 4
    });
    encode(&y)
}

fn model(isotropic: BTreeSet<u64>) -> Model {
    let kernel: BTreeSet<_> = (0..4096)
        .filter(|&code| {
            let v = digits(code);
            v[0] % 2 == 0
                && v[1] % 2 == 0
                && v[4] == (v[2] + 2 * v[3]) % 4
                && v[5] == (2 * v[2] + 3 * v[3]) % 4
        })
        .collect();
    let (mut two_bits, mut doubled_bits, mut doubled_ids) = (0, 0, BTreeSet::new());
    for &code in &kernel {
        let v = digits(code);
        if v.iter().all(|x| x % 2 == 0) {
            let label: u8 = v.iter().enumerate().map(|(i, &x)| (x / 2) << i).sum();
            two_bits |= 1_u64 << label;
        }
        let label: u8 = v.iter().enumerate().map(|(i, &x)| (x % 2) << i).sum();
        doubled_bits |= 1_u64 << label;
        doubled_ids.insert(encode(&v.map(|x| 2 * x % 4)));
    }
    Model {
        isotropic,
        kernel,
        two_bits,
        doubled_bits,
        doubled_ids,
    }
}

fn validate(data: &Data, model: &Model) -> Result<(), String> {
    if data.ls.keys().copied().collect::<BTreeSet<_>>() != model.isotropic {
        return Err("complete L set differs from independent RREF enumeration".into());
    }
    if data.ks.keys().copied().collect::<BTreeSet<_>>() != model.kernel {
        return Err("complete K set differs from independent linear constraints".into());
    }
    if data.perp != model.kernel || data.doubled != model.doubled_ids {
        return Err("orthogonal/doubled lists differ from kernel constraints".into());
    }
    for (&bits, row) in &data.ls {
        let expected = image(bits);
        let first =
            bits & model.two_bits == bits && bits & model.doubled_bits == model.doubled_bits;
        if row.image != expected || row.stable != (expected == bits) || row.first != first {
            return Err(format!("L image/flags mismatch at {bits:016x}"));
        }
    }
    for (&code, row) in &data.ks {
        let order = if code == 0 {
            1
        } else if digits(code).iter().all(|x| x % 2 == 0) {
            2
        } else {
            4
        };
        if row.image != f4(code) || row.order != order || !model.kernel.contains(&row.image) {
            return Err(format!("K image/order mismatch at {code}"));
        }
    }
    Ok(())
}

fn gram_entry(x: &[i32; 6], y: &[i32; 6]) -> i32 {
    let mut sum = 0;
    for (i, &xi) in x.iter().enumerate() {
        for (j, &yj) in y.iter().enumerate() {
            let matrix_entry = if i % 2 == 0 && j == i + 1 {
                1
            } else if j % 2 == 0 && i == j + 1 {
                -1
            } else {
                0
            };
            sum += xi * matrix_entry * yj;
        }
    }
    sum.rem_euclid(4)
}

fn main() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: verifier full-native-A.stdout".into());
    }
    let data = parse(&std::fs::read_to_string(&args[1]).map_err(|e| e.to_string())?)?;
    let (all, isotropic) = rref_subspaces();
    assert_eq!(all.len(), 1395);
    assert_eq!(isotropic.len(), 135);
    assert_eq!(all.iter().filter(|&&x| image(x) == x).count(), 0);
    for bits in &all {
        println!("R {bits:016x}");
    }
    let model = model(isotropic);
    assert_eq!(model.kernel.len(), 64);
    assert_eq!(model.two_bits.count_ones(), 16);
    assert_eq!(model.doubled_bits.count_ones(), 4);
    let generators = [
        [2, 0, 0, 0, 0, 0],
        [0, 2, 0, 0, 0, 0],
        [0, 0, 1, 0, 1, 2],
        [0, 0, 0, 1, 2, 3],
    ];
    for x in &generators {
        for y in &generators {
            assert_eq!(gram_entry(x, y), 0);
        }
    }
    println!("independent_generator_Gram16=zero; linear_constraint_K_cardinality=64");
    validate(&data, &model)?;
    let mut bad_l = data.clone();
    bad_l.ls.first_entry().unwrap().get_mut().image ^= 1;
    let failure_l = validate(&bad_l, &model).expect_err("corrupt L image must be rejected");
    println!("negative_L_image_control={failure_l}");
    let mut bad_k = data.clone();
    let id = *bad_k.ks.keys().find(|&&x| x != 0).unwrap();
    let row = bad_k.ks.remove(&id).unwrap();
    assert!(bad_k.ks.insert(id ^ 1, row).is_none());
    let failure_k = validate(&bad_k, &model).expect_err("corrupt K id must be rejected");
    println!("negative_K_id_control={failure_k}");
    validate(&data, &model)?;
    println!("canonical_K_ids={:?}", model.kernel);
    println!("RESULT RREF_all_rank3=1395 isotropic=135 all_rank3_F_stable=0 K=64 full_A_tables_match=true both_mutants_rejected=true pristine_restored=true");
    println!("SCOPE second_Rust_algorithm_same_model_and_published_data; no_external_worker_Lean_geometry_or_solver_cost_verdict");
    Ok(())
}
