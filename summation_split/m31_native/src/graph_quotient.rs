//! Quotient coordinates for monic square lifts on the four-point section.
//!
//! The ambient variable order is `[t, u, g, z0]` for one square and
//! `[t, u, g, z0, z1, h]` for two. The graph relations identify
//! `g = t*u`, `z_i^2 = Delta_i`, and `h = z0*z1` when present.

use crate::quotient::{field_add, field_mul, field_pow, field_sub};
use crate::{Error, Poly, Ring, M31};

const PARAMETER_BASIS_SIZE: usize = 4;
const PARAMETER_VARIABLES: usize = PARAMETER_BASIS_SIZE - 1;

/// The monic-square graph quotient with `t^2 = t` and `u^2 = u`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphQuotient {
    // Values at (0,0), (1,0), (0,1), (1,1) for each Delta_i.
    square_values: Vec<[u64; PARAMETER_BASIS_SIZE]>,
}

impl GraphQuotient {
    /// Supply the right-hand sides of one or two square relations as
    /// coefficient vectors in `[1, t, u, g]`.
    /// Coefficients are taken modulo M31.
    pub fn new(squares: &[[u64; PARAMETER_BASIS_SIZE]]) -> Result<Self, Error> {
        if !matches!(squares.len(), 1 | 2) {
            return Err(Error::Invalid("expected one or two graph squares".into()));
        }
        let square_values = squares
            .iter()
            .map(|&[constant, t, u, g]| {
                let at_t = field_add(constant, t);
                [
                    constant % M31,
                    at_t,
                    field_add(constant, u),
                    field_add(at_t, field_add(u, g)),
                ]
            })
            .collect();
        Ok(Self { square_values })
    }

    pub fn nvars(&self) -> usize {
        PARAMETER_VARIABLES + self.witness_basis_size() - 1
    }

    fn witness_basis_size(&self) -> usize {
        1 << self.square_values.len()
    }

    fn basis_monomial(&self, parameter: usize, witness: usize) -> Vec<u64> {
        let mut exponents = vec![0; self.nvars()];
        if parameter != 0 {
            exponents[parameter - 1] = 1;
        }
        if witness != 0 {
            exponents[PARAMETER_VARIABLES + witness - 1] = 1;
        }
        exponents
    }

    /// Exponent vectors for `C·W`, in witness-major order. The parameter
    /// factors are `[1, t, u, g]` and the witness factors are `[1, z0]`
    /// or `[1, z0, z1, h]`.
    pub fn basis_monomials(&self) -> Vec<Vec<u64>> {
        (0..self.witness_basis_size())
            .flat_map(|witness| {
                (0..PARAMETER_BASIS_SIZE)
                    .map(move |parameter| self.basis_monomial(parameter, witness))
            })
            .collect()
    }

    /// The unique representative modulo the base graph ideal in `span(C·W)`.
    pub fn representative(&self, polynomial: &Poly) -> Result<Poly, Error> {
        polynomial.validate()?;
        if polynomial.nvars() != self.nvars() {
            return Err(Error::Invalid(
                "polynomial and graph ring widths differ".into(),
            ));
        }

        // Work over the four parameter fibers, retaining formal witness
        // coordinates even in a fiber where Delta_i vanishes.
        let mut witness_values = [[0; PARAMETER_BASIS_SIZE]; 4];
        for (monomial, &coefficient) in &polynomial.terms {
            let has_g = monomial[2] != 0;
            let parameter_mask = usize::from(has_g || monomial[0] != 0)
                | (usize::from(has_g || monomial[1] != 0) << 1);

            // Validation bounds the total degree, so each exponent sum fits.
            let h = monomial.get(5).copied().unwrap_or(0);
            let powers = [monomial[3] + h, monomial.get(4).copied().unwrap_or(0) + h];
            let witness = ((powers[0] & 1) | ((powers[1] & 1) << 1)) as usize;
            for (point, value) in witness_values[witness].iter_mut().enumerate() {
                if point & parameter_mask == parameter_mask {
                    let term = self.square_values.iter().zip(powers).fold(
                        coefficient,
                        |term, (square, power)| {
                            field_mul(term, field_pow(square[point], power / 2))
                        },
                    );
                    *value = field_add(*value, term);
                }
            }
        }

        // Expand each witness coefficient in the parameter basis [1, t, u, g].
        let mut terms = Vec::new();
        for (witness, [at_zero, at_t, at_u, at_g]) in witness_values
            .into_iter()
            .take(self.witness_basis_size())
            .enumerate()
        {
            let t = field_sub(at_t, at_zero);
            let u = field_sub(at_u, at_zero);
            let g = field_sub(field_sub(at_g, at_u), t);
            for (parameter, coefficient) in [at_zero, t, u, g].into_iter().enumerate() {
                if coefficient != 0 {
                    terms.push((self.basis_monomial(parameter, witness), coefficient));
                }
            }
        }
        Poly::from_terms(self.nvars(), terms)
    }

    fn monomial_relation(&self, exponents: Vec<u64>) -> Result<Poly, Error> {
        let monomial = Poly::from_terms(self.nvars(), [(exponents, 1)])?;
        monomial.sub(&self.representative(&monomial)?)
    }

    /// Generators of the base graph ideal `B` in the ambient ring.
    pub fn base_generators(&self) -> Result<Vec<Poly>, Error> {
        let mut factors = vec![[0, 0], [1, 1], [0, 1], [3, 3]];
        if self.square_values.len() == 2 {
            factors.extend([[4, 4], [3, 4]]);
        }
        factors
            .into_iter()
            .map(|[first, second]| {
                let mut exponents = vec![0; self.nvars()];
                exponents[first] += 1;
                exponents[second] += 1;
                self.monomial_relation(exponents)
            })
            .collect()
    }

    /// Ambient generators of `B + (terminal)`: the base relations, one
    /// relation `m - representative(m)` per cubic monomial, and the
    /// representative of `terminal`. Each row has degree at most three.
    pub fn preclosed_generators(&self, terminal: &Poly) -> Result<Vec<Poly>, Error> {
        let terminal = self.representative(terminal)?;
        let nvars = self.nvars();
        let monomial_count = (nvars + 1) * (nvars + 2) * (nvars + 3) / 6;
        let monomials = Ring::new(0, nvars)?.monomials(3, monomial_count)?;
        let mut generators = self.base_generators()?;
        for monomial in monomials {
            if monomial.iter().sum::<u64>() == 3 {
                generators.push(self.monomial_relation(monomial)?);
            }
        }
        generators.push(terminal);
        Ok(generators)
    }
}
