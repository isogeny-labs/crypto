#[allow(dead_code)]
#[path = "h116_field.rs"]
mod h116_field;

use h116_field::{first_irreducible, Felt, Field};
use std::collections::HashMap;
use std::fmt::Write as _;
use std::time::Instant;

#[derive(Clone, Copy)]
struct Cell {
    slug: &'static str,
    p: u64,
    degree: usize,
}

const CELLS: [Cell; 8] = [
    Cell { slug: "fp5-a2-b1-d2", p: 5, degree: 2 },
    Cell { slug: "fp5-a2-b1-d3", p: 5, degree: 3 },
    Cell { slug: "fp5-a2-b1-d4", p: 5, degree: 4 },
    Cell { slug: "fp7-a1-b1-d2", p: 7, degree: 2 },
    Cell { slug: "fp7-a1-b1-d3", p: 7, degree: 3 },
    Cell { slug: "fp7-a1-b1-d4", p: 7, degree: 4 },
    Cell { slug: "fp11-a1-b1-d2", p: 11, degree: 2 },
    Cell { slug: "fp11-a1-b1-d3", p: 11, degree: 3 },
];

fn require(condition: bool, message: impl Into<String>) -> Result<(), String> {
    if condition { Ok(()) } else { Err(message.into()) }
}

fn gcd(mut left: u64, mut right: u64) -> u64 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left
}

fn scalar_mod(value: i64, modulus: u64) -> u64 {
    let modulus_i = i128::from(modulus);
    u64::try_from((i128::from(value) % modulus_i + modulus_i) % modulus_i)
        .expect("reduced scalar")
}

fn pow_mod(mut base: u64, mut exponent: usize, modulus: u64) -> u64 {
    let mut result = 1 % modulus;
    while exponent != 0 {
        if exponent & 1 == 1 { result = result * base % modulus; }
        base = base * base % modulus;
        exponent >>= 1;
    }
    result
}

fn evaluate(field: &Field, polynomial: &[Felt], value: Felt) -> Felt {
    polynomial.iter().rev().fold(field.zero(), |accumulator, &coefficient| {
        field.add(field.mul(accumulator, value), coefficient)
    })
}

fn derivative_evaluate(field: &Field, polynomial: &[Felt], value: Felt) -> Felt {
    polynomial.iter().enumerate().skip(1).rev().fold(
        field.zero(),
        |accumulator, (index, &coefficient)| {
            field.add(
                field.mul(accumulator, value),
                field.mul(field.from_base(index as u64), coefficient),
            )
        },
    )
}

fn divide_by_x_minus_root(field: &Field, polynomial: &[Felt], root: Felt) -> Result<Vec<Felt>, String> {
    require(polynomial.len() >= 2, "cannot divide a constant polynomial")?;
    let degree = polynomial.len() - 1;
    let mut quotient = vec![field.zero(); degree];
    quotient[degree - 1] = polynomial[degree];
    for index in (1..degree).rev() {
        quotient[index - 1] = field.add(polynomial[index], field.mul(root, quotient[index]));
    }
    let remainder = field.add(polynomial[0], field.mul(root, quotient[0]));
    require(remainder == field.zero(), "synthetic division left nonzero remainder")?;
    Ok(quotient)
}

fn root_multiplicity(field: &Field, polynomial: &[Felt], root: Felt) -> Result<usize, String> {
    let mut quotient = polynomial.to_vec();
    let mut multiplicity = 0_usize;
    while quotient.len() >= 2 && evaluate(field, &quotient, root) == field.zero() {
        quotient = divide_by_x_minus_root(field, &quotient, root)?;
        multiplicity += 1;
    }
    Ok(multiplicity)
}

fn parse_base_polynomial(field: &Field, encoded: &str) -> Result<Vec<Felt>, String> {
    encoded.split(',').map(|value| {
        let coefficient = value.parse::<u64>().map_err(|error| error.to_string())?;
        require(coefficient < field.p, "parent coefficient is not in the base field")?;
        Ok(field.from_base(coefficient))
    }).collect()
}

fn read_parent(path: &str) -> Result<HashMap<String, HashMap<String, String>>, String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    let mut lines = text.lines();
    let header: Vec<&str> = lines.next().ok_or("empty parent table")?.split('\t').collect();
    let slug_index = header.iter().position(|&name| name == "slug").ok_or("missing slug")?;
    let mut rows = HashMap::new();
    for line in lines {
        let fields: Vec<&str> = line.split('\t').collect();
        require(fields.len() == header.len(), "malformed parent row")?;
        let row = header.iter().zip(&fields)
            .map(|(&name, &value)| (name.to_string(), value.to_string()))
            .collect::<HashMap<_, _>>();
        rows.insert(fields[slug_index].to_string(), row);
    }
    Ok(rows)
}

