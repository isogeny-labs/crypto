use std::{env, fs, path::Path};
use torus_s4::sha256;
use torus_s4_tree::{actual_four_control, identity, s3_statement};
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 || args[1] != "actual-four" {
        return Err("usage: torus-s4-tree actual-four NEW_DIRECTORY".into());
    }
    let out = Path::new(&args[2]);
    fs::create_dir(out)?;
    fs::write(
        out.join("construction-pre-call.json"),
        serde_json::to_vec_pretty(
            &serde_json::json!({"schema":"torus-s4-tree.construction-pre-call/v1","purpose":"planted actual-field construction correctness; no solver measurement","binary_sha256":sha256(&fs::read(env::current_exe()?)?),"arity":4,"subgroup_prime_factors":[60824497u64,213156431u64],"coset_indices":[1,2,3,4],"public_torus_exponents":[18,35,52,69]}),
        )?,
    )?;
    let (params, input, tree, points) = actual_four_control()?;
    fs::write(
        out.join("parameters.json"),
        serde_json::to_vec_pretty(&params.record())?,
    )?;
    fs::write(out.join("input.json"), serde_json::to_vec_pretty(&input)?)?;
    fs::write(
        out.join("system.json"),
        serde_json::to_vec_pretty(&tree.system.canonical_json())?,
    )?;
    fs::write(out.join("input.sing"), tree.system.singular())?;
    fs::write(
        out.join("energy-arithmetic.json"),
        serde_json::to_vec_pretty(&torus_s4_tree::energy::arithmetic_certificate(&params)?)?,
    )?;
    let curve = torus_s4::Curve::from_case(&input.curve)?;
    let fiber = torus_s4_tree::transport::sections(&curve, &input.curve.point()?)?;
    fs::create_dir(out.join("prime-sections"))?;
    for (j, point) in fiber.raw_sections.iter().enumerate() {
        let (u, v) = point.as_ref().ok_or("identity raw section")?;
        let mut section = input.clone();
        section.curve.target = [u.to_string().parse()?, v.to_string().parse()?];
        section.curve.id = format!("prime_section_{j}");
        let section_system = torus_s4_tree::TreeSystem::compile(&section)?;
        let dir = out.join("prime-sections").join(j.to_string());
        fs::create_dir(&dir)?;
        fs::write(dir.join("input.json"), serde_json::to_vec_pretty(&section)?)?;
        fs::write(
            dir.join("system.json"),
            serde_json::to_vec_pretty(&section_system.system.canonical_json())?,
        )?;
        fs::write(dir.join("input.sing"), section_system.system.singular())?;
    }
    fs::write(
        out.join("prime-transport.json"),
        serde_json::to_vec_pretty(
            &serde_json::json!({"prime_target":fiber.prime_target.as_ref().map(|(u,v)|[u.to_string(),v.to_string()]),"torsion_generator":fiber.torsion_generator.as_ref().map(|(u,v)|[u.to_string(),v.to_string()]),"raw_section_count":fiber.raw_sections.len(),"every_section_clears_to_prime_target":"producer_pass","pcc_standing":"unverified"}),
        )?,
    )?;
    let certificate = identity::issue(
        s3_statement(),
        2026111504,
        32,
        "Producer issue; maintained native checker reused; signed independent verification pending",
        None,
    )?;
    let checked = identity::check_with_extra(&certificate, Some((2026111509, 32)))?;
    if !matches!(checked, identity::Verdict::Accept { .. }) {
        return Err("identity certificate check failed".into());
    }
    fs::write(
        out.join("s3-identity.json"),
        serde_json::to_vec_pretty(&certificate)?,
    )?;
    for plus in [true, false] {
        let cert = identity::issue(
            torus_s4_tree::elliptic_map_statement(plus),
            2026111511,
            32,
            "Producer issue; maintained native identity checker, formalization pending",
            None,
        )?;
        if !matches!(
            identity::check_with_extra(&cert, Some((2026111517, 32)))?,
            identity::Verdict::Accept { .. }
        ) {
            return Err("elliptic-map identity check failed".into());
        }
        fs::write(
            out.join(if plus {
                "map-plus-identity.json"
            } else {
                "map-minus-identity.json"
            }),
            serde_json::to_vec_pretty(&cert)?,
        )?;
    }
    fs::write(
        out.join("known-points.json"),
        serde_json::to_vec_pretty(
            &points
                .iter()
                .map(|p| p.as_ref().map(|(u, v)| [u.to_string(), v.to_string()]))
                .collect::<Vec<_>>(),
        )?,
    )?;
    let values = tree.witness(&input, &points)?;
    tree.system.check(&values)?;
    fs::write(
        out.join("construction-post-call.json"),
        serde_json::to_vec_pretty(
            &serde_json::json!({"schema":"torus-s4-tree.construction-post-call/v1","pre_call_sha256":sha256(&fs::read(out.join("construction-pre-call.json"))?),"arity":4,"variables":tree.system.variables.len(),"equations":tree.system.equations.len(),"maximum_input_degree":tree.system.equations.iter().map(|p|p.degree()).max(),"planted_witness_check":"producer_pass","identity_certificate_id":certificate.id,"solver_invoked":false,"pcc_standing":"unverified"}),
        )?,
    )?;
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
