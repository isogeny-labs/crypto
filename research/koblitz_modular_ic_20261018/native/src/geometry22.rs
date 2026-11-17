//! Native CAS input emitted directly from the frozen Tate/QR circuit.
use super::*;
pub(crate) fn emit(dir: &Path) {
    let (p, _, _) = circuit(false);
    let mut s=String::from("// CM22 exact unchanged CM17 circuit; native Singular backend\noption(redSB);\nring r=2,(n,l,b,c),dp;\nideal nn; ideal dd; poly gg;\n");
    for (i, o) in p.nodes.iter().enumerate() {
        let k = i + 1;
        let (n, d) = match o {
            Op::Input(j) => (["b", "c", "l", "n"][*j].to_owned(), "1".into()),
            Op::Constant(v) => (v.to_string(), "1".into()),
            Op::Add(a, b) => (
                format!("nn[{}]*dd[{}]+nn[{}]*dd[{}]", a + 1, b + 1, b + 1, a + 1),
                format!("dd[{}]*dd[{}]", a + 1, b + 1),
            ),
            Op::Mul(a, b) => (
                format!("nn[{}]*nn[{}]", a + 1, b + 1),
                format!("dd[{}]*dd[{}]", a + 1, b + 1),
            ),
            Op::Inverse(a) => (format!("dd[{}]", a + 1), format!("nn[{}]", a + 1)),
        };
        s.push_str(&format!("nn[{k}]={n};dd[{k}]={d};gg=gcd(nn[{k}],dd[{k}]);nn[{k}]=nn[{k}]/gg;dd[{k}]=dd[{k}]/gg;\n"));
    }
    s.push_str("poly unitprod=1;\n");
    for u in &p.units {
        s.push_str(&format!("unitprod=unitprod*nn[{}];\n", u + 1));
    }
    s.push_str(&format!("poly tate=gcd(nn[{}],nn[{}]);\ngg=gcd(tate,unitprod);while(gg!=1){{tate=tate/gg;gg=gcd(tate,unitprod);}}\nprint(\"BEGIN_TATE11\");tate;print(\"END_TATE11\");\nideal original;\n",p.res[0]+1,p.res[1]+1));
    for (j, i) in p.res.iter().enumerate() {
        s.push_str(&format!("original[{}]=nn[{}];\n", j + 1, i + 1));
    }
    s.push_str(
        "print(\"BEGIN_ORIGINAL\");original;print(\"END_ORIGINAL\");\nprint(\"BEGIN_UNITS\");\n",
    );
    for u in &p.units {
        s.push_str(&format!("print(\"node{u}\");nn[{}];\n", u + 1));
    }
    s.push_str("print(\"END_UNITS\");\nLIB \"elim.lib\";\nprint(\"BEGIN_SATURATION\");\nideal source=sat(original,ideal(unitprod));\nprint(\"SATURATION_EXPONENT_NOT_RECORDED\");\nprint(\"SATURATED_DIMENSION\");dim(source);\nsource;\nprint(\"END_SATURATION\");\nwrite(\"cm22-saturated.dump\",\"ring r=2,(n,l,b,c),dp;\");\nwrite(\"cm22-saturated.dump\",\"ideal source=\"+string(source)+\";\");\nLIB \"normal.lib\";\nprint(\"BEGIN_NORMALIZATION\");\nlist normalized=normal(source,\"withRing\");\nnormalized;\nprint(\"END_NORMALIZATION\");\nquit;\n");
    assert!(!dir.join("cm22-incidence.sing").exists());
    std::fs::write(dir.join("cm22-incidence.sing"), s).unwrap();
    crate::round02::save(
        dir,
        "cm22-input-circuit.json",
        &json!({"schema":"koblitz.round22/cm22-input-v1","source":"unchanged carrier16::circuit(false)","nodes":p.nodes,"residuals":p.res,"units":p.units,"arithmetic":"F2 rational polynomial gates","backend":"Singular4.4.1"}),
    );
}

