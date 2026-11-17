#include "Vcrypto_kernels_top.h"
#include "test.h"

#include <openssl/evp.h>
#include <verilated.h>

#include <array>
#include <cstdint>
#include <memory>
#include <random>
#include <vector>

namespace {
using Bytes = std::vector<uint8_t>;
using Polynomial = std::array<uint32_t, 256>;
constexpr uint64_t Seed = 0x4650433252544cULL; // Public, deterministic test fixtures.

struct Sim {
  VerilatedContext context;
  std::unique_ptr<Vcrypto_kernels_top> top;
  uint64_t cycles = 0;

  Sim() {
    context.threads(1);
    context.randSeed(0x46504332);
    context.randReset(2);
    top = std::make_unique<Vcrypto_kernels_top>(&context);
    auto &d = *top;
    d.clk = d.clear = d.rst_n = 0;
    d.perm_start = 0;
    for (size_t i = 0; i < 50; ++i)
      d.perm_state_in[i] = 0;
    d.shake_start = d.shake_mode_256 = d.shake_in_valid = d.shake_out_ready = 0;
    d.shake_input_length = d.shake_output_length = d.shake_in_byte = 0;
    d.mul_start = d.mul_mode_dsa = d.mul_a = d.mul_b = 0;
    d.ntt_start = d.ntt_mode_dsa = d.ntt_inverse = d.ntt_in_valid = d.ntt_out_ready = 0;
    d.ntt_in_coeff = 0;
    tick();
    tick();
    d.rst_n = 1;
    for (size_t i = 0; i < 300 && !d.ntt_ready; ++i)
      tick();
    CHECK(d.ntt_ready && d.shake_ready && d.mul_ready && d.perm_ready);
  }

