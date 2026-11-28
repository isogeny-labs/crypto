#include <algorithm>
#include <array>
#include <functional>
#include <gmpxx.h>
#include <iostream>
#include <numeric>
#include <stdexcept>
#include <string>
#include <vector>

using Big = mpz_class;
using Support = std::vector<int>;

mpq_class fraction(unsigned long a, unsigned long b) {
  mpq_class result{Big(a), Big(b)};
  result.canonicalize();
  return result;
}

mpq_class randomMappingExpectation(int p) {
  mpq_class survival = 1, expectation = 1;
  for (int k = 1; k < p; ++k) {
    survival *= fraction(p-k, p);
    expectation += survival;
  }
  return expectation;
}

void require(bool condition, const char* message) {
  if (!condition) throw std::runtime_error(message);
}

void printInts(const std::vector<int>& xs) {
  std::cout << '[';
  for (size_t i = 0; i < xs.size(); ++i) {
    if (i) std::cout << ',';
    std::cout << xs[i];
  }
  std::cout << ']';
}

void printBigs(const std::vector<Big>& xs) {
  std::cout << '[';
  for (size_t i = 0; i < xs.size(); ++i) {
    if (i) std::cout << ',';
    std::cout << '"' << xs[i].get_str() << '"';
  }
  std::cout << ']';
}

void supports(int p, int r, const std::function<void(const Support&)>& visit) {
  Support current;
  std::function<void(int)> next = [&](int first) {
    if (int(current.size()) == r) {
      visit(current);
      return;
    }
    for (int x = first; x < p; ++x) {
      current.push_back(x);
      next(x + 1);
      current.pop_back();
    }
  };
  next(0);
}

struct MixResult {
  int step;
  bool censored;
  Big numerator, denominator, previousNumerator, previousDenominator, histories;
  std::vector<Big> histogram;
};

// Exact TV = sum_x |p*count[x]-histories| / (2*p*histories).
Big tvNumerator(int p, const std::vector<Big>& histogram, const Big& histories) {
  Big n = 0;
  for (const auto& c : histogram) {
    Big difference = p * c - histories;
    n += abs(difference);
  }
  return n;
}

MixResult mixing(int p, const Support& support) {
  int r = int(support.size());
  const int cap = 4 * p * p;
  std::vector<Big> histogram(p);
  histogram[0] = 1;
  Big histories = 1;
  Big previousA = 0, previousD = 0;
  for (int t = 0; t <= cap; ++t) {
    Big mass = 0;
    for (const auto& c : histogram) mass += c;
    require(mass == histories, "histogram mass mismatch");
    Big a = tvNumerator(p, histogram, histories);
    Big d = 2 * p * histories;
    if (t) require(a * previousD <= previousA * d, "TV contraction failed");
    bool mixed = 2 * a <= p * histories;
    if (mixed || t == cap) {
      return {t, !mixed, a, d, previousA, previousD, histories, histogram};
    }
    previousA = a;
    previousD = d;
    std::vector<Big> sparse(p), dense(p);
    for (int x = 0; x < p; ++x) {
      for (int increment : support) sparse[(x + increment) % p] += histogram[x];
    }
    // Independently index the full transition matrix by target-source difference.
    for (int y = 0; y < p; ++y) {
      for (int x = 0; x < p; ++x) {
        int difference = (y - x + p) % p;
        if (std::binary_search(support.begin(), support.end(), difference)) {
          dense[y] += histogram[x];
        }
      }
    }
    require(sparse == dense, "sparse versus matrix distribution mismatch");
    histogram = std::move(sparse);
    histories *= r;
  }
  throw std::runtime_error("unreachable mixing exit");
}