pub(crate) fn generic_qr(dir: &Path) {
    let full = std::fs::read_to_string(dir.join("cm22-incidence.sing")).unwrap();
    let prefix = full
        .split("poly unitprod=1;")
        .next()
        .unwrap()
        .replace("ring r=2,(n,l,b,c),dp;", "ring r=(2,b,c),(n,l),lp;");
    let (p, _, _) = circuit(false);
    let mut s = prefix;
    s.push_str("ideal qr;\n");
    for (j, i) in p.res.iter().skip(2).enumerate() {
        s.push_str(&format!("qr[{}]=nn[{}];\n", j + 1, i + 1));
    }
    s.push_str("print(\"BEGIN_GENERIC_QR\");ideal generic=std(qr);generic;print(\"VDIM\");vdim(generic);print(\"END_GENERIC_QR\");quit;\n");
    assert!(!dir.join("cm22-generic-qr.sing").exists());
    std::fs::write(dir.join("cm22-generic-qr.sing"), s).unwrap();
    let base = r#"// CM22 exact base11 birational identities; all displayed residuals must be zero.
ring r=2,(u,v),dp;
poly e=v^2+v+u^3+u;
ideal E=std(ideal(e));
poly rn=(u+1)*v+1;poly rd=u^3;
poly cn=rn+rd;poly cd=u*rd;
poly bn=rn*cn;poly bd=rd*cd;
poly F=bn*bd^4*cn^7+bn^2*bd^3*cn^5*cd^2+bn*bd^4*cn^6*cd+bn^2*bd^3*cn^4*cd^3+bd^5*cn^6*cd+bn^5*cd^7+bn^4*bd*cn*cd^6+bn^3*bd^2*cn^2*cd^5+bn^2*bd^3*cn^3*cd^4;
print("FORWARD_TATE_RESIDUAL");reduce(F,E);
print("INVERSE_U_RESIDUAL");reduce(u*cn^2*bd+(bn*cd+cn*bd)*cd,E);
print("INVERSE_V_RESIDUAL");reduce(v*(u+1)*cn*bd+(bn*cd*u^3+cn*bd),E);
LIB "brnoeth.lib";
list base=Adj_div(e);
print("BASE_DEGREE_GENUS");base[2];
print("BASE_PLACES");base[3];
print("BASE_CONDUCTOR");base[4];
quit;
"#;
    assert!(!dir.join("cm22-base11.sing").exists());
    std::fs::write(dir.join("cm22-base11.sing"), base).unwrap();
}

/// Construct the degree32 projection over the proved genus-one base.
/// This emits exact elimination, with no assumed pole multiplicities.
pub(crate) fn projection(dir: &Path) {
    let raw = std::fs::read_to_string(dir.join("cm22-generic-qr-01.log")).unwrap();
    let line = raw.lines().find(|s| s.starts_with("generic[1]=")).unwrap();
    let f = line.strip_prefix("generic[1]=").unwrap();
    let nline = raw.lines().find(|s| s.starts_with("generic[2]=")).unwrap();
    let nf = nline.strip_prefix("generic[2]=").unwrap();
    // Singular's compact polynomial output is valid input for the same ring.
    let mut s=format!("ring original=2,(n,l,b,c),dp;\npoly lambda={f};\npoly recovernu={nf};\nring r=(2,u),(v,l,n),dp;\npoly e=v^2+v+u^3+u;\npoly rn=(u+1)*v+1;number rd=u^3;\npoly cn=rn+rd;number cd=u*rd;\npoly b=rn*cn/(rd*cd);poly c=cn/cd;\nmap substitute=original,n,l,b,c;\npoly f=reduce(substitute(lambda),std(ideal(e)));\npoly rnurelation=reduce(substitute(recovernu),std(ideal(e)));\nprint(\"BEGIN_LAMBDA_TOWER\");f;print(\"END_LAMBDA_TOWER\");\nprint(\"BEGIN_NU_TOWER\");rnurelation;print(\"END_NU_TOWER\");\npoly fv=subst(f,v,v+1);\npoly plane=reduce(f*fv,std(ideal(e)));\nprint(\"BEGIN_PLANE32\");plane;print(\"END_PLANE32\");\nwrite(\"cm22-plane32.dump\",\"ring r=(2,u),(v,l,n),dp;\");\nwrite(\"cm22-plane32.dump\",\"poly plane=\"+string(plane)+\";\");\nwrite(\"cm22-plane32.dump\",\"poly tower=\"+string(f)+\";\");\nprint(\"PLANE_V_DEGREE\");deg(plane,v);\nprint(\"BIRATIONAL_GCD\");ideal inverse=std(ideal(f,plane));\nLIB \"elim.lib\";\n// Clear only powers of u; all functions currently have polynomial u-denominators.\nring affine=2,(v,l,n,u),dp;\nmap lift=r,v,l,n;\npoly numerator=lift(plane);\nprint(\"BEGIN_CLEARED_PLANE\");numerator;print(\"END_CLEARED_PLANE\");\nquit;\n");
    // Parameters cannot be specialized into a polynomial ring by a ring map.
    // Use cleardenom while u is a parameter, then parse its printed numerator
    // in a subsequent plane chart. Retain the exact rational tower first.
    s = s
        .split("print(\"PLANE_V_DEGREE\");")
        .next()
        .unwrap()
        .to_owned();
    s.push_str(
        "print(\"BEGIN_CLEARED_PLANE\");cleardenom(plane);print(\"END_CLEARED_PLANE\");quit;\n",
    );
    assert!(!dir.join("cm22-projection.sing").exists());
    std::fs::write(dir.join("cm22-projection.sing"), s).unwrap();
}

