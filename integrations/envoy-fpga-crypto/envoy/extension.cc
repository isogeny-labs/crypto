#include "fpga/boringssl.h"
#include "fpga/mailbox.h"

#include "envoy/event/dispatcher.h"
#include "envoy/http/filter.h"
#include "envoy/http/header_map.h"
#include "envoy/registry/registry.h"
#include "envoy/server/filter_config.h"
#include "envoy/ssl/private_key/private_key.h"
#include "envoy/ssl/private_key/private_key_config.h"

#include "envoy/common/exception.h"
#include "source/common/protobuf/protobuf.h"
#include "source/extensions/fpga_crypto/config.pb.h"

#include "openssl/rand.h"

#include <cmath>
#include <map>
#include <set>

#ifndef OPENSSL_IS_BORINGSSL
#error "The FPGA TLS record bridge requires the pinned BoringSSL build"
#endif

namespace Envoy::Extensions::FpgaCrypto {
namespace {
using Struct = Protobuf::Struct;
using Value = Protobuf::Value;
const Value *field(const Struct &config, const std::string &name) {
  auto it = config.fields().find(name);
  return it == config.fields().end() ? nullptr : &it->second;
}
void allowed(const Struct &config, const std::set<std::string> &names) {
  for (const auto &item : config.fields())
    if (!names.count(item.first))
      throw EnvoyException("unknown FPGA configuration field: " + item.first);
}
uint32_t number(const Struct &config, const std::string &name, uint32_t fallback, uint32_t low,
                uint32_t high) {
  const auto *v = field(config, name);
  if (!v)
    return fallback;
  if (v->kind_case() != Value::kNumberValue || !std::isfinite(v->number_value()) ||
      v->number_value() < low || v->number_value() > high ||
      std::floor(v->number_value()) != v->number_value())
    throw EnvoyException("invalid FPGA numeric field: " + name);
  return static_cast<uint32_t>(v->number_value());
}
std::string string(const Struct &config, const std::string &name, const std::string &fallback) {
  const auto *v = field(config, name);
  if (!v)
    return fallback;
  if (v->kind_case() != Value::kStringValue || v->string_value().empty())
    throw EnvoyException("invalid FPGA string field: " + name);
  return v->string_value();
}
bool boolean(const Struct &config, const std::string &name) {
  const auto *v = field(config, name);
  if (!v)
    return false;
  if (v->kind_case() != Value::kBoolValue)
    throw EnvoyException("invalid FPGA boolean field: " + name);
  return v->bool_value();
}
fpga::AwsOptions options(const Struct &config) {
  fpga::AwsOptions result;
  result.slot = number(config, "slot_id", 0, 0, 7);
  result.library = string(config, "sdk_library", result.library);
  result.lock_directory = string(config, "lock_directory", result.lock_directory);
  result.queue_capacity = number(config, "queue_capacity", 128, 1, 65536);
  return result;
}
std::chrono::milliseconds timeout(const Struct &config) {
  return std::chrono::milliseconds(number(config, "timeout_ms", 20, 1, 60000));
}

class Connection {
public:
  Connection(SSL *ssl, Ssl::PrivateKeyConnectionCallbacks &callbacks, Event::Dispatcher &dispatcher,
             std::shared_ptr<fpga::Engine> engine, uint32_t key_slot,
             std::chrono::milliseconds limit)
      : callbacks_(callbacks) {
    timer_ = dispatcher.createTimer([this] {
      if (session_->ready()) {
        callbacks_.onPrivateKeyMethodComplete();
        return;
      }
      if (session_->pending())
        timer_->enableTimer(std::chrono::milliseconds(1));
    });
    session_ =
        std::make_unique<fpga::PrivateKeySession>(ssl, std::move(engine), key_slot, limit, [this] {
          timer_->enableTimer(std::chrono::milliseconds(1));
        });
  }
  ~Connection() {
    timer_->disableTimer();
    session_.reset();
  }

private:
  Ssl::PrivateKeyConnectionCallbacks &callbacks_;
  Event::TimerPtr timer_;
  std::unique_ptr<fpga::PrivateKeySession> session_;
};

class Provider final : public Ssl::PrivateKeyMethodProvider {
public:
  explicit Provider(const Struct &config) {
    allowed(config, {"slot_id", "key_slot", "sdk_library", "lock_directory", "queue_capacity",
                     "timeout_ms", "offload_tls_records"});
    key_slot_ = number(config, "key_slot", 0, 1, UINT32_MAX);
    if (!key_slot_)
      throw EnvoyException("FPGA key_slot must identify a provisioned signing key");
    timeout_ = timeout(config);
    engine_ = fpga::awsEngine(options(config));
    if (boolean(config, "offload_tls_records"))
      fpga::enableRecordOffload(engine_, timeout_);
    method_ = std::make_shared<SSL_PRIVATE_KEY_METHOD>(*fpga::PrivateKeySession::method());
  }
  void registerPrivateKeyMethod(SSL *ssl, Ssl::PrivateKeyConnectionCallbacks &callbacks,
                                Event::Dispatcher &dispatcher) override {
    std::lock_guard<std::mutex> lock(mutex_);
    if (connections_.count(ssl))
      throw EnvoyException("duplicate FPGA SSL registration");
    connections_[ssl] =
        std::make_unique<Connection>(ssl, callbacks, dispatcher, engine_, key_slot_, timeout_);
  }
  void unregisterPrivateKeyMethod(SSL *ssl) override {
    std::lock_guard<std::mutex> lock(mutex_);
    connections_.erase(ssl);
  }
  bool checkFips() override { return false; }
  bool isAvailable() override { return engine_->healthy(); }
  Ssl::BoringSslPrivateKeyMethodSharedPtr getBoringSslPrivateKeyMethod() override {
    return method_;
  }

private:
  std::shared_ptr<fpga::Engine> engine_;
  uint32_t key_slot_;
  std::chrono::milliseconds timeout_{20};
  Ssl::BoringSslPrivateKeyMethodSharedPtr method_;
  std::mutex mutex_;
  std::map<SSL *, std::unique_ptr<Connection>> connections_;
};
class ProviderFactory final : public Ssl::PrivateKeyMethodProviderInstanceFactory {
public:
  std::string name() const override { return "fpga"; }
  Ssl::PrivateKeyMethodProviderSharedPtr createPrivateKeyMethodProviderInstance(
      const envoy::extensions::transport_sockets::tls::v3::PrivateKeyProvider &config,
      Server::Configuration::TransportSocketFactoryContext &) override {
    Struct settings;
    if (!config.typed_config().UnpackTo(&settings))
      throw EnvoyException("FPGA provider expects google.protobuf.Struct");
    try {
      return std::make_shared<Provider>(settings);
    } catch (const std::exception &error) {
      throw EnvoyException(error.what());
    }
  }
};

struct FilterConfig {
  struct Route {
    fpga::Operation operation;
    uint32_t key_slot;
  };
  std::shared_ptr<fpga::Engine> engine;
  std::chrono::milliseconds timeout;
  std::string prefix;
  std::map<std::string, Route> routes;
  explicit FilterConfig(const Struct &config) : timeout(FpgaCrypto::timeout(config)) {
    allowed(config, {"slot_id", "sdk_library", "lock_directory", "queue_capacity", "timeout_ms",
                     "path_prefix", "key_slots"});
    prefix = string(config, "path_prefix", "/_fpga/");
    if (prefix.front() != '/' || prefix.back() != '/' ||
        prefix.find_first_of("?#") != std::string::npos)
      throw EnvoyException("FPGA path_prefix must start and end with /");
    routes[prefix + "sha256"] = {fpga::Operation::Sha256, 0};
    if (const auto *keys = field(config, "key_slots")) {
      if (keys->kind_case() != Value::kStructValue)
        throw EnvoyException("FPGA key_slots must be an object");
      const auto &slots = keys->struct_value();
      allowed(slots, {"aes128", "aes256", "ecdsa", "rsa"});
      auto add = [&](const std::string &name, const std::string &path, fpga::Operation operation) {
        if (field(slots, name))
          routes[prefix + path] = {operation, number(slots, name, 0, 1, UINT32_MAX)};
      };
      add("aes128", "aes128/seal", fpga::Operation::Aes128Seal);
      add("aes128", "aes128/open", fpga::Operation::Aes128Open);
      add("aes256", "aes256/seal", fpga::Operation::Aes256Seal);
      add("aes256", "aes256/open", fpga::Operation::Aes256Open);
      add("ecdsa", "sign/ecdsa", fpga::Operation::EcdsaP256Sha256);
      add("rsa", "sign/rsa-pss", fpga::Operation::RsaPssSha256);
      add("rsa", "sign/rsa-pkcs1", fpga::Operation::RsaPkcs1Sha256);
    }
    engine = fpga::awsEngine(options(config));
  }
};
class Filter final : public Http::StreamDecoderFilter {
public:
  explicit Filter(std::shared_ptr<const FilterConfig> config) : config_(std::move(config)) {}
  void setDecoderFilterCallbacks(Http::StreamDecoderFilterCallbacks &callbacks) override {
    callbacks_ = &callbacks;
  }
  void onDestroy() override {
    destroyed_ = true;
    ticket_.cancel();
    timer_.reset();
  }
  Http::FilterHeadersStatus decodeHeaders(Http::RequestHeaderMap &headers,
                                          bool end_stream) override {
    const std::string path(headers.getPathValue());
    if (path.compare(0, config_->prefix.size(), config_->prefix) != 0)
      return Http::FilterHeadersStatus::Continue;
    active_ = true;
    if (headers.getMethodValue() != "POST") {
      fail(Http::Code::MethodNotAllowed, "POST required");
      return Http::FilterHeadersStatus::StopIteration;
    }
    auto route = config_->routes.find(path);
    if (route == config_->routes.end()) {
      fail(Http::Code::NotFound, "unknown crypto operation");
      return Http::FilterHeadersStatus::StopIteration;
    }
    request_.operation = route->second.operation;
    request_.key_slot = route->second.key_slot;
    if (fpga::isAes(request_.operation)) {
      auto readHex = [&](const char *name) -> std::optional<fpga::Bytes> {
        auto values = headers.get(Http::LowerCaseString(name));
        if (values.empty())
          return fpga::Bytes{};
        if (values.size() != 1)
          return std::nullopt;
        return fpga::unhex(std::string(values[0]->value().getStringView()));
      };
      auto aad = readHex("x-fpga-aad");
      if (!aad || aad->size() > fpga::MaxAad) {
        fail(Http::Code::BadRequest, "invalid AAD");
        return Http::FilterHeadersStatus::StopIteration;
      }
      request_.aad = std::move(*aad);
      if (fpga::isSeal(request_.operation)) {
        request_.nonce.resize(12);
        if (!RAND_bytes(request_.nonce.data(), request_.nonce.size())) {
          fail(Http::Code::ServiceUnavailable, "nonce generation failed");
          return Http::FilterHeadersStatus::StopIteration;
        }
        response_nonce_ = fpga::hex(request_.nonce);
      } else {
        auto nonce = readHex("x-fpga-nonce");
        if (!nonce || nonce->size() != 12) {
          fail(Http::Code::BadRequest, "96-bit nonce required");
          return Http::FilterHeadersStatus::StopIteration;
        }
        request_.nonce = std::move(*nonce);
      }
    }
    if (end_stream)
      start();
    return Http::FilterHeadersStatus::StopIteration;
  }
  Http::FilterDataStatus decodeData(Buffer::Instance &data, bool end_stream) override {
    if (!active_)
      return Http::FilterDataStatus::Continue;
    if (!submitted_ && !replied_) {
      const size_t limit = fpga::MaxData + (fpga::isOpen(request_.operation) ? 16 : 0);
      if (data.length() > limit - request_.data.size()) {
        data.drain(data.length());
        fail(Http::Code::PayloadTooLarge, "crypto payload exceeds operation limit");
        return Http::FilterDataStatus::StopIterationNoBuffer;
      } else {
        const size_t previous = request_.data.size();
        request_.data.resize(previous + data.length());
        if (data.length())
          data.copyOut(0, data.length(), request_.data.data() + previous);
      }
    }
    data.drain(data.length());
    if (end_stream && !replied_)
      start();
    return Http::FilterDataStatus::StopIterationNoBuffer;
  }
  Http::FilterTrailersStatus decodeTrailers(Http::RequestTrailerMap &) override {
    if (!active_)
      return Http::FilterTrailersStatus::Continue;
    if (!submitted_ && !replied_)
      start();
    return Http::FilterTrailersStatus::StopIteration;
  }

private:
  void fail(Http::Code code, const std::string &message) {
    replied_ = true;
    callbacks_->sendLocalReply(code, message, nullptr, absl::nullopt, "fpga_crypto_error");
  }
  void start() {
    if (submitted_ || replied_)
      return;
    submitted_ = true;
    if (!fpga::valid(request_)) {
      fail(Http::Code::BadRequest, "invalid crypto request");
      return;
    }
    ticket_ = config_->engine->submit(std::move(request_), config_->timeout);
    timer_ = callbacks_->dispatcher().createTimer([this] { poll(); });
    timer_->enableTimer(std::chrono::milliseconds(1));
  }
  void poll() {
    if (destroyed_)
      return;
    auto result = ticket_.take();
    if (!result) {
      timer_->enableTimer(std::chrono::milliseconds(1));
      return;
    }
    if (!*result) {
      const auto code = result->code == fpga::Code::AuthFailed ? Http::Code::BadRequest
                        : result->code == fpga::Code::Timeout  ? Http::Code::GatewayTimeout
                                                               : Http::Code::ServiceUnavailable;
      fail(code, fpga::codeName(result->code));
      return;
    }
    replied_ = true;
    const std::string body(result->data.begin(), result->data.end());
    fpga::wipe(result->data);
    // sendLocalReply may destroy this filter; capture only values in header callback.
    const std::string nonce = response_nonce_;
    callbacks_->sendLocalReply(
        Http::Code::OK, body,
        [nonce](Http::ResponseHeaderMap &headers) {
          headers.setContentType("application/octet-stream");
          headers.setCopy(Http::LowerCaseString("cache-control"), "no-store");
          if (!nonce.empty())
            headers.setCopy(Http::LowerCaseString("x-fpga-nonce"), nonce);
        },
        absl::nullopt, "fpga_crypto_success");
  }
  std::shared_ptr<const FilterConfig> config_;
  Http::StreamDecoderFilterCallbacks *callbacks_{nullptr};
  fpga::Request request_;
  fpga::Ticket ticket_;
  Event::TimerPtr timer_;
  std::string response_nonce_;
  bool active_{false}, submitted_{false}, replied_{false}, destroyed_{false};
};
class FilterFactory final : public Server::Configuration::NamedHttpFilterConfigFactory {
public:
  std::string name() const override { return "envoy.filters.http.fpga_crypto"; }
  ProtobufTypes::MessagePtr createEmptyConfigProto() override {
    return std::make_unique<envoy::extensions::fpga_crypto::v3::FpgaHttp>();
  }
  ProtobufTypes::MessagePtr createEmptyRouteConfigProto() override { return nullptr; }
  absl::StatusOr<Http::FilterFactoryCb>
  createFilterFactoryFromProto(const Protobuf::Message &config, const std::string &,
                               Server::Configuration::FactoryContext &) override {
    return create(config);
  }
  absl::StatusOr<Http::FilterFactoryCb>
  createHttpFilterFactoryFromProto(const Protobuf::Message &config, const std::string &,
                                   Server::Configuration::ServerFactoryContext &) override {
    return create(config);
  }

private:
  static Http::FilterFactoryCb create(const Protobuf::Message &config) {
    const auto *typed =
        Protobuf::DynamicCastMessage<envoy::extensions::fpga_crypto::v3::FpgaHttp>(&config);
    if (!typed || !typed->has_value())
      throw EnvoyException("FPGA filter requires FpgaHttp.value");
    std::shared_ptr<FilterConfig> shared;
    try {
      shared = std::make_shared<FilterConfig>(typed->value());
    } catch (const std::exception &error) {
      throw EnvoyException(error.what());
    }
    return [shared](Http::FilterChainFactoryCallbacks &callbacks) {
      callbacks.addStreamDecoderFilter(std::make_shared<Filter>(shared));
    };
  }
};
} // namespace
REGISTER_FACTORY(ProviderFactory, Ssl::PrivateKeyMethodProviderInstanceFactory);
REGISTER_FACTORY(FilterFactory, Server::Configuration::NamedHttpFilterConfigFactory);
} // namespace Envoy::Extensions::FpgaCrypto
