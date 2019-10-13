require 'ffi'

module FFIEx
  extend FFI::Library
  ffi_lib './liblib.dylib'

  attach_function :process, [], :void
end
