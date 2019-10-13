require 'ffi'

module FFIEx
  extend FFI::Library
  ffi_lib File.join(__dir__, 'lib.dylib')

  attach_function :process, [], :void
end
