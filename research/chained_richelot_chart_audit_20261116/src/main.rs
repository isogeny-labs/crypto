//! Separate implementation of protocol11. No producer library is imported.
//! Mathematical inputs are the frozen CSV; timing text is metadata only.
use std::{env, fmt, fs, process};

type F = u128;
const P: F = 101;
const TESTS: [&str; 4] = [
    "canonical_inverse_and_domain_controls",
    "every_frozen_curve_full_rational_chart_and_projective_identity",
    "normalization_reversal_coefficient_and_invalid_point_controls",
    "opposite_sign_is_a_valid_curve_isomorphism_not_a_dual_orientation_certificate",
];

fn add(a: F, b: F) -> F {
    (a + b) % P
}
fn sub(a: F, b: F) -> F {
    (a + P - b) % P
}
fn mul(a: F, b: F) -> F {
    a * b % P
}
fn power(a: F, n: usize) -> F {
    (0..n).fold(1, |v, _| mul(v, a))
}
fn inverse(a: F) -> Result<F, String> {
    if a == 0 || a >= P {
        return Err("Euclid requires a nonzero canonical element".into());
    }
    let (mut r0, mut r1) = (P as i128, a as i128);
    let (mut t0, mut t1) = (0_i128, 1_i128);
    while r1 != 0 {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (t0, t1) = (t1, t0 - q * t1);
    }
    if r0 != 1 {
        return Err("Euclid returned a nonunit".into());
    }
    Ok(t0.rem_euclid(P as i128) as F)
}
fn evaluate(c: &[F; 7], x: F) -> F {
    c.iter()
        .enumerate()
        .fold(0, |s, (i, &v)| add(s, mul(v, power(x, i))))
}
fn homogeneous(c: &[F; 7], u: F, v: F) -> F {
    c.iter().enumerate().fold(0, |s, (i, &coefficient)| {
        add(s, mul(coefficient, mul(power(u, i), power(v, 6 - i))))
    })
}

/// Sylvester determinant of a degree-six polynomial and its derivative.
/// This executable computes the finite determinant, not a generic theorem.
fn sylvester(c: &[F; 7]) -> Result<F, String> {
    let mut m = [[0_u128; 11]; 11];
    for (shift, row) in m.iter_mut().take(5).enumerate() {
        for (j, &coefficient) in c.iter().rev().enumerate() {
            row[shift + j] = coefficient;
        }
    }
    let derivative: Vec<_> = (1..=6).map(|i| mul(i as F, c[i])).collect();
    for (shift, row) in m.iter_mut().skip(5).enumerate() {
        for (j, &coefficient) in derivative.iter().rev().enumerate() {
            row[shift + j] = coefficient;
        }
    }
    let mut determinant = 1;
    for column in 0..11 {
        let Some(pivot) = (column..11).find(|&row| m[row][column] != 0) else {
            return Ok(0);
        };
        if pivot != column {
            m.swap(column, pivot);
            determinant = sub(0, determinant);
        }
        let pivot_row = m[column];
        determinant = mul(determinant, pivot_row[column]);
        let pivot_inverse = inverse(pivot_row[column])?;
        for row in m.iter_mut().skip(column + 1) {
            let factor = mul(row[column], pivot_inverse);
            for (j, entry) in row.iter_mut().enumerate().skip(column + 1) {
                *entry = sub(*entry, mul(factor, pivot_row[j]));
            }
            row[column] = 0;
        }
    }
    Ok(determinant)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Point {
    A(F, F),
    I(F),
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::A(x, y) => write!(f, "A,{x},{y}"),
            Self::I(t) => write!(f, "I,{t},-"),
        }
    }
}
impl Point {
    fn opposite(self) -> Self {
        match self {
            Self::A(x, y) => Self::A(x, sub(0, y)),
            Self::I(t) => Self::I(sub(0, t)),
        }
    }
}

fn square_table() -> Vec<Vec<F>> {
    let mut table = vec![Vec::new(); P as usize];
    for y in 0..P {
        table[mul(y, y) as usize].push(y);
    }
    table
}
fn points(c: &[F; 7], squares: &[Vec<F>]) -> Vec<Point> {
    let mut out = Vec::new();
    for x in 0..P {
        for &y in &squares[evaluate(c, x) as usize] {
            out.push(Point::A(x, y));
        }
    }
    out.extend(squares[c[6] as usize].iter().copied().map(Point::I));
    out
}
fn belongs(c: &[F; 7], point: Point) -> bool {
    match point {
        Point::A(x, y) => mul(y, y) == evaluate(c, x),
        Point::I(t) => t != 0 && mul(t, t) == c[6],
    }
}

