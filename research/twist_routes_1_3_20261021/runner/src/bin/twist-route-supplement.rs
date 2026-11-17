//! Independent executable for equal-original-height and pairing controls.
#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    error::Error,
    fs,
    path::Path,
    process::{Command, Stdio},
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
#[path = "../pari.rs"]
mod pari;
#[path = "../registry.rs"]
mod registry;
#[path = "../../../../../src/hash/sha256.rs"]
mod source_sha256;
#[path = "../../../../../src/cryptanalysis/twist_ic_field.rs"]
mod toy_field;
use toy_field::*;
#[derive(Deserialize)]
struct Fixture {
    id: String,
    p: u64,
    a: u64,
    b: u64,
    order: u64,
    role: String,
}
impl Fixture {
    fn curve(&self) -> Quad {
        Quad::new(self.p, self.a, self.b)
    }
}
mod covers {
    #[derive(serde::Deserialize)]
    pub struct Selected {
        pub fixture: String,
        pub c: u64,
        pub order: u64,
    }
}
fn write_json(path: impl AsRef<Path>, v: &impl Serialize) -> Result<()> {
    fs::write(path, serde_json::to_string_pretty(v)? + "\n")?;
    Ok(())
}
fn line(e: Quad, p: P, u: P, q: P) -> Q {
    let (Some((x, y)), Some((a, b)), Some((qx, qy))) = (p, u, q) else {
        return Q(1, 0);
    };
    let sum = e.pa(p, u);
    let Some((sx, _)) = sum else {
        return e.sub(qx, x);
    };
    let slope = if x == a {
        e.mul(
            e.add(e.scale(e.mul(x, x), 3), Q(e.a, 0)),
            e.inv(e.scale(y, 2)),
        )
    } else {
        e.mul(e.sub(b, y), e.inv(e.sub(a, x)))
    };
    let numerator = e.sub(e.sub(qy, y), e.mul(slope, e.sub(qx, x)));
    e.mul(numerator, e.inv(e.sub(qx, sx)))
}
fn qpow(e: Quad, mut x: Q, mut n: u64) -> Q {
    let mut r = Q(1, 0);
    while n > 0 {
        if n & 1 != 0 {
            r = e.mul(r, x);
        }
        x = e.mul(x, x);
        n >>= 1;
    }
    r
}
fn tate(e: Quad, p: P, q: P, n: u64) -> Q {
    let mut t = p;
    let mut f = Q(1, 0);
    for b in (0..(63 - n.leading_zeros())).rev() {
        f = e.mul(e.mul(f, f), line(e, t, t, q));
        t = e.pa(t, t);
        if n >> b & 1 != 0 {
            f = e.mul(f, line(e, t, p, q));
            t = e.pa(t, p);
        }
    }
    assert_eq!(t, None);
    qpow(e, f, (e.p * e.p - 1) / n)
}
fn pairing_points() -> (Quad, P, P) {
    let e = Quad {
        p: 43,
        d: 42,
        a: 1,
        b: 0,
    };
    assert_eq!(e.rational().len(), 44);
    assert_eq!(e.points().len(), 1936);
    let p = e
        .rational()
        .into_iter()
        .find(|p| p.is_some() && e.pm(*p, 11).is_none())
        .unwrap();
    let (x, y) = p.unwrap();
    let q = Some((e.neg(x), e.mul(Q(0, 1), y)));
    assert!(e.on(q));
    assert_eq!(e.pm(q, 11), None);
    assert!((0..11).all(|k| e.pm(p, k) != q));
    (e, p, q)
}
fn prepare_pairing(out: &Path) -> Result<()> {
    fs::create_dir(out)?;
    let (e, p, q) = pairing_points();
    let g = tate(e, p, q, 11);
    assert_ne!(g, Q(1, 0));
    assert_eq!(qpow(e, g, 11), Q(1, 0));
    let targets: Vec<_> = (1..11)
        .map(|k| {
            let point = e.pm(p, k);
            assert_eq!(tate(e, point, q, 11), qpow(e, g, k as u64));
            let (x, y) = point.unwrap();
            json!({"control_scalar":k,"point":[x.0,y.0]})
        })
        .collect();
    let mut ids = vec![
        registry::identity(43, 1, 0, 44, None),
        registry::identity(43, 1, 0, 1936, Some(&[1, 0])),
    ];
    for id in &mut ids {
        id["sources"] = json!([
            "research/twist_routes_1_3_20261021/execution-v1/pairing-fixture/pairing-fixture.json"
        ]);
        id["ec1_unresolved"]=json!("Supersingular positive control only. The linked fixture specifies x^2+1 over F43, its exact source subgroup of order 11, generator, distortion point and embedding; no cross-repository EC1 comparison is claimed.");
    }
    write_json(out.join("registry-additions.json"), &ids)?;
    write_json(
        out.join("pairing-fixture.json"),
        &json!({"source":ids[0]["slug"],"extension":ids[1]["slug"],"p":43,"A":1,"B":0,"source_order":44,"extension_order":1936,"r":11,"embedding_degree":2,"Fp2_modulus":"w^2+1","generator":p,"distortion_point":q,"native_reduced_Tate_generator":g,"known_scalar_controls":targets,"novelty_claim":false,"NIST_applicability":false}),
    )?;
    Ok(())
}
fn pairing(fixture: &Path, out: &Path) -> Result<()> {
    fs::create_dir(out)?;
    let manifest: Value = serde_json::from_slice(&fs::read(fixture)?)?;
    let (e, p, q) = pairing_points();
    let g = tate(e, p, q, 11);
    assert_eq!(manifest["native_reduced_Tate_generator"], json!(g));
    let (px, py) = p.unwrap();
    let target_list: Vec<_> = manifest["known_scalar_controls"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x["point"].clone())
        .collect();
    let script = format!(
        r#"p=43;r=11;w=ffgen(Mod(1,p)*(t^2+1),'w);E=ellinit([w^0,0*w]);P=[{}*w^0,{}*w^0];Q=[-P[1],w*P[2]];
g=elltatepairing(E,P,Q,r)^168;if(g==1 || g^r!=1,error("degenerate pairing"));
targets={};logs=List();
for(j=1,#targets,T=[targets[j][1]*w^0,targets[j][2]*w^0];v=elltatepairing(E,T,Q,r)^168;k=fflog(v,g,r);if(ellmul(E,P,k)!=T,error("wrong source logarithm"));listput(logs,k));
jstr(z)=Str(Strchr(34),z,Strchr(34));
print("{{",jstr("recovered_logs"),":",Vec(logs),",",jstr("nondegenerate_order"),":11,",jstr("source_logarithms_checked"),":10}}");quit;
"#,
        px.0,
        py.0,
        serde_json::to_string(&target_list)?
    );
    let gp = out.join("pairing.gp");
    fs::write(&gp, &script)?;
    let status = Command::new("/usr/bin/timeout")
        .args(["15s", "/tmp/twist-cas/usr/bin/gp", "-f", "-q"])
        .arg(&gp)
        .stdout(Stdio::from(fs::File::create(out.join("pari.stdout"))?))
        .stderr(Stdio::from(fs::File::create(out.join("pari.stderr"))?))
        .status()?;
    let output = fs::read_to_string(out.join("pari.stdout"))?;
    let value: Option<Value> = output
        .lines()
        .find(|l| l.starts_with('{'))
        .and_then(|s| serde_json::from_str(s).ok());
    write_json(
        out.join("PROCESS.json"),
        &json!({"exit_code":status.code(),"parsed":value.is_some()}),
    )?;
    let v = value.ok_or("no valid PARI pairing result")?;
    assert!(status.success());
    let logs: Vec<u64> = serde_json::from_value(v["recovered_logs"].clone())?;
    assert_eq!(logs, (1..11).collect::<Vec<_>>());
    for (k, row) in logs
        .iter()
        .zip(manifest["known_scalar_controls"].as_array().unwrap())
    {
        let point = e.pm(p, *k as u128);
        let (x, y) = point.unwrap();
        assert_eq!(row["point"], json!([x.0, y.0]));
        assert_eq!(tate(e, point, q, 11), qpow(e, g, *k));
    }
    write_json(
        out.join("E16-pairing-positive-control.json"),
        &json!({"source":manifest["source"],"extension":manifest["extension"],"r":11,"embedding_degree":2,"native_reduced_Tate_generator":g,"independent_PARI_logs":logs,"source_group_replay":true,"native_Miller_bilinearity_controls":10,"full_cost_comparison":false,"ordinary_or_NIST_transfer":false,"known_scalar_control_only":true}),
    )?;
    Ok(())
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    match args.get(1).map(String::as_str){
        Some("prepare-pairing")=>prepare_pairing(Path::new(args.get(2).ok_or("fresh directory")?)),
        Some("pairing")=>pairing(Path::new(args.get(2).ok_or("fixture")?),Path::new(args.get(3).ok_or("fresh directory")?)),
        Some("equal-height")=>{let f:Value=serde_json::from_slice(&fs::read(args.get(2).ok_or("fixtures")?)?)?;let fixtures:Vec<Fixture>=serde_json::from_value(f["fixtures"].clone())?;let selected:Vec<covers::Selected>=serde_json::from_slice(&fs::read(args.get(3).ok_or("covers")?)?)?;let out=Path::new(args.get(4).ok_or("fresh directory")?);fs::create_dir(out)?;let rows=pari::run_original_height(&fixtures,&selected,out)?;println!("{}",json!({"status":"finished","rows":rows}));Ok(())},
        _=>Err("usage: prepare-pairing FRESH_DIR | pairing FIXTURE FRESH_DIR | equal-height FIXTURES COVERS FRESH_DIR".into())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reduced_tate_pairing_is_nondegenerate_and_bilinear_on_positive_control() {
        let (e, p, q) = pairing_points();
        let g = tate(e, p, q, 11);
        assert_ne!(g, Q(1, 0));
        assert_eq!(qpow(e, g, 11), Q(1, 0));
        for k in 1..11 {
            assert_eq!(tate(e, e.pm(p, k), q, 11), qpow(e, g, k as u64));
        }
    }
}
