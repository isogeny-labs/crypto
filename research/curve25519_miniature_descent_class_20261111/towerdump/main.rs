use crypto_lib::cryptanalysis::ic_boundary::GroupOps;
use crypto_lib::cryptanalysis::ic_framework::prime_tower::isogeny_tower;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let p: u64 = args[1].parse().unwrap();
    let t: u32 = args[2].parse().unwrap();
    let seed: u64 = args[3].parse().unwrap();
    let mut ops = GroupOps::default();
    let tw = isogeny_tower(p, t, seed, &mut ops).expect("tower");
    let v: Vec<String> = tw.v.iter().map(|x| x.to_string()).collect();
    println!("{{\"p\":{},\"t\":{},\"seed\":{},\"v\":[{}]}}", p, t, seed, v.join(","));
}
