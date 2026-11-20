fn before_vsquare(a: &[u64], p: u64) -> Vec<u64> {
    // The general fused product wins for short 32-bit-field inputs in
    // actual Rust benchmarks; use symmetric squaring for larger polynomials,
    // and for 61-bit arithmetic where its crossover occurs earlier.
    if a.len() <= 2 || (a.len() <= 4 && p <= u32::MAX as u64) {
        return vproduct(a, a, p);
    }
    // (c + d*x^k)^2 needs exactly three field multiplications, and
    // its nonzero leading coefficient makes output trimming unnecessary.
    // This binomial shape occurs frequently in root certificates.
    if a.len() >= 6 && a[0] != 0 && a[1] == 0 && a[2] == 0
        && *a.last().unwrap() != 0
        && a[3..a.len() - 1].iter().all(|&x| x == 0)
    {
        let k = a.len() - 1;
        let mut result = vec![0; 2 * k + 1];
        result[0] = vmul(a[0], a[0], p);
        let cross = vmul(a[0], a[k], p);
        let twice = cross + cross;
        result[k] = if twice >= p { twice - p } else { twice };
        result[2 * k] = vmul(a[k], a[k], p);
        return result;
    }
    // Preserve the previous two-term fast path exactly; it is frequent in
    // certificates and a generic three-slot scan regressed ~4% in Rust.
    // Only inspect the remainder for an additional term after a third
    // nonzero is encountered.
    if a.len() >= 6 && a[1] == 0 && a[2] == 0 {
        let mut terms = [(0usize, 0u64); 2];
        let mut count = 0usize;
        let mut third = None;
        for (i, &x) in a.iter().enumerate() {
            if x == 0 { continue; }
            if count == 2 {
                third = Some((i, x));
                break;
            }
            terms[count] = (i, x);
            count += 1;
        }
        if third.is_none() {
            if count == 0 { return vec![]; }
            let mut result = vec![0; 2 * a.len() - 1];
            let (i, x) = terms[0];
            result[2 * i] = vmul(x, x, p);
            if count == 2 {
                let (j, y) = terms[1];
                result[2 * j] = vmul(y, y, p);
                let cross = vmul(x, y, p);
                let twice = cross + cross;
                result[i + j] = if twice >= p { twice - p } else { twice };
            }
            return trim(result);
        }
        if let Some((k, z)) = third {
            // The existing three-term fast path already scans the tail.
            // Reuse that scan to recognize four-term squares without
            // charging another pass on the common dense path.
            let mut fourth = None;
            let mut too_many = false;
            for (offset, &w) in a[k + 1..].iter().enumerate() {
                if w == 0 { continue; }
                if fourth.is_some() {
                    too_many = true;
                    break;
                }
                fourth = Some((k + 1 + offset, w));
            }
            if !too_many && fourth.is_none() {
                let (i, x) = terms[0];
                let (j, y) = terms[1];
                let mut result = vec![0; 2 * a.len() - 1];
                result[2 * i] = vmul(x, x, p);
                result[2 * j] = vmul(y, y, p);
                result[2 * k] = vmul(z, z, p);
                let cross = vmul(x, y, p);
                let twice = cross + cross;
                result[i + j] = if twice >= p { twice - p } else { twice };
                let cross = vmul(x, z, p);
                let twice = cross + cross;
                let twice = if twice >= p { twice - p } else { twice };
                let sum = result[i + k] + twice;
                result[i + k] = if sum >= p { sum - p } else { sum };
                let cross = vmul(y, z, p);
                let twice = cross + cross;
                let twice = if twice >= p { twice - p } else { twice };
                let sum = result[j + k] + twice;
                result[j + k] = if sum >= p { sum - p } else { sum };
                return trim(result);
            }
            if !too_many {
                if let Some((l, w)) = fourth {
                    // Four nonzero coefficients cost ten field products;
                    // the dense symmetric path would use n(n+1)/2.
                    // Accumulate, rather than assign, cross terms because
                    // exponent sums can collide (e.g. 0+6 == 3+3).
                    let nonzero = [terms[0], terms[1], (k, z), (l, w)];
                    let mut result = vec![0; 2 * a.len() - 1];
                    for (i, &(degree, x)) in nonzero.iter().enumerate() {
                        let diagonal = vmul(x, x, p);
                        let sum = result[2 * degree] + diagonal;
                        result[2 * degree] = if sum >= p { sum - p } else { sum };
                        for &(other_degree, y) in &nonzero[i + 1..] {
                            let cross = vmul(x, y, p);
                            let twice = cross + cross;
                            let twice = if twice >= p { twice - p } else { twice };
                            let sum = result[degree + other_degree] + twice;
                            result[degree + other_degree] = if sum >= p { sum - p } else { sum };
                        }
                    }
                    return trim(result);
                }
            }
        }
    }
    let mut result = vec![0; 2 * a.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        let diagonal = vmul(x, x, p);
        let sum = result[2 * i] + diagonal;
        result[2 * i] = if sum >= p { sum - p } else { sum };
        for (j, &y) in a.iter().enumerate().skip(i + 1) {
            let cross = vmul(x, y, p);
            let twice = cross + cross;
            let twice = if twice >= p { twice - p } else { twice };
            let sum = result[i + j] + twice;
            result[i + j] = if sum >= p { sum - p } else { sum };
        }
    }
    trim(result)
}
fn before_vdiv(a: &[u64], b: &[u64], p: u64) -> Result<(Vec<u64>, Vec<u64>), String> {
    if b.is_empty() {
        return Err("division by zero polynomial".into());
    }
    // Degree-short dividends need neither a quotient allocation nor a
    // costly field inversion. The input is already canonical here.
    if a.len() < b.len() {
        return Ok((vec![], a.to_vec()));
    }
    // A monic constant divisor leaves the dividend unchanged. Preserve
    // canonical quotient trimming even if a standalone caller supplies
    // trailing zero coefficients.
    if b.len() == 1 {
        if b[0] == 0 { return Err("division by zero polynomial".into()); }
        if b[0] == 1 { return Ok((trim(a.to_vec()), vec![])); }
        // Degree-zero division has no remainder. Avoid allocating a
        // temporary dividend and trimming after every quotient digit.
        let inverse = vpow(b[0], p - 2, p);
        let quotient = a.iter().map(|&x| vmul(x, inverse, p)).collect();
        return Ok((trim(quotient), vec![]));
    }
    // Division by x^k is a coefficient split, not a polynomial long
    // division. The guard avoids a full scan for dense divisors.
    if b.len() > 1 && b[0] == 0 && *b.last().unwrap() == 1
        && b[1..b.len()-1].iter().all(|&x| x == 0)
    {
        let shift = b.len() - 1;
        return Ok((trim(a[shift..].to_vec()), trim(a[..shift].to_vec())));
    }
    // Synthetic division by x + c needs only one running remainder and
    // one quotient allocation, rather than a mutable dividend and a
    // trim pass on every quotient digit.
    if b.len() == 2 && b[1] == 1 {
        let mut q = vec![0; a.len() - 1];
        let mut carry = *a.last().unwrap();
        for i in (1..a.len()).rev() {
            q[i - 1] = carry;
            let product = vmul(carry, b[0], p);
            let value = a[i - 1];
            carry = if value >= product { value - product }
                    else { value + p - product };
        }
        return Ok((trim(q), if carry == 0 { vec![] } else { vec![carry] }));
    }
    // A monic binomial x^k + c (k >= 3, c != 0) only changes
    // one remainder coefficient per quotient digit.  In particular,
    // do not multiply and subtract the k-1 known zero coefficients.
    if b.len() >= 4 && b[0] != 0 && *b.last().unwrap() == 1
        && b[1..b.len() - 1].iter().all(|&x| x == 0)
    {
        let mut r = a.to_vec();
        let mut q = vec![0; a.len() - b.len() + 1];
        while !r.is_empty() && r.len() >= b.len() {
            let shift = r.len() - b.len();
            let c = *r.last().unwrap();
            q[shift] = c;
            r.pop();
            let product = vmul(c, b[0], p);
            let old = r[shift];
            r[shift] = if old >= product { old - product }
                       else { old + p - product };
            r = trim(r);
        }
        return Ok((trim(q), r));
    }
    let mut r = a.to_vec();
    let mut q = vec![0; a.len() - b.len() + 1];
    // GCD factors, X^p mod f divisors and root certificates are commonly
    // monic. Avoid a full exponentiation to invert their leading 1.
    let lead = *b.last().unwrap();
    if lead == 1 {
        // Synthetic division by a monic polynomial. The leading term
        // cancels exactly, so neither its multiplication nor subtraction
        // needs to be performed.
        while !r.is_empty() && r.len() >= b.len() {
            let shift = r.len() - b.len();
            let c = *r.last().unwrap();
            q[shift] = c;
            r.pop();
            for (i, &x) in b[..b.len() - 1].iter().enumerate() {
                let product = vmul(c, x, p);
                let old = r[shift + i];
                r[shift + i] = if old >= product { old - product }
                               else { old + p - product };
            }
            r = trim(r);
        }
    } else {
        // Preserve the nonmonic route, including its original reduction
        // order, rather than adding a branch to each inner iteration.
        let inv = vpow(lead, p - 2, p);
        while !r.is_empty() && r.len() >= b.len() {
            let shift = r.len() - b.len();
            let c = vmul(*r.last().unwrap(), inv, p);
            q[shift] = c;
            // c*lead cancels the top term exactly, even for nonmonic b.
            r.pop();
            for (i, &x) in b[..b.len() - 1].iter().enumerate() {
                let product = vmul(c, x, p);
                let old = r[shift + i];
                r[shift + i] = if old >= product { old - product }
                               else { old + p - product };
            }
            r = trim(r);
        }
    }
    Ok((trim(q), r))
}