/// Expand sum h_i U^(6-i) (1+aU)^i through successive multiplications.
/// This deliberately does not use binomial coefficients or producer code.
fn transform(c: &[F; 7], a: F, b: F) -> Result<[F; 7], String> {
    let mut out = [0; 7];
    let mut term = vec![1];
    for (i, &coefficient) in c.iter().enumerate() {
        for (j, &v) in term.iter().enumerate() {
            out[6 - i + j] = add(out[6 - i + j], mul(coefficient, v));
        }
        if i != 6 {
            let mut next = vec![0; term.len() + 1];
            for (j, &v) in term.iter().enumerate() {
                next[j] = add(next[j], v);
                next[j + 1] = add(next[j + 1], mul(a, v));
            }
            term = next;
        }
    }
    let scale = inverse(mul(b, b))?;
    for coefficient in &mut out {
        *coefficient = mul(*coefficient, scale);
    }
    Ok(out)
}
fn forward(a: F, b: F, point: Point) -> Result<Point, String> {
    let bi = inverse(b)?;
    Ok(match point {
        Point::I(t) => Point::A(0, mul(t, bi)),
        Point::A(x, y) if x == a => Point::I(mul(y, bi)),
        Point::A(x, y) => {
            let u = inverse(sub(x, a))?;
            Point::A(u, mul(mul(y, power(u, 3)), bi))
        }
    })
}
fn backward(a: F, b: F, point: Point) -> Result<Point, String> {
    Ok(match point {
        Point::I(t) => Point::A(a, mul(b, t)),
        Point::A(0, v) => Point::I(mul(b, v)),
        Point::A(u, v) => Point::A(add(a, inverse(u)?), mul(mul(b, v), inverse(power(u, 3))?)),
    })
}

fn fixtures() -> Result<Vec<(String, [F; 7])>, String> {
    let mut lines = include_str!("../fixtures-11.csv").lines();
    if lines.next() != Some("name,h0,h1,h2,h3,h4,h5,h6") {
        return Err("frozen fixture header".into());
    }
    let mut out = Vec::new();
    for line in lines {
        let fields: Vec<_> = line.split(',').collect();
        if fields.len() != 8 || fields[0].is_empty() {
            return Err("fixture shape".into());
        }
        let mut coefficients = [0; 7];
        for (entry, raw) in coefficients.iter_mut().zip(&fields[1..]) {
            *entry = raw.parse::<F>().map_err(|_| "fixture integer")?;
            if *entry >= P || entry.to_string() != *raw {
                return Err("noncanonical fixture".into());
            }
        }
        if coefficients[6] == 0 {
            return Err("fixture degree".into());
        }
        if out.iter().any(|(name, _)| name == fields[0]) {
            return Err("duplicate fixture".into());
        }
        out.push((fields[0].to_owned(), coefficients));
    }
    if out.len() != 12 {
        return Err("fixture coverage".into());
    }
    Ok(out)
}

struct Reconstruction {
    events: Vec<String>,
    observations: Vec<String>,
}
fn reconstruct() -> Result<Reconstruction, String> {
    let squares = square_table();
    let mut events = Vec::new();
    let mut sign_events = Vec::new();
    let mut observations = Vec::new();
    for value in 1..P {
        let inv = inverse(value)?;
        if mul(value, inv) != 1 {
            return Err("independent base inverse check".into());
        }
        events.push(format!("trace_base_inverse,{value},{inv}"));
    }
    for (name, source) in fixtures()? {
        let a = (0..P)
            .find(|&x| {
                let value = evaluate(&source, x);
                value != 0 && !squares[value as usize].is_empty()
            })
            .ok_or("no unramified rational chart parameter")?;
        let b = *squares[evaluate(&source, a) as usize]
            .first()
            .ok_or("missing square root")?;
        if b == 0 {
            return Err("ramified chart parameter".into());
        }
        let target = transform(&source, a, b)?;
        let source_determinant = sylvester(&source)?;
        let target_determinant = sylvester(&target)?;
        if source_determinant == 0 || target_determinant == 0 || target[6] != 1 {
            return Err(format!("{name}: independent degree/smoothness/monicity"));
        }
        let input = points(&source, &squares);
        let output = points(&target, &squares);
        let scale = inverse(mul(b, b))?;
        for (u, v) in (0..P).map(|u| (u, 1)).chain(std::iter::once((1, 0))) {
            let actual = homogeneous(&target, u, v);
            let expected = mul(homogeneous(&source, add(mul(a, u), v), u), scale);
            if actual != expected {
                return Err(format!(
                    "{name}: independently constructed homogeneous identity"
                ));
            }
            events.push(format!(
                "trace_projective,{name},{u},{v},{actual},{expected}"
            ));
        }
        let mut images = Vec::new();
        for &point in &input {
            let image = forward(a, b, point)?;
            let recovered = backward(a, b, image)?;
            if !belongs(&target, image) || recovered != point || images.contains(&image) {
                return Err(format!(
                    "{name}: independent forward equation/roundtrip/injectivity"
                ));
            }
            images.push(image);
            events.push(format!("trace_forward,{name},{point},{image},{recovered}"));
            let negative = forward(a, sub(0, b), point)?;
            if negative != image.opposite() || backward(a, sub(0, b), negative)? != point {
                return Err(format!("{name}: independent sign relation"));
            }
            sign_events.push(format!("trace_sign,{name},{point},{image},{negative}"));
        }
        if images.len() != output.len() || output.iter().any(|point| !images.contains(point)) {
            return Err(format!("{name}: independent full image coverage"));
        }
        for &point in &output {
            let preimage = backward(a, b, point)?;
            let recovered = forward(a, b, preimage)?;
            if !belongs(&source, preimage) || recovered != point {
                return Err(format!("{name}: independent inverse equation/roundtrip"));
            }
            events.push(format!(
                "trace_inverse,{name},{point},{preimage},{recovered}"
            ));
        }
        events.push(format!("fixture={name}; a={a}; b={b}; source_points={}; target_points={}; target_coefficients={target:?}; projective_arguments=102; full_roundtrips=ok; lost_infinity_control=rejected", input.len(), output.len()));
        sign_events.push(format!(
            "fixture={name}; both_b_signs_valid=true; dual_orientation=unestablished"
        ));
        observations.push(format!("audit_fixture={name}; a={a}; b={b}; input_points={}; output_points={}; input_sylvester={source_determinant}; output_sylvester={target_determinant}; coefficients={target:?}", input.len(), output.len()));
    }
    events.extend(sign_events);
    Ok(Reconstruction {
        events,
        observations,
    })
}

