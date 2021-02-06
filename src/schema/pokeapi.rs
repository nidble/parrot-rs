use serde::Deserialize;

#[derive(Deserialize)]
pub struct PokeApi {
    pub effect_entries: Vec<EffectEntry>,
}

#[derive(Deserialize)]
pub struct EffectEntry {
    pub effect: String,
    pub language: Language,
}

#[derive(Deserialize)]
pub struct Language {
    pub name: String,
}
