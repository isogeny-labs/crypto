//! Exhaustive fixed-domain factor bases, with both signs, for public synthetic curves.
#[path = "../field.rs"]
mod field;
use field::*;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

const FAMILIES: [&str; 3] = ["koblitz", "binary", "prime"];
const BLOCK: u64 = 4096;
// Exact candidate map: move selector bit26 to a chosen high coordinate bit.
// Bits 27..=40 keep the prime x^3 calculation within u128.
fn domain_x(index: u64, coordinate_bit: u8) -> u64 {
    assert!(index < (1u64 << 27));
    assert!((27..=40).contains(&coordinate_bit));
    (index & ((1u64 << 26) - 1)) | ((index >> 26) << coordinate_bit)
}
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn save(path: &Path, v: &Value) {
    fs::write(
        path,
        format!("{}\n", serde_json::to_string_pretty(v).unwrap()),
    )
    .unwrap();
}
fn rhs_prime(x: u128) -> u128 {
    (x * x * x + 2 * x + 3) % P
} // x < 2^41, so x^3+2x+3 fits u128.
fn binary_ab(f: usize) -> (u128, u128) {
    if f == 0 {
        (0, 1)
    } else {
        (1, B)
    }
}
fn on_curve(f: usize, x: u128, y: u128) -> bool {
    if f == 2 {
        y < P && from_mont(pmul(to_mont(y), to_mont(y))) == rhs_prime(x)
    } else {
        let (a, b) = binary_ab(f);
        y <= MASK && bmul(y, y) ^ bmul(x, y) == bmul(bmul(x, x), x) ^ bmul(a, bmul(x, x)) ^ b
    }
}
fn pair(f: usize, x: u64, y: u128) -> Vec<u128> {
    let other = if f == 2 {
        if y == 0 {
            0
        } else {
            P - y
        }
    } else {
        y ^ x as u128
    };
    if other == y {
        vec![y]
    } else {
        vec![y.min(other), y.max(other)]
    }
}
fn header(f: usize, ell: u8, start: u64, end: u64, coordinate_bit: u8) -> Vec<u8> {
    assert!((27..=40).contains(&coordinate_bit));
    let mut v = b"N83FB02\n".to_vec();
    v.push(f as u8);
    v.push(ell);
    v.extend_from_slice(&[coordinate_bit, 0, 0, 0, 0, 0]);
    v.extend_from_slice(&start.to_le_bytes());
    v.extend_from_slice(&end.to_le_bytes());
    v
}
fn parse_header(raw: &[u8]) -> (usize, u8, u64, u64, u8) {
    assert!(
        raw.len() >= 32 && (raw.len() - 32) % 20 == 0,
        "record length"
    );
    assert_eq!(&raw[..8], b"N83FB02\n");
    let coordinate_bit = raw[10];
    assert!((27..=40).contains(&coordinate_bit));
    assert_eq!(&raw[11..16], &[0, 0, 0, 0, 0]);
    let (f, ell) = (raw[8] as usize, raw[9]);
    assert!(f < 3 && ell > 0 && ell <= 27);
    let start = u64::from_le_bytes(raw[16..24].try_into().unwrap());
    let end = u64::from_le_bytes(raw[24..32].try_into().unwrap());
    assert!(start < end && end <= 1u64 << ell);
    (f, ell, start, end, coordinate_bit)
}
fn record(raw: &[u8], offset: usize) -> Option<(u64, u128)> {
    if offset == raw.len() {
        None
    } else {
        Some((
            u32::from_le_bytes(raw[offset..offset + 4].try_into().unwrap()) as u64,
            u128::from_le_bytes(raw[offset + 4..offset + 20].try_into().unwrap()),
        ))
    }
}
fn roots(f: usize, x: u64, inv: u128, bs: &BinaryTables, ps: &PrimeSqrt) -> Vec<u128> {
    if f == 2 {
        return ps
            .root(rhs_prime(x as u128))
            .map(|y| pair(f, x, y))
            .unwrap_or_default();
    }
    let (a, b) = binary_ab(f);
    if x == 0 {
        return vec![bpow(b, 1u128 << 82)];
    }
    let c = x as u128 ^ a ^ bmul(b, bmul(inv, inv));
    if bs.trace(c) != 0 {
        return vec![];
    }
    let h = bs.half(c);
    assert_eq!(bmul(h, h) ^ h, c);
    pair(f, x, bmul(x as u128, h))
}
fn generate(
    f: usize,
    ell: u8,
    start: u64,
    end: u64,
    coordinate_bit: u8,
    bs: &BinaryTables,
    ps: &PrimeSqrt,
) -> Vec<u8> {
    let mut raw = header(f, ell, start, end, coordinate_bit);
    for lo in (start..end).step_by(BLOCK as usize) {
        let hi = (lo + BLOCK).min(end);
        let inverses = if f < 2 {
            batch_inverse(lo, hi, coordinate_bit)
        } else {
            vec![0; (hi - lo) as usize]
        };
        for index in lo..hi {
            let x = domain_x(index, coordinate_bit);
            for y in roots(f, x, inverses[(index - lo) as usize], bs, ps) {
                assert!(on_curve(f, x as u128, y));
                raw.extend_from_slice(&(index as u32).to_le_bytes());
                raw.extend_from_slice(&y.to_le_bytes());
            }
        }
    }
    raw
}
// Every x is checked, including omissions. For prime, omissions use Euler's
// criterion (not the square-root routine). Binary uses the trace criterion.
fn verify(raw: &[u8], bs: &BinaryTables) -> Value {
    let (f, ell, start, end, coordinate_bit) = parse_header(raw);
    let mut offset = 32;
    let mut liftable = 0u64;
    let mut singles = 0u64;
    let mut coverage = Sha256::new();
    let mut samples = Vec::new();
    for lo in (start..end).step_by(BLOCK as usize) {
        let hi = (lo + BLOCK).min(end);
        let inverses = if f < 2 {
            batch_inverse(lo, hi, coordinate_bit)
        } else {
            vec![0; (hi - lo) as usize]
        };
        for index in lo..hi {
            let x = domain_x(index, coordinate_bit);
            let mut ys = Vec::new();
            while let Some((rx, y)) = record(raw, offset) {
                assert!(rx >= index, "unsorted, duplicate or out-of-domain x");
                if rx != index {
                    break;
                }
                ys.push(y);
                offset += 20;
                assert!(ys.len() <= 2);
            }
            let expected = if f == 2 {
                let a = rhs_prime(x as u128);
                if a == 0 {
                    1
                } else if ys.is_empty() {
                    assert_eq!(
                        ppow(to_mont(a), (P - 1) / 2),
                        to_mont(P - 1),
                        "omitted prime lift"
                    );
                    0
                } else {
                    2
                }
            } else if x == 0 {
                1
            } else {
                let (a, b) = binary_ab(f);
                let inv = inverses[(index - lo) as usize];
                let c = x as u128 ^ a ^ bmul(b, bmul(inv, inv));
                if bs.trace(c) == 0 {
                    2
                } else {
                    0
                }
            };
            assert_eq!(ys.len(), expected, "missing or extra point at x={x}");
            if !ys.is_empty() {
                for &y in &ys {
                    assert!(on_curve(f, x as u128, y), "off-curve point");
                }
                assert_eq!(ys, pair(f, x, ys[0]), "sign pair/order");
                liftable += 1;
                if ys.len() == 1 {
                    singles += 1;
                }
                if samples.len() < 4 {
                    samples.push(
                        json!({"x":x,"ys":ys.iter().map(ToString::to_string).collect::<Vec<_>>()}),
                    );
                }
            }
            coverage.update([expected as u8]);
            // Distributed slow-arithmetic cross-checks: independent multiplication,
            // Euler criterion / 83-term trace. Includes absent x values.
            if (index - start) % 8191 == 0 || index + 1 == end {
                if f == 2 {
                    let a = rhs_prime(x as u128);
                    let l = slow_ppow(a, (P - 1) / 2);
                    assert_eq!(!ys.is_empty(), l != P - 1);
                    for &y in &ys {
                        assert_eq!(slow_pmul(y, y), a);
                    }
                } else {
                    let (a, b) = binary_ab(f);
                    if x != 0 {
                        let inv = inverses[(index - lo) as usize];
                        assert_eq!(slow_bmul(x as u128, inv), 1);
                        let c = x as u128 ^ a ^ slow_bmul(b, slow_bmul(inv, inv));
                        let (mut tr, mut v) = (0, c);
                        for _ in 0..83 {
                            tr ^= v;
                            v = slow_bmul(v, v);
                        }
                        assert_eq!(!ys.is_empty(), tr == 0);
                    }
                    for &y in &ys {
                        assert_eq!(
                            slow_bmul(y, y) ^ slow_bmul(x as u128, y),
                            slow_bmul(slow_bmul(x as u128, x as u128), x as u128)
                                ^ slow_bmul(a, slow_bmul(x as u128, x as u128))
                                ^ b
                        );
                    }
                }
            }
        }
    }
    assert_eq!(offset, raw.len(), "trailing points");
    json!({"family":FAMILIES[f],"ell":ell,"start":start,"end":end,"candidates":end-start,
        "liftable_x":liftable,"point_count":(raw.len()-32)/20,"single_root_x":singles,
        "raw_bytes":raw.len(),"raw_sha256":hash(raw),"coverage_sha256":format!("{:x}",coverage.finalize()),
        "verified_all_candidates":true,"samples":samples,
        "candidate_id":format!("spike{coordinate_bit}"),
        "coordinate_bit_positions":(0u8..26).chain([coordinate_bit]).collect::<Vec<_>>()})
}
fn shard(
    f: usize,
    ell: u8,
    id: usize,
    chunk: u64,
    coordinate_bit: u8,
    out: &Path,
    bs: &BinaryTables,
    ps: &PrimeSqrt,
) -> Value {
    let start = id as u64 * chunk;
    let end = (start + chunk).min(1u64 << ell);
    let dir = out.join(FAMILIES[f]);
    fs::create_dir_all(&dir).unwrap();
    let name = format!("part-{id:04}.bin.zst");
    let path = dir.join(&name);
    let raw = generate(f, ell, start, end, coordinate_bit, bs, ps);
    let packed = zstd::stream::encode_all(raw.as_slice(), 3).unwrap();
    let temp = path.with_extension("partial");
    fs::write(&temp, &packed).unwrap();
    fs::rename(temp, &path).unwrap();
    let mut result = verify(
        &zstd::stream::decode_all(fs::File::open(&path).unwrap()).unwrap(),
        bs,
    );
    result["file"] = json!(name);
    result["bytes"] = json!(packed.len());
    result["sha256"] = json!(hash(&packed));
    save(&dir.join(format!("part-{id:04}.json")), &result);
    println!(
        "{} shard {id}: {} points, {} bytes, verified",
        FAMILIES[f],
        result["point_count"],
        packed.len()
    );
    result
}
fn run(out: &Path, ell: u8, chunk_bits: u8, threads: usize, coordinate_bit: u8) {
    assert!(ell > 0 && ell <= 27 && chunk_bits <= ell && threads > 0 && threads <= 24);
    assert!((27..=40).contains(&coordinate_bit));
    assert!(!out.exists(), "use a fresh output directory");
    fs::create_dir_all(out).unwrap();
    let bs = BinaryTables::new();
    let ps = PrimeSqrt::new();
    let count = 1usize << (ell - chunk_bits);
    let next = Arc::new(AtomicUsize::new(0));
    let results = std::thread::scope(|scope| {
        let mut handles = Vec::new();
        for _ in 0..threads {
            let next = next.clone();
            let (bs, ps) = (&bs, &ps);
            handles.push(scope.spawn(move || {
                let mut rows = Vec::new();
                loop {
                    let i = next.fetch_add(1, Ordering::Relaxed);
                    if i >= 3 * count {
                        break;
                    }
                    // Start prime work early; output remains sorted by family and x.
                    let f = [2, 0, 1][i % 3];
                    let id = i / 3;
                    rows.push(shard(
                        f,
                        ell,
                        id,
                        1u64 << chunk_bits,
                        coordinate_bit,
                        out,
                        bs,
                        ps,
                    ));
                }
                rows
            }));
        }
        handles
            .into_iter()
            .flat_map(|h| h.join().unwrap())
            .collect::<Vec<_>>()
    });
    let mut manifests = Vec::new();
    for family in FAMILIES {
        let mut rows: Vec<_> = results
            .iter()
            .filter(|v| v["family"] == family)
            .cloned()
            .collect();
        rows.sort_by_key(|v| v["start"].as_u64().unwrap());
        assert_eq!(rows.len(), count);
        let points: u64 = rows
            .iter()
            .map(|v| v["point_count"].as_u64().unwrap())
            .sum();
        let xs: u64 = rows.iter().map(|v| v["liftable_x"].as_u64().unwrap()).sum();
        let bytes: u64 = rows.iter().map(|v| v["bytes"].as_u64().unwrap()).sum();
        let candidate_id = format!("spike{coordinate_bit}");
        let bit_positions = (0u8..26).chain([coordinate_bit]).collect::<Vec<_>>();
        let manifest = json!({"schema":"n83.factor-base/v2","candidate_id":candidate_id,"coordinate_bit_positions":bit_positions,"family":family,"ell":ell,
            "candidate_x_count":1u64<<ell,"reuse_for_m":[2,3,4,5,6,7,8,9],
            "point_count":points,"liftable_x":xs,"compressed_bytes":bytes,
            "point_policy":"all affine lifts, both signs, no infinity, no subgroup filtering",
            "ordering":"increasing x then increasing encoded y","format":format!("N83FB02 header32 with byte10={coordinate_bit}; records selector_index:u32-le,y:u128-le; x=(index&((1<<26)-1))|((index>>26)<<{coordinate_bit}); zstd"),
            "curve":if family=="prime" {json!({"p":P.to_string(),"a":"2","b":"3","equation":"y^2=x^3+2x+3"})}
                else {let (a,b)=binary_ab(if family=="koblitz" {0}else{1});json!({"modulus":"z^83+z^45+z^2+z+1","a":a.to_string(),"b":b.to_string(),"equation":"y^2+xy=x^3+ax^2+b","encoding":"polynomial coefficient i is bit i"})},
            "shards":rows});
        let path = out.join(family).join("manifest.json");
        save(&path, &manifest);
        manifests.push(json!({"family":family,"manifest":format!("{family}/manifest.json"),
            "manifest_sha256":hash(&fs::read(&path).unwrap()),"point_count":points,"liftable_x":xs,"compressed_bytes":bytes,
            "same_base_for_m":[2,3,4,5,6,7,8,9]}));
    }
    save(
        &out.join("index.json"),
        &json!({"schema":"n83.shared-factor-bases/v2","candidate_id":format!("spike{coordinate_bit}"),"coordinate_bit_positions":(0u8..26).chain([coordinate_bit]).collect::<Vec<_>>(),"ell":ell,
        "candidate_x_count_per_curve":1u64<<ell,"families":manifests,"complete":true}),
    );
}
fn inspect(path: &Path, limit: usize) {
    let raw = zstd::stream::decode_all(fs::File::open(path).unwrap()).unwrap();
    let (f, ell, start, end, coordinate_bit) = parse_header(&raw);
    eprintln!(
        "{} ell={ell} x=[{start},{end}) records={}",
        FAMILIES[f],
        (raw.len() - 32) / 20
    );
    let mut out = io::BufWriter::new(io::stdout().lock());
    writeln!(out, "selector_index,x,y").unwrap();
    for i in 0..limit.min((raw.len() - 32) / 20) {
        let (x, y) = record(&raw, 32 + 20 * i).unwrap();
        writeln!(out, "{x},{},{y}", domain_x(x, coordinate_bit)).unwrap();
    }
}
fn selfcheck() -> Value {
    let bs = BinaryTables::new();
    let ps = PrimeSqrt::new();
    let mut out = Vec::new();
    for f in 0..3 {
        let raw = generate(f, 6, 0, 64, 40, &bs, &ps);
        out.push(verify(&raw, &bs));
        // Mutations must be rejected by the exact verifier, not merely hashes.
        let mut bad = raw.clone();
        bad[36] ^= 1;
        assert!(std::panic::catch_unwind(|| verify(&bad, &bs)).is_err());
        let mut missing = raw.clone();
        missing.drain(32..52);
        assert!(std::panic::catch_unwind(|| verify(&missing, &bs)).is_err());
    }
    json!({"controls_passed":true,"positive":out,"negative":"altered y and removed point rejected for all three families"})
}
fn main() {
    check_cpu();
    let a: Vec<String> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("run")=>run(&PathBuf::from(&a[2]),a[3].parse().unwrap(),a[4].parse().unwrap(),a[5].parse().unwrap(),a.get(6).map(|s|s.parse().unwrap()).unwrap_or(40)),
        Some("verify")=>{let bs=BinaryTables::new();let raw=zstd::stream::decode_all(fs::File::open(&a[2]).unwrap()).unwrap();println!("{}",verify(&raw,&bs));},
        Some("csv")=>inspect(Path::new(&a[2]),a.get(3).map(|s|s.parse().unwrap()).unwrap_or(usize::MAX)),
        Some("selfcheck")=>println!("{}",selfcheck()),
        _=>panic!("usage: n83-factor-base run OUT ELL CHUNK_BITS THREADS [COORDINATE_BIT_27_TO_40] | verify SHARD | csv SHARD [LIMIT] | selfcheck"),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn candidate_map_and_prime_rhs() {
        for bit in 27..=40 {
            assert_eq!(domain_x((1 << 26) - 1, bit), (1 << 26) - 1);
            assert_eq!(domain_x(1 << 26, bit), 1u64 << bit);
            for i in [0, 1, (1 << 26) - 1, 1 << 26, (1 << 27) - 1] {
                let x = domain_x(i, bit);
                assert_eq!((x & ((1 << 26) - 1)) | ((x >> bit) << 26), i);
                assert!(x < (1u64 << 41));
                assert_eq!(
                    rhs_prime(x as u128),
                    (slow_pmul(slow_pmul(x as u128, x as u128), x as u128) + 2 * x as u128 + 3) % P
                );
            }
        }
    }
    #[test]
    fn positive_and_negative_controls() {
        check_cpu();
        assert_eq!(selfcheck()["controls_passed"], true);
    }
    #[test]
    fn shard_boundaries_and_encoding() {
        check_cpu();
        let b = BinaryTables::new();
        let p = PrimeSqrt::new();
        for f in 0..3 {
            let whole = generate(f, 10, 0, 1024, 40, &b, &p);
            let a = generate(f, 10, 0, 333, 40, &b, &p);
            let z = generate(f, 10, 333, 1024, 40, &b, &p);
            assert_eq!([&a[32..], &z[32..]].concat(), whole[32..]);
            verify(&a, &b);
            verify(&z, &b);
            let boundary = generate(f, 27, (1 << 26) - 16, (1 << 26) + 16, 40, &b, &p);
            verify(&boundary, &b);
            let h = generate(f, 27, (1 << 27) - 32, 1 << 27, 40, &b, &p);
            verify(&h, &b);
            for bit in [27, 33, 39, 40] {
                let variant = generate(f, 10, 0, 1024, bit, &b, &p);
                let (_, _, _, _, parsed_bit) = parse_header(&variant);
                assert_eq!(parsed_bit, bit);
                verify(&variant, &b);
            }
        }
    }
}
