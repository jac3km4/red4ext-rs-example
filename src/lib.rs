use red4ext_rs::prelude::*;

define_plugin! {
    name: "example",
    author: "jekky",
    version: 1:0:0,
    on_register: {
        register_function!("SumInts", sum_ints);
        register_function!("PluginName", plugin_name);
        register_function!("CreateTweakDBID", create_tweakdb_id);
        register_function!("AppendToTweakDBID", append_to_tweakdb_id);
    }
}

/// SumInts
/// 
/// test in CET like:
/// ```swift
/// LogChannel(CName.new("DEBUG"), SumInts({2000, 77}));
/// ```
fn sum_ints(ints: Vec<i32>) -> i32 {
    ints.iter().sum()
}

/// PluginName
/// 
/// test in CET like:
/// ```swift
/// LogChannel(CName.new("DEBUG"), PluginName());
/// ```
fn plugin_name() -> String {
    String::from("RED4EXT.RS.EXAMPLE")
}

/// CreateTweakDBID
/// 
/// test in CET like:
/// ```swift
/// LogChannel(CName.new("DEBUG"), TDBID.ToStringDEBUG(CreateTweakDBID("A.Test")));
/// ```
fn create_tweakdb_id(name: String) -> TweakDBID {
    TweakDBID::new(&name)
}


/// AppendToTweakDBID
/// 
/// test in CET like:
/// ```swift
/// LogChannel(CName.new("DEBUG"), TDBID.Create("A.Test") == AppendToTweakDBID(TDBID.Create("A."), "Test"));
/// ```
fn append_to_tweakdb_id(base: TweakDBID, suffix: String) -> TweakDBID {
    TweakDBID::new_from_base(&base, suffix.as_str())
}
