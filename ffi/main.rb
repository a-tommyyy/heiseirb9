require 'ffi'

module RustEx
  extend FFI::Library

  ffi_lib './liblib.dylib'

  attach_function :greet, [], :char
end
