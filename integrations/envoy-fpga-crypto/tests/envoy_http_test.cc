// Live native Envoy tests. Run from the extension root with the pinned BoringSSL.
#include "fpga/pq.h"
#include "test.h"

#include <openssl/bytestring.h>
#include <openssl/evp.h>
#include <openssl/mem.h>
#include <openssl/mlkem.h>

#include <arpa/inet.h>
#include <fcntl.h>
#include <signal.h>
#include <sys/socket.h>
#include <sys/wait.h>
#include <unistd.h>

#include <array>
#include <cctype>
#include <cerrno>
#include <filesystem>
#include <fstream>
#include <iterator>
#include <map>
#include <regex>
#include <sstream>

using namespace fpga;
using namespace std::chrono_literals;
namespace {
std::string envoyBinary, pluginLibrary, runRoot;

struct Fd {
  int value;
  explicit Fd(int v) : value(v) { CHECK(value >= 0); }
  ~Fd() { close(value); }
  Fd(const Fd &) = delete;
  Fd &operator=(const Fd &) = delete;
};
struct Child {
  pid_t pid = -1;
  ~Child() {
    if (pid <= 0)
      return;
    kill(pid, SIGTERM);
    const auto end = Clock::now() + 3s;
    int status = 0;
    while (Clock::now() < end) {
      const auto waited = waitpid(pid, &status, WNOHANG);
      if (waited == pid || (waited < 0 && errno == ECHILD))
        return;
      std::this_thread::sleep_for(20ms);
    }
    kill(pid, SIGKILL);
    while (waitpid(pid, &status, 0) < 0 && errno == EINTR) {
    }
  }
  bool running() {
    int status = 0;
    const auto waited = waitpid(pid, &status, WNOHANG);
    if (waited == 0)
      return true;
    if (waited == pid)
      pid = -1;
    return false;
  }
};
struct Response {
  unsigned status = 0;
  std::map<std::string, std::string> headers;
  Bytes body;
};
std::string string(const Bytes &bytes) { return {bytes.begin(), bytes.end()}; }
std::string readFile(const std::string &path) {
  std::ifstream file(path);
  return {std::istreambuf_iterator<char>(file), std::istreambuf_iterator<char>()};
}
uint16_t port(const std::string &text) {
  size_t consumed = 0;
  const auto n = std::stoul(text, &consumed);
  CHECK(consumed == text.size() && n > 0 && n <= 65535);
  return static_cast<uint16_t>(n);
}
void replaceOnce(std::string &text, const std::string &from, const std::string &to) {
  const auto offset = text.find(from);
  CHECK(offset != std::string::npos && text.find(from, offset + from.size()) == std::string::npos);
  text.replace(offset, from.size(), to);
}
std::string yamlString(const std::string &text) {
  std::string result = "'";
  for (const unsigned char c : text) {
    CHECK(c >= 32 && c != 127);
    if (c == '\'')
      result += "''";
    else
      result += static_cast<char>(c);
  }
  return result + "'";
}

Response http(uint16_t listen_port, const std::string &method, const std::string &path,
              const Bytes &body = {}, const std::string &extra_headers = "") {
  Fd socket(::socket(AF_INET, SOCK_STREAM | SOCK_CLOEXEC, 0));
  const timeval timeout{10, 0};
  CHECK(setsockopt(socket.value, SOL_SOCKET, SO_RCVTIMEO, &timeout, sizeof(timeout)) == 0);
  CHECK(setsockopt(socket.value, SOL_SOCKET, SO_SNDTIMEO, &timeout, sizeof(timeout)) == 0);
  sockaddr_in address{};
  address.sin_family = AF_INET;
  address.sin_port = htons(listen_port);
  address.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
  CHECK(connect(socket.value, reinterpret_cast<sockaddr *>(&address), sizeof(address)) == 0);
  const auto header = method + " " + path +
                      " HTTP/1.1\r\nHost: localhost\r\n"
                      "Connection: close\r\nContent-Length: " +
                      std::to_string(body.size()) + "\r\n" + extra_headers + "\r\n";
  Bytes request(header.begin(), header.end());
  request.insert(request.end(), body.begin(), body.end());
  size_t sent = 0;
  while (sent < request.size()) {
    const auto n = send(socket.value, request.data() + sent, request.size() - sent, MSG_NOSIGNAL);
    if (n < 0 && errno == EINTR)
      continue;
    CHECK(n > 0);
    sent += static_cast<size_t>(n);
  }
  Bytes raw;
  std::array<uint8_t, 16384> buffer{};
  for (;;) {
    const auto n = recv(socket.value, buffer.data(), buffer.size(), 0);
    if (n < 0 && errno == EINTR)
      continue;
    CHECK(n >= 0);
    if (n == 0)
      break;
    CHECK(raw.size() + static_cast<size_t>(n) <= MaxPqWire + 65536);
    raw.insert(raw.end(), buffer.begin(), buffer.begin() + n);
  }
  const std::string wire(raw.begin(), raw.end());
  const auto end = wire.find("\r\n\r\n");
  CHECK(end != std::string::npos);
  std::istringstream lines(wire.substr(0, end));
  std::string line, version;
  Response response;
  CHECK(static_cast<bool>(std::getline(lines, line)));
  std::istringstream first(line);
  CHECK(static_cast<bool>(first >> version >> response.status));
  CHECK(version == "HTTP/1.1");
  while (std::getline(lines, line)) {
    if (!line.empty() && line.back() == '\r')
      line.pop_back();
    const auto colon = line.find(':');
    CHECK(colon != std::string::npos);
    auto key = line.substr(0, colon), value = line.substr(colon + 1);
    for (auto &c : key)
      c = static_cast<char>(std::tolower(static_cast<unsigned char>(c)));
    const auto begin = value.find_first_not_of(" \t");
    value = begin == std::string::npos ? "" : value.substr(begin);
    CHECK(response.headers.emplace(key, value).second);
  }
  const auto payload = wire.substr(end + 4);
  const auto transfer = response.headers.find("transfer-encoding");
  if (transfer != response.headers.end()) {
    CHECK(transfer->second == "chunked");
    size_t position = 0;
    for (;;) {
      const auto line_end = payload.find("\r\n", position);
      CHECK(line_end != std::string::npos);
      const auto token = payload.substr(position, line_end - position);
      size_t consumed = 0;
      const auto chunk = std::stoull(token, &consumed, 16);
      CHECK(consumed == token.size());
      position = line_end + 2;
      CHECK(chunk <= payload.size() - position);
      if (chunk == 0) {
        CHECK(payload.substr(position) == "\r\n");
        break;
      }
      response.body.insert(response.body.end(), payload.begin() + static_cast<ptrdiff_t>(position),
                           payload.begin() + static_cast<ptrdiff_t>(position + chunk));
      position += chunk;
      CHECK(payload.substr(position, 2) == "\r\n");
      position += 2;
    }
  } else
    response.body.assign(payload.begin(), payload.end());
  const auto length = response.headers.find("content-length");
  if (length != response.headers.end())
    CHECK(std::stoull(length->second) == response.body.size());
  return response;
}

class EnvoyProcess {
public:
  EnvoyProcess() {
    std::string pattern = runRoot + "/run-XXXXXX";
    std::vector<char> name(pattern.begin(), pattern.end());
    name.push_back('\0');
    CHECK(mkdtemp(name.data()) != nullptr);
    directory_ = name.data();
    std::cout << "  Envoy evidence: " << directory_ << '\n';
    const auto config_path = directory_ + "/envoy.yaml";
    const auto address_path = directory_ + "/admin-address.txt";
    auto config = readFile("examples/pq-software.yaml");
    replaceOnce(config, "/opt/envoy/lib/libfpga_pq_oqs.so", yamlString(pluginLibrary));
    replaceOnce(config, "port_value: 8088", "port_value: 0");
    config += "\nadmin:\n  address:\n    socket_address: {address: 127.0.0.1, port_value: 0}\n";
    {
      std::ofstream output(config_path);
      output << config;
      CHECK(output.good());
    }
    Fd log(open((directory_ + "/envoy.log").c_str(), O_WRONLY | O_CREAT | O_EXCL, 0600));
    child_.pid = fork();
    CHECK(child_.pid >= 0);
    if (child_.pid == 0) {
      if (dup2(log.value, STDOUT_FILENO) < 0 || dup2(log.value, STDERR_FILENO) < 0)
        _exit(126);
      close(log.value);
      execl(envoyBinary.c_str(), envoyBinary.c_str(), "--disable-hot-restart", "--concurrency", "1",
            "--log-level", "error", "--admin-address-path", address_path.c_str(), "-c",
            config_path.c_str(), static_cast<char *>(nullptr));
      _exit(127);
    }
    const auto deadline = Clock::now() + 30s;
    std::string last_error;
    while (Clock::now() < deadline) {
      if (!child_.running())
        break;
      try {
        const auto address = readFile(address_path);
        if (address.rfind("127.0.0.1:", 0) == 0) {
          const auto admin_port = port(address.substr(10));
          if (http(admin_port, "GET", "/ready").status == 200) {
            const auto listeners = http(admin_port, "GET", "/listeners");
            std::smatch match;
            const auto text = string(listeners.body);
            if (std::regex_search(text, match,
                                  std::regex("pq_software_demo::127\\.0\\.0\\.1:([0-9]+)"))) {
              port_ = port(match[1]);
              return;
            }
          }
        }
      } catch (const std::exception &e) {
        last_error = e.what();
      }
      std::this_thread::sleep_for(50ms);
    }
    throw std::runtime_error("Envoy startup failed: " + directory_ + "/envoy.log; " + last_error);
  }
  Response post(const std::string &path, const Bytes &body = {}, const std::string &headers = "") {
    return http(port_, "POST", "/_pq/" + path, body, headers);
  }
  Response request(const std::string &method, const std::string &path) {
    return http(port_, method, path);
  }

private:
  Child child_;
  std::string directory_;
  uint16_t port_ = 0;
};

void success(const Response &response, const char *content_type = "application/octet-stream") {
  CHECK(response.status == 200);
  const auto type = response.headers.find("content-type");
  // Envoy prepareLocalReply removes content-type for a zero-length body.
  if (response.body.empty())
    CHECK(type == response.headers.end());
  else
    CHECK(type != response.headers.end() && type->second == content_type);
  const auto cache = response.headers.find("cache-control");
  const auto backend = response.headers.find("x-crypto-backend");
  CHECK(cache != response.headers.end() && cache->second == "no-store");
  CHECK(backend != response.headers.end() && backend->second == "software-liboqs");
}
} // namespace

