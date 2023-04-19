use red4ext_rs::{prelude::*, interop::TweakDBID};

define_plugin! {
    name: "example",
    author: "jekky",
    version: 1:0:0,
    on_register: {
        register_function!("SumInts", sum_ints);
        register_function!("PluginName", plugin_name);
        register_function!("Convert", convert);
    }
}

fn sum_ints(ints: Vec<i32>) -> i32 {
    ints.iter().sum()
}

fn plugin_name() -> String {
    String::from("RED4EXT.RS.EXAMPLE")
}

fn convert(name: String) -> TweakDBID {
    TweakDBID::new(&name)
}
