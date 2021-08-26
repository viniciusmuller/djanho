// Use this for Lua and Vimscript generators
pub trait ConfigGenerator {
    fn link(&self) -> ();
    fn highlight(&self) -> ();
    fn setup(&self) -> ();
}