  Vcrypto_kernels_top &d() { return *top; }
  void settle() {
    top->clk = 0;
    top->eval();
  }
  void tick() {
    settle();
    context.timeInc(1);
    top->clk = 1;
    top->eval();
    context.timeInc(1);
    settle();
    ++cycles;
  }
  void clear() {
    auto &t = d();
    t.perm_start = t.shake_start = t.mul_start = t.ntt_start = 0;
    t.shake_in_valid = t.shake_out_ready = t.ntt_in_valid = t.ntt_out_ready = 0;
    t.clear = 1;
    tick();
    CHECK(!t.shake_out_valid && t.shake_out_byte == 0);
    CHECK(!t.ntt_out_valid && t.ntt_out_coeff == 0);
    CHECK(t.mul_result == 0 && !t.mul_error);
    for (size_t i = 0; i < 50; ++i)
      CHECK(t.perm_state_out[i] == 0);
    t.clear = 0;
    settle();
    CHECK(!t.ntt_ready && t.ntt_busy);
    for (size_t i = 0; i < 255; ++i) {
      tick();
      CHECK(!t.ntt_ready);
    }
    tick();
    CHECK(t.ntt_ready && !t.ntt_done && !t.ntt_error);
    CHECK(t.shake_ready && t.mul_ready && t.perm_ready);
  }
};

Bytes unhex(const std::string &value) {
  CHECK(value.size() % 2 == 0);
  auto digit = [](char c) -> uint8_t {
    if (c >= '0' && c <= '9')
      return static_cast<uint8_t>(c - '0');
    if (c >= 'a' && c <= 'f')
      return static_cast<uint8_t>(c - 'a' + 10);
    throw std::runtime_error("invalid fixture hex");
  };
  Bytes result;
  for (size_t i = 0; i < value.size(); i += 2)
    result.push_back(static_cast<uint8_t>(16 * digit(value[i]) + digit(value[i + 1])));
  return result;
}

Bytes opensslShake(const Bytes &input, size_t length, bool mode256) {
  std::unique_ptr<EVP_MD_CTX, decltype(&EVP_MD_CTX_free)> ctx(EVP_MD_CTX_new(), EVP_MD_CTX_free);
  CHECK(ctx != nullptr);
  CHECK(EVP_DigestInit_ex(ctx.get(), mode256 ? EVP_shake256() : EVP_shake128(), nullptr) == 1);
  if (!input.empty())
    CHECK(EVP_DigestUpdate(ctx.get(), input.data(), input.size()) == 1);
  Bytes result(length);
  uint8_t unused = 0;
  CHECK(EVP_DigestFinalXOF(ctx.get(), length ? result.data() : &unused, length) == 1);
  return result;
}

Bytes shake(Sim &sim, const Bytes &input, size_t length, bool mode256, bool stalls = true) {
  auto &d = sim.d();
  CHECK(d.shake_ready);
  d.shake_start = 1;
  d.shake_mode_256 = mode256;
  d.shake_input_length = static_cast<uint32_t>(input.size());
  d.shake_output_length = static_cast<uint32_t>(length);
  sim.tick();
  CHECK(d.shake_busy);
  // Public command inputs must be latched; these changes must have no effect.
  d.shake_start = 0;
  d.shake_mode_256 = !mode256;
  d.shake_input_length = 0xffffffffU;
  d.shake_output_length = 0;
  std::mt19937_64 rng(Seed ^ input.size() ^ length ^ static_cast<uint64_t>(mode256));
  Bytes output;
  size_t consumed = 0;
  bool pending = false, held = false, finished = false;
  uint8_t held_byte = 0;
  for (size_t cycle = 0; cycle < 200000; ++cycle) {
    if (!pending && consumed < input.size() && (!stalls || rng() % 4))
      pending = true;
    d.shake_in_valid = pending;
    d.shake_in_byte = consumed < input.size() ? input[consumed] : 0;
    d.shake_out_ready = !stalls || rng() % 4;
    d.shake_start = (cycle % 31 == 7); // A busy command must never restart the engine.
    sim.settle();
    if (held)
      CHECK(d.shake_out_valid && d.shake_out_byte == held_byte);
    if (d.shake_in_valid && d.shake_in_ready) {
      ++consumed;
      pending = false;
    }
    if (d.shake_out_valid && d.shake_out_ready)
      output.push_back(d.shake_out_byte);
    held = d.shake_out_valid && !d.shake_out_ready;
    held_byte = d.shake_out_byte;
    sim.tick();
    if (d.shake_done) {
      finished = true;
      break;
    }
  }
  d.shake_start = d.shake_in_valid = d.shake_out_ready = 0;
  CHECK(finished && consumed == input.size() && output.size() == length);
  CHECK(d.shake_ready && !d.shake_busy && !d.shake_out_valid && d.shake_out_byte == 0);
  sim.tick();
  CHECK(!d.shake_done);
  return output;
}

uint32_t modulus(bool dsa) { return dsa ? 8380417 : 3329; }
uint32_t power(uint32_t base, uint32_t exponent, uint32_t q) {
  uint64_t r = 1, b = base;
  while (exponent) {
    if (exponent & 1)
      r = r * b % q;
    b = b * b % q;
    exponent >>= 1;
  }
  return static_cast<uint32_t>(r);
}
uint32_t reverseBits(uint32_t value, unsigned bits) {
  uint32_t r = 0;
  for (unsigned i = 0; i < bits; ++i) {
    r = 2 * r + (value & 1);
    value >>= 1;
  }
  return r;
}

// Independent O(n^2) evaluation/interpolation. This does not repeat the RTL's
// butterfly algorithm, twiddle ROM, scheduling, or Barrett reduction.
Polynomial directTransform(const Polynomial &input, bool dsa, bool inverse) {
  const uint32_t q = modulus(dsa), root = dsa ? 1753 : 17;
  const size_t stride = dsa ? 1 : 2, count = 256 / stride;
  const unsigned bits = dsa ? 8 : 7;
  Polynomial result{};
  for (size_t i = 0; i < count; ++i) {
    const auto point = power(root, 2 * reverseBits(static_cast<uint32_t>(i), bits) + 1, q);
    for (size_t parity = 0; parity < stride; ++parity) {
      if (!inverse) {
        uint64_t value = 0;
        for (size_t j = count; j-- > 0;)
          value = (value * point + input[stride * j + parity]) % q;
        result[stride * i + parity] = static_cast<uint32_t>(value);
      } else {
        const auto inverse_point = power(point, q - 2, q);
        uint64_t basis = 1;
        for (size_t j = 0; j < count; ++j) {
          const auto index = stride * j + parity;
          result[index] =
              static_cast<uint32_t>((result[index] + input[stride * i + parity] * basis) % q);
          basis = basis * inverse_point % q;
        }
      }
    }
  }
  if (inverse) {
    const auto scale = power(static_cast<uint32_t>(count), q - 2, q);
    for (auto &value : result)
      value = static_cast<uint32_t>(uint64_t(value) * scale % q);
  }
  return result;
}

struct TransformResult {
  Polynomial data;
  uint64_t cycles;
};
TransformResult transform(Sim &sim, const Polynomial &input, bool dsa, bool inverse,
                          bool stalls = true) {
  auto &d = sim.d();
  CHECK(d.ntt_ready);
  const auto begin = sim.cycles;
  d.ntt_start = 1;
  d.ntt_mode_dsa = dsa;
  d.ntt_inverse = inverse;
  sim.tick();
  CHECK(d.ntt_busy && !d.ntt_ready);
  d.ntt_start = 0;
  d.ntt_mode_dsa = !dsa;
  d.ntt_inverse = !inverse;
  std::mt19937_64 rng(Seed ^ static_cast<uint64_t>(dsa) ^ static_cast<uint64_t>(inverse));
  size_t sent = 0, received = 0;
  bool pending = false, held = false, finished = false;
  uint32_t held_value = 0;
  Polynomial result{};
  for (size_t cycle = 0; cycle < 30000; ++cycle) {
    if (!pending && sent < input.size() && (!stalls || rng() % 4))
      pending = true;
    d.ntt_in_valid = pending;
    d.ntt_in_coeff = sent < input.size() ? input[sent] : 0xffffffffU;
    d.ntt_out_ready = !stalls || rng() % 4;
    d.ntt_start = cycle % 41 == 3;
    sim.settle();
    if (held)
      CHECK(d.ntt_out_valid && d.ntt_out_coeff == held_value);
    if (d.ntt_in_ready && d.ntt_in_valid) {
      ++sent;
      pending = false;
    }
    if (d.ntt_out_valid && d.ntt_out_ready) {
      CHECK(received < result.size());
      CHECK(d.ntt_out_coeff < modulus(dsa));
      result[received++] = d.ntt_out_coeff;
    }
    held = d.ntt_out_valid && !d.ntt_out_ready;
    held_value = d.ntt_out_coeff;
    sim.tick();
    CHECK(!d.ntt_error);
    if (d.ntt_done) {
      finished = true;
      break;
    }
  }
  d.ntt_start = d.ntt_in_valid = d.ntt_out_ready = 0;
  CHECK(finished && sent == input.size() && received == result.size());
  CHECK(d.ntt_ready && !d.ntt_busy && !d.ntt_out_valid && d.ntt_out_coeff == 0);
  const auto clocks = sim.cycles - begin;
  sim.tick();
  CHECK(!d.ntt_done);
  return {result, clocks};
}

uint32_t multiply(Sim &sim, uint32_t a, uint32_t b, bool dsa) {
  auto &d = sim.d();
  CHECK(d.mul_ready);
  d.mul_mode_dsa = dsa;
  d.mul_a = a;
  d.mul_b = b;
  d.mul_start = 1;
  sim.tick();
  d.mul_start = 0;
  d.mul_a = d.mul_b = 0xffffffffU;
  d.mul_mode_dsa = !dsa;
  for (size_t i = 0; i < 3; ++i) {
    CHECK(d.mul_busy && !d.mul_done);
    sim.tick();
  }
  CHECK(d.mul_done && d.mul_ready && !d.mul_error);
  return d.mul_result;
}

std::vector<Polynomial> fixtures(bool dsa) {
  const auto q = modulus(dsa);
  std::vector<Polynomial> cases(8);
  cases[1].fill(1);
  cases[2].fill(q - 1);
  cases[3][0] = 1;
  cases[4][255] = q - 1;
  for (size_t i = 0; i < 256; ++i)
    cases[5][i] = static_cast<uint32_t>((i * i + 17) % q);
  std::mt19937_64 rng(Seed ^ q);
  for (size_t i = 6; i < cases.size(); ++i)
    for (auto &coefficient : cases[i])
      coefficient = static_cast<uint32_t>(rng() % q);
  return cases;
}

Polynomial schoolbook(const Polynomial &a, const Polynomial &b, bool dsa) {
  Polynomial result{};
  const uint32_t q = modulus(dsa);
  for (size_t i = 0; i < 256; ++i)
    for (size_t j = 0; j < 256; ++j) {
      const auto product = uint64_t(a[i]) * b[j] % q;
      const auto index = (i + j) % 256;
      result[index] = static_cast<uint32_t>(i + j < 256 ? (result[index] + product) % q
                                                        : (result[index] + q - product) % q);
    }
  return result;
}
} // namespace

