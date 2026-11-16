// Uses the byte-preserved v8 field/polynomial/map core in the binary prefix.
fn gm(r: Ring, a: u64, b: u64, n: u64) -> (Rat, Rat) {
    let p = (Rat::poly(vec![0, 1]), Rat::poly(vec![1]));
    let (mut q, mut acc, mut k) = (p, None, n);
    while k > 0 {
        if k & 1 == 1 {
            acc = Some(match acc {
                None => q.clone(),
                Some(z) => generic_add(r, a, b, &z, &q, false),
            });
        }
        k >>= 1;
        if k > 0 {
            q = generic_add(r, a, b, &q, &q, true);
        }
    }
    acc.unwrap()
}
fn rat_at(r: Ring, f: &Rat, x: &Rat) -> Rat {
    compose(r, &f.n, x).div(r, &compose(r, &f.d, x))
}
fn point_at(r: Ring, f: &(Rat, Rat), g: &(Rat, Rat)) -> (Rat, Rat) {
    (rat_at(r, &f.0, &g.0), g.1.mul(r, &rat_at(r, &f.1, &g.0)))
}
fn endo_produce(out: &Path) -> Result<()> {
    if out.exists() {
        return Err("fresh endomorphism output required".into());
    }
    fs::create_dir_all(out)?;
    let e = Quad::new(827, 1, 15);
    let t = Quad::new(827, 4, 120);
    let r = Ring(827);
    let h = Some((Q(2, 0), Q(386, 0)));
    assert_eq!(order(t, h, 833), 119);
    let kt = t.pm(h, 7);
    assert_eq!(order(t, kt, 833), 17);
    let kernel: BTreeSet<_> = (0..17).map(|k| embed(e, t.pm(kt, k))).collect();
    assert_eq!(kernel.len(), 17);
    for p in &kernel {
        assert!(e.on(*p));
        assert_eq!(e.pm(*p, 17), None);
        assert_eq!(e.frob(*p), e.pn(*p));
    }
    let kr = roots(&kernel);
    assert_eq!(kr.len(), 8);
    let map = Velu::roots(r, 1, 15, &kr);
    assert!(map.certify(r, 1, 15));
    let u = inv(7, 827);
    let ui = 7;
    assert_eq!(map.a, toy_field::pow(u, 4, 827));
    assert_eq!(map.b, 15 * toy_field::pow(u, 6, 827) % 827);
    let p = (Rat::poly(vec![0, 1]), Rat::poly(vec![1]));
    let a = map.rat(r, &p.0, &p.1);
    let alpha = (a.0.scale(r, 49), a.1.scale(r, 343));
    let conjugate = generic_add(
        r,
        1,
        15,
        &p,
        &(alpha.0.clone(), alpha.1.scale(r, 826)),
        false,
    );
    let seventeen = gm(r, 1, 15, 17);
    assert_eq!(point_at(r, &conjugate, &alpha), seventeen);
    assert_eq!(point_at(r, &alpha, &conjugate), seventeen);
    let seven = gm(r, 1, 15, 7);
    let composed = point_at(r, &seven, &alpha);
    let mut xp = vec![0; 828];
    xp[827] = 1;
    let frob = (Rat::poly(xp), Rat::poly(r.pow(&[15, 1, 0, 1], 413)));
    let frob_plus_one = generic_add(r, 1, 15, &frob, &p, false);
    assert_eq!(composed, frob_plus_one);
    let g = Some((Q(2, 0), Q(5, 0)));
    let q = e.pm(g, 37);
    let eval = |p| scale_point(e, map.eval(e, p), ui);
    let mut rational_checks = 0;
    let mut wrong = 0;
    for p in rational(e) {
        let a = eval(p);
        assert!(e.on(a));
        assert_eq!(a, e.pm(p, 353));
        assert_eq!(eval(e.pa(p, g)), e.pa(a, eval(g)));
        assert_eq!(e.pm(a, 415), p);
        rational_checks += 1;
        if p.is_some() {
            assert_ne!(a, e.pm(p, 354));
            wrong += 1;
        }
    }
    let mut images = BTreeSet::new();
    let mut twist_kernel = 0;
    let mut twist_checks = 0;
    for p in rational(t) {
        let ep = embed(e, p);
        let a = eval(ep);
        assert!(e.on(a));
        assert_eq!(e.pm(a, 7), None);
        images.insert(a);
        if a.is_none() {
            twist_kernel += 1;
        }
        twist_checks += 1;
    }
    assert_eq!((images.len(), twist_kernel), (49, 17));
    assert_eq!((rational_checks,twist_checks),(823,833));
    let mut lambda_order = 1;
    let mut z = 353;
    while z != 1 {
        z = z * 353 % 823;
        lambda_order += 1;
        assert!(lambda_order <= 822);
    }
    let result = json!({"schema":"twist.endo-symmetry/v1","p":827,"a":1,"b":15,"source":registry::identity(827,1,15,823,None),"twist_generator":pair(h),"kernel_generator_twist":pair(kt),"kernel_roots":kr,"velu":map,"intermediate_model":registry::identity(827,map.a,map.b,823,None),"isomorphism_inverse_scale":7,"alpha":alpha,"conjugate_one_minus_alpha":conjugate,"generic_norm_compositions_verified":2,"generic_frobenius_plus_one_identity_verified":true,"characteristic_polynomial_coefficients":[17,826,1],"characteristic_polynomial_integer":"X^2-X+17","discriminant":-67,"frobenius_relation":"[7] alpha = pi + [1]","rational_scalar_action":353,"rational_scalar_inverse":415,"scalar_multiplicative_order":lambda_order,"mapped_generator":pair(eval(g)),"mapped_target":pair(eval(q)),"source_rational_checks":rational_checks,"twist_rational_checks":twist_checks,"twist_kernel_count":twist_kernel,"twist_image_count":images.len(),"wrong_scalar_controls_rejected":wrong,"unknown_log_recovery_or_cost_claim":false});
    let mut result=result;
    result["intermediate_generator"]=json!(pair(map.eval(e,g)));
    fs::write(out.join("RESULTS.json"),serde_json::to_string_pretty(&result)?+"\n")?;
    let mut gp=String::from("default(parisizemax,4000000000);\np=827;\nck(c,s)=if(!c,error(s));\nE=ellinit([Mod(1,p),Mod(15,p)]);T=ellinit([Mod(4,p),Mod(120,p)]);G=Mod(1,p)*[2,5];Q=ellmul(E,G,37);ck(ellcard(E)==823&&ellcard(T)==833&&ellorder(E,G)==823,\"cardinalities\");\n");
    gp+=&format!("kt=ellmul(T,Mod(1,p)*[2,386],7);ck(ellorder(T,kt)==17,\"kernel order\");rr=Set(vector(16,i,lift(ellmul(T,kt,i)[1]/2)));ck(rr=={},\"kernel coordinates\");v=ellisogeny(E,{});ck(v[1]==[0,0,0,Mod({},p),Mod({},p)]&&v[2]=={},\"map\");H=v[2][3];A=[v[2][1]/H^2*49,v[2][2]/y/H^3*343];ck(A==[{},{}],\"endomorphism coordinates\");\n",serde_json::to_string(&kr)?,ppoly(&map.h),map.a,map.b,gp_map(&map),prat(&alpha.0),prat(&alpha.1));
    gp+="F=Mod(1,p)*(x^3+x+15);\nga(a,b,d=0)={my(l,xx,ss);l=if(d,(3*a[1]^2+1)/(2*F*a[2]),(b[2]-a[2])/(b[1]-a[1]));xx=F*l^2-a[1]-b[1];ss=l*(a[1]-xx)-a[2];[xx,ss]};\ngc(a,b)=[subst(a[1],x,b[1]),b[2]*subst(a[2],x,b[1])];\nP=[Mod(1,p)*x,Mod(1,p)];C=ga(P,[A[1],-A[2]]);P2=ga(P,P,1);P3=ga(P2,P);P6=ga(P3,P3,1);P7=ga(P6,P);P4=ga(P2,P2,1);P8=ga(P4,P4,1);P16=ga(P8,P8,1);P17=ga(P16,P);ck(gc(C,A)==P17&&gc(A,C)==P17,\"generic norm\");B=ga([Mod(1,p)*x^827,F^413],P);ck(gc(P7,A)==B,\"generic Frobenius plus one\");\n";
    gp+="ev(P,tw=0)={my(xx,xx0,yy);if(P==[0],return([0]));xx0=P[1]/if(tw,2,1);if(subst(H,x,xx0)==0,return([0]));xx=subst(A[1],x,xx0)*if(tw,2,1);yy=P[2]*subst(A[2],x,xx0);[xx,yy]};\nfor(i=0,822,P=ellmul(E,G,i);Z=ev(P);ck(ellisoncurve(E,Z)&&Z==ellmul(E,P,353)&&ellmul(E,Z,415)==P,\"source scalar\");if(i,ck(Z!=ellmul(E,P,354),\"wrong scalar\")));\n";
    gp+="vv=List();kc=1;listput(vv,[0]);tc=1;for(j=0,826,z=Mod(j^3+4*j+120,p);if(issquare(z),yy=sqrt(z);for(k=0,1,P=[Mod(j,p),(-1)^k*yy];Z=ev(P,1);ck(ellisoncurve(T,Z)&&ellmul(T,Z,7)==[0],\"twist image\");tc++;if(Z==[0],kc++);listput(vv,lift(Z)))));ck(tc==833&&kc==17&&#Set(Vec(vv))==49,\"twist counts\");\n";
    gp+=&format!("ck(znorder(Mod(353,823))=={},\"scalar orbit length\");print(\"ALL_TWIST_ENDOMORPHISM_OK 17 823 833\");\n",lambda_order);
    fs::write(out.join("verify.gp"), gp)?;
    println!(
        "{}",
        json!({"degree":17,"rational_checks":rational_checks,"twist_checks":twist_checks,"scalar":353,"scalar_order":lambda_order})
    );
    Ok(())
}
fn endo_admit(raw: &Path, pari: &Path, out: &Path) -> Result<()> {
    if out.exists() {
        return Err("fresh admission required".into());
    }
    let data: Value = serde_json::from_slice(&fs::read(raw.join("RESULTS.json"))?)?;
    let receipt: Value = serde_json::from_slice(&fs::read(pari.join("RECEIPT.json"))?)?;
    let hash = |p: &Path| -> Result<String> { Ok(registry::digest(&fs::read(p)?)) };
    assert_eq!(receipt["exit_code"], 0);
    assert!(receipt["signal"].is_null());
    assert_eq!(receipt["script_sha256"], hash(&raw.join("verify.gp"))?);
    let stdout = fs::read(pari.join("pari.stdout"))?;
    let stderr = fs::read(pari.join("pari.stderr"))?;
    assert_eq!(receipt["stdout_sha256"], registry::digest(&stdout));
    assert_eq!(receipt["stderr_sha256"], registry::digest(&stderr));
    assert_eq!(
        String::from_utf8(stdout)?.trim(),
        "ALL_TWIST_ENDOMORPHISM_OK 17 823 833"
    );
    let err = String::from_utf8(stderr)?;
    assert!(
        err.trim().is_empty()
            || err.trim()
                == "***   Warning: new maximum stack size = 4000002048 (3814.699 Mbytes)."
    );
    let mut result = data;
    result["status"] = json!("native-generic-and-independent-PARI-endomorphism-verified");
    result["native_result_sha256"] = json!(hash(&raw.join("RESULTS.json"))?);
    result["pari_receipt_sha256"] = json!(hash(&pari.join("RECEIPT.json"))?);
    fs::write(out, serde_json::to_string_pretty(&result)? + "\n")?;
    Ok(())
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() == 1 {
        return endo_produce(Path::new(&a[0]));
    }
    if a.len() == 4 && a[0] == "admit" {
        return endo_admit(Path::new(&a[1]), Path::new(&a[2]), Path::new(&a[3]));
    }
    Err("FRESH_OUT | admit RAW PARI FRESH_ADMISSION".into())
}
#[cfg(test)]
mod endo_controls {
    use super::*;
    #[test]
    fn generic_seventeen_control() {
        let e = Quad::new(101, 2, 3);
        let r = Ring(101);
        let (x, s) = gm(r, 2, 3, 17);
        for p in rational(e) {
            if let Some((xx, yy)) = p {
                let xd = r.eval(&x.d, xx.0);
                let sd = r.eval(&s.d, xx.0);
                if xd != 0 && sd != 0 {
                    assert_eq!(
                        Some((
                            Q(r.eval(&x.n, xx.0) * inv(xd, 101) % 101, 0),
                            Q(yy.0 * r.eval(&s.n, xx.0) % 101 * inv(sd, 101) % 101, 0)
                        )),
                        e.pm(p, 17)
                    );
                }
            }
        }
    }
}
