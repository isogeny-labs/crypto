//! Native preparation and exact projected-output checking. This CLI never
//! submits a benchmark or silently starts msolve.
use m31_projective_preclosure::multiplicity;
use m31_projective_preclosure::quotient::{field_add,field_mul,field_pow,field_sub};
use m31_projective_preclosure::systems::{self,Arm,WitnessMode};
use m31_projective_preclosure::M31;
use std::{collections::BTreeSet,env,fs,path::Path,process};

fn option<'a>(args:&'a[String], key:&str)->Option<&'a str> {
    args.windows(2).find(|x|x[0]==key).map(|x|x[1].as_str())
}
fn number(args:&[String],key:&str,default:usize)->Result<usize,String> {
    option(args,key).map_or(Ok(default),|x|x.parse().map_err(|_|format!("invalid {key}")))
}
fn required<'a>(args:&'a[String],key:&str)->Result<&'a str,String> {
    option(args,key).ok_or_else(||format!("missing {key}"))
}
fn validate_options(args:&[String],allowed:&[&str])->Result<(),String> {
    if args.len()%2!=0 {return Err("every option requires exactly one value".into());}
    let mut seen=BTreeSet::new();
    for pair in args.chunks_exact(2) {
        if !allowed.contains(&pair[0].as_str()) || !seen.insert(pair[0].as_str()) {
            return Err(format!("unknown or duplicate option {}",pair[0]));
        }
    }
    Ok(())
}
fn write_new(path:&Path,contents:impl AsRef<[u8]>)->Result<(),String> {
    use std::io::Write;
    let mut file=fs::OpenOptions::new().write(true).create_new(true).open(path).map_err(|e|format!("{}: {e}",path.display()))?;
    file.write_all(contents.as_ref()).map_err(|e|e.to_string())
}
fn point_string(points:&BTreeSet<[u64;2]>)->String {
    points.iter().map(|x|format!("{}:{}",x[0],x[1])).collect::<Vec<_>>().join(";")
}
fn prepare(args:&[String])->Result<(),String> {
    validate_options(args,&["--out","--cases","--max-summands","--mode"])?;
    let out=Path::new(required(args,"--out")?);
    if out.exists() { return Err("output directory already exists; preparation is append-free".into()); }
    let cases=number(args,"--cases",50)?;
    if cases<50 { return Err("comparison preparation requires at least 50 cases per configuration".into()); }
    if cases>500 { return Err("bounded preparation supports at most 500 cases per configuration".into()); }
    let max_m=number(args,"--max-summands",4)?;
    if !(3..=4).contains(&max_m) { return Err("qualified preparation supports m=3 or m=3,4; S6 construction remains separate".into()); }
    let mode=match option(args,"--mode").unwrap_or("fp2") {
        "fp2"=>WitnessMode::ExplicitFp2,"geometric"=>WitnessMode::Geometric,
        _=>return Err("--mode must be fp2 or geometric".into()),
    };
    fs::create_dir_all(out).map_err(|e|e.to_string())?;
    write_new(&out.join("PREPARING"),"Preparation is incomplete until PREPARED is present. No solver has been run.\n")?;
    let mut manifest=String::from("m\tcase\tfamily\tarm\tmode\tvariables\tvariable_order\tgenerator_slots\tmax_generator_degree\teliminate_first\tguard_euler_powers\texpected_projection\tinput_file\n");
    let mut fixtures=String::from("m\tcase\tseed\tfamily\ttarget_x\taffine_forms_c_d_e\n");
    let mut hashes=String::new();
    let mut total=0;
    for m in 3..=max_m { for case in 0..cases {
        let plane=systems::fixture_plane(m,case)?;
        let expected=systems::expected_projection(&plane)?;
        fixtures.push_str(&format!("{m}\t{case}\t{}\t{}\t{}\t{}\n",plane.seed,plane.family,plane.target_x,
            plane.forms.iter().map(|c|format!("{},{},{}",c[0],c[1],c[2])).collect::<Vec<_>>().join(";")));
        for arm in Arm::ALL {
            let system=systems::build(&plane,arm,mode).map_err(|e|e.to_string())?;
            let variable_order=system.msolve_variable_order().join(",");
            let name=format!("m{m}-case{case:03}-{}.ms",arm.name());
            write_new(&out.join(&name),system.msolve_input())?;
            let hash=process::Command::new("sha256sum").arg(&name).current_dir(out).output().map_err(|e|e.to_string())?;
            if !hash.status.success() { return Err("sha256sum failed".into()); }
            hashes.push_str(std::str::from_utf8(&hash.stdout).map_err(|e|e.to_string())?);
            manifest.push_str(&format!("{m}\t{case}\t{}\t{}\t{}\t{}\t{variable_order}\t{}\t{}\t{}\t{}\t{}\t{name}\n",
                plane.family,arm.name(),if mode==WitnessMode::ExplicitFp2{"fp2"}else{"geometric"},
                system.variables.len(),system.equations.len(),system.max_generator_degree(),system.parameters[0],
                system.guard_euler_powers,point_string(&expected)));
            total+=1;
        }
    }}
    write_new(&out.join("manifest.tsv"),manifest)?;
    write_new(&out.join("fixtures.tsv"),fixtures)?;
    write_new(&out.join("SHA256SUMS"),hashes)?;
    write_new(&out.join("PREPARED"),format!("schema=m31-projective-comparison-inputs/v1\nfield={M31}\ncases_per_configuration={cases}\ninput_files={total}\nsolver_executions=0\nsection=t*(t-1)=u*(u-1)=0\nstatus=prepared-not-measured\n"))?;
    println!("Prepared {total} exact M31 inputs; {cases} cases per configuration; solver executions: 0.");
    Ok(())
}

