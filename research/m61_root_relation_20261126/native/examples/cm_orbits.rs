//! Positive control: short CM automorphism relations and their exact row rank.
use ecdlp_pieces::{P, ROOT_GENERATOR, ROOT_ORDER};

fn mul(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % P as u128) as u64
}
fn power(mut a: u64, mut exponent: u64) -> u64 {
    let mut result = 1;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = mul(result, a);
        }
        a = mul(a, a);
        exponent >>= 1;
    }
    result
}
fn add(a: u64, b: u64) -> u64 {
    let sum = a + b;
    if sum >= P {
        sum - P
    } else {
        sum
    }
}
fn rhs(x: u64) -> u64 {
    add(mul(mul(x, x), x), 40)
}
fn sqrt(value: u64) -> Option<u64> {
    let root = power(value, (P + 1) / 4);
    (mul(root, root) == value).then_some(root)
}

fn main() {
    let third = ROOT_ORDER / 3;
    let zeta = power(ROOT_GENERATOR, third);
    assert_ne!(zeta, 1);
    assert_eq!(power(zeta, 3), 1);
    assert_eq!(add(add(1, zeta), mul(zeta, zeta)), 0);

    // Each orbit is {z^i, z^(i+third), z^(i+2*third)}.
    let mut x = 1;
    let mut orbit_rows = 0_usize;
    let mut first_witness = None;
    for _ in 0..third {
        let x1 = mul(zeta, x);
        let x2 = mul(zeta, x1);
        assert_eq!(mul(zeta, x2), x);
        assert_eq!(add(add(x, x1), x2), 0);
        assert_eq!(rhs(x), rhs(x1));
        assert_eq!(rhs(x), rhs(x2));
        if let Some(y) = sqrt(rhs(x)) {
            // Three points (x,y), (ζx,y), (ζ²x,y) lie on a horizontal
            // line, so their elliptic group sum is zero. Their supports
            // are disjoint across orbits; these rows are independent.
            orbit_rows += 1;
            first_witness.get_or_insert((x, x1, x2, y));
        }
        x = mul(x, ROOT_GENERATOR);
    }
    let columns = 3 * orbit_rows;
    println!(
        "field={P} curve_b=40 curve_a=0 root_order={ROOT_ORDER} zeta={zeta} lift_valid_columns={columns} orbit_relations={orbit_rows} exact_orbit_row_rank={orbit_rows} nullity={}",
        columns - orbit_rows
    );
    println!(
        "additional_independent_rows_needed_for_rank_B_minus_1={}",
        columns - 1 - orbit_rows
    );
    println!("first_horizontal_line_witness={first_witness:?}");
}
