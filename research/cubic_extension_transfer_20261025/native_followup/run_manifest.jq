{
  schema: "cubic-native-stage-protocol/v1",
  scope: "Fixed quartic stages and direct source controls; full transfer is not implemented",
  baseline: "Pinned native quartic reference; source rho with and without negation, using adjugate Fp3 inversion",
  samples_per_arm: 5,
  primes: [109, 157],
  arms: ["source-rho", "source-rho-neg", "target-ic", "target-rho", "target-rho-neg"],
  timing_admission: "Strict PSI required; missing metrics block qualification. Diagnostic runs remain unqualified.",
  jobs: (
    [range(20261025; 20261030) as $seed
      | [109,157][] as $p
      | ["source-rho", "source-rho-neg", "target-ic", "target-rho", "target-rho-neg"][] as $arm
      | {p:$p, seed:$seed, arm:$arm, control:"main",
         command:["./build/quartic_followup", "fixture", ("fixtures/p"+($p|tostring)+".fixture"), $arm, ($seed|tostring)]}]
    + [ [109,157][] as $p
      | ["source-rho-neg", "target-ic"][] as $arm
      | range(0;2) as $repeat
      | {p:$p, seed:20261025, arm:$arm, control:"AA", repeat:$repeat,
         command:["./build/quartic_followup", "fixture", ("fixtures/p"+($p|tostring)+".fixture"), $arm, "20261025"]}]
  )
}