pub(crate) fn plane_places(dir: &Path) {
    let raw = std::fs::read_to_string(dir.join("cm22-projection-01.log")).unwrap();
    let plane = raw
        .split("BEGIN_CLEARED_PLANE\n")
        .nth(1)
        .unwrap()
        .split("\nEND_CLEARED_PLANE")
        .next()
        .unwrap();
    assert!(!plane.contains('?') && !plane.contains('/'));
    let s=format!("ring r=2,(u,l),dp;\npoly f={plane};\nprint(\"PLANE_DEGREE\");deg(f);\nLIB \"brnoeth.lib\";\nlist C=Adj_div(f);\nprint(\"PLANE_DEGREE_GENUS\");C[2];\nprint(\"PLANE_PLACES\");C[3];\nprint(\"PLANE_CONDUCTOR\");C[4];\nprint(\"NORMAL_PLACES_FINISHED\");\nwrite(\"cm22-places.ssi\",C);\nquit;\n");
    assert!(!dir.join("cm22-plane-places.sing").exists());
    std::fs::write(dir.join("cm22-plane-places.sing"), s).unwrap();
}

pub(crate) fn toric_model(dir: &Path) {
    let raw = std::fs::read_to_string(dir.join("cm22-projection-01.log")).unwrap();
    let plane = raw
        .split("BEGIN_CLEARED_PLANE\n")
        .nth(1)
        .unwrap()
        .split("\nEND_CLEARED_PLANE")
        .next()
        .unwrap();
    let mut pieces = vec![];
    let (mut depth, mut start) = (0i32, 0usize);
    for (i, c) in plane.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            '+' if depth == 0 => {
                pieces.push(&plane[start..i]);
                start = i + 1;
            }
            _ => {}
        }
        assert!(depth >= 0);
    }
    assert_eq!(depth, 0);
    pieces.push(&plane[start..]);
    let mut support = BTreeSet::new();
    for term in pieces {
        let end = term.find(')').unwrap();
        assert!(term.starts_with('('));
        let tail = &term[end + 1..];
        let le = if tail.is_empty() {
            0
        } else {
            let p = tail.strip_prefix("*l").unwrap();
            if p.is_empty() {
                1
            } else {
                p.parse::<i32>().unwrap()
            }
        };
        for c in term[1..end].split('+') {
            let ue = if c == "1" {
                0
            } else {
                let p = c.strip_prefix('u').unwrap();
                if p.is_empty() {
                    1
                } else {
                    p.parse::<i32>().unwrap()
                }
            };
            assert!(support.insert((ue, le)), "unexpected F2 duplicate");
        }
    }
    let mut best = None;
    let mut tested = 0;
    for a in -8i32..=8 {
        for b in -8i32..=8 {
            for c in -8i32..=8 {
                for d in -8i32..=8 {
                    let det = a * d - b * c;
                    if det.abs() != 1 {
                        continue;
                    }
                    tested += 1;
                    let su = support.iter().map(|(u, l)| a * u + c * l).min().unwrap();
                    let sl = support.iter().map(|(u, l)| b * u + d * l).min().unwrap();
                    let out = support
                        .iter()
                        .map(|(u, l)| (a * u + c * l - su, b * u + d * l - sl))
                        .collect::<BTreeSet<_>>();
                    assert_eq!(out.len(), support.len());
                    let degree = out.iter().map(|(x, y)| x + y).max().unwrap();
                    let key = (degree, a, b, c, d);
                    if best.as_ref().is_none_or(|(k, _, _, _)| key < *k) {
                        best = Some((key, su, sl, out));
                    }
                }
            }
        }
    }
    let ((degree, a, b, c, d), su, sl, out) = best.unwrap();
    let det = a * d - b * c;
    let inverse = [[d / det, -b / det], [-c / det, a / det]];
    assert_eq!(a * inverse[0][0] + b * inverse[1][0], 1);
    assert_eq!(a * inverse[0][1] + b * inverse[1][1], 0);
    assert_eq!(c * inverse[0][0] + d * inverse[1][0], 0);
    assert_eq!(c * inverse[0][1] + d * inverse[1][1], 1);
    let poly = out
        .iter()
        .map(|(x, y)| format!("x^{x}*y^{y}"))
        .collect::<Vec<_>>()
        .join("+");
    let s=format!("// Exact GL2(Z) torus change; proper branches added by normalization.\nring r=2,(x,y),dp;\npoly f={poly};\nprint(\"TORIC_DEGREE\");deg(f);\nLIB \"brnoeth.lib\";\nlist C=Adj_div(f);\nprint(\"TORIC_DEGREE_GENUS\");C[2];\nprint(\"TORIC_PLACES\");C[3];\nprint(\"TORIC_CONDUCTOR\");C[4];\nprint(\"TORIC_PLACES_FINISHED\");\nquit;\n");
    assert!(!dir.join("cm22-toric-places.sing").exists());
    std::fs::write(dir.join("cm22-toric-places.sing"), s).unwrap();
    crate::round02::save(
        dir,
        "cm22-toric-model.json",
        &json!({"schema":"koblitz.round22/cm22-toric-v1","old_support":support,"transformed_support":out,"matrix_rows_old_exponents_to_new_coordinates":[[a,b],[c,d]],"inverse_matrix":inverse,"common_monomial_exponents":[su,sl],"degree_before":96,"degree_after":degree,"tested_unimodular_matrices":tested,"parameter_grid":[-8,8],"determinant":det,"birational_on_torus":true,"proper_normalization_complete":false}),
    );
    println!("CM22 toric model:{tested} unimodular changes;degree96->{degree};matrix[{a},{b};{c},{d}];det{det}");
}

pub(crate) fn chart_checks(dir: &Path) {
    let full = std::fs::read_to_string(dir.join("cm22-incidence.sing")).unwrap();
    let prefix = full.split("print(\"BEGIN_TATE11\");").next().unwrap();
    let (p, _, _) = circuit(false);
    let mut s = prefix.to_owned();
    s.push_str("ideal original;\n");
    for (j, i) in p.res.iter().enumerate() {
        s.push_str(&format!("original[{}]=nn[{}];\n", j + 1, i + 1));
    }
    s.push_str("LIB \"elim.lib\";\nideal singular=original+minor(jacob(original),3);\nprint(\"BEGIN_OPEN_SINGULAR_LOCUS\");\nideal bad=sat(singular,ideal(unitprod));bad;\nprint(\"END_OPEN_SINGULAR_LOCUS\");quit;\n");
    assert!(!dir.join("cm22-open-smooth.sing").exists());
    std::fs::write(dir.join("cm22-open-smooth.sing"), s).unwrap();
    let generic = std::fs::read_to_string(dir.join("cm22-generic-qr.sing")).unwrap();
    let mut g = generic
        .split("print(\"BEGIN_GENERIC_QR\");")
        .next()
        .unwrap()
        .to_owned();
    g.push_str("ideal generic=std(qr);matrix forward=lift(qr,generic);matrix backward=lift(generic,qr);\nprint(\"GENERIC_FORWARD_IDENTITY\");matrix forwardResidual=matrix(qr)*forward-matrix(generic);forwardResidual;\nprint(\"GENERIC_BACKWARD_IDENTITY\");matrix bres=matrix(generic)*backward-matrix(qr);bres;\nprint(\"BEGIN_GENERIC_FORWARD_CERTIFICATE\");forward;print(\"END_GENERIC_FORWARD_CERTIFICATE\");\nprint(\"BEGIN_GENERIC_BACKWARD_CERTIFICATE\");backward;print(\"END_GENERIC_BACKWARD_CERTIFICATE\");quit;\n");
    assert!(!dir.join("cm22-generic-certificate.sing").exists());
    std::fs::write(dir.join("cm22-generic-certificate.sing"), g).unwrap();
    let projection = std::fs::read_to_string(dir.join("cm22-projection.sing")).unwrap();
    let mut g = projection
        .split("print(\"BEGIN_CLEARED_PLANE\");")
        .next()
        .unwrap()
        .to_owned();
    g.push_str("poly slope=diff(f,v);poly intercept=subst(f,v,0);\nprint(\"LINEAR_V_RECOVERY_RESIDUAL\");f+intercept+v*slope;\nprint(\"NORM_FORMULA_RESIDUAL\");plane+intercept^2+intercept*slope+(u^3+u)*slope^2;\nprint(\"RECOVERY_SLOPE_GCD\");gcd(plane,slope);\nprint(\"BEGIN_PLANE_FACTORISATION\");factorize(plane);print(\"END_PLANE_FACTORISATION\");quit;\n");
    assert!(!dir.join("cm22-projection-certificate.sing").exists());
    std::fs::write(dir.join("cm22-projection-certificate.sing"), g).unwrap();
    let model = crate::round04::read(&dir.join("cm22-toric-model.json"));
    let tops = model["transformed_support"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|p| p[1] == 32)
        .collect::<Vec<_>>();
    assert_eq!(tops.len(), 1);
    assert_eq!(
        tops[0][0], 0,
        "monic degree32 in the second variable required"
    );
    let toric = std::fs::read_to_string(dir.join("cm22-toric-places.sing")).unwrap();
    let mut g = toric
        .split("print(\"TORIC_DEGREE\");")
        .next()
        .unwrap()
        .to_owned();
    g.push_str("LIB \"integralbasis.lib\";\nprint(\"BEGIN_INTEGRAL_BASIS\");\nlist basis=integralBasis(f,2,\"normal\",\"local\",\"nonModular\");\nbasis;print(\"END_INTEGRAL_BASIS\");quit;\n");
    assert!(!dir.join("cm22-integral-basis.sing").exists());
    std::fs::write(dir.join("cm22-integral-basis.sing"), g).unwrap();
}

