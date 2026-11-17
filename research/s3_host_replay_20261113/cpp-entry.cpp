// Entry selection only. The generated translation unit prefixes the unchanged
// original semaev.cpp functions, excluding its broader S4/S5 main entry point.
int main() {
  try {
    for (int p : {5, 7, 11, 13, 17, 19}) odd(p, false);
  } catch (const std::exception& e) {
    std::cerr << e.what() << '\n';
    return 1;
  }
  return 0;
}