TEST(keccak_f1600_zero_known_answer_and_busy_input_latching) {
  Sim sim;
  auto &d = sim.d();
  // Keccak team's Keccak-f[1600] zero-state permutation, lane order x+5*y.
  const std::array<uint64_t, 25> expected{
      0xf1258f7940e1dde7ULL, 0x84d5ccf933c0478aULL, 0xd598261ea65aa9eeULL, 0xbd1547306f80494dULL,
      0x8b284e056253d057ULL, 0xff97a42d7f8e6fd4ULL, 0x90fee5a0a44647c4ULL, 0x8c5bda0cd6192e76ULL,
      0xad30a6f71b19059cULL, 0x30935ab7d08ffc64ULL, 0xeb5aa93f2317d635ULL, 0xa9a6e6260d712103ULL,
      0x81a57c16dbcf555fULL, 0x43b831cd0347c826ULL, 0x01f22f1a11a5569fULL, 0x05e5635a21d9ae61ULL,
      0x64befef28cc970f2ULL, 0x613670957bc46611ULL, 0xb87c5a554fd00ecbULL, 0x8c3ee88a1ccf32c8ULL,
      0x940c7922ae3a2614ULL, 0x1841f924a2c509e4ULL, 0x16f53526e70465c2ULL, 0x75f644e97f30a13bULL,
      0xeaf1ff7b5ceca249ULL};
  d.perm_start = 1;
  sim.tick();
  for (size_t i = 0; i < 50; ++i)
    d.perm_state_in[i] = 0xffffffffU;
  for (size_t i = 0; i < 24; ++i) {
    CHECK(d.perm_busy && !d.perm_ready && !d.perm_done);
    sim.tick();
  }
  d.perm_start = 0;
  CHECK(d.perm_done && d.perm_ready);
  for (size_t i = 0; i < expected.size(); ++i) {
    const uint64_t actual = d.perm_state_out[2 * i] | (uint64_t(d.perm_state_out[2 * i + 1]) << 32);
    CHECK(actual == expected[i]);
  }
  sim.tick();
  CHECK(!d.perm_done);
  sim.clear();
}

