@addMethod(PlayerPuppet)
public func TryTDBID() -> Void {
 let name = PluginName();
 let plugin = Convert(name);
 let expected = TDBID.Create(name);
 if Equals(plugin, expected) {
  LogChannel(n"DEBUG", s"OK => TweakID: \(TDBID.ToStringDEBUG(plugin))");
 } else {
  LogChannel(n"ASSERT", s"ERROR => TDBID.Create: \(TDBID.ToStringDEBUG(expected)) vs Plugin.Convert: \(TDBID.ToStringDEBUG(plugin))");
 }
}