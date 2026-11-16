// Native matched instrument. Timings on shared RunPod hosts are diagnostic;
// transfer_bytes counts API payloads from the observed launched batches/hits.
#include "../../gpu/explicit-relations/device.cuh"
#include <chrono>
#include <iomanip>
#include <iostream>

using namespace explicit_relations;
using Clock = std::chrono::steady_clock;
static double ms(Clock::time_point a, Clock::time_point b) {
  return std::chrono::duration<double, std::milli>(b - a).count();
}
static void require(bool ok, const char *why) {
  if (!ok)
    throw std::runtime_error(why);
}
static U mix(U &state) {
  state += 0x9e3779b97f4a7c15ULL;
  U z = state;
  z = (z ^ (z >> 30)) * 0xbf58476d1ce4e5b9ULL;
  z = (z ^ (z >> 27)) * 0x94d049bb133111ebULL;
  return z ^ (z >> 31);
}
__global__ void normalize_fixture(Curve c, const Jac *in, Point *out, U n) {
  U i = U(blockIdx.x) * BLOCK + threadIdx.x;
  Point result = normalize_block(c, i < n ? in[i] : Jac{});
  if (i < n)
    out[i] = result;
}
static double normalization(Curve c, U n, U seed, U &digest, bool timed) {
  Point base;
  for (U x = 0; !c.lift(x, 0, base); ++x) {
  }
  std::vector<Jac> input(n);
  for (U i = 0; i < n; ++i) {
    if (i % 17 == 0)
      continue;
    U z = fp64_to_mont(c.f, 1 + mix(seed) % (c.f.p - 1)), zz = c.sq(z);
    input[i] = {c.mul(fp64_to_mont(c.f, base.x), zz),
                c.mul(fp64_to_mont(c.f, base.y), c.mul(zz, z)), z};
  }
  thrust::device_vector<Jac> device_input(input.begin(), input.end());
  thrust::device_vector<Point> output(n);
  unsigned blocks = (unsigned)((n + BLOCK - 1) / BLOCK);
  auto launch = [&] {
    normalize_fixture<<<blocks, BLOCK>>>(c, data(device_input), data(output),
                                         n);
    cuda_check(cudaGetLastError());
  };
  launch();
  cuda_check(cudaDeviceSynchronize());
  float elapsed = 0;
  if (timed) {
    cudaEvent_t begin, end;
    cuda_check(cudaEventCreate(&begin));
    cuda_check(cudaEventCreate(&end));
    cuda_check(cudaEventRecord(begin));
    for (unsigned i = 0; i < 8; ++i)
      launch();
    cuda_check(cudaEventRecord(end));
    cuda_check(cudaEventSynchronize(end));
    cuda_check(cudaEventElapsedTime(&elapsed, begin, end));
    cuda_check(cudaEventDestroy(begin));
    cuda_check(cudaEventDestroy(end));
  }
  std::vector<Point> got(n);
  cuda_check(cudaMemcpy(got.data(), data(output), n * sizeof(Point),
                        cudaMemcpyDeviceToHost));
  for (U i = 0; i < n; ++i) {
    Point expected = i % 17 == 0 ? Point{} : base;
    require(equal(got[i], expected), "projective normalization mismatch");
    digest = digest * 1099511628211ULL ^ (got[i].x + got[i].y + got[i].inf);
  }
  return elapsed / 8;
}
static void check() {
  for (U p : {U(97), U(211), U(2305843009213693951ULL)}) {
    Curve c = curve(p, 0, 7);
    for (U n : {U(1), U(31), U(32), U(255), U(256), U(257), U(4097)}) {
      U digest = 0;
      normalization(c, n, 42, digest, false);
    }
  }
  std::cout << "PASS normalization boundaries, infinity lanes, random scales, "
               "61-bit field\n";
}
int main(int argc, char **argv) {
  try {
    if (argc == 2 && std::string(argv[1]) == "--check") {
      check();
      return 0;
    }
    if (argc != 4)
      throw std::runtime_error(
          "usage: benchmark ARM INSTANCE[0..2] ROUND[0..9]");
    std::string arm = argv[1];
    require(arm == "u" || arm == "v" || arm == "w" || arm == "x",
            "invalid arm");
    unsigned instance = std::stoul(argv[2]), round = std::stoul(argv[3]);
    require(instance < 3 && round < 10, "invalid cell");
    U sizes[] = {16, 64, 256}, counts[] = {257, 65537, 262145};
    U size = sizes[instance], n = counts[instance], seed = 111 * (instance + 1);
    auto cold_start = Clock::now();
    Curve c = curve(2305843009213693951ULL, 0, 7);
    std::vector<Point> ps;
    for (U x = seed; ps.size() < 4 * size + 2; ++x) {
      Point q;
      if (c.lift(x, 0, q))
        ps.push_back(q);
    }
    std::vector<std::vector<Point>> bases(4);
    for (unsigned j = 0; j < 4; ++j)
      bases[j].assign(ps.begin() + j * size, ps.begin() + (j + 1) * size);
    std::vector<Point> targets;
    for (U k : {U(0), size - 1}) {
      Jac sum;
      for (unsigned j = 0; j < 4; ++j)
        sum = c.mixed(sum, bases[j][k]);
      targets.push_back(c.affine(sum));
    }
    targets.push_back(ps[4 * size]);
    targets.push_back(ps[4 * size + 1]);
    Plan p = plan(c, bases, U(1) << 20);
    DeviceBackend device(c, U(1) << 20, 4096);
    auto prepare_start = Clock::now();
    device.prepare(p);
    auto prepared = Clock::now();
    std::vector<Result> answers;
    for (Point target : targets)
      answers.push_back(device.find(target, p.left + p.right));
    auto finished = Clock::now();
    // Independent host oracle is validation overhead, outside timed intervals.
    HostBackend host(c, U(1) << 20, 4096);
    host.prepare(p);
    U digest = 0;
    for (size_t i = 0; i < targets.size(); ++i) {
      Result ref = host.find(targets[i], p.left + p.right);
      require(ref.found == answers[i].found &&
                  ref.indices == answers[i].indices,
              "matched host witness mismatch");
      require(answers[i].found == (i < 2),
              "positive/negative control mismatch");
      digest = digest * 1099511628211ULL ^ answers[i].found;
      for (U index : answers[i].indices)
        digest = digest * 1099511628211ULL ^ index;
    }
    U norm_digest = 0;
    double norm_ms = normalization(c, n, seed, norm_digest, true);
    std::cout << std::setprecision(12) << arm << ',' << instance << ',' << seed
              << ',' << round << ',' << (round == 0 ? "true" : "false")
              << ",passed,";
    U batches = 0, hits = 0;
    for (const Result &answer : answers) {
      batches += (answer.visited + 4095) / 4096;
      hits += answer.found;
      digest = digest * 1099511628211ULL ^ answer.visited;
    }
#ifdef ER_QUERY_REFERENCE
    std::cout << 16 * batches + 8 * hits;
#elif defined(ER_QUERY_PACKED_ONLY)
    std::cout << 16 * batches;
#else
    std::cout << 8 * batches;
#endif
    std::cout << ',' << norm_ms << ',' << ms(prepare_start, prepared) << ','
              << ms(prepared, finished) << ',' << ms(cold_start, finished)
              << ',' << digest << ',' << norm_digest << ','
              << ms(cold_start, finished) / 1000 << ",L0\n";
    return 0;
  } catch (const std::exception &e) {
    std::cerr << e.what() << '\n';
    return 1;
  }
}