int mixCensus(bool preflight) {
  std::vector<int> primes = preflight ? std::vector<int>{3,17} : std::vector<int>{5,7,11,13};
  std::vector<int> sizes = preflight ? std::vector<int>{2} : std::vector<int>{2,3,4,6};
  int cells = 0, censored = 0;
  for (int p : primes) for (int r : sizes) {
    if (r > p) continue;
    int population = 0, completed = 0, sum = 0, minimum = 4*p*p+1, maximum = -1;
    supports(p, r, [&](const Support& support) {
      if (preflight && p == 17 && support != Support{0,1}) return;
      auto m = mixing(p, support);
      if (preflight && p == 3) require(m.step == 2 && m.numerator == 4 && m.denominator == 24,
                             "mixing preflight known-answer failure");
      ++cells;
      ++population;
      censored += m.censored;
      if (!m.censored) {
        ++completed;
        sum += m.step;
        minimum = std::min(minimum, m.step);
        maximum = std::max(maximum, m.step);
      }
      std::cout << "{\"kind\":\"mix\",\"p\":" << p << ",\"r\":" << r << ",\"support\":";
      printInts(support);
      std::cout << ",\"quarter_step\":";
      if (m.censored) std::cout << "null"; else std::cout << m.step;
      std::cout << ",\"cap\":" << 4*p*p << ",\"evaluated_step\":" << m.step
        << ",\"censored\":" << (m.censored ? "true" : "false")
        << ",\"tv_num\":\"" << m.numerator.get_str() << "\",\"tv_den\":\""
        << m.denominator.get_str() << "\",\"previous_tv_num\":\""
        << m.previousNumerator.get_str() << "\",\"previous_tv_den\":\""
        << m.previousDenominator.get_str() << "\",\"histories\":\""
        << m.histories.get_str() << "\",\"histogram\":";
      printBigs(m.histogram);
      std::cout << ",\"matrix_check\":true,\"mass_check\":true,\"contraction_check\":true}\n";
    });
    std::cerr << "{\"kind\":\"mix_population\",\"p\":" << p << ",\"r\":" << r
      << ",\"supports\":" << population << ",\"completed\":" << completed
      << ",\"censored\":" << population-completed << ",\"completed_step_sum\":" << sum
      << ",\"completed_min\":" << (completed ? minimum : -1)
      << ",\"completed_max\":" << maximum << "}\n";
  }
  require(cells == (preflight ? 4 : 3930), "mixing population count mismatch");
  std::cerr << "{\"mode\":\"mix\",\"supports\":" << cells
    << ",\"censored\":" << censored << ",\"preflight\":" << (preflight?"true":"false") << "}\n";
  return 0;
}

struct CollisionResult {
  unsigned long maps = 0;
  unsigned long sum = 0;
  std::vector<int> histogram;
  unsigned long inDegreeSum = 0, inDegreeSquareSum = 0;
};

CollisionResult collisions(int p, const Support& support) {
  int r = int(support.size());
  unsigned long maps = 1;
  for (int i = 0; i < p; ++i) maps *= r;
  CollisionResult result{maps, 0, std::vector<int>(p+1)};
  for (unsigned long code = 0; code < maps; ++code) {
    std::vector<int> labels(p), seen(p, -1);
    auto digits = code;
    for (int x = 0; x < p; ++x) {
      labels[x] = int(digits % r);
      digits /= r;
    }
    auto step = [&](int x) { return (x + support[labels[x]]) % p; };
    int inDegree = 0;
    for (int v = 0; v < p; ++v) inDegree += step(v) == 0;
    result.inDegreeSum += inDegree;
    result.inDegreeSquareSum += inDegree * inDegree;
    int x = 0, t = 0;
    while (seen[x] == -1) {
      seen[x] = t++;
      x = step(x);
      require(t <= p, "first-repeat pigeonhole bound failed");
    }
    int directMu = seen[x], directLambda = t - seen[x];
    // Independent cycle detection: Floyd, followed by tail/cycle recovery.
    int tortoise = step(0), hare = step(step(0)), budget = 0;
    while (tortoise != hare) {
      tortoise = step(tortoise);
      hare = step(step(hare));
      require(++budget <= 2*p, "Floyd meeting budget");
    }
    int mu = 0;
    tortoise = 0;
    while (tortoise != hare) {
      tortoise = step(tortoise);
      hare = step(hare);
      require(++mu <= p, "Floyd tail budget");
    }
    int lambda = 1;
    hare = step(tortoise);
    while (tortoise != hare) {
      hare = step(hare);
      require(++lambda <= p, "Floyd cycle budget");
    }
    require(mu == directMu && lambda == directLambda && mu + lambda == t,
            "Floyd versus visited-array mismatch");
    ++result.histogram[t];
    result.sum += t;
  }
  require(std::accumulate(result.histogram.begin(), result.histogram.end(), 0UL) == maps,
          "collision population mass mismatch");
  require(result.inDegreeSum == maps &&
          r * (result.inDegreeSquareSum-maps) == (r-1) * maps,
          "exact in-degree moments disagree with distinct-predecessor formula");
  return result;
}