TEST(keccak_f1600_clear_cancels_each_round) {
  Sim sim;
  auto &d = sim.d();
  for (size_t round = 0; round < 24; ++round) {
    d.perm_start = 1;
    sim.tick();
    d.perm_start = 0;
    for (size_t i = 0; i < round; ++i)
      sim.tick();
    sim.clear();
    CHECK(!d.perm_busy && !d.perm_done);
  }
}

TEST(modular_multiply_edges_and_independent_random_residues) {
  Sim sim;
  std::mt19937_64 rng(Seed);
  for (bool dsa : {false, true}) {
    const uint32_t q = modulus(dsa);
    for (uint32_t a : {0U, 1U, 2U, q / 2, q - 2, q - 1})
      for (uint32_t b : {0U, 1U, 2U, q / 2, q - 2, q - 1})
        CHECK(multiply(sim, a, b, dsa) == uint64_t(a) * b % q);
    for (size_t i = 0; i < 2048; ++i) {
      const auto a = static_cast<uint32_t>(rng() % q), b = static_cast<uint32_t>(rng() % q);
      CHECK(multiply(sim, a, b, dsa) == uint64_t(a) * b % q);
    }
  }
}

TEST(modular_multiply_rejects_noncanonical_inputs_and_clears_pipeline) {
  Sim sim;
  auto &d = sim.d();
  for (bool dsa : {false, true}) {
    for (uint32_t invalid : {modulus(dsa), 0x80000000U, 0xffffffffU}) {
      for (bool invalid_b : {false, true}) {
        d.mul_mode_dsa = dsa;
        d.mul_a = invalid_b ? 1 : invalid;
        d.mul_b = invalid_b ? invalid : 1;
        d.mul_start = 1;
        sim.tick();
        d.mul_start = 0;
        CHECK(d.mul_done && d.mul_error && d.mul_ready && d.mul_result == 0);
        CHECK(multiply(sim, 7, 9, dsa) == 63);
      }
    }
    for (size_t phase = 0; phase < 3; ++phase) {
      d.mul_mode_dsa = dsa;
      d.mul_a = d.mul_b = modulus(dsa) - 1;
      d.mul_start = 1;
      sim.tick();
      d.mul_start = 0;
      for (size_t i = 0; i < phase; ++i)
        sim.tick();
      sim.clear();
      CHECK(multiply(sim, 3, 5, dsa) == 15);
    }
  }
}