fn audit(text: &str) -> Result<Vec<String>, String> {
    let mut starts = [0; 4];
    let mut running = 0;
    let mut summary = 0;
    let mut events = Vec::new();
    for line in text.lines() {
        let mut payload = line;
        if let Some(rest) = line.strip_prefix("test tests::") {
            let (name, body) = rest.split_once(" ... ").ok_or("malformed test start")?;
            let index = TESTS
                .iter()
                .position(|&x| x == name)
                .ok_or("unknown test start")?;
            starts[index] += 1;
            payload = body;
        }
        if line == "running 4 tests" {
            running += 1;
        } else if line.starts_with("running ") {
            return Err("wrong harness test population".into());
        }
        if let Some(rest) = line.strip_prefix("test result:") {
            let timing = rest
                .strip_prefix(
                    " ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in ",
                )
                .and_then(|s| s.strip_suffix('s'))
                .ok_or("nonpassing harness summary")?;
            if timing.is_empty()
                || !timing.chars().all(|c| c.is_ascii_digit() || c == '.')
                || !timing
                    .parse::<f64>()
                    .is_ok_and(|v| v.is_finite() && v >= 0.0)
            {
                return Err("malformed elapsed metadata".into());
            }
            summary += 1;
        }
        if [
            "trace_base_inverse,",
            "trace_projective,",
            "trace_forward,",
            "trace_inverse,",
            "trace_sign,",
            "fixture=",
        ]
        .iter()
        .any(|tag| payload.starts_with(tag))
        {
            events.push(payload.to_owned());
        } else if payload.contains("trace_") || payload.contains("fixture=") {
            return Err("unknown or malformed trace record".into());
        }
    }
    if running != 1 || summary != 1 || starts != [1; 4] {
        return Err(format!(
            "harness coverage running={running} summary={summary} starts={starts:?}"
        ));
    }
    let reconstructed = reconstruct()?;
    for (i, (actual, expected)) in events.iter().zip(&reconstructed.events).enumerate() {
        if actual != expected {
            return Err(format!(
                "ordered event {i}: observed {actual:?}; expected {expected:?}"
            ));
        }
    }
    if events.len() != reconstructed.events.len() {
        return Err(format!(
            "event coverage {} != {}",
            events.len(),
            reconstructed.events.len()
        ));
    }
    let mut observations = reconstructed.observations;
    observations.push(format!("audit=accepted; fixtures=12; ordered_events={}; base_inverse=100; projective=1224; forward=1440; inverse=1440; sign=1440; model_summaries=12; sign_summaries=12; elapsed_metadata=excluded; dual_orientation=unestablished", events.len()));
    Ok(observations)
}

