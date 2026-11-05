# Native data interpretation

The full raw input/log is authoritative for each ring and basis declaration.
All exports write individual entries; no Singular bulk-list writer is used.

- prefix34-verify01: all27candidate coefficient relations with81lift entries;
  denominator list contains682distinct polynomial EXPRESSIONS, not irreducible
  factors. Zero entries atQ specify an inadmissible coefficient localization.
- primitive34-01:17vectors, all scale factors1. qpoly34-01: exact base minimal
  polynomial q and evaluated old4x17matrix, relative4x3Jacobian and68products.
- qcolon34-01: lifted20relation columns, exact q-quotients, and37total source
  vectors in F2[n,l,v,u]. These are integral before point specialization.
- Source point witnesses preserve every ideal slot, including zeros, and a
  matrix witness whose product is literally1. Do not index by size(ideal),
  which counts only nonzero generators.
- jets34-01: source coordinate jets in F32[z,T]/(z9,T2+T+1), units/inverses,
 68descended jets,196quadratic jets, and45-bit evaluation columns. The five
  conjugates refer to coefficient descent, with the physicalAS identification.
- couple34-01: coupledColumns34[i] encodes an F2 vector of length128 as a
  polynomial in the LABEL w. Bits0..82 are the83point detector; bits83..127
  are the45padding constraints. coupledKernel34[j] is a DIFFERENT label
  vector of length196: bit i is the coefficient of section i+1 in the same
  selected196quadratic basis from R33. These w symbols are not curve functions.
  All68kernel vectors and all128pivots are recombined against original columns.

No secret scalar or witness is generated or stored; these are fixed public
research curve/point/certificate data. Restore context and source hashes
before interpreting matrices. Complete vanishing spaces are not a group-law
implementation, projected factor image, IC relation rank or key recovery.
