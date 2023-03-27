use sample_protocol_host::add_one;

wai_bindgen_rust::export!("../sample-protocol-plugin.wai");
wai_bindgen_rust::import!("../sample-protocol-host.wai");

struct SampleProtocolPlugin;

impl sample_protocol_plugin::SampleProtocolPlugin for SampleProtocolPlugin {
    fn add_three(num: u32) -> u32 {
        add_one(num + 1) + 1
    }
}
