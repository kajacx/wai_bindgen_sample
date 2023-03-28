use sample_protocol_plugin::{SampleProtocolPlugin, SampleProtocolPluginData};
use wai_bindgen_wasmtime::wasmtime::{Config, Engine, Linker, Module, Store};

const PLUGIN_BYTES: &'static [u8] = include_bytes!(
    "../../wai-sample-plugin/target/wasm32-unknown-unknown/debug/wai_sample_plugin.wasm"
);

wai_bindgen_wasmtime::import!("../sample-protocol-plugin.wai");
wai_bindgen_rust::export!("../sample-protocol-host.wai");

struct SampleProtocolHost;

impl sample_protocol_host::SampleProtocolHost for SampleProtocolHost {
    fn add_one(num: u32) -> u32 {
        num + 1
    }
}

type MyT = SampleProtocolPluginData;

fn main() {
    // let compiler = ::wasmer::Cranelift::default();
    // let engine = ::wasmer::Universal::new(compiler).engine();
    // let mut store = Store::new(unsafe { std::mem::transmute(&engine) }); //wasmer and wai_bindgen_wasmer are in conflict?

    // let module = Module::new(&store, PLUGIN_BYTES).expect("should create module");

    // let mut imports = imports! {};

    // let (sample, _instance) = SampleProtocolPlugin::instantiate(&mut store, &module, &mut imports)
    //     .expect("should create instance");

    // println!("{:?}", sample.add_three(&mut store, 5));

    let config = Config::new();
    let engine = Engine::new(&config).expect("should create engine");

    let mut store = Store::new(&engine, SampleProtocolPluginData::default());

    let mut linker = Linker::<MyT>::new(&engine);
    // let wasi_state = WasiS::new("wasi_snapshot_preview1")
    //     .finalize()
    //     .expect("failed to initialize WASI");
    // wasi_state
    //     .add_to_linker(&mut linker)
    //     .expect("failed to link WASI");

    //let mut data = SampleProtocolPluginData::default();

    let module = Module::new(&engine, PLUGIN_BYTES).expect("should load module from bytes");

    let (plugin, _instance) = sample_protocol_plugin::SampleProtocolPlugin::instantiate(
        &mut store,
        &module,
        &mut linker,
        |data| data,
    )
    .expect("should create instance");

    println!("{:?}", plugin.add_three(&mut store, 5));
}
