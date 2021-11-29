require 'ffi'

module Example
  extend FFI::Library
  ffi_lib 'target/debug/libruby_ffi.dylib'

  attach_function :hello_rust, [], :string
  attach_function :make_big_data, [], :pointer
  attach_function :update_big_data, [ :pointer ], :void
  attach_function :how_much_data, [ :pointer ], :ushort
  attach_function :free_big_data, [ :pointer ], :void
end

str = Example.hello_rust
puts str

data = Example.make_big_data
Example.update_big_data(data)
Example.update_big_data(data)
Example.update_big_data(data)
puts "This much data: #{Example.how_much_data(data)}"
Example.free_big_data(data)