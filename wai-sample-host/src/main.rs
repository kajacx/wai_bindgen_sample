use sample_protocol_plugin::SampleProtocolPlugin;
use wai_bindgen_wasmer::wasmer::*;

const PLUGIN_BYTES: &'static [u8] = include_bytes!(
    "../../wai-sample-plugin/target/wasm32-unknown-unknown/debug/wai_sample_plugin.wasm"
);

wai_bindgen_wasmer::import!("../sample-protocol-plugin.wai");
wai_bindgen_rust::export!("../sample-protocol-host.wai");

struct SampleProtocolHost;

impl sample_protocol_host::SampleProtocolHost for SampleProtocolHost {
    fn add_one(num: u32) -> u32 {
        num + 1
    }
}

fn main() {
    let compiler = ::wasmer::Cranelift::default();
    let engine = ::wasmer::Universal::new(compiler).engine();
    let mut store = Store::new(&engine); //wasmer and wai_bindgen_wasmer are in conflict?

    let module = Module::new(&store, PLUGIN_BYTES).expect("should create module");

    let mut imports = imports! {};

    let (sample, _instance) = SampleProtocolPlugin::instantiate(&mut store, &module, &mut imports)
        .expect("should create instance");

    println!("{:?}", sample.add_three(&mut store, 5));
}