fn parent_value<'a>(row: &'a HashMap<String, String>, name: &str) -> Result<&'a str, String> {
    row.get(name).map(String::as_str).ok_or_else(|| format!("missing parent field {name}"))
}

fn run_cell(cell: Cell, parent: &HashMap<String, String>) -> Result<(String, bool, bool), String> {
    let started = Instant::now();
    require(parent_value(parent, "p")?.parse::<u64>().map_err(|e| e.to_string())? == cell.p,
        "parent characteristic mismatch")?;
    require(parent_value(parent, "degree")?.parse::<usize>().map_err(|e| e.to_string())? == cell.degree,
        "parent extension degree mismatch")?;
    require(parent_value(parent, "accepted")? == "1", "parent row was not accepted")?;
    let order = parent_value(parent, "base_count")?.parse::<u64>().map_err(|e| e.to_string())?;
    let trace = parent_value(parent, "trace")?.parse::<i64>().map_err(|e| e.to_string())?;
    let parent_poles = parent_value(parent, "pole_x_count")?.parse::<u64>().map_err(|e| e.to_string())?;
    let field = Field::new(cell.p, cell.degree, &first_irreducible(cell.p, cell.degree)?)?;
    let numerator = parse_base_polynomial(&field, parent_value(parent, "numerator_first")?)?;
    let denominator = parse_base_polynomial(&field, parent_value(parent, "denominator_first")?)?;
    let expected_degree = usize::try_from(order - 1).map_err(|_| "degree too large")?;
    require(denominator.len() == expected_degree + 1, "parent denominator length mismatch")?;
    require(*denominator.last().ok_or("empty denominator")? == field.one(), "denominator not monic")?;

    let lambda = scalar_mod(trace - 1, order);
    require(gcd(lambda, order) == 1, "frozen cyclic action is not invertible")?;
    let lambda_power = pow_mod(lambda, cell.degree, order);
    let plus_difference = (lambda_power + order - 1) % order;
    let minus_sum = (lambda_power + 1) % order;
    let g_plus = gcd(order, plus_difference);
    let g_minus = gcd(order, minus_sum);
    let g_intersection = gcd(g_plus, g_minus);
    let union_size = g_plus + g_minus - g_intersection;
    let epsilon = u64::from(order % 2 == 0);
    require((union_size - 1 + epsilon) % 2 == 0, "nonintegral x-orbit prediction")?;
    let predicted_roots = (union_size - 1 + epsilon) / 2;
    let predicted_simple = epsilon;
    let predicted_double = predicted_roots - predicted_simple;
    let predicted_weighted = predicted_simple + 2 * predicted_double;
    let plus_only = g_plus / 2;
    let sign_sensitive = predicted_roots > plus_only;

    field.reset_counters();
    let mut root_indices = Vec::new();
    let mut simple = 0_u64;
    let mut double = 0_u64;
    let mut higher = 0_u64;
    let mut weighted = 0_u64;
    let mut derivative_classification_failures = 0_u64;
    let mut numerator_cancellations = 0_u64;
    let mut mutation_survivors = 0_u64;
    let mut denominator_mutation = denominator.clone();
    denominator_mutation[0] = field.add(denominator_mutation[0], field.one());
    for index in 0..field.q {
        let x = field.from_index(index);
        if evaluate(&field, &denominator, x) != field.zero() { continue; }
        root_indices.push(index);
        let multiplicity = root_multiplicity(&field, &denominator, x)?;
        weighted += u64::try_from(multiplicity).map_err(|_| "multiplicity too large")?;
        let derivative_zero = derivative_evaluate(&field, &denominator, x) == field.zero();
        derivative_classification_failures += u64::from(derivative_zero != (multiplicity >= 2));
        match multiplicity {
            1 => simple += 1,
            2 => double += 1,
            _ => higher += 1,
        }
        numerator_cancellations += u64::from(evaluate(&field, &numerator, x) == field.zero());
        mutation_survivors += u64::from(evaluate(&field, &denominator_mutation, x) == field.zero());
    }
    let observed_roots = u64::try_from(root_indices.len()).map_err(|_| "root count too large")?;
    let non_point_visible = observed_roots.checked_sub(parent_poles).ok_or("parent poles exceed roots")?;
    let root_encoding = if root_indices.is_empty() {
        "-".to_string()
    } else {
        root_indices.iter().map(u64::to_string).collect::<Vec<_>>().join(",")
    };
    let snapshot = field.snapshot();
    let accepted = observed_roots == predicted_roots
        && simple == predicted_simple && double == predicted_double
        && higher == 0 && weighted == predicted_weighted
        && derivative_classification_failures == 0
        && parent_poles == plus_only && non_point_visible == predicted_roots - plus_only
        && numerator_cancellations == 0 && mutation_survivors == 0;
    let fields = vec![
        cell.slug.to_string(), cell.p.to_string(), cell.degree.to_string(), order.to_string(), trace.to_string(),
        lambda.to_string(), lambda_power.to_string(), g_plus.to_string(), g_minus.to_string(),
        g_intersection.to_string(), union_size.to_string(), epsilon.to_string(), plus_only.to_string(),
        predicted_roots.to_string(), predicted_simple.to_string(), predicted_double.to_string(),
        predicted_weighted.to_string(), observed_roots.to_string(), simple.to_string(), double.to_string(),
        higher.to_string(), weighted.to_string(), parent_poles.to_string(), non_point_visible.to_string(),
        u8::from(sign_sensitive).to_string(), root_encoding, numerator_cancellations.to_string(),
        mutation_survivors.to_string(), derivative_classification_failures.to_string(), expected_degree.to_string(),
        snapshot.extension_additions.to_string(), snapshot.extension_multiplications.to_string(),
        started.elapsed().as_nanos().to_string(), u8::from(accepted).to_string(),
    ];
    Ok((format!("{}\n", fields.join("\t")), accepted, sign_sensitive))
}

fn run(parent_path: &str, output_path: &str) -> Result<(), String> {
    let parents = read_parent(parent_path)?;
    require(parents.len() == CELLS.len(), "parent cell count mismatch")?;
    let mut output = String::from(
        "slug\tp\tdegree\tbase_count\ttrace\tlambda\tlambda_power\tg_plus\tg_minus\tg_intersection\tunion_size\torder_two_indicator\tplus_only_roots\tpredicted_roots\tpredicted_simple\tpredicted_double\tpredicted_weighted\tobserved_roots\tobserved_simple\tobserved_double\tobserved_higher\tobserved_weighted\tparent_point_visible_roots\tnon_point_visible_roots\tsign_sensitive\troot_indices\tnumerator_cancellations\tmutation_survivors\tderivative_classification_failures\tdenominator_degree\textension_additions\textension_multiplications\telapsed_ns\taccepted\n",
    );
    let mut accepted = true;
    let mut sensitive = Vec::new();
    for cell in CELLS {
        let parent = parents.get(cell.slug).ok_or_else(|| format!("missing parent row {}", cell.slug))?;
        let (row, cell_accepted, sign_sensitive) = run_cell(cell, parent)?;
        write!(output, "{row}").map_err(|error| error.to_string())?;
        accepted &= cell_accepted;
        if sign_sensitive { sensitive.push(cell.slug); }
    }
    std::fs::write(output_path, output).map_err(|error| error.to_string())?;
    require(accepted, "one or more kernel x-orbit cells failed")?;
    require(sensitive == ["fp5-a2-b1-d3", "fp7-a1-b1-d2"],
        "wrong sign-sensitive control set")?;
    println!("h116 kernel x-orbit: 8/8 cells accepted; 2/2 sign-sensitive controls rejected plus-only");
    Ok(())
}

fn selftest() -> Result<(), String> {
    require(gcd(14, 8) == 2, "gcd selftest failed")?;
    require(pow_mod(5, 3, 7) == 6, "modular power selftest failed")?;
    let field = Field::new(5, 2, &first_irreducible(5, 2)?)?;
    let root = field.from_index(7);
    let polynomial = vec![field.square(root), field.neg(field.add(root, root)), field.one()];
    require(root_multiplicity(&field, &polynomial, root)? == 2, "root multiplicity selftest failed")?;
    require(derivative_evaluate(&field, &polynomial, root) == field.zero(), "derivative selftest failed")?;
    println!("h116 kernel x-orbit selftest ok");
    Ok(())
}

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let result = match arguments.as_slice() {
        [_, flag] if flag == "--selftest" => selftest(),
        [_, parent, output] => run(parent, output),
        _ => Err("usage: h116_kernel_x_orbit [--selftest | PARENT.tsv OUTPUT.tsv]".to_string()),
    };
    if let Err(error) = result { eprintln!("error: {error}"); std::process::exit(1); }
}
