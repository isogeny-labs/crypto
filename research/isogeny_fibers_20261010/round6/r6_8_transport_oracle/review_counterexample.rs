//! Exact finite witnesses for gaps in the superseded transport proof.
use std::collections::BTreeSet;

fn sums(points: &[u32]) -> BTreeSet<u32> {
    points
        .iter()
        .enumerate()
        .flat_map(|(i, a)| points[i..].iter().map(move |b| (a + b) % 5))
        .collect()
}

fn main() {
    let original = sums(&[0, 1]);
    let removed = sums(&[1]);
    assert_eq!(original, BTreeSet::from([0, 1, 2]));
    assert_eq!(removed, BTreeSet::from([2]));
    assert!(original.contains(&1) && !removed.contains(&1));
    assert!(
        original.iter().filter(|&&x| x != 0).count() > removed.iter().filter(|&&x| x != 0).count()
    );
    assert_eq!(sums(&[0, 1]), original);

    let points = [[1_u32, 0], [1, 1]];
    let target = [2, 1];
    let mut lifts = Vec::new();
    for (i, a) in points.iter().enumerate() {
        for b in &points[i..] {
            if [(a[0] + b[0]) % 5, (a[1] + b[1]) % 2] == target {
                lifts.push([*a, *b]);
            }
        }
    }
    assert_eq!(lifts, vec![[[1, 0], [1, 1]]]);
    let chosen = points[0];
    let arbitrary = [2 * chosen[0] % 5, 2 * chosen[1] % 2];
    assert_ne!(arbitrary, target);
    assert_eq!(arbitrary[0], target[0]);
    let original: Vec<_> = original.into_iter().collect();
    let removed: Vec<_> = removed.into_iter().collect();
    println!(
        r#"{{"status":"PASS_COUNTEREXAMPLES","identity_removal":{{"group_order":5,"arity":2,"original_sums":{original:?},"identity_removed_sums":{removed:?},"lost_nonzero_target":1}},"arbitrary_lift":{{"group":"C5 x C2","projection":"first coordinate","target":{target:?},"valid_lifts":{lifts:?},"arbitrary_lift_sum":{arbitrary:?}}},"general_simulation_status":"NOT_ESTABLISHED"}}"#
    );
}