TEST(shake_fips202_empty_and_abc_known_answers) {
  Sim sim;
  const Bytes abc{'a', 'b', 'c'};
  const auto empty128 = unhex("7f9c2ba4e88f827d616045507605853ed73b8093f6efbc88eb1a6eacfa66ef26");
  const auto abc128 = unhex("5881092dd818bf5cf8a3ddb793fbcba74097d5c526a6d35f97b83351940f2cc8");
  const auto empty256 = unhex("46b9dd2b0ba88d13233b3feb743eeb243fcd52ea62b81b82b50c27646ed5762f"
                              "d75dc4ddd8c0f200cb05019d67b592f6fc821c49479ab48640292eacb3b7c4be");
  const auto abc256 = unhex("483366601360a8771c6863080cc4114d8db44530f8f1e1ee4f94ea37e78b5739d"
                            "5a15bef186a5386c75744c0527e1faa9f8726e462a12a4feb06bd8801e751e4");
  CHECK(shake(sim, {}, 32, false) == empty128);
  CHECK(shake(sim, abc, 32, false) == abc128);
  CHECK(shake(sim, {}, 64, true) == empty256);
  CHECK(shake(sim, abc, 64, true) == abc256);
  CHECK(opensslShake(abc, 64, true) == abc256);
}

TEST(shake_rate_boundaries_random_messages_squeeze_and_backpressure) {
  Sim sim;
  std::mt19937_64 rng(Seed);
  for (bool mode256 : {false, true}) {
    const size_t rate = mode256 ? 136 : 168;
    const std::vector<size_t> lengths{
        0, 1, rate - 2, rate - 1, rate, rate + 1, 2 * rate - 1, 2 * rate, 2 * rate + 1, 4096};
    for (const auto length : lengths) {
      Bytes input(length);
      for (auto &b : input)
        b = static_cast<uint8_t>(rng());
      for (const size_t out : {size_t(0), size_t(1), rate - 1, rate, rate + 1, 2 * rate + 17})
        CHECK(shake(sim, input, out, mode256) == opensslShake(input, out, mode256));
    }
    for (size_t i = 0; i < 12; ++i) {
      Bytes input(static_cast<size_t>(rng() % 2049));
      for (auto &b : input)
        b = static_cast<uint8_t>(rng());
      const auto out = static_cast<size_t>(rng() % 1025);
      CHECK(shake(sim, input, out, mode256) == opensslShake(input, out, mode256));
    }
  }
}

TEST(shake_clear_during_absorb_permutation_and_stalled_output) {
  Sim sim;
  auto &d = sim.d();
  for (unsigned phase = 0; phase < 3; ++phase) {
    d.shake_start = 1;
    d.shake_mode_256 = 1;
    d.shake_input_length = phase == 0 ? 10 : 0;
    d.shake_output_length = 64;
    sim.tick();
    d.shake_start = 0;
    if (phase == 0) {
      d.shake_in_byte = 0xa5;
      d.shake_in_valid = 1;
      sim.tick();
      d.shake_in_valid = 0;
    } else if (phase == 1) {
      for (unsigned i = 0; i < 10; ++i)
        sim.tick();
    } else {
      for (unsigned i = 0; i < 60 && !d.shake_out_valid; ++i)
        sim.tick();
      CHECK(d.shake_out_valid);
      const auto held = d.shake_out_byte;
      for (unsigned i = 0; i < 20; ++i) {
        sim.tick();
        CHECK(d.shake_out_byte == held);
      }
    }
    sim.clear();
    CHECK(shake(sim, Bytes{1, 2, 3}, 200, true) == opensslShake(Bytes{1, 2, 3}, 200, true));
  }
}

