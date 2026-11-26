//! Lower certificates for the direct Semaev coordinate-row space on tiny cosets.
use super::*;

type Polynomial = Vec<u64>;

fn add(a: &[u64], b: &[u64]) -> Polynomial {
    let mut out = vec![0; a.len().max(b.len())];
    for (slot, &x) in out.iter_mut().zip(a) {
        *slot ^= x;
    }
    for (slot, &x) in out.iter_mut().zip(b) {
        *slot ^= x;
    }
    out
}

fn mul(f: &Gf2, a: &[u64], b: &[u64]) -> Polynomial {
    let mut out = vec![0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate().filter(|&(_, x)| *x != 0) {
        for (j, &y) in b.iter().enumerate().filter(|&(_, y)| *y != 0) {
            out[i + j] ^= f.mul(x, y);
        }
    }
    out
}

fn square(f: &Gf2, a: &[u64]) -> Polynomial {
    let mut out = vec![0; 2 * a.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        out[2 * i] = f.sqr(x);
    }
    out
}

fn resultant_step(f: &Gf2, previous: &[u64], x: u64) -> Polynomial {
    // The fixed homogeneous degree also accounts for factors at infinity.
    let degree = previous.len() - 1;
    let x2 = f.sqr(x);
    let a = [x2, 0, 1];
    let b = [0, x];
    let ac = mul(f, &a, &[1, 0, x2]);
    // Reduce A^degree P(Z/A) using Z² + BZ + AC.
    let mut u = vec![0];
    let mut v = vec![previous[degree]];
    let mut a_power = vec![1];
    for &coefficient in previous[..degree].iter().rev() {
        a_power = mul(f, &a_power, &a);
        let weighted: Vec<_> = a_power.iter().map(|&el| f.mul(el, coefficient)).collect();
        (u, v) = (add(&v, &mul(f, &b, &u)), add(&weighted, &mul(f, &ac, &u)));
    }
    let uv = mul(f, &u, &v);
    let norm = add(
        &add(&square(f, &v), &mul(f, &b, &uv)),
        &mul(f, &ac, &square(f, &u)),
    );
    // Divide out A^degree exactly. Keeping a zero leading coefficient
    // preserves the homogeneous degree for the next resultant.
    let a_degree = a_power.len() - 1;
    assert_eq!(a_power[a_degree], 1);
    let mut remainder = norm;
    let mut out = vec![0; 2 * degree + 1];
    for j in (a_degree..remainder.len()).rev() {
        let coefficient = remainder[j];
        if coefficient != 0 {
            out[j - a_degree] = coefficient;
            for (i, &divisor) in a_power.iter().enumerate() {
                remainder[j - a_degree + i] ^= f.mul(coefficient, divisor);
            }
        }
    }
    assert!(remainder.iter().all(|&x| x == 0));
    out
}

pub(super) fn evaluation(f: &Gf2, polynomial: &[u64], t: u64) -> u64 {
    polynomial
        .iter()
        .rev()
        .fold(0, |acc, &coefficient| f.mul(acc, t) ^ coefficient)
}

pub(super) fn polynomial<'a>(
    f: &Gf2,
    values: &[u64],
    key: &[u8],
    cache: &'a mut HashMap<Vec<u8>, Polynomial>,
) -> &'a Polynomial {
    if !cache.contains_key(key) {
        let (&last, prefix) = key.split_last().unwrap();
        let x = values[last as usize];
        let p = if prefix.is_empty() {
            vec![x, 1]
        } else {
            let prior = polynomial(f, values, prefix, cache);
            resultant_step(f, prior, x)
        };
        cache.insert(key.to_vec(), p);
    }
    &cache[key]
}

fn field_word_span(words: impl IntoIterator<Item = u64>) -> ([u64; 64], usize) {
    let mut pivots = [0; 64];
    let mut dimension = 0;
    for mut word in words {
        while word != 0 {
            let pivot = &mut pivots[word.ilog2() as usize];
            if *pivot == 0 {
                *pivot = word;
                dimension += 1;
                break;
            }
            word ^= *pivot;
        }
    }
    (pivots, dimension)
}

fn input_witness(values: &[u64], coefficients: &[u64], degree: u32) -> Option<usize> {
    let (pivots, _) = field_word_span(
        coefficients
            .iter()
            .enumerate()
            .filter_map(|(mask, &word)| (mask.count_ones() >= degree).then_some(word)),
    );
    values.iter().enumerate().find_map(|(i, &word)| {
        if word == 0 {
            return None;
        }
        let mut remainder = word;
        while remainder != 0 {
            let pivot = pivots[remainder.ilog2() as usize];
            if pivot == 0 {
                return None;
            }
            remainder ^= pivot;
        }
        Some(i)
    })
}