fn fields(line: &str, tag: &str) -> Option<Vec<String>> {
    line.trim_end_matches('\n').strip_prefix(tag).map(|rest| {
        std::iter::once(tag.trim_end_matches(',').to_owned())
            .chain(rest.split(',').map(str::to_owned))
            .collect()
    })
}
fn replace_fields(lines: &mut [String], index: usize, cells: &[String]) {
    let end = if lines[index].ends_with('\n') {
        "\n"
    } else {
        ""
    };
    lines[index] = cells.join(",") + end;
}
fn make_control(kind: &str, text: &str) -> Result<String, String> {
    let mut lines: Vec<String> = text.split_inclusive('\n').map(str::to_owned).collect();
    match kind {
        "forward-coordinate" => {
            let (i, mut row) = lines
                .iter()
                .enumerate()
                .find_map(|(i, line)| {
                    let row = fields(line, "trace_forward,")?;
                    (row.len() == 11 && row[5] == "A").then_some((i, row))
                })
                .ok_or("no finite forward image")?;
            let y = row[7].parse::<F>().map_err(|_| "coordinate integer")?;
            row[7] = add(y, 1).to_string();
            replace_fields(&mut lines, i, &row);
        }
        "drop-projective" => {
            let i = lines
                .iter()
                .position(|line| line.starts_with("trace_projective,"))
                .ok_or("no standalone projective row")?;
            lines.remove(i);
        }
        "duplicate-forward" => {
            let indices: Vec<_> = lines
                .iter()
                .enumerate()
                .filter_map(|(i, line)| line.starts_with("trace_forward,").then_some(i))
                .take(2)
                .collect();
            if indices.len() != 2 {
                return Err("two forward rows required".into());
            }
            lines[indices[1]] = lines[indices[0]].clone();
        }
        "sign-relation" => {
            let (i, mut row) = lines
                .iter()
                .enumerate()
                .find_map(|(i, line)| {
                    let row = fields(line, "trace_sign,")?;
                    (row.len() == 11 && row[5..8] != row[8..11]).then_some((i, row))
                })
                .ok_or("no differing sign images")?;
            let normal = row[5..8].to_vec();
            row[8..11].clone_from_slice(&normal);
            replace_fields(&mut lines, i, &row);
        }
        "summary-coefficient" => {
            let i = lines
                .iter()
                .position(|line| line.contains("target_coefficients=["))
                .ok_or("no model summary")?;
            let start = lines[i]
                .find("target_coefficients=[")
                .ok_or("coefficient marker")?
                + "target_coefficients=[".len();
            let end = start + lines[i][start..].find(',').ok_or("coefficient separator")?;
            let v = lines[i][start..end]
                .parse::<F>()
                .map_err(|_| "coefficient integer")?;
            lines[i].replace_range(start..end, &add(v, 1).to_string());
        }
        "lost-infinity" => {
            let find = |kind: &str| {
                lines.iter().enumerate().find_map(|(i, line)| {
                    let row = fields(line, "trace_forward,")?;
                    (row.len() == 11 && row[5] == kind).then_some(i)
                })
            };
            let infinity = find("I").ok_or("no infinity image")?;
            let finite = find("A").ok_or("no finite image")?;
            lines[infinity] = lines[finite].clone();
        }
        "timing-only" => {
            let i = lines
                .iter()
                .position(|line| line.starts_with("test result:"))
                .ok_or("no harness summary")?;
            let start =
                lines[i].find("finished in ").ok_or("no elapsed marker")? + "finished in ".len();
            let end = start + lines[i][start..].find('s').ok_or("no elapsed suffix")?;
            lines[i].replace_range(start..end, "999.00");
        }
        _ => return Err("unknown control kind".into()),
    }
    let out = lines.concat();
    if out == text {
        return Err("control did not change the input".into());
    }
    Ok(out)
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let result: Result<(), String> = match args.get(1).map(String::as_str) {
        Some("audit") if args.len() == 3 => {
            let bytes = match fs::read(&args[2]) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("audit I/O: {e}");
                    process::exit(2);
                }
            };
            let accepted = std::str::from_utf8(&bytes)
                .map_err(|_| "non-UTF8 transcript".to_owned())
                .and_then(audit);
            match accepted {
                Ok(rows) => {
                    for row in rows {
                        println!("{row}");
                    }
                    Ok(())
                }
                Err(e) => {
                    eprintln!("audit rejected: {e}");
                    process::exit(1);
                }
            }
        }
        Some("make-control") if args.len() == 5 => fs::read_to_string(&args[3])
            .map_err(|e| format!("control input I/O: {e}"))
            .and_then(|s| make_control(&args[2], &s))
            .and_then(|s| fs::write(&args[4], s).map_err(|e| format!("control output I/O: {e}"))),
        _ => Err("usage: audit_trace audit INPUT | make-control KIND INPUT OUTPUT".into()),
    };
    if let Err(e) = result {
        eprintln!("auditor usage/generation/I/O: {e}");
        process::exit(2);
    }
}
