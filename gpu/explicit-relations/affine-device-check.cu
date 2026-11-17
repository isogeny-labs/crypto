#include "device.cuh"
#include "affine-fixtures.hpp"
#include <iostream>
using namespace explicit_relations;
__global__ void affine_check(Curve c, const AffineCase *cases, Point *out, U n) {
  U i = U(blockIdx.x) * BLOCK + threadIdx.x;
  Jac j;
  if (i < n) j = c.affine_sub(cases[i].target, cases[i].right);
  Point p = normalize_block(c, j);
  if (i < n) out[i] = p;
}
int main() {
  size_t count = 0;
  for (auto c : affine_curves()) {
    auto cases = affine_cases(c);
    thrust::device_vector<AffineCase> input(cases.begin(), cases.end());
    thrust::device_vector<Point> output(cases.size());
    affine_check<<<(cases.size()+BLOCK-1)/BLOCK,BLOCK>>>(c,data(input),data(output),cases.size());
    cuda_check(cudaGetLastError());
    std::vector<Point> got(cases.size());
    cuda_check(cudaMemcpy(got.data(),data(output),got.size()*sizeof(Point),cudaMemcpyDeviceToHost));
    for (size_t i=0;i<cases.size();++i)
      if (!equal(got[i],cases[i].expected))
        throw std::runtime_error("affine subtraction device differential failed");
    count += cases.size();
  }
  std::cout << "PASS affine subtraction device cases=" << count << '\n';
}
