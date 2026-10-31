use pencil_target_information::information;

fn main() {
    let fixtures = [
        ("nuisance_only", 1, vec![vec![1, 0, 0]]),
        ("target_only", 1, vec![vec![0, 1, 3]]),
        ("two_nuisances", 2, vec![vec![1, 0, 0, 0], vec![0, 1, 0, 0]]),
        (
            "target_after_elimination",
            2,
            vec![vec![1, 0, 0, 0], vec![0, 1, 0, 0], vec![1, 1, 1, 3]],
        ),
        ("inconsistent", 1, vec![vec![0, 0, 1], vec![0, 1, 3]]),
    ];
    for (name, n, rows) in fixtures {
        let i = information(7, n, &rows).expect("fixed algebraic fixture");
        println!("{{\"fixture\":\"{name}\",\"modulus\":7,\"nuisance_columns\":{n},\"rows\":{rows:?},\"rank_a\":{},\"rank_ab\":{},\"rank_abc\":{},\"consistent\":{},\"target_determined\":{},\"scientific_trial\":false,\"cost_ratio\":null}}",
            i.nuisance_rank, i.nuisance_and_target_rank, i.full_rank, i.consistent, i.target_determined);
    }
}
