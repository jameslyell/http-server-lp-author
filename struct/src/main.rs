#[derive(Debug)]
enum Lang {
  English,
  Spanish,
  Chinese,
  Texan,
  French,
  Afrikaans
}

struct Greeting {
    message: String,
    lang: Lang,
}

fn main() {
  let mut v :Vec<Greeting> = Vec::new();

  let g : Greeting = Greeting { lang: Lang::English, message: String::from("Hello WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Spanish, message: String::from("Hola WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Texan, message: String::from("Howdy WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Chinese, message: String::from("WasmEdge 你好!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::French, message: String::from("Bonjour WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Afrikaans, message: String::from("Hoesit WasmEdge!") };
  v.push(g);

  for e in v {
    println!("{:?} {}", e.lang, e.message);
  }
}

/*
OUTPUT:

 % wasmedge target/wasm32-wasi/release/struct.wasm
English Hello WasmEdge!
Spanish Hola WasmEdge!
Texan Howdy WasmEdge!
Chinese WasmEdge 你好!
French Bonjour WasmEdge!
Afrikaans Hoesit WasmEdge!
 */