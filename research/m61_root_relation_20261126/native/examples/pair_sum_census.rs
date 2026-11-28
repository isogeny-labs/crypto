//! Exact M61 pair-sum x-support census for the selected root factor base.
use ecdlp_pieces::{selected_root_factor_base, Point, P, SELECTED};
use std::collections::HashSet;
use std::time::Instant;

fn coordinate(point: Point) -> u64 {
    match point {
        Point::Affine { x, .. } => x,
        Point::Infinity => panic!("unexpected zero pair sum"),
    }
}

fn projected_coordinate(point: Point) -> u64 {
    match point {
        Point::Affine { x, .. } => x,
        Point::Infinity => P, // sentinel beyond every field x-coordinate
    }
}

fn main() {
    let start = Instant::now();
    let base = selected_root_factor_base();
    let b = base.len();
    let roots = base.iter().copied().map(coordinate).collect::<HashSet<_>>();
    assert_eq!(roots.len(), b);
    let projected_base = base
        .iter()
        .map(|&point| SELECTED.mul(7, point).unwrap())
        .collect::<Vec<_>>();

    // The pair-sum divisor counts B diagonal doubles and two x-classes
    // for each unordered distinct-column pair: B + 2*C(B,2) = B^2.
    let mut support = Vec::with_capacity(b * b);
    let mut projected_support = Vec::with_capacity(b * b);
    let mut root_returns = 0_u64;
    for (i, &left) in base.iter().enumerate() {
        let doubled = coordinate(SELECTED.add(left, left).unwrap());
        root_returns += u64::from(roots.contains(&doubled));
        support.push((doubled, (i * b + i) as u32));
        projected_support.push(projected_coordinate(
            SELECTED.add(projected_base[i], projected_base[i]).unwrap(),
        ));
        for (j, &right) in base.iter().enumerate().skip(i + 1) {
            let plus = coordinate(SELECTED.add(left, right).unwrap());
            let minus = coordinate(SELECTED.add(left, SELECTED.neg(right)).unwrap());
            root_returns += u64::from(roots.contains(&plus));
            root_returns += u64::from(roots.contains(&minus));
            support.push((plus, (i * b + j) as u32));
            support.push((minus, (j * b + i) as u32));
            projected_support.push(projected_coordinate(
                SELECTED.add(projected_base[i], projected_base[j]).unwrap(),
            ));
            projected_support.push(projected_coordinate(
                SELECTED
                    .add(projected_base[i], SELECTED.neg(projected_base[j]))
                    .unwrap(),
            ));
        }
    }
    assert_eq!(support.len(), b * b);
    assert_eq!(projected_support.len(), b * b);
    let generation_ms = start.elapsed().as_millis();
    support.sort_unstable_by_key(|entry| entry.0);
    let mut collided_classes = 0_u64;
    let mut duplicate_entries = 0_u64;
    let mut first_collision = None;
    let mut cursor = 0;
    while cursor < support.len() {
        let mut end = cursor + 1;
        while end < support.len() && support[end].0 == support[cursor].0 {
            end += 1;
        }
        if end - cursor > 1 {
            collided_classes += 1;
            duplicate_entries += (end - cursor - 1) as u64;
            first_collision.get_or_insert((support[cursor], support[cursor + 1]));
        }
        cursor = end;
    }
    println!(
        "field={P} base={b} counted_pair_x={} distinct_pair_x={} collided_x_classes={collided_classes} duplicate_entries={duplicate_entries} root_x_returns={root_returns}",
        support.len(),
        support.len() as u64 - duplicate_entries
    );
    println!("first_collision={first_collision:?}");
    projected_support.sort_unstable();
    let projected_duplicate_entries = projected_support
        .windows(2)
        .filter(|window| window[0] == window[1])
        .count();
    let projected_zero_entries = projected_support.iter().filter(|&&x| x == P).count();
    println!(
        "projected_distinct_pair_x={} projected_duplicate_entries={projected_duplicate_entries} projected_zero_entries={projected_zero_entries}",
        projected_support.len() - projected_duplicate_entries
    );
    println!(
        "generation_ms={generation_ms} total_ms={}",
        start.elapsed().as_millis()
    );
}
