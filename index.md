---
theme: default
paginate: true
---

## Ruby Native Extension

---

# whoami
Akifumi Tomiyama
Studyplus Inc,
server-side enginner

- github: @atomiyama
- Twitter: @atomiyama1216

---

# 好きな言語はなんですか?

---

# Rust & Ruby

---

## RubyからRustを呼んでみたい

---

## Rubyから他の言語を呼び出す方法
- FFI (Foreign Function Interface)
- Native Extension

---

## FFI (foreign function interface)
![](ruby-ffi.png)

色々な言語で定義された関数とかを含むダイナミックライブラリを読み込んでRubyから呼べるようにしてくれる．

--- 

### Rustで適当に関数を定義
```rust
// hello_world.rs
#[no_mangle]
pub extern fn hello_world() {
    println!("Hello World, I am Rust!");
}
```

### コンパイルしてdylibファイル生成
```sh
$ rustc --crate-type="dylib" hello_world.rs
$ nm libhello_world.dylib | grep hello_world
0000000000000f10 T _hello_world
0000000000090b60 S _rust_metadata_hello_world_8787f43e282added376259c1adb08b80
```

---

## dylib, ffiでrubyから関数を呼び出す

```ruby
require 'ffi'

module RustEx
  extend FFI::Library
  ffi_lib "libhello_world.dylib"

  attach_function :hello_world, [], :void
end

pp RustEx::hello_world #=> "Hello World, I am Rust!"
```

---

# Native Extension

---