const EVAL_WORDS: usize = 4;

fn evaluation_row(monomials: &[usize], point: usize) -> [u64; EVAL_WORDS] {
    let mut row = [0; EVAL_WORDS];
    for (i, &mask) in monomials.iter().enumerate() {
        if mask & point == mask {
            row[i / 64] |= 1 << (i % 64);
        }
    }
    row
}

fn reduce_evaluation(
    mut row: [u64; EVAL_WORDS],
    pivots: &[[u64; EVAL_WORDS]],
) -> Option<(usize, [u64; EVAL_WORDS])> {
    for j in (0..EVAL_WORDS).rev() {
        while row[j] != 0 {
            let i = 64 * j + row[j].ilog2() as usize;
            if pivots[i][j] == 0 {
                return Some((i, row));
            }
            for (part, &pivot) in row.iter_mut().zip(&pivots[i]) {
                *part ^= pivot;
            }
        }
    }
    None
}

fn quadratic_witness(values: &[u64], width: usize) -> Option<usize> {
    let mut monomials = vec![0];
    for i in 0..width {
        monomials.push(1 << i);
        for j in 0..i {
            monomials.push((1 << i) | (1 << j));
        }
    }
    assert!(monomials.len() <= 64 * EVAL_WORDS);
    let mut pivots = vec![[0; EVAL_WORDS]; monomials.len()];
    for (i, &value) in values.iter().enumerate() {
        if value == 0 {
            let row = evaluation_row(&monomials, i);
            if let Some((lead, remainder)) = reduce_evaluation(row, &pivots) {
                pivots[lead] = remainder;
            }
        }
    }
    values.iter().enumerate().find_map(|(i, &value)| {
        let row = || evaluation_row(&monomials, i);
        (value != 0 && reduce_evaluation(row(), &pivots).is_none()).then_some(i)
    })
}

fn higher_basis_lead(values: &[u64], width: usize) -> Option<usize> {
    let points: Vec<_> = values
        .iter()
        .enumerate()
        .filter_map(|(i, &value)| (value == 0).then_some(i))
        .collect();
    let mut pivots = vec![Vec::new(); points.len()];
    let mut leads = Vec::new();
    let max_degree = (points.len() + 1).ilog2();
    for degree in 0..=max_degree {
        for mask in (0..(1usize << width))
            .rev()
            .filter(|m| m.count_ones() == degree)
        {
            if leads.iter().any(|&lead| mask & lead == lead) {
                continue;
            }
            let mut column = vec![0u64; points.len().div_ceil(64)];
            for (i, &point) in points.iter().enumerate() {
                if mask & point == mask {
                    column[i / 64] |= 1 << (i % 64);
                }
            }
            let mut independent = false;
            for j in (0..column.len()).rev() {
                while column[j] != 0 {
                    let i = 64 * j + column[j].ilog2() as usize;
                    if pivots[i].is_empty() {
                        pivots[i] = column.clone();
                        independent = true;
                        break;
                    }
                    for (part, &pivot) in column.iter_mut().zip(&pivots[i]) {
                        *part ^= pivot;
                    }
                }
                if independent {
                    break;
                }
            }
            if !independent {
                if degree >= 3 {
                    return Some(mask);
                }
                leads.push(mask);
            }
        }
    }
    None
}

pub(super) fn signed_sum(c: &Coset, points: &[Point]) -> BTreeSet<Option<Point>> {
    let mut sums = BTreeSet::from([None]);
    for &point in points {
        let mut next = BTreeSet::new();
        for previous in sums {
            for p in [point, negate(point)] {
                next.insert(previous.map_or(Some(p), |q| c.add(q, p, None)));
            }
        }
        sums = next;
    }
    sums
}

