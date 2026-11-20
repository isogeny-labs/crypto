#include <m4ri/m4ri.h>
#include <chrono>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <stdexcept>
#include <string>
#include <sys/resource.h>
#include <vector>

using Clock = std::chrono::steady_clock;
static uint32_t u32(std::istream& in) {
  unsigned char b[4];
  if (!in.read(reinterpret_cast<char*>(b), 4)) throw std::runtime_error("short header");
  return uint32_t(b[0]) | uint32_t(b[1]) << 8 | uint32_t(b[2]) << 16 | uint32_t(b[3]) << 24;
}
static uint16_t u16(std::istream& in) {
  unsigned char b[2];
  if (!in.read(reinterpret_cast<char*>(b), 2)) throw std::runtime_error("short row");
  return uint16_t(b[0]) | uint16_t(b[1]) << 8;
}
static double seconds(Clock::time_point a, Clock::time_point b) {
  return std::chrono::duration<double>(b-a).count();
}

int main(int argc, char** argv) {
  if (argc != 3) return 2;
  std::ifstream columns_in(argv[2]);
  std::vector<int> selected;
  int value;
  while (columns_in >> value) selected.push_back(value);
  if (selected.size() != 15856) return 3;
  std::vector<int> base_index(18352, -1), omit_index(18352, -1), omitted;
  for (size_t i=0; i<selected.size(); ++i) {
    if (selected[i] < 0 || selected[i] >= 18352 || (i && selected[i] <= selected[i-1])) return 4;
    base_index[selected[i]] = int(i);
  }
  for (int j=0; j<18352; ++j) if (base_index[j] < 0) {
    omit_index[j] = int(omitted.size());
    omitted.push_back(j);
  }
  std::ifstream in(argv[1], std::ios::binary);
  char magic[8];
  if (!in.read(magic, 8) || std::string(magic, 8) != std::string("G2M4R1\0\0", 8) ||
      u32(in)!=16399 || u32(in)!=18352 || u32(in)!=32 || u32(in)!=4) return 5;
  auto t0 = Clock::now();
  mzd_t* A = mzd_init(16399, 15856);
  mzd_t* B = mzd_init(16399, omitted.size());
  for (int row=0; row<16399; ++row) {
    int count=u16(in), last=-1;
    for (int k=0; k<count; ++k) {
      int col=u16(in);
      if (col<=last || col>=18352) return 6;
      last=col;
      if (base_index[col]>=0) mzd_write_bit(A,row,base_index[col],1);
      else mzd_write_bit(B,row,omit_index[col],1);
    }
  }
  if (in.get()!=std::char_traits<char>::eof()) return 7;
  auto t1 = Clock::now();
  mzd_t* AT = mzd_transpose(nullptr,A);
  mzd_free(A);
  mzd_t* K = mzd_kernel_left_pluq(AT,0);
  mzd_free(AT);
  if (!K || K->nrows != 16399 || K->ncols != 564) return 8;
  auto t2 = Clock::now();
  mzd_t* KT = mzd_transpose(nullptr,K);
  mzd_free(K);
  mzd_t* C = mzd_mul(nullptr,KT,B,0);
  mzd_free(KT); mzd_free(B);
  auto t3 = Clock::now();
  int rank = mzd_echelonize_m4ri(C,0,5);
  if (rank != 6) return 9;
  std::vector<int> pivots;
  for (int row=0; row<rank; ++row) {
    int pivot=-1;
    for (int col=0; col<int(omitted.size()); ++col)
      if (mzd_read_bit(C,row,col)) {pivot=col;break;}
    if (pivot<0 || (!pivots.empty() && omitted[pivot]<=pivots.back())) {
      std::cerr << "pivot_order_error row=" << row << " pivot=" << pivot
                << " previous=" << (pivots.empty() ? -1 : pivots.back()) << '\n';
      return 10;
    }
    pivots.push_back(omitted[pivot]);
  }
  mzd_free(C);
  auto t4 = Clock::now();
  struct rusage usage {};
  if (getrusage(RUSAGE_SELF, &usage) != 0) return 11;
  std::cout << "{\"status\":\"PASS\",\"base_rank\":15835,\"kernel_dim\":564,"
            << "\"quotient_rank\":" << rank << ",\"selected_columns\":[";
  for (size_t i=0;i<pivots.size();++i) {if(i)std::cout<<',';std::cout<<pivots[i];}
  std::cout << "],\"load_seconds_exploratory\":" << seconds(t0,t1)
            << ",\"kernel_seconds_exploratory\":" << seconds(t1,t2)
            << ",\"multiply_seconds_exploratory\":" << seconds(t2,t3)
            << ",\"quotient_seconds_exploratory\":" << seconds(t3,t4)
            << ",\"peak_rss_bytes_process\":" << usage.ru_maxrss << "}\n";
}
