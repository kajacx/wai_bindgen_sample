use sample_protocol_plugin::SampleProtocolPluginData;
use wai_bindgen_wasmtime::wasmtime::{
    Config, Engine, Func, FuncType, Linker, Module, Store, Val, ValType,
};

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
    let config = Config::new();
    let engine = Engine::new(&config).expect("should create engine");

    let mut store = Store::new(&engine, SampleProtocolPluginData::default());

    let mut linker = Linker::<MyT>::new(&engine);
    linker
        .define(
            "sample-protocol-host",
            "add-one",
            Func::new(
                &mut store,
                FuncType::new([ValType::I32], [ValType::I32]),
                |_, args, ret| {
                    ret[0] = Val::I32(args[0].unwrap_i32() + 1);
                    Ok(())
                },
            ),
        )
        .expect("should define a function");

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