TEST(ntt_forward_matches_independent_polynomial_evaluation) {
  Sim sim;
  for (bool dsa : {false, true})
    for (const auto &input : fixtures(dsa))
      CHECK(transform(sim, input, dsa, false).data == directTransform(input, dsa, false));
}

TEST(ntt_inverse_matches_independent_interpolation) {
  Sim sim;
  for (bool dsa : {false, true})
    for (const auto &input : fixtures(dsa))
      CHECK(transform(sim, input, dsa, true).data == directTransform(input, dsa, true));
}

TEST(ntt_polynomial_products_match_schoolbook_negacyclic_convolution) {
  Sim sim;
  for (bool dsa : {false, true}) {
    const auto q = modulus(dsa);
    const auto cases = fixtures(dsa);
    for (size_t pair = 4; pair < cases.size(); ++pair) {
      const auto &a = cases[pair];
      const auto &b = cases[(pair + 1) % cases.size()];
      const auto x = transform(sim, a, dsa, false).data;
      const auto y = transform(sim, b, dsa, false).data;
      Polynomial product{};
      // Public C++ sequencing of pointwise/base multiplication is a test oracle,
      // not a claimed hardware polynomial-multiplication command.
      if (dsa) {
        for (size_t i = 0; i < 256; ++i)
          product[i] = static_cast<uint32_t>(uint64_t(x[i]) * y[i] % q);
      } else {
        for (size_t i = 0; i < 128; ++i) {
          const auto zeta = power(17, 2 * reverseBits(static_cast<uint32_t>(i), 7) + 1, q);
          const auto a0 = uint64_t(x[2 * i]), a1 = uint64_t(x[2 * i + 1]);
          const auto b0 = uint64_t(y[2 * i]), b1 = uint64_t(y[2 * i + 1]);
          product[2 * i] = static_cast<uint32_t>((a0 * b0 + (a1 * b1 % q) * zeta) % q);
          product[2 * i + 1] = static_cast<uint32_t>((a0 * b1 + a1 * b0) % q);
        }
      }
      CHECK(transform(sim, product, dsa, true).data == schoolbook(a, b, dsa));
    }
  }
}

TEST(ntt_rejects_noncanonical_inputs_and_scrubs_before_reuse) {
  Sim sim;
  auto &d = sim.d();
  for (bool dsa : {false, true})
    for (uint32_t invalid : {modulus(dsa), 0x80000000U, 0xffffffffU})
      for (size_t invalid_at : {size_t(0), size_t(127), size_t(255)}) {
        d.ntt_mode_dsa = dsa;
        d.ntt_inverse = 0;
        d.ntt_start = 1;
        sim.tick();
        d.ntt_start = 0;
        d.ntt_in_valid = 1;
        for (size_t i = 0; i <= invalid_at; ++i) {
          CHECK(d.ntt_in_ready);
          d.ntt_in_coeff = i == invalid_at ? invalid : modulus(dsa) - 1;
          sim.tick();
        }
        d.ntt_in_valid = 0;
        CHECK(d.ntt_busy && !d.ntt_ready && !d.ntt_out_valid);
        for (size_t i = 0; i < 255; ++i) {
          sim.tick();
          CHECK(!d.ntt_ready && !d.ntt_done && !d.ntt_out_valid);
        }
        sim.tick();
        CHECK(d.ntt_ready && d.ntt_done && d.ntt_error && d.ntt_out_coeff == 0);
        CHECK(transform(sim, Polynomial{}, dsa, false).data == Polynomial{});
      }
}