fn baseline(n: u32, k: u32, modulus: u64, summands: usize) {
    let c = Coset::new(n, k, modulus);
    let values: Vec<_> = span(&c.basis).into_iter().map(|u| u ^ c.shift).collect();
    let rational: Vec<_> = values
        .iter()
        .enumerate()
        .filter_map(|(i, &x)| c.lift(x).map(|point| (i as u8, point)))
        .collect();
    assert!(!rational.is_empty());
    let mut cache = HashMap::new();
    for i in 0..values.len() {
        for j in i..values.len() {
            let p = polynomial(&c.f, &values, &[i as u8, j as u8], &mut cache);
            for t in [0, 1, c.shift] {
                assert_eq!(evaluation(&c.f, p, t), c.s3(values[i], values[j], t));
            }
        }
    }
    let mut seed = 0x94ad12f1 ^ ((n as u64) << 8) ^ summands as u64;
    let mut targets = BTreeSet::new();
    for _ in 0..24 {
        let entries: Vec<_> = (0..summands)
            .map(|_| rational[random(&mut seed) % rational.len()])
            .collect();
        let sums = signed_sum(&c, &entries.iter().map(|e| e.1).collect::<Vec<_>>());
        let mut key: Vec<_> = entries.iter().map(|e| e.0).collect();
        key.sort_unstable();
        let p = polynomial(&c.f, &values, &key, &mut cache);
        let mut reversed = entries.iter().rev();
        let mut control = vec![values[reversed.next().unwrap().0 as usize], 1];
        for &(position, _) in reversed {
            control = resultant_step(&c.f, &control, values[position as usize]);
        }
        assert_eq!(&control, p);
        for point in sums.iter().flatten() {
            targets.insert(point.0);
            assert_eq!(evaluation(&c.f, p, point.0), 0);
        }
        for _ in 0..6 {
            let t = random(&mut seed) as u64 & ((1 << n) - 1);
            assert_eq!(
                evaluation(&c.f, p, t) == 0,
                sums.iter().flatten().any(|point| point.0 == t)
            );
        }
    }
    let width = k as usize * summands;
    let started = Instant::now();
    let mut keys = Vec::with_capacity(1 << width);
    for bits in 0..(1 << width) {
        let mut key: Vec<_> = (0..summands)
            .map(|i| ((bits >> (i * k as usize)) & ((1 << k) - 1)) as u8)
            .collect();
        key.sort_unstable();
        polynomial(&c.f, &values, &key, &mut cache);
        keys.push(key);
    }
    let setup = started.elapsed().as_secs_f64();
    for &t in targets.iter().take(4) {
        let started = Instant::now();
        let values: Vec<_> = keys
            .iter()
            .map(|key| evaluation(&c.f, &cache[key], t))
            .collect();
        let roots = values.iter().filter(|&&v| v == 0).count();
        assert!(roots > 0 && roots < values.len());
        let quadratic = quadratic_witness(&values, width);
        let higher_lead = if width >= 20 && quadratic.is_none() {
            higher_basis_lead(&values, width)
        } else {
            None
        };
        let as_json =
            |value: Option<usize>| value.map_or_else(|| "null".to_string(), |x| x.to_string());
        let mut anf = values.clone();
        for i in 0..width {
            let stride = 1 << i;
            for chunk in anf.chunks_exact_mut(2 * stride) {
                let (left, right) = chunk.split_at_mut(stride);
                for (output, &value) in right.iter_mut().zip(left.iter()) {
                    *output ^= value;
                }
            }
        }
        let dimension = field_word_span(anf.iter().copied()).1;
        let rank_at = |degree| {
            field_word_span(anf.iter().enumerate().filter_map(|(mask, &word)| {
                ((mask.count_ones() as usize) >= degree).then_some(word)
            }))
            .1
        };
        let minimum = (0..=width)
            .take_while(|&d| rank_at(d) == dimension)
            .last()
            .unwrap();
        let maximum = anf
            .iter()
            .enumerate()
            .filter_map(|(mask, &word)| (word != 0).then_some(mask.count_ones()))
            .max()
            .unwrap();
        let (lower_degree, witness) = (minimum as u32..=maximum)
            .rev()
            .find_map(|degree| input_witness(&values, &anf, degree).map(|point| (degree, point)))
            .unwrap();
        println!(
            concat!(
                "{{\"n\":{},\"k\":{},\"summands\":{},\"target\":{},\"variables\":{},\"roots\":{},",
                "\"input_rank\":{},\"rank_at_degree_ten\":{},\"minimum_input_degree\":{},",
                "\"maximum_input_degree\":{},\"quadratic_witness\":{},",
                "\"higher_reduced_basis_lead\":{},",
                "\"solving_degree_lower_bound\":{},\"input_witness\":{},",
                "\"polynomial_setup_seconds\":{:.8},\"certificate_seconds\":{:.8}}}"
            ),
            n,
            k,
            summands,
            t,
            width,
            roots,
            dimension,
            rank_at(10),
            minimum,
            maximum,
            as_json(quadratic),
            as_json(higher_lead),
            lower_degree,
            witness,
            setup,
            started.elapsed().as_secs_f64()
        );
    }
}

pub(super) fn run() {
    for summands in 3..=6 {
        baseline(9, 3, 0x203, summands);
    }
    for summands in 4..=5 {
        baseline(12, 4, 0x1009, summands);
    }
}
