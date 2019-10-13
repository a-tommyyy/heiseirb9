require "./ffi/main"
require "./purerb/main"
require "./ext/rustex.bundle"
require "benchmark/ips"

Benchmark.ips do |x|
  x.report "Ruby Func" do
    PureRuby.process
  end

  x.report "FFI Func" do
    FFIEx.process
  end

  x.report "Rust Func" do
    RustEx.process
  end
  x.compare!
end
