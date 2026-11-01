import os
d = os.path.join(os.path.dirname(os.path.abspath(__file__)), "emitter", "src")
s = open(f"{d}/encode.rs").read()
def rep(s, a, b):
    assert a in s, a
    return s.replace(a, b)
s = rep(s, "    pub e3: Option<Vec<Lit>>,\n}", "    pub e3: Option<Vec<Lit>>,\n    pub xr: Vec<Lit>,\n    pub yr: Vec<Lit>,\n}")
s = rep(s, "    use_xor: bool,\n) -> Option<(Cnf, Decoder)> {", "    use_xor: bool,\n    tv: bool,\n) -> Option<(Cnf, Decoder)> {")
s = rep(s, "    let xrc = Cnf::const_field(xr, p, n);\n    let yrc = Cnf::const_field(yr, p, n);",
        "    let (xrc, yrc) = if tv { (cnf.fresh_field(p, n), cnf.fresh_field(p, n)) } else { (Cnf::const_field(xr, p, n), Cnf::const_field(yr, p, n)) };")
s = rep(s, "        (2, _) => {\n            s3_zero(&mut cnf, &xs[0], &xs[1], xr, curve, n);",
        "        (2, _) => {\n            if tv { s3_zero_var(&mut cnf, &xs[0], &xs[1], &xrc, curve, n); } else {\n            s3_zero(&mut cnf, &xs[0], &xs[1], xr, curve, n); }")
s = rep(s, "            s3_zero(&mut cnf, &xt, &xs[2], xr, curve, n);",
        "            if tv { s3_zero_var(&mut cnf, &xt, &xs[2], &xrc, curve, n); } else { s3_zero(&mut cnf, &xt, &xs[2], xr, curve, n); }")
s = rep(s, "Decoder { x1: xs[0].clone(), x2: xs[1].clone(), x3, e1, e2, e3 }",
        "Decoder { x1: xs[0].clone(), x2: xs[1].clone(), x3, e1, e2, e3, xr: xrc.clone(), yr: yrc.clone() }")
open(f"{d}/encode.rs", "w").write(s)
m = open(f"{d}/main.rs").read()
m = rep(m, "encode(&curve, family, enc, m, t.r, sym, solver.uses_xor())", "encode(&curve, family, enc, m, t.r, sym, solver.uses_xor(), false)")
emit = r'''fn emit(args: &Args) {
    let seed = args.num("seed", 1);
    let mut rng = Rng::new(seed);
    let curve = pick_curve(args, &mut rng);
    let l = args.num("l", 3) as u32;
    let m = args.num("m", 3) as u32;
    let nt = args.num("nt", 16) as usize;
    let enc = Encoding::parse(&args.get("encoding", "bits-s3")).unwrap();
    let family = make_family(&curve, "small-x", l, &mut rng).unwrap();
    let fb = FactorBase::build(&curve, family);
    let tg = targets(&curve, &fb, m, (nt / 2) as u64, (nt - nt / 2) as u64, &mut rng);
    let (pl, un): (Vec<&Target>, Vec<&Target>) = tg.iter().partition(|t| t.kind == "planted");
    let mut order: Vec<&Target> = Vec::new();
    for i in 0..pl.len().max(un.len()) {
        if i < pl.len() { order.push(pl[i]); }
        if i < un.len() { order.push(un[i]); }
    }
    let dummy = random_point(&curve, &mut rng);
    let (cnf, dec) = encode(&curve, family, enc, m, dummy, true, false, true).unwrap();
    let out = PathBuf::from(args.get("out", "."));
    fs::create_dir_all(&out).unwrap();
    {
        let mut f = std::io::BufWriter::new(fs::File::create(out.join("inst.cnf")).unwrap());
        cnf.write_dimacs(&mut f).unwrap();
    }
    let mut tf = fs::File::create(out.join("targets.tsv")).unwrap();
    let (nv, nc, _) = cnf.stats();
    writeln!(tf, "# p={} a={} b={} bits={} l={} m={} enc={} seed={} nvars={} nclauses={}", curve.p, curve.a, curve.b, curve.bits(), l, m, enc.name(), seed, nv, nc).unwrap();
    for t in order {
        let Pt::A(xr, yr) = t.r else { continue };
        let (o, _, t_enum) = oracle(&curve, &fb, m, t.r);
        let mut lits: Vec<i32> = Vec::new();
        for (i, &v) in dec.xr.iter().enumerate() { lits.push(if (xr >> i) & 1 == 1 { v } else { -v }); }
        if enc == Encoding::BitsGroupLaw { for (i, &v) in dec.yr.iter().enumerate() { lits.push(if (yr >> i) & 1 == 1 { v } else { -v }); } }
        let ls: Vec<String> = lits.iter().map(|x| x.to_string()).collect();
        writeln!(tf, "{}\t{}\t{}\t{}", t.kind, o.is_some() as u8, t_enum, ls.join(" ")).unwrap();
    }
}

fn main() {
    let args = Args::parse();
    match args.positional.first().map(|s| s.as_str()) {
        Some("emit") => emit(&args),'''
m = rep(m, "fn main() {\n    let args = Args::parse();\n    match args.positional.first().map(|s| s.as_str()) {", emit)
open(f"{d}/main.rs", "w").write(m)
print("patched")
