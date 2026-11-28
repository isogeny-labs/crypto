namespace AnchoredPairYield

-- Closed parameter obligations only; no native map or sampling theorem.
theorem signed_pair_plus_zero_lt :
    (2 * (213156431 * 20633)) ^ 2 + 1 < 2 ^ 87 := by decide

theorem ell_ge_two_pow252 :
    2 ^ 252 ≤ 2 ^ 252 + 27742317777372353535851937790883648493 := by decide

#print axioms signed_pair_plus_zero_lt
#print axioms ell_ge_two_pow252
end AnchoredPairYield