pub(crate) fn trace_tower(dir: &Path) {
    let full = std::fs::read_to_string(dir.join("cm22-generic-qr.sing")).unwrap();
    let mut s = full
        .split("print(\"BEGIN_GENERIC_QR\");")
        .next()
        .unwrap()
        .to_owned();
    s.push_str("ideal generic=std(qr);poly aa=1+c;poly delta=b3*(b+c*aa^3);\npoly compact=l16+aa^8*l8+delta*l4+aa^3*delta*l+b4*(b4+b*c*aa^3+c2*aa^4);\nprint(\"COMPACT_LAMBDA_RESIDUAL\");generic[1]+compact;\nprint(\"NU_DENOMINATOR_RESIDUAL\");diff(generic[2],n)+aa^2*delta;\npoly tracePolynomial=n8+delta*n2+aa^2*delta*n+b4*(b2+aa^2*b+c*aa^4);\nprint(\"TRACE_TOWER_RESIDUAL\");generic[1]+subst(tracePolynomial,n,l2+aa*l+b);\nprint(\"BEGIN_TRACE_POLYNOMIAL\");tracePolynomial;print(\"END_TRACE_POLYNOMIAL\");quit;\n");
    assert!(!dir.join("cm22-trace-tower.sing").exists());
    std::fs::write(dir.join("cm22-trace-tower.sing"), s).unwrap();
}

