//! Public-header equality and archived known-answer controls. No solver calls.
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Output},
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const SEEDS: [u64; 8] = [101, 202, 303, 404, 505, 606, 707, 808];
const PLANNED: usize = 186;
fn check(ok: bool, why: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(why.into())
    }
}
fn new_file(path: impl AsRef<Path>, bytes: &[u8]) -> Result<()> {
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?;
    f.write_all(bytes)?;
    Ok(())
}
fn number(s: &str) -> Result<u64> {
    let n = s.parse::<u64>()?;
    check(n.to_string() == s, "noncanonical unsigned field")?;
    Ok(n)
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct Header {
    case: usize,
    seed: u64,
    fields: [u64; 8],
    text: String,
}
fn header(text: &str) -> Result<Header> {
    let line = text.strip_suffix('\n').ok_or("missing header newline")?;
    check(!line.contains(['\n', '\r']), "exactly one header line")?;
    let v = line.split('\t').collect::<Vec<_>>();
    check(v.len() == 11, "eleven public fields, no scalar")?;
    let case = usize::try_from(number(v[1])?)?;
    let seed = number(v[2])?;
    check(case < 6 && SEEDS.contains(&seed), "header case/seed bounds")?;
    check(v[0] == format!("case{case}-seed{seed}"), "bound header id")?;
    let mut fields = [0; 8];
    for (n, s) in fields.iter_mut().zip(&v[3..]) {
        *n = number(s)?;
    }
    check(fields[0] >= 5 && fields[3] >= 3, "header field/order range")?;
    check(
        [1, 2, 4, 5, 6, 7].iter().all(|i| fields[*i] < fields[0]),
        "canonical curve/coordinates",
    )?;
    Ok(Header {
        case,
        seed,
        fields,
        text: text.to_owned(),
    })
}
struct Run {
    out: PathBuf,
    cpu: String,
    progress: fs::File,
    attempted: usize,
    passed: usize,
    headers: usize,
    new_headers: usize,
}
impl Run {
    fn child(&mut self, id: &str, binary: &str, args: &[String]) -> Result<Output> {
        self.attempted += 1;
        writeln!(self.progress, "BEGIN\t{id}")?;
        self.progress.flush()?;
        new_file(
            self.out.join(format!("{id}.command.txt")),
            format!("{binary}\n{}\n", args.join("\n")).as_bytes(),
        )?;
        let output = Command::new("timeout")
            .args([
                "--foreground",
                "--signal=TERM",
                "--kill-after=5",
                "60",
                "taskset",
                "-c",
                &self.cpu,
                binary,
            ])
            .args(args)
            .output()?;
        new_file(self.out.join(format!("{id}.stdout")), &output.stdout)?;
        new_file(self.out.join(format!("{id}.stderr")), &output.stderr)?;
        writeln!(
            self.progress,
            "END\t{id}\t{}",
            output.status.code().unwrap_or(-1)
        )?;
        self.progress.flush()?;
        Ok(output)
    }
}
fn scalar_args(h: &Header, k: u64) -> Vec<String> {
    let mut a = vec![h.case.to_string(), h.seed.to_string()];
    a.extend(h.fields.iter().map(u64::to_string));
    a.push(k.to_string());
    a
}
fn execute(r: &mut Run, a: &[String]) -> Result<()> {
    let mut legacy = BTreeMap::new();
    for line in fs::read_to_string(&a[5])?.lines() {
        let h = header(&format!("{line}\n"))?;
        check([101, 202].contains(&h.seed), "legacy seed")?;
        check(
            legacy.insert((h.case, h.seed), h).is_none(),
            "duplicate legacy header",
        )?;
    }
    check(legacy.len() == 12, "twelve legacy headers")?;
    for case in 0..6 {
        for seed in [101, 202] {
            check(legacy.contains_key(&(case, seed)), "legacy grid")?;
        }
    }
    let mut all = BTreeMap::new();
    let mut points: [BTreeSet<(u64, u64)>; 6] = std::array::from_fn(|_| BTreeSet::new());
    let mut all_text = String::new();
    let mut new_text = String::new();
    for (case, seen) in points.iter_mut().enumerate() {
        for seed in SEEDS {
            let mut expected: Option<Header> = None;
            for (binary, label, mode) in [
                (&a[4], "c", "target"),
                (&a[2], "parent", "t"),
                (&a[3], "candidate", "t"),
            ] {
                let mut args = vec![mode.to_owned(), case.to_string(), seed.to_string()];
                if mode == "t" {
                    args.push("-".into());
                }
                let o = r.child(&format!("header-{case}-{seed}-{label}"), binary, &args)?;
                check(
                    o.status.success() && o.stderr.is_empty(),
                    "target-only child failed",
                )?;
                let h = header(std::str::from_utf8(&o.stdout)?)?;
                check(
                    h.case == case && h.seed == seed,
                    "requested header identity",
                )?;
                check(
                    h.fields[..6] == legacy[&(case, 101)].fields[..6],
                    "legacy domain/generator changed",
                )?;
                if let Some(old) = legacy.get(&(case, seed)) {
                    check(&h == old, "archived public header mismatch")?;
                }
                if let Some(ref old) = expected {
                    check(&h == old, "C/parent/candidate target disagreement")?;
                } else {
                    expected = Some(h);
                }
                r.passed += 1;
            }
            let h = expected.ok_or("no target source")?;
            check(
                seen.insert((h.fields[6], h.fields[7])),
                "duplicate target coordinate in domain",
            )?;
            all_text.push_str(&h.text);
            r.headers += 1;
            if seed > 202 {
                new_text.push_str(&h.text);
                r.new_headers += 1;
            }
            all.insert((case, seed), h);
        }
    }
    check(
        points.iter().all(|p| p.len() == 8),
        "eight distinct targets per domain",
    )?;
    new_file(r.out.join("all-public-headers.tsv"), all_text.as_bytes())?;
    new_file(r.out.join("new-public-headers.tsv"), new_text.as_bytes())?;
    for (binary, label, mode) in [
        (&a[4], "c", "target"),
        (&a[2], "parent", "t"),
        (&a[3], "candidate", "t"),
    ] {
        for (case, seed) in [(6, 303), (0, 999)] {
            let mut args = vec![mode.to_owned(), case.to_string(), seed.to_string()];
            if mode == "t" {
                args.push("-".into());
            }
            let o = r.child(&format!("invalid-{case}-{seed}-{label}"), binary, &args)?;
            let intended = if label == "c" {
                o.stderr == b"REJECTED unrecognized fixture\n"
            } else {
                std::str::from_utf8(&o.stderr)?.contains("invalid fixture")
            };
            check(
                o.status.code() == Some(1) && o.stdout.is_empty() && intended,
                "invalid fixture did not fail for intended cause",
            )?;
            r.passed += 1;
        }
    }
    let mut scalars = BTreeMap::new();
    for line in fs::read_to_string(&a[6])?.lines() {
        let v = line.split('\t').collect::<Vec<_>>();
        check(v.len() == 3, "scalar-validation width")?;
        let key = (usize::try_from(number(v[0])?)?, number(v[1])?);
        check(
            legacy.contains_key(&key),
            "scalar input outside legacy grid",
        )?;
        check(
            scalars.insert(key, number(v[2])?).is_none(),
            "duplicate archived scalar",
        )?;
    }
    check(scalars.len() == 12, "twelve archived scalar controls")?;
    for ((case, seed), k) in scalars {
        let h = &all[&(case, seed)];
        let order = h.fields[3];
        check(
            k > 0 && k < order && h.fields[7] > 0,
            "legacy scalar/control range",
        )?;
        for mode in ["positive", "wrong-scalar", "substituted-target"] {
            let mut altered = h.clone();
            let mut scalar = k;
            if mode == "wrong-scalar" {
                scalar = (k + 1) % order;
            }
            if mode == "substituted-target" {
                altered.fields[7] = h.fields[0] - h.fields[7];
                scalar = order - k;
            }
            let o = r.child(
                &format!("legacy-{case}-{seed}-{mode}"),
                &a[4],
                &scalar_args(&altered, scalar),
            )?;
            let valid =
                if mode == "positive" {
                    o.status.success() && o.stderr.is_empty()
                && o.stdout==b"OPENSSL VERIFIED public-target binding, nonidentity rP=O and kP=Q\n"
                } else {
                    o.status.code() == Some(1)
                        && o.stdout.is_empty()
                        && o.stderr
                            == if mode == "wrong-scalar" {
                                b"REJECTED kP != Q\n".as_slice()
                            } else {
                                b"REJECTED public target mismatch\n".as_slice()
                            }
                };
            check(valid, "legacy point/scalar control unexpected outcome")?;
            r.passed += 1;
        }
    }
    check(
        r.attempted == PLANNED && r.passed == PLANNED && r.headers == 48 && r.new_headers == 36,
        "final prospective control census mismatch",
    )
}
fn json_string(s: &str) -> String {
    let mut out = String::from("\"");
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c < ' ' => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
fn main() -> Result<()> {
    let a = std::env::args().collect::<Vec<_>>();
    check(
        a.len() == 8,
        "OUT PARENT CANDIDATE C_CHECKER LEGACY_HEADERS LEGACY_SCALARS CPU",
    )?;
    number(&a[7])?;
    let out = PathBuf::from(&a[1]);
    fs::create_dir(&out)?;
    let progress = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(out.join("progress.tsv"))?;
    let mut r = Run {
        out,
        cpu: a[7].clone(),
        progress,
        attempted: 0,
        passed: 0,
        headers: 0,
        new_headers: 0,
    };
    let result = execute(&mut r, &a);
    let error = result
        .as_ref()
        .err()
        .map(|e| json_string(&e.to_string()))
        .unwrap_or("null".into());
    new_file(r.out.join("summary.json"),format!("{{\"scope\":\"target-only equality and archived validation, no new solver\",\"checks_passed\":{},\"planned_child_calls\":{PLANNED},\"attempted\":{},\"passed\":{},\"unattempted\":{},\"public_headers\":{},\"new_headers\":{},\"error\":{error}}}\n",
        result.is_ok(),r.attempted,r.passed,PLANNED-r.attempted,r.headers,r.new_headers).as_bytes())?;
    result?;
    println!("PASS 186 child controls, 48 agreeing public headers, 36 new distinct targets; no new solver");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    // Synthetic grammar only; curve/seed binding is checked by real children.
    const LINE: &str = "case0-seed101\t0\t101\t5\t1\t1\t7\t1\t1\t3\t2\n";
    #[test]
    fn public_shape_roundtrips() {
        assert_eq!(header(LINE).unwrap().text, LINE);
    }
    #[test]
    fn scalar_or_second_line_rejected() {
        assert!(header(&LINE.replace("\n", "\t9\n")).is_err());
        assert!(header(&format!("{LINE}{LINE}")).is_err());
    }
    #[test]
    fn wrong_or_unknown_identity_rejected() {
        assert!(header(&LINE.replacen("\t101\t", "\t202\t", 1)).is_err());
        assert!(header(&LINE.replace("101", "999")).is_err());
    }
    #[test]
    fn alias_and_noncanonical_number_rejected() {
        assert!(header(&LINE.replacen("\t5\t", "\t05\t", 1)).is_err());
        assert!(header(&LINE.replace("\t3\t2\n", "\t5\t2\n")).is_err());
    }
}