/// Strict scalar evaluation of a canonical msolve polynomial in t,u. An
/// auxiliary variable means elimination is incomplete and is rejected.
fn eval_term(term:&str,point:[u64;2])->Result<u64,String> {
    let mut out=1;
    for factor in term.split('*') {
        if factor.is_empty() { return Err("empty polynomial factor".into()); }
        if let Ok(c)=factor.parse::<u64>() { out=field_mul(out,c%M31);continue; }
        let mut pieces=factor.split('^');
        let var=pieces.next().unwrap();
        let exponent=pieces.next().map_or(Ok(1u64),|v|v.parse::<u64>().map_err(|_|"invalid exponent".to_string()))?;
        if pieces.next().is_some() { return Err("multiple exponents".into()); }
        let index=match var {"t"=>0,"u"=>1,_=>return Err(format!("nonprojected or unknown variable {var}"))};
        out=field_mul(out,field_pow(point[index],exponent));
    }
    Ok(out)
}
fn eval_polynomial(text:&str,point:[u64;2])->Result<u64,String> {
    if text.is_empty() {return Err("empty polynomial".into());}
    let bytes=text.as_bytes();let mut offset=0;let mut out=0;
    while offset<bytes.len() {
        let mut sign=1;
        if bytes[offset]==b'+'||bytes[offset]==b'-' {if bytes[offset]==b'-'{sign=-1;}offset+=1;}
        let start=offset;
        while offset<bytes.len()&&bytes[offset]!=b'+'&&bytes[offset]!=b'-' {offset+=1;}
        if start==offset {return Err("empty signed term".into());}
        let value=eval_term(&text[start..offset],point)?;
        out=if sign==1{field_add(out,value)}else{field_sub(out,value)};
    }
    Ok(out)
}
fn projected_points(basis:&str)->Result<BTreeSet<[u64;2]>,String> {
    let characteristic=basis.lines().find_map(|l|l.strip_prefix("#field characteristic:"))
        .ok_or("missing msolve field characteristic header")?.trim();
    if characteristic!="2147483647" {return Err("basis field is not M31".into());}
    let body=basis.lines().filter(|l|!l.starts_with('#')).collect::<String>();
    let clean=body.chars().filter(|c|!c.is_whitespace()).collect::<String>();
    let polys=clean.strip_prefix('[').and_then(|s|s.strip_suffix("]:"))
        .ok_or("expected complete msolve reduced-basis format [...]:")?;
    if polys.is_empty(){return Err("empty basis cannot certify the prescribed finite section".into());}
    let polys=polys.split(',').collect::<Vec<_>>();
    let mut result=BTreeSet::new();
    for t in 0..=1 {for u in 0..=1 {
        let mut all=true;
        // Parse every polynomial even after a nonzero value, to fail closed
        // on leftover auxiliaries or a truncated/malformed tail.
        for poly in &polys {if eval_polynomial(poly,[t,u])?!=0 {all=false;}}
        if all {result.insert([t,u]);}
    }}
    Ok(result)
}
fn check_projection(args:&[String])->Result<(),String> {
    validate_options(args,&["--summands","--case","--basis"])?;
    let m=number(args,"--summands",3)?;
    let case=number(args,"--case",0)?;
    let expected=systems::expected_projection(&systems::fixture_plane(m,case)?)?;
    let basis=fs::read_to_string(required(args,"--basis")?).map_err(|e|e.to_string())?;
    let observed=projected_points(&basis)?;
    if observed!=expected {return Err(format!("projected set mismatch: expected {}, observed {}",point_string(&expected),point_string(&observed)));}
    println!("M31 m={m} case={case}: exact projected set matches ({})",point_string(&observed));
    Ok(())
}
fn run()->Result<(),String> {
    let args=env::args().collect::<Vec<_>>();
    match args.get(1).map(String::as_str) {
        Some("prepare")=>prepare(&args[2..]),
        Some("check-projection")=>check_projection(&args[2..]),
        Some("multiplicity-gate")=>{if args.len()!=2{return Err("multiplicity-gate accepts no options".into());}println!("{}",multiplicity::run_multiplicity_gate()?.json());Ok(())},
        _=>Err("Usage: m31-projective-preclosure prepare --out NEW_DIR [--cases 50] [--max-summands 4] [--mode fp2|geometric]\n       m31-projective-preclosure check-projection --summands 3 --case 0 --basis REDUCED_BASIS\n       m31-projective-preclosure multiplicity-gate\nThis program does not launch a scientific benchmark.".into()),
    }
}
fn main(){if let Err(error)=run(){eprintln!("{error}");process::exit(1);}}

