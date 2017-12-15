include!(concat!(env!("OUT_DIR"), "/audio.rs"));

impl Sound {
    pub fn play(&self) {
        let id = self.clone() as u32;
        js! { audio.play_sound(@{id}); }
    }
}

impl Music {
    pub fn play(&self) {
        let id = self.clone() as u32;
        js! { audio.play_music(@{id}); }
    }
}
