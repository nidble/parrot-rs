use serde::Deserialize;

#[derive(Deserialize)]
pub struct PokeApi {
    pub effect_entries: Vec<EffectEntry>,
}

impl PokeApi {
    pub fn get_description(&self) -> Result<String, String> {
        self.effect_entries
            .iter()
            .find(|e| e.language.name == "en")
            .map(|e| {
                e.effect
                    .replace("\n", "")
                    .replace("\t", "")
                    .replace("\r", "")
            })
            .ok_or("english description not found".to_string())
    }
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

#[cfg(test)]
mod tests {
    use super::PokeApi;
    use serde_json::json;

    #[test]
    fn it_return_an_en_description() {
        let p: PokeApi = serde_json::from_value(json!({
            "effect_entries": [
                { "effect": "Mit jedem Treffer besteht.", "language": { "name": "de" } },
                { "effect": "\tSalutamo picciotti.\r\n", "language": { "name": "en"  } }
            ]}
        ))
        .unwrap();

        assert_eq!(p.get_description().unwrap(), "Salutamo picciotti.");
    }

    #[test]
    fn it_return_an_error_if_no_descriptions_are_available() {
        let result = serde_json::from_value::<PokeApi>(json!({
            "effect_entries": [
                { "effect": "Mit jedem Treffer besteht.", "language": { "name": "de" } },
            ]}
        ))
        .unwrap();

        let expected = result.get_description().err();
        assert_eq!(expected, Some("english description not found".to_string()));
    }
}
