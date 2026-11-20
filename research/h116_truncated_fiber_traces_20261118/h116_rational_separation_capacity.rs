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
    a: u64,
    b: u64,
    degree: usize,
}

const CELLS: [Cell; 8] = [
    Cell { slug: "fp5-a2-b1-d2", p: 5, a: 2, b: 1, degree: 2 },
    Cell { slug: "fp5-a2-b1-d3", p: 5, a: 2, b: 1, degree: 3 },
    Cell { slug: "fp5-a2-b1-d4", p: 5, a: 2, b: 1, degree: 4 },
    Cell { slug: "fp7-a1-b1-d2", p: 7, a: 1, b: 1, degree: 2 },
    Cell { slug: "fp7-a1-b1-d3", p: 7, a: 1, b: 1, degree: 3 },
    Cell { slug: "fp7-a1-b1-d4", p: 7, a: 1, b: 1, degree: 4 },
    Cell { slug: "fp11-a1-b1-d2", p: 11, a: 1, b: 1, degree: 2 },
    Cell { slug: "fp11-a1-b1-d3", p: 11, a: 1, b: 1, degree: 3 },
];

fn require(condition: bool, message: impl Into<String>) -> Result<(), String> {
    if condition { Ok(()) } else { Err(message.into()) }
}

fn curve_rhs(field: &Field, a: Felt, b: Felt, x: Felt) -> Felt {
    field.add(
        field.add(field.mul(field.square(x), x), field.mul(a, x)),
        b,
    )
}

fn affine_x_fibers(
    field: &Field,
    a: Felt,
    b: Felt,
) -> Result<(Vec<Felt>, u64, u64), String> {
    let mut samples = Vec::new();
    let mut singleton_fibers = 0_u64;
    let mut paired_fibers = 0_u64;
    for index in 0..field.q {
        let x = field.from_index(index);
        let rhs = curve_rhs(field, a, b, x);
        if let Some(y) = field.sqrt(rhs)? {
            samples.push(x);
            if y == field.zero() {
                singleton_fibers += 1;
            } else {
                paired_fibers += 1;
            }
        }
    }
    Ok((samples, singleton_fibers, paired_fibers))
}

fn base_distinct_x_count(p: u64, a: u64, b: u64) -> u64 {
    (0..p)
        .filter(|&x| {
            let rhs = (((x * x % p) * x % p) + (a * x % p) + b) % p;
            (0..p).any(|y| y * y % p == rhs)
        })
        .count()
        .try_into()
        .expect("base-field count fits u64")
}

fn multiply_by_linear(field: &Field, coefficients: &[Felt], root: Felt) -> Vec<Felt> {
    let mut result = vec![field.zero(); coefficients.len() + 1];
    let negative_root = field.neg(root);
    for (index, &coefficient) in coefficients.iter().enumerate() {
        result[index] = field.add(result[index], field.mul(negative_root, coefficient));
        result[index + 1] = field.add(result[index + 1], coefficient);
    }
    result
}

fn vanishing_polynomial(field: &Field, samples: &[Felt]) -> Vec<Felt> {
    samples.iter().fold(vec![field.one()], |coefficients, &root| {
        multiply_by_linear(field, &coefficients, root)
    })
}

fn evaluate_polynomial(field: &Field, coefficients: &[Felt], value: Felt) -> Felt {
    coefficients.iter().rev().fold(field.zero(), |result, &coefficient| {
        field.add(field.mul(result, value), coefficient)
    })
}

