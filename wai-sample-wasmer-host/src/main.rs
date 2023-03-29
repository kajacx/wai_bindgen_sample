use sample_protocol_plugin::SampleProtocolPluginData;

const PLUGIN_BYTES: &'static [u8] = include_bytes!(
    "../../wai-sample-plugin/target/wasm32-unknown-unknown/debug/wai_sample_plugin.wasm"
);

wai_bindgen_wasmer::import!("../sample-protocol-plugin.wai");
wai_bindgen_wasmer::export!("../sample-protocol-host.wai");

impl sample_protocol_host::SampleProtocolHost for SampleProtocolPluginData {
    fn add_one(&mut self, num: u32) -> u32 {
        num + 1
    }
}

fn main() {
    let compiler = wai_bindgen_wasmer::wasmer::Cranelift::default();

    let mut store = Store::new(&engine, SampleProtocolPluginData::default());

    let mut linker = Linker::<SampleProtocolPluginData>::new(&engine);
    sample_protocol_host::add_to_linker(&mut linker, |data| data).expect("should link host fns");

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
