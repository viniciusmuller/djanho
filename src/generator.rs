use crate::vim::VimHighlight;

// Use this for Lua and Vimscript generators
pub trait ConfigGenerator {
    fn collect(&self) -> String;
    fn link(&mut self, group: &str, target: &str);
    fn highlight(&mut self, options: &VimHighlight);
    fn newline(&mut self);
}