fn encode_coefficients(field: &Field, coefficients: &[Felt]) -> String {
    coefficients
        .iter()
        .map(|&coefficient| field.to_index(coefficient).to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn parent_group_orders(path: &str) -> Result<HashMap<String, u64>, String> {
    let text = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    let mut lines = text.lines();
    let header: Vec<&str> = lines.next().ok_or("empty parent result")?.split('\t').collect();
    let slug_index = header.iter().position(|&name| name == "slug").ok_or("missing slug")?;
    let count_index = header
        .iter()
        .position(|&name| name == "extension_count")
        .ok_or("missing extension_count")?;
    let mut result = HashMap::new();
    for line in lines {
        let fields: Vec<&str> = line.split('\t').collect();
        let count = fields[count_index].parse::<u64>().map_err(|error| error.to_string())?;
        result.insert(fields[slug_index].to_string(), count);
    }
    Ok(result)
}

fn run_cell(cell: Cell, parent_group_order: u64) -> Result<(String, bool, bool), String> {
    let started = Instant::now();
    let modulus = first_irreducible(cell.p, cell.degree)?;
    let field = Field::new(cell.p, cell.degree, &modulus)?;
    let a = field.from_base(cell.a);
    let b = field.from_base(cell.b);
    let (samples, singleton_fibers, paired_fibers) = affine_x_fibers(&field, a, b)?;
    let distinct_x_count = u64::try_from(samples.len()).map_err(|_| "sample too large")?;
    let affine_point_count = singleton_fibers + 2 * paired_fibers;
    let base_x_count = base_distinct_x_count(cell.p, cell.a, cell.b);
    let extension_x_growth = distinct_x_count.saturating_sub(base_x_count);
    let raw_capacity = distinct_x_count.checked_sub(1).ok_or("empty x sample")?;

    field.reset_counters();
    let coefficients = vanishing_polynomial(&field, &samples);
    let build_counts = field.snapshot();

    field.reset_counters();
    let vanishing_eval_failures: u64 = samples
        .iter()
        .filter(|&&x| evaluate_polynomial(&field, &coefficients, x) != field.zero())
        .count()
        .try_into()
        .map_err(|_| "failure count too large")?;
    let vanishing_eval_counts = field.snapshot();

    field.reset_counters();
    let mut mutation = coefficients.clone();
    mutation[0] = field.add(mutation[0], field.one());
    let mutation_rejections: u64 = samples
        .iter()
        .filter(|&&x| evaluate_polynomial(&field, &mutation, x) != field.zero())
        .count()
        .try_into()
        .map_err(|_| "mutation count too large")?;
    let mutation_eval_counts = field.snapshot();

    let vanishing_degree = u64::try_from(coefficients.len() - 1)
        .map_err(|_| "degree too large")?;
    let vanishing_monic = coefficients.last() == Some(&field.one());
    let vanishing_nonzero = coefficients.iter().any(|&coefficient| coefficient != field.zero());
    let fiber_count_identity = distinct_x_count == singleton_fibers + paired_fibers;
    let accepted = parent_group_order == affine_point_count + 1
        && fiber_count_identity
        && distinct_x_count == singleton_fibers + paired_fibers
        && vanishing_degree == distinct_x_count
        && vanishing_monic
        && vanishing_nonzero
        && vanishing_eval_failures == 0
        && mutation_rejections == distinct_x_count;

    let fields = vec![
        cell.slug.to_string(),
        cell.p.to_string(),
        cell.degree.to_string(),
        parent_group_order.to_string(),
        affine_point_count.to_string(),
        singleton_fibers.to_string(),
        paired_fibers.to_string(),
        distinct_x_count.to_string(),
        base_x_count.to_string(),
        extension_x_growth.to_string(),
        format!("{distinct_x_count}/{base_x_count}"),
        raw_capacity.to_string(),
        vanishing_degree.to_string(),
        u8::from(vanishing_monic).to_string(),
        u8::from(vanishing_nonzero).to_string(),
        encode_coefficients(&field, &coefficients),
        vanishing_eval_failures.to_string(),
        mutation_rejections.to_string(),
        build_counts.extension_additions.to_string(),
        build_counts.extension_multiplications.to_string(),
        vanishing_eval_counts.extension_additions.to_string(),
        vanishing_eval_counts.extension_multiplications.to_string(),
        mutation_eval_counts.extension_additions.to_string(),
        mutation_eval_counts.extension_multiplications.to_string(),
        u8::from(fiber_count_identity).to_string(),
        started.elapsed().as_nanos().to_string(),
        u8::from(accepted).to_string(),
    ];
    let row = format!("{}\n", fields.join("\t"));
    Ok((row, accepted, extension_x_growth > 0))
}

fn run(parent_path: &str, output_path: &str) -> Result<(), String> {
    let parents = parent_group_orders(parent_path)?;
    let mut output = String::from(
        "slug\tp\tdegree\tparent_group_order\taffine_point_count\tsingleton_fibers\tpaired_fibers\tdistinct_x_count\tbase_distinct_x_count\textension_x_growth\textension_to_base_ratio\traw_strict_degree_capacity\tvanishing_degree\tvanishing_monic\tvanishing_nonzero\tvanishing_coefficients\tvanishing_eval_failures\tmutation_rejections\tbuild_extension_additions\tbuild_extension_multiplications\tvanishing_eval_extension_additions\tvanishing_eval_extension_multiplications\tmutation_eval_extension_additions\tmutation_eval_extension_multiplications\tfiber_count_identity\telapsed_ns\taccepted\n",
    );
    let mut all_accepted = true;
    let mut any_growth = false;
    for cell in CELLS {
        let parent = *parents.get(cell.slug).ok_or("missing parent cell")?;
        let (row, accepted, growth) = run_cell(cell, parent)?;
        write!(output, "{row}").map_err(|error| error.to_string())?;
        all_accepted &= accepted;
        any_growth |= growth;
    }
    std::fs::write(output_path, output).map_err(|error| error.to_string())?;
    require(all_accepted, "one or more separation-capacity cells failed")?;
    require(any_growth, "extension-growth control failed")?;
    println!("h116 rational separation capacity: 8/8 cells accepted");
    Ok(())
}

fn selftest() -> Result<(), String> {
    h116_field::selftest()?;
    let field = Field::new(3, 1, &[0, 1])?;
    let samples = (0..3).map(|index| field.from_index(index)).collect::<Vec<_>>();
    let coefficients = vanishing_polynomial(&field, &samples);
    require(
        encode_coefficients(&field, &coefficients) == "0,2,0,1",
        "F3 vanishing polynomial mismatch",
    )?;
    require(
        samples.iter().all(|&x| evaluate_polynomial(&field, &coefficients, x) == field.zero()),
        "F3 vanishing evaluation failed",
    )?;
    println!("h116 rational separation capacity selftest ok");
    Ok(())
}

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let result = match arguments.as_slice() {
        [_, flag] if flag == "--selftest" => selftest(),
        [_, parent, output] => run(parent, output),
        _ => Err("usage: h116_rational_separation_capacity [--selftest | PARENT.tsv OUTPUT.tsv]".to_string()),
    };
    if let Err(error) = result {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

