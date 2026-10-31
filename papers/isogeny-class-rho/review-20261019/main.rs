mod stats;

fn main() {
    let sample = [1.0, 1.0];
    let (d, p) = stats::ks(&sample, &sample);
    println!("Historical KS identical tied samples: D={d}, approximate p={p}; correct D=0");
    assert_eq!(
        d, 0.0,
        "identical empirical distributions must have KS distance zero"
    );
}
