//! Exact static row-space certificates. This crate performs no root search.
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
pub mod check;
pub mod controls;
pub type Result<T> = std::result::Result<T, String>;
pub type Polynomial = BTreeMap<Vec<usize>, BigUint>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Term {
    pub monomial: Vec<usize>,
    pub coefficient: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawSystem {
    pub schema: String,
    pub p: String,
    pub variables: Vec<String>,
    pub equations: Vec<Vec<Term>>,
}
#[derive(Clone)]
pub struct System {
    pub raw: RawSystem,
    pub p: BigUint,
    pub equations: Vec<Polynomial>,
}
pub fn decimal(s: &str) -> Result<BigUint> {
    if s.is_empty()
        || s.len() > 78
        || (s.len() > 1 && s.starts_with('0'))
        || !s.bytes().all(|b| b.is_ascii_digit())
    {
        return Err("Invalid: canonical decimal".into());
    }
    BigUint::parse_bytes(s.as_bytes(), 10).ok_or("Invalid: decimal".into())
}
pub fn decode(terms: &[Term], n: usize, p: &BigUint) -> Result<Polynomial> {
    let mut out = BTreeMap::new();
    let mut previous: Option<&Vec<usize>> = None;
    for t in terms {
        if t.monomial.len() > 2
            || t.monomial.iter().any(|&v| v >= n)
            || !t.monomial.windows(2).all(|w| w[0] <= w[1])
            || previous.is_some_and(|v| v >= &t.monomial)
        {
            return Err("Invalid: monomial shape/order".into());
        }
        let c = decimal(&t.coefficient)?;
        if c.is_zero() || &c >= p {
            return Err("Invalid: coefficient".into());
        }
        out.insert(t.monomial.clone(), c);
        previous = Some(&t.monomial);
    }
    Ok(out)
}
pub fn encode(p: &Polynomial) -> Vec<Term> {
    p.iter()
        .map(|(m, c)| Term {
            monomial: m.clone(),
            coefficient: c.to_string(),
        })
        .collect()
}
impl System {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() > 2 << 20 {
            return Err("Cap: input bytes".into());
        }
        let raw: RawSystem = serde_json::from_slice(bytes).map_err(|e| format!("Invalid: {e}"))?;
        let p = decimal(&raw.p)?;
        let actual = (BigUint::one() << 255usize) - 19u32;
        if p != BigUint::from(29u32) && p != actual {
            return Err("Invalid: unsupported field".into());
        }
        if raw.schema != "torus-s4.quadratic/v1"
            || raw.variables.len() > 4096
            || raw.equations.len() > 8192
            || raw.equations.iter().map(Vec::len).sum::<usize>() > 32768
        {
            return Err("Invalid: system schema or admission shape".into());
        }
        if raw.variables.iter().any(|s| s.len() > 128)
            || raw.variables.iter().collect::<BTreeSet<_>>().len() != raw.variables.len()
        {
            return Err("Invalid: variable names".into());
        }
        let equations = raw
            .equations
            .iter()
            .map(|r| decode(r, raw.variables.len(), &p))
            .collect::<Result<_>>()?;
        Ok(Self { raw, p, equations })
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Limits {
    pub work: u64,
    pub writes: u64,
    pub entries: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            work: 50_000_000,
            writes: 8_000_000,
            entries: 1_000_000,
        }
    }
}
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Meter {
    pub events: u64,
    pub coefficient_writes: u64,
}
impl Meter {
    pub fn event(&mut self, n: u64, l: &Limits) -> Result<()> {
        self.events = self.events.checked_add(n).ok_or("Cap: event overflow")?;
        if self.events > l.work {
            return Err("Cap: work".into());
        }
        Ok(())
    }
    pub fn write(&mut self, n: u64, l: &Limits) -> Result<()> {
        self.coefficient_writes = self
            .coefficient_writes
            .checked_add(n)
            .ok_or("Cap: write overflow")?;
        if self.coefficient_writes > l.writes {
            return Err("Cap: writes".into());
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Step {
    pub source: usize,
    pub weights: Vec<(usize, String)>,
    pub row: Vec<Term>,
    pub pivot: Option<Vec<usize>>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RankCertificate {
    pub steps: Vec<Step>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Audit {
    pub schema: String,
    pub keep: Vec<usize>,
    pub original_input_rows: Vec<usize>,
    pub projection: RankCertificate,
    pub images: Vec<Vec<Term>>,
    pub baseline: RankCertificate,
    pub combined: RankCertificate,
    pub generator_meter: Meter,
}

fn subtract<K: Ord + Clone>(
    a: &mut BTreeMap<K, BigUint>,
    b: &BTreeMap<K, BigUint>,
    c: &BigUint,
    p: &BigUint,
    m: &mut Meter,
    l: &Limits,
) -> Result<()> {
    for (key, value) in b {
        m.event(3, l)?;
        m.write(1, l)?;
        let product = (c * value) % p;
        let current = a.get(key).cloned().unwrap_or_default();
        let next = (current + p - product) % p;
        if next.is_zero() {
            a.remove(key);
        } else {
            a.insert(key.clone(), next);
        }
    }
    Ok(())
}
pub(crate) fn rank(
    gens: &[Polynomial],
    p: &BigUint,
    m: &mut Meter,
    l: &Limits,
) -> Result<RankCertificate> {
    let mut basis: Vec<(Vec<usize>, Polynomial, BTreeMap<usize, BigUint>)> = vec![];
    let mut steps = vec![];
    let mut entries = 0usize;
    for (source, original) in gens.iter().enumerate() {
        m.event(1 + original.len() as u64, l)?;
        m.write(1 + original.len() as u64, l)?;
        let mut row = original.clone();
        let mut weights = BTreeMap::from([(source, BigUint::one())]);
        for (pivot, other, origins) in &basis {
            m.event(1, l)?;
            if let Some(c) = row.get(pivot).cloned() {
                subtract(&mut row, other, &c, p, m, l)?;
                subtract(&mut weights, origins, &c, p, m, l)?;
            }
        }
        let pivot = row.keys().next().cloned();
        if let Some(key) = &pivot {
            m.event(2 * p.bits(), l)?;
            let inverse = row[key].modpow(&(p - 2u32), p);
            for value in row.values_mut().chain(weights.values_mut()) {
                m.event(1, l)?;
                m.write(1, l)?;
                *value = (&*value * &inverse) % p;
            }
        }
        entries = entries
            .checked_add(row.len() + weights.len())
            .ok_or("Cap: entries overflow")?;
        if entries > l.entries {
            return Err("Cap: certificate entries".into());
        }
        steps.push(Step {
            source,
            weights: weights.iter().map(|(i, c)| (*i, c.to_string())).collect(),
            row: encode(&row),
            pivot: pivot.clone(),
        });
        if let Some(pivot) = pivot {
            basis.push((pivot, row, weights));
        }
    }
    Ok(RankCertificate { steps })
}
pub(crate) fn combine(
    gens: &[Polynomial],
    weights: &[(usize, String)],
    p: &BigUint,
    m: &mut Meter,
    l: &Limits,
) -> Result<Polynomial> {
    let mut out = Polynomial::new();
    for (i, c) in weights {
        let c = decimal(c)?;
        for (key, value) in &gens[*i] {
            m.event(2, l)?;
            m.write(1, l)?;
            let next = (out.get(key).cloned().unwrap_or_default() + &c * value) % p;
            if next.is_zero() {
                out.remove(key);
            } else {
                out.insert(key.clone(), next);
            }
        }
    }
    Ok(out)
}
pub fn audit(s: &System, keep: Vec<usize>, l: &Limits) -> Result<Audit> {
    if !keep.windows(2).all(|w| w[0] < w[1]) || keep.iter().any(|&i| i >= s.raw.variables.len()) {
        return Err("Invalid: keep set".into());
    }
    let keep_set: BTreeSet<_> = keep.iter().copied().collect();
    let input_only = |r: &Polynomial| {
        r.keys()
            .all(|monomial| monomial.iter().all(|i| keep_set.contains(i)))
    };
    let original_input_rows: Vec<_> = s
        .equations
        .iter()
        .enumerate()
        .filter(|(_, r)| input_only(r))
        .map(|(i, _)| i)
        .collect();
    let original_input: Vec<_> = original_input_rows
        .iter()
        .map(|&i| s.equations[i].clone())
        .collect();
    let remaining: Vec<_> = s
        .equations
        .iter()
        .filter(|r| !input_only(r))
        .cloned()
        .collect();
    let projected: Vec<_> = remaining
        .iter()
        .map(|r| {
            r.iter()
                .filter(|(monomial, _)| monomial.iter().any(|i| !keep_set.contains(i)))
                .map(|(m, c)| (m.clone(), c.clone()))
                .collect()
        })
        .collect();
    let mut meter = Meter::default();
    meter.event(s.equations.iter().map(|r| r.len() as u64).sum(), l)?;
    let projection = rank(&projected, &s.p, &mut meter, l)?;
    let mut images = vec![];
    for step in &projection.steps {
        if step.pivot.is_none() {
            images.push(encode(&combine(
                &remaining,
                &step.weights,
                &s.p,
                &mut meter,
                l,
            )?));
        }
    }
    let baseline = rank(&original_input, &s.p, &mut meter, l)?;
    let mut combined_gens = original_input;
    for image in &images {
        combined_gens.push(decode(image, s.raw.variables.len(), &s.p)?);
    }
    let combined = rank(&combined_gens, &s.p, &mut meter, l)?;
    let out = Audit {
        schema: "torus.rowspace-audit/v1".into(),
        keep,
        original_input_rows,
        projection,
        images,
        baseline,
        combined,
        generator_meter: meter,
    };
    if serde_json::to_vec(&out).map_err(|e| e.to_string())?.len() > 32 << 20 {
        return Err("Cap: serialized certificate".into());
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    #[test]
    fn exact_rank_and_adversarial_contracts() {
        super::controls::small_contracts().unwrap();
    }
}
