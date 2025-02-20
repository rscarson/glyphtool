use crate::error::Result;
mod espeakng;

mod phonemes;
pub use phonemes::PhonemeExt;

mod server;
use server::NimbusServer;

pub struct Autophononimbus {
    server: NimbusServer,
}
impl Autophononimbus {
    pub fn new() -> Result<Self> {
        let server = NimbusServer::new()?;
        Ok(Self { server })
    }

    pub fn syllabify(&mut self, word: &str) -> Option<String> {
        let phonemes = word.to_glyphs()?;
        self.server.syllabify(&phonemes)
    }
}