TEST(ntt_clear_during_load_arithmetic_and_stalled_output) {
  Sim sim;
  auto &d = sim.d();
  for (unsigned phase = 0; phase < 3; ++phase) {
    d.ntt_start = 1;
    d.ntt_mode_dsa = 1;
    d.ntt_inverse = 1;
    sim.tick();
    d.ntt_start = 0;
    d.ntt_in_valid = 1;
    d.ntt_in_coeff = 8380416;
    for (size_t i = 0; i < (phase == 0 ? 17U : 256U); ++i)
      sim.tick();
    d.ntt_in_valid = 0;
    if (phase == 1)
      for (unsigned i = 0; i < 20; ++i)
        sim.tick();
    if (phase == 2) {
      for (size_t i = 0; i < 15000 && !d.ntt_out_valid; ++i)
        sim.tick();
      CHECK(d.ntt_out_valid);
      const auto held = d.ntt_out_coeff;
      for (unsigned i = 0; i < 20; ++i) {
        sim.tick();
        CHECK(d.ntt_out_coeff == held);
      }
    }
    sim.clear();
    CHECK(transform(sim, Polynomial{}, true, true).data == Polynomial{});
  }
}

TEST(ntt_fixed_schedule_for_different_coefficients_without_io_stalls) {
  Sim sim;
  for (bool dsa : {false, true}) {
    Polynomial maximal{};
    maximal.fill(modulus(dsa) - 1);
    for (bool inverse : {false, true}) {
      const auto zero = transform(sim, Polynomial{}, dsa, inverse, false);
      const auto other = transform(sim, maximal, dsa, inverse, false);
      CHECK(zero.cycles == other.cycles);
      CHECK(other.data == directTransform(maximal, dsa, inverse));
    }
  }
}

TEST(independent_hash_and_ntt_engines_preserve_stalled_results) {
  Sim sim;
  auto &d = sim.d();
  d.shake_start = 1;
  d.shake_mode_256 = 1;
  d.shake_input_length = 0;
  d.shake_output_length = 512;
  sim.tick();
  d.shake_start = 0;
  for (unsigned i = 0; i < 60 && !d.shake_out_valid; ++i)
    sim.tick();
  CHECK(d.shake_out_valid);
  const auto held = d.shake_out_byte;
  CHECK(transform(sim, fixtures(true).back(), true, false).data ==
        directTransform(fixtures(true).back(), true, false));
  CHECK(d.shake_out_valid && d.shake_out_byte == held);
  Bytes output;
  d.shake_out_ready = 1;
  for (size_t i = 0; i < 2000 && !d.shake_done; ++i) {
    if (d.shake_out_valid)
      output.push_back(d.shake_out_byte);
    sim.tick();
  }
  d.shake_out_ready = 0;
  CHECK(d.shake_done && output == opensslShake({}, 512, true));
}

TEST(asynchronous_reset_cancels_active_kernels_and_scrubs_before_reuse) {
  Sim sim;
  auto &d = sim.d();
  d.ntt_start = d.shake_start = d.mul_start = d.perm_start = 1;
  d.ntt_mode_dsa = d.mul_mode_dsa = d.shake_mode_256 = 1;
  d.ntt_inverse = 0;
  d.shake_input_length = 100;
  d.shake_output_length = 100;
  d.mul_a = d.mul_b = 8380416;
  sim.tick();
  d.ntt_start = d.shake_start = d.mul_start = d.perm_start = 0;
  d.ntt_in_valid = d.shake_in_valid = 1;
  d.ntt_in_coeff = 8380416;
  d.shake_in_byte = 0xff;
  sim.tick();
  d.ntt_in_valid = d.shake_in_valid = 0;
  d.rst_n = 0;
  sim.settle();
  CHECK(!d.ntt_out_valid && !d.shake_out_valid && !d.mul_done && !d.perm_done);
  CHECK(d.mul_result == 0);
  for (size_t i = 0; i < 50; ++i)
    CHECK(d.perm_state_out[i] == 0);
  sim.tick();
  d.rst_n = 1;
  sim.settle();
  for (size_t i = 0; i < 255; ++i) {
    sim.tick();
    CHECK(!d.ntt_ready);
  }
  sim.tick();
  CHECK(d.ntt_ready && !d.ntt_done && !d.ntt_error);
  CHECK(transform(sim, Polynomial{}, true, false).data == Polynomial{});
  CHECK(shake(sim, {}, 64, true) == opensslShake({}, 64, true));
  CHECK(multiply(sim, 3, 7, true) == 21);
}

int main(int argc, char **argv) {
  Verilated::commandArgs(argc, argv);
  return tests::run();
}
