//! Mature producer scripts and strict transformation-matrix stream parser.
use certified_relations::{
    degree,
    replay::{CanonicalTerm, CofactorTerm, Member},
    Poly, System,
};
use std::collections::BTreeSet;
fn expression(f: &Poly) -> String {
    if f.is_empty() {
        return "0".into();
    }
    f.iter()
        .map(|(m, c)| {
            let mut s = c.to_string();
            for (i, e) in m.iter().enumerate() {
                if *e != 0 {
                    s += &format!("*x{i}^{e}");
                }
            }
            s
        })
        .collect::<Vec<_>>()
        .join("+")
}
pub fn msolve_input(system: &System) -> Result<String, String> {
    if system.prime != 2147483647 || !(3..=16).contains(&system.variables.len()) {
        return Err("external msolve field/shape".into());
    }
    let input = system.decode()?;
    if input.is_empty()
        || input.len() > 32
        || input
            .iter()
            .any(|f| f.is_empty() || f.len() > 32 || degree(f) > 4)
    {
        return Err("external original input budget".into());
    }
    Ok(format!(
        "{}\n{}\n{}\n",
        (0..system.variables.len())
            .map(|i| format!("x{i}"))
            .collect::<Vec<_>>()
            .join(","),
        system.prime,
        input.iter().map(expression).collect::<Vec<_>>().join(",\n")
    ))
}
pub fn program(system: &System, method: &str, basis: Option<&[Poly]>) -> Result<String, String> {
    if !["std", "slimgb"].contains(&method)
        || system.prime != 2147483647
        || !(3..=16).contains(&system.variables.len())
    {
        return Err("external certificate producer field/method/shape".into());
    }
    let input = system.decode()?;
    if input.is_empty()
        || input.len() > 32
        || input
            .iter()
            .any(|f| f.is_empty() || f.len() > 32 || degree(f) > 4)
    {
        return Err("external producer original input budget/zero".into());
    }
    let mut code = format!(
        "ring r={},({}),dp;\noption(redSB);\nideal I={};\nmatrix T;\n",
        system.prime,
        (0..system.variables.len())
            .map(|i| format!("x{i}"))
            .collect::<Vec<_>>()
            .join(","),
        input.iter().map(expression).collect::<Vec<_>>().join(",")
    );
    if let Some(g) = basis {
        if g.is_empty() || g.len() > 256 {
            return Err("external lift basis budget".into());
        }
        code += &format!(
            "ideal G={};\nmatrix U;\nT=lift(I,G,U,\"{method}\");\n",
            g.iter().map(expression).collect::<Vec<_>>().join(",")
        );
    } else {
        code += &format!("ideal G=liftstd(I,T,\"{method}\");\n");
    }
    code+="int i,j;poly f;\nfor(j=1;j<=size(G);j++){print(\"ROW\");print(\"POLY\");f=G[j];while(f!=0){print(\"TERM:\"+string(leadcoef(f))+\":\"+string(leadexp(f)));f=f-lead(f);}print(\"ENDPOLY\");for(i=1;i<=size(I);i++){f=T[i,j];if(f!=0){print(\"PROOF:\"+string(i));while(f!=0){print(\"TERM:\"+string(leadcoef(f))+\":\"+string(leadexp(f)));f=f-lead(f);}print(\"ENDPROOF\");}}print(\"ENDROW\");}\nprint(\"CERTDONE\");\nquit;\n";
    Ok(code)
}
#[derive(Clone, Copy)]
enum Slot {
    None,
    Polynomial,
    Proof(usize),
}
pub fn parse(text: &str, n: usize, p: u64, inputs: usize) -> Result<Vec<Member>, String> {
    if n == 0 || n > 16 || p < 3 || p > i64::MAX as u64 || inputs == 0 || inputs > 32 {
        return Err("external parser field/shape".into());
    }
    let mut rows = Vec::new();
    let mut current: Option<Member> = None;
    let mut slot = Slot::None;
    let mut poly_done = false;
    let mut done = false;
    let mut proof_blocks = BTreeSet::new();
    for line in text.lines().map(str::trim).filter(|s| !s.is_empty()) {
        if done {
            return Err("external trailing output".into());
        }
        match line {
            "ROW" if current.is_none() && matches!(slot, Slot::None) => {
                current = Some(Member {
                    polynomial: vec![],
                    proof: vec![],
                });
                poly_done = false;
                proof_blocks.clear();
            }
            "POLY" if current.is_some() && !poly_done && matches!(slot, Slot::None) => {
                slot = Slot::Polynomial
            }
            "ENDPOLY" if matches!(slot, Slot::Polynomial) => {
                if current.as_ref().unwrap().polynomial.is_empty() {
                    return Err("external zero row".into());
                }
                poly_done = true;
                slot = Slot::None;
            }
            "ENDPROOF" if matches!(slot, Slot::Proof(_)) => slot = Slot::None,
            "ENDROW" if poly_done && matches!(slot, Slot::None) => {
                rows.push(current.take().ok_or("external row state")?);
                if rows.len() > 256 {
                    return Err("external row budget".into());
                }
                poly_done = false;
            }
            "CERTDONE" if current.is_none() && !rows.is_empty() => done = true,
            _ => {
                if let Some(index) = line.strip_prefix("PROOF:") {
                    if !poly_done || current.is_none() || !matches!(slot, Slot::None) {
                        return Err("external proof state".into());
                    }
                    let index: usize = index.parse().map_err(|_| "external proof index")?;
                    if index == 0 || index > inputs || !proof_blocks.insert(index) {
                        return Err("external duplicate/invalid proof index".into());
                    }
                    slot = Slot::Proof(index - 1);
                } else if let Some(term) = line.strip_prefix("TERM:") {
                    let (c, m) = term.split_once(':').ok_or("external term separator")?;
                    let c = c
                        .parse::<i128>()
                        .map_err(|_| "external coefficient")?
                        .rem_euclid(p as i128) as u64;
                    let m: Vec<usize> = m
                        .split(',')
                        .map(|s| s.trim().parse().map_err(|_| "external exponent"))
                        .collect::<Result<_, _>>()?;
                    if c == 0
                        || m.len() != n
                        || m.iter()
                            .try_fold(0usize, |s, e| s.checked_add(*e))
                            .is_none_or(|d| d > 64)
                    {
                        return Err("external term shape/residue".into());
                    }
                    let row = current.as_mut().ok_or("external term outside row")?;
                    match slot {
                        Slot::Polynomial => {
                            row.polynomial.push(CanonicalTerm {
                                exponents: m,
                                coefficient: c,
                            });
                            if row.polynomial.len() > 5000 {
                                return Err("external polynomial term budget".into());
                            }
                        }
                        Slot::Proof(generator) => {
                            row.proof.push(CofactorTerm {
                                generator,
                                exponents: m,
                                coefficient: c,
                            });
                            if row.proof.len() > 50000 {
                                return Err("external proof term budget".into());
                            }
                        }
                        Slot::None => return Err("external term state".into()),
                    }
                } else {
                    return Err("unexpected external producer output".into());
                }
            }
        }
    }
    if !done || current.is_some() || !matches!(slot, Slot::None) {
        return Err("incomplete external certificate".into());
    }
    Ok(rows)
}
#[cfg(test)]
mod tests {
    use super::*;
    const ONE: &str =
        "ROW\nPOLY\nTERM:1:1,0,0\nENDPOLY\nPROOF:1\nTERM:-1:0,0,0\nENDPROOF\nENDROW\nCERTDONE\n";
    #[test]
    fn parses_matrix_column_orientation_and_signed_residues() {
        let r = parse(ONE, 3, 2147483647, 2).unwrap();
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].proof[0].generator, 0);
        assert_eq!(r[0].proof[0].coefficient, 2147483646);
    }
    #[test]
    fn rejects_incomplete_reordered_and_malformed_streams() {
        for text in [
            ONE.replace("CERTDONE", ""),
            ONE.replace("PROOF:1", "PROOF:3"),
            ONE.replace("1,0,0", "1,0"),
            format!("{ONE}? error\n"),
            ONE.replace("ENDPOLY", "ENDROW"),
        ] {
            assert!(parse(&text, 3, 2147483647, 2).is_err());
        }
    }
}