pub(crate) fn crosscheck(dir: &Path) {
    let full = std::fs::read_to_string(dir.join("cm22-incidence.sing")).unwrap();
    let mut s = full.split("poly unitprod=1;").next().unwrap().to_owned();
    let (p, _, _) = circuit(false);
    for (i, o) in p.nodes.iter().enumerate() {
        let k = i + 1;
        let (n, d) = match o {
            Op::Input(j) => (["b", "c", "l", "n"][*j].to_owned(), "1".into()),
            Op::Constant(v) => (v.to_string(), "1".into()),
            Op::Add(a, b) => (
                format!("nn[{}]*dd[{}]+nn[{}]*dd[{}]", a + 1, b + 1, b + 1, a + 1),
                format!("dd[{}]*dd[{}]", a + 1, b + 1),
            ),
            Op::Mul(a, b) => (
                format!("nn[{}]*nn[{}]", a + 1, b + 1),
                format!("dd[{}]*dd[{}]", a + 1, b + 1),
            ),
            Op::Inverse(a) => (format!("dd[{}]", a + 1), format!("nn[{}]", a + 1)),
        };
        s.push_str(&format!("if(dd[{k}]==0){{print(\"FAIL_ZERO_DENOMINATOR_{i}\");quit;}}\nif(nn[{k}]*({d})+dd[{k}]*({n})!=0){{print(\"FAIL_GATE_{i}\");quit;}}\n"));
    }
    s.push_str(&format!(
        "print(\"PASS_ALL_{}_RATIONAL_GATE_IDENTITIES\");quit;\n",
        p.nodes.len()
    ));
    assert!(!dir.join("cm22-rational-gate-check.sing").exists());
    std::fs::write(dir.join("cm22-rational-gate-check.sing"), s).unwrap();
}

/// Normalize the smaller trace subfield before adjoining lambda.
/// Inverts the proved base coordinate to reduce pole clearing at u=0.
pub(crate) fn intermediate(dir: &Path) {
    let s = r#"// Exact degree8 trace subfield; source H identity checked separately.
ring tower=(2,u),(v,z),dp;
number x=1/u;
poly e=v^2+v+x^3+x;
ideal E=std(ideal(e));
poly rn=(x+1)*v+1;
poly cn=rn+x^3;
poly b=rn*cn/x^7;
poly c=cn/x^4;
poly aa=1+c;
poly delta=b^3*(b+c*aa^3);
poly H=reduce(z^8+delta*z^2+aa^2*delta*z+b^4*(b^2+aa^2*b+c*aa^4),E);
poly plane=reduce(H*subst(H,v,v+1),E);
print("TRACE_PLANE_V_DEGREE");deg(plane,intvec(1,0));
print("BEGIN_TRACE_PLANE_FACTORS");factorize(plane);print("END_TRACE_PLANE_FACTORS");
string equation=string(cleardenom(plane));
print("BEGIN_TRACE_CLEARED");equation;print("END_TRACE_CLEARED");
ring affine=2,(u,z),dp;
execute("poly f="+equation+";");
print("TRACE_TOTAL_DEGREE");deg(f);
write("cm22-intermediate-plane.dump","ring affine=2,(u,z),dp;");
write("cm22-intermediate-plane.dump","poly f="+string(f)+";");
LIB "brnoeth.lib";
print("BEGIN_TRACE_ADJUNCTION");
list normalizedTrace=Adj_div(f);
print("TRACE_DEGREE_GENUS");normalizedTrace[2];
print("TRACE_PLACES");normalizedTrace[3];
print("TRACE_CONDUCTOR");normalizedTrace[4];
print("END_TRACE_ADJUNCTION");
write("cm22-intermediate-places.ssi",normalizedTrace);
quit;
"#;
    assert!(!dir.join("cm22-intermediate.sing").exists());
    std::fs::write(dir.join("cm22-intermediate.sing"), s).unwrap();
    let metadata = format!("{}quit;\n", s.split("LIB \"brnoeth.lib\";").next().unwrap());
    assert!(!dir.join("cm22-intermediate-metadata.sing").exists());
    std::fs::write(dir.join("cm22-intermediate-metadata.sing"), metadata).unwrap();
}