#[cfg(test)]
mod tests {
    use super::*;
    const HEADER:&str="#Reduced Groebner basis data\n#field characteristic: 2147483647\n";
    #[test]
    fn projection_parser_handles_exact_coefficients_and_rejects_auxiliary_or_wrong_field() {
        assert_eq!(projected_points(&format!("{HEADER}[t,u]:\n")).unwrap(),BTreeSet::from([[0,0]]));
        assert_eq!(projected_points(&format!("{HEADER}[t^2-t,u^2-u]:\n")).unwrap().len(),4);
        assert_eq!(projected_points(&format!("{HEADER}[2147483646*t+1,u]:\n")).unwrap(),BTreeSet::from([[1,0]]));
        assert!(projected_points(&format!("{HEADER}[t,z0]:\n")).is_err());
        assert!(projected_points("#field characteristic: 101\n[t,u]:\n").is_err());
        assert!(projected_points(&format!("{HEADER}[t,u]")).is_err());
        assert!(projected_points(&format!("{HEADER}[]:")).is_err());
    }
    #[test]
    fn changed_result_cannot_match_a_positive_fixture() {
        let expected=systems::expected_projection(&systems::fixture_plane(3,0).unwrap()).unwrap();
        assert!(expected.contains(&[0,0]));
        assert_ne!(projected_points(&format!("{HEADER}[1]:\n")).unwrap(),expected);
    }
}
