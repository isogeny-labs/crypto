use std::{collections::BTreeMap, fs};
fn main() {
    let s = fs::read_to_string("/tmp/hec-generic-biquadratic.forms").unwrap();
    let start = s.find("BBB := matrix").unwrap();
    let mut forms = vec![BTreeMap::<([u8; 4], [u8; 4], u8), i64>::new(); 16];
    let mut seen = 0;
    for part in s[start..].split("BBB[").skip(1) {
        let (indices, tail) = part.split_once(']').unwrap();
        let ij: Vec<usize> = indices
            .split(',')
            .map(|x| x.trim().parse::<usize>().unwrap() - 1)
            .collect();
        let expr = tail.split_once(":=").unwrap().1.split(':').next().unwrap();
        let expr: String = expr.chars().filter(|c| !c.is_whitespace()).collect();
        let mut terms = vec![];
        let mut begin = 0;
        for (j, ch) in expr.char_indices() {
            if j > 0 && (ch == '+' || ch == '-') {
                terms.push(&expr[begin..j]);
                begin = j
            }
        }
        terms.push(&expr[begin..]);
        let out = &mut forms[4 * ij[0] + ij[1]];
        for term in terms {
            let mut coeff = 1i64;
            let mut kp = [0u8; 4];
            let mut lp = [0u8; 4];
            let mut cp = 0;
            let mut term = term;
            if let Some(t) = term.strip_prefix('-') {
                coeff = -1;
                term = t
            } else if let Some(t) = term.strip_prefix('+') {
                term = t
            }
            for factor in term.split('*') {
                let (var, e) = if let Some((v, e)) = factor.split_once('^') {
                    (v, e.parse::<u8>().unwrap())
                } else {
                    (factor, 1)
                };
                match var.as_bytes()[0] {
                    b'k' => kp[var[1..].parse::<usize>().unwrap() - 1] += e,
                    b'l' => lp[var[1..].parse::<usize>().unwrap() - 1] += e,
                    b'f' => match var[1..].parse::<usize>().unwrap() {
                        0 => cp += e,
                        5 => {}
                        _ => coeff = 0,
                    },
                    _ => coeff *= var.parse::<i64>().unwrap().pow(e as u32),
                }
            }
            if coeff != 0 {
                assert_eq!(kp.iter().sum::<u8>(), 2);
                assert_eq!(lp.iter().sum::<u8>(), 2);
                *out.entry((kp, lp, cp)).or_default() += coeff;
            }
        }
        out.retain(|_, v| *v != 0);
        seen += 1;
    }
    assert_eq!(seen, 16);
    for i in 0..4 {
        for j in 0..4 {
            assert_eq!(forms[4 * i + j], forms[4 * j + i]);
            let reversed: BTreeMap<_, _> = forms[4 * i + j]
                .iter()
                .map(|(&(k, l, c), &v)| ((l, k, c), v))
                .collect();
            assert_eq!(reversed, forms[4 * i + j]);
        }
    }
    let mut out="// Algebraic specialization of E. V. Flynn's published generic biquadratic forms.\n// Source URL and SHA256 are retained in SOURCES.md. No original prose or Maple program is copied.\npub type Term=(i64,u8,[u8;4],[u8;4]);\npub const FORMS:[&[Term];16]=[\n".to_string();
    for f in &forms {
        out.push_str("&[\n");
        for (&(k, l, c), &v) in f {
            out.push_str(&format!("({v},{c},{k:?},{l:?}),\n"))
        }
        out.push_str("],\n")
    }
    out.push_str("];\n");
    fs::write(
        "research/hec_residual_span_20261116/generic-kummer/terms.rs",
        out,
    )
    .unwrap();
    eprintln!(
        "forms=16 p_q_degree_2=true symmetry=true terms={}",
        forms.iter().map(BTreeMap::len).sum::<usize>()
    );
}