TEST(live_envoy_catalog_and_route_boundaries) {
  EnvoyProcess server;
  const auto catalog = server.post("algorithms");
  success(catalog, "application/json");
  const auto json = string(catalog.body);
  CHECK(json.find("ML-DSA-65") != std::string::npos);
  CHECK(json.find("ML-KEM-768") != std::string::npos);
  CHECK(json.find("Falcon") == std::string::npos && json.find("FN-DSA") == std::string::npos);
  CHECK(server.request("GET", "/_pq/algorithms").status == 405);
  CHECK(server.post("missing").status == 404);
  CHECK(server.request("POST", "/outside-prefix").status == 404);
}

TEST(live_envoy_mldsa_signing_is_independently_verified) {
  EnvoyProcess server;
  const auto key = server.post("mldsa/keygen");
  success(key);
  const auto public_key = server.post("mldsa/public-key");
  success(public_key);
  CHECK(public_key.body == key.body);
  CHECK(server.post("mldsa/keygen").status == 400);
  const Bytes message{0, 1, 2, 0, 255, 128, 3};
  const auto signature = server.post("mldsa/sign", message);
  success(signature);
  bssl::UniquePtr<EVP_PKEY> pk(
      EVP_PKEY_new_raw_public_key(EVP_PKEY_ML_DSA_65, nullptr, key.body.data(), key.body.size()));
  CHECK(pk);
  bssl::ScopedEVP_MD_CTX verify;
  CHECK(EVP_DigestVerifyInit(verify.get(), nullptr, nullptr, nullptr, pk.get()));
  CHECK(EVP_DigestVerify(verify.get(), signature.body.data(), signature.body.size(), message.data(),
                         message.size()) == 1);
  const auto verified = server.post("mldsa/verify", packParts({key.body, signature.body, message}));
  success(verified);
  CHECK(verified.body.empty());
  auto tampered = message;
  tampered[0] ^= 1;
  CHECK(server.post("mldsa/verify", packParts({key.body, signature.body, tampered})).status == 400);
  success(server.post("mldsa/erase"));
  CHECK(server.post("mldsa/public-key").status == 400);
  CHECK(server.post("mldsa/sign", message).status == 400);
}

TEST(live_envoy_context_binding_and_large_binary_messages) {
  EnvoyProcess server;
  const auto key = server.post("mldsa/keygen");
  success(key);
  Bytes message(256 * 1024);
  for (size_t i = 0; i < message.size(); ++i)
    message[i] = static_cast<uint8_t>(i);
  const auto signature = server.post("mldsa/sign", message, "x-pq-context: 010203\r\n");
  success(signature);
  const auto framed = packParts({key.body, signature.body, message});
  success(server.post("mldsa/verify", framed, "x-pq-context: 010203\r\n"));
  CHECK(server.post("mldsa/verify", framed).status == 400);
  CHECK(server.post("mldsa/sign", message, "x-pq-context: gg\r\n").status == 400);
  CHECK(server.post("mldsa/sign", {}, "x-pq-context: 00\r\nx-pq-context: 01\r\n").status == 400);
}

TEST(live_envoy_mlkem_interoperates_with_boringssl_in_both_directions) {
  EnvoyProcess server;
  const auto key = server.post("mlkem/keygen");
  success(key);
  MLKEM768_public_key public_key;
  CBS cbs;
  CBS_init(&cbs, key.body.data(), key.body.size());
  CHECK(MLKEM768_parse_public_key(&public_key, &cbs) && CBS_len(&cbs) == 0);
  Bytes ciphertext(MLKEM768_CIPHERTEXT_BYTES), secret(MLKEM_SHARED_SECRET_BYTES);
  MLKEM768_encap(ciphertext.data(), secret.data(), &public_key);
  const auto decapsulated = server.post("mlkem/decapsulate", ciphertext);
  success(decapsulated);
  CHECK(decapsulated.body == secret);
  ciphertext[0] ^= 1;
  const auto rejected = server.post("mlkem/decapsulate", ciphertext);
  success(rejected); // ML-KEM implicit rejection still returns a pseudorandom secret.
  CHECK(rejected.body.size() == MLKEM_SHARED_SECRET_BYTES && rejected.body != secret);
  MLKEM768_private_key private_key;
  Bytes independent_key(MLKEM768_PUBLIC_KEY_BYTES);
  MLKEM768_generate_key(independent_key.data(), nullptr, &private_key);
  const auto encapsulated = server.post("mlkem/encapsulate", independent_key);
  success(encapsulated);
  const auto parts = unpackParts(encapsulated.body, 2);
  CHECK(parts);
  CHECK(MLKEM768_decap(secret.data(), (*parts)[0].data(), (*parts)[0].size(), &private_key));
  CHECK(secret == (*parts)[1]);
  OPENSSL_cleanse(&private_key, sizeof(private_key));
  wipe(secret);
  success(server.post("mlkem/erase"));
  CHECK(server.post("mlkem/decapsulate", ciphertext).status == 400);
}

TEST(live_envoy_malformed_requests_fail_without_disabling_listener) {
  EnvoyProcess server;
  CHECK(server.post("mldsa/verify", Bytes{0}).status == 400);
  CHECK(server.post("mlkem/encapsulate", Bytes{1, 2}).status == 400);
  CHECK(server.post("mlkem/decapsulate", Bytes{1, 2}).status == 400);
  CHECK(server.post("mldsa/keygen", Bytes{1}).status == 400);
  success(server.post("algorithms"), "application/json");
}

int main(int argc, char **argv) {
  if (argc != 4) {
    std::cerr << "usage: envoy_http_test /absolute/envoy /absolute/plugin.so /run-directory\n";
    return 2;
  }
  try {
    envoyBinary = std::filesystem::canonical(argv[1]);
    pluginLibrary = std::filesystem::canonical(argv[2]);
    runRoot = std::filesystem::absolute(argv[3]);
    std::filesystem::create_directories(runRoot);
    return tests::run();
  } catch (const std::exception &e) {
    std::cerr << e.what() << '\n';
    return 1;
  }
}