int rhoCensus(bool preflight) {
  std::vector<int> primes = preflight ? std::vector<int>{2,11} : std::vector<int>{3,5,7};
  std::vector<int> sizes = {2,3};
  unsigned long totalMaps = 0;
  int cells = 0;
  for (int p : primes) for (int r : sizes) {
    if (r > p) continue;
    if (preflight && p == 11 && r != 3) continue;
    int population = 0;
    unsigned long mapSum = 0, stepSum = 0;
    int nonzeroPopulation = 0;
    unsigned long nonzeroMapSum = 0, nonzeroStepSum = 0;
    auto baseline = randomMappingExpectation(p);
    supports(p, r, [&](const Support& support) {
      if (preflight && p == 11 && support != Support{0,1,2}) return;
      auto c = collisions(p, support);
      if (preflight && p == 2) require(c.maps == 4 && c.sum == 6 && c.histogram[1] == 2 &&
                            c.histogram[2] == 2, "rho preflight known-answer failure");
      // Full support at p=3 is exactly the uniform random-mapping model.
      if (p == 3 && r == 3) {
        require(c.maps == 27 && c.sum == 51 && c.histogram[1] == 9 &&
                c.histogram[2] == 12 && c.histogram[3] == 6, "full-support mapping control");
      }
      ++cells;
      ++population;
      totalMaps += c.maps;
      mapSum += c.maps;
      stepSum += c.sum;
      bool hasZero = std::binary_search(support.begin(), support.end(), 0);
      if (!hasZero) {
        ++nonzeroPopulation;
        nonzeroMapSum += c.maps;
        nonzeroStepSum += c.sum;
      }
      std::cout << "{\"kind\":\"rho\",\"p\":" << p << ",\"r\":" << r << ",\"support\":";
      printInts(support);
      std::cout << ",\"has_zero_increment\":" << (hasZero?"true":"false")
        << ",\"maps\":" << c.maps << ",\"mean_num\":" << c.sum
        << ",\"mean_den\":" << c.maps << ",\"first_repeat_histogram\":";
      printInts(c.histogram);
      std::cout << ",\"indegree_mean_num\":" << c.inDegreeSum
        << ",\"indegree_square_sum\":" << c.inDegreeSquareSum
        << ",\"variance_num\":" << c.inDegreeSquareSum-c.maps
        << ",\"variance_den\":" << c.maps
        << ",\"floyd_check\":true,\"mass_check\":true,\"pigeonhole_check\":true,\"variance_check\":true}\n";
    });
    auto summary = [&](const std::string& stratum, int n, unsigned long maps, unsigned long sum) {
      std::cerr << "{\"kind\":\"rho_population\",\"p\":" << p << ",\"r\":" << r
        << ",\"stratum\":\"" << stratum << "\",\"supports\":" << n << ",\"maps\":" << maps;
      if (maps) {
        auto mean = fraction(sum, maps);
        mpq_class relative = mean / baseline;
        std::cerr << ",\"mean_num\":\"" << mean.get_num() << "\",\"mean_den\":\"" << mean.get_den()
          << "\",\"mapping_baseline_num\":\"" << baseline.get_num()
          << "\",\"mapping_baseline_den\":\"" << baseline.get_den()
          << "\",\"relative_num\":\"" << relative.get_num()
          << "\",\"relative_den\":\"" << relative.get_den()
          << "\",\"heuristic_penalty_squared_num\":" << r
          << ",\"heuristic_penalty_squared_den\":" << r-1;
      }
      std::cerr << "}\n";
    };
    summary("all", population, mapSum, stepSum);
    summary("no_zero", nonzeroPopulation, nonzeroMapSum, nonzeroStepSum);
  }
  require(cells == (preflight ? 2 : 80), "rho support count mismatch");
  require(totalMaps == (preflight ? 177151UL : 82034UL), "rho hash-function count mismatch");
  std::cerr << "{\"mode\":\"rho\",\"supports\":" << cells << ",\"maps\":" << totalMaps
    << ",\"preflight\":" << (preflight?"true":"false") << "}\n";
  return 0;
}

int main(int argc, char** argv) {
  try {
    require(argc == 2, "usage: census --mix|--rho|--preflight-mix|--preflight-rho");
    const std::string mode = argv[1];
    if (mode == "--mix") return mixCensus(false);
    if (mode == "--rho") return rhoCensus(false);
    if (mode == "--preflight-mix") return mixCensus(true);
    if (mode == "--preflight-rho") return rhoCensus(true);
    throw std::runtime_error("unknown mode");
  } catch (const std::exception& error) {
    std::cerr << "{\"error\":\"" << error.what() << "\"}\n";
    return 1;
  }
}
