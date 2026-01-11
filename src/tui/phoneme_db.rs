#[derive(Debug, Clone, PartialEq)]
pub struct Phoneme {
    pub ipa: String,
    pub espeak: String,
    pub description_ja: String,
    pub key: char,
    pub category: PhonemeCategory,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PhonemeCategory {
    Vowel,
    Consonant,
}

pub struct PhonemeDatabase {
    vowels: Vec<Phoneme>,
    consonants: Vec<Phoneme>,
}

impl PhonemeDatabase {
    pub fn new() -> Self {
        let vowels = vec![
            Phoneme {
                ipa: "a".to_string(),
                espeak: "a".to_string(),
                description_ja: "日本語「あ」".to_string(),
                key: 'a',
                category: PhonemeCategory::Vowel,
            },
            Phoneme {
                ipa: "i".to_string(),
                espeak: "i".to_string(),
                description_ja: "日本語「い」".to_string(),
                key: 'i',
                category: PhonemeCategory::Vowel,
            },
            Phoneme {
                ipa: "u".to_string(),
                espeak: "u".to_string(),
                description_ja: "日本語「う」".to_string(),
                key: 'u',
                category: PhonemeCategory::Vowel,
            },
            Phoneme {
                ipa: "e".to_string(),
                espeak: "e".to_string(),
                description_ja: "日本語「え」".to_string(),
                key: 'e',
                category: PhonemeCategory::Vowel,
            },
            Phoneme {
                ipa: "o".to_string(),
                espeak: "o".to_string(),
                description_ja: "日本語「お」".to_string(),
                key: 'o',
                category: PhonemeCategory::Vowel,
            },
            Phoneme {
                ipa: "ə".to_string(),
                espeak: "@".to_string(),
                description_ja: "曖昧母音 (about)".to_string(),
                key: '@',
                category: PhonemeCategory::Vowel,
            },
            Phoneme {
                ipa: "ɑ".to_string(),
                espeak: "A".to_string(),
                description_ja: "後舌開母音 (father)".to_string(),
                key: 'A',
                category: PhonemeCategory::Vowel,
            },
            Phoneme {
                ipa: "ɔ".to_string(),
                espeak: "O".to_string(),
                description_ja: "後舌半開円唇母音 (thought)".to_string(),
                key: 'O',
                category: PhonemeCategory::Vowel,
            },
        ];

        let consonants = vec![
            Phoneme {
                ipa: "p".to_string(),
                espeak: "p".to_string(),
                description_ja: "無声両唇破裂音「ぱ」".to_string(),
                key: 'p',
                category: PhonemeCategory::Consonant,
            },
            Phoneme {
                ipa: "b".to_string(),
                espeak: "b".to_string(),
                description_ja: "有声両唇破裂音「ば」".to_string(),
                key: 'b',
                category: PhonemeCategory::Consonant,
            },
            Phoneme {
                ipa: "t".to_string(),
                espeak: "t".to_string(),
                description_ja: "無声歯茎破裂音「た」".to_string(),
                key: 't',
                category: PhonemeCategory::Consonant,
            },
            Phoneme {
                ipa: "d".to_string(),
                espeak: "d".to_string(),
                description_ja: "有声歯茎破裂音「だ」".to_string(),
                key: 'd',
                category: PhonemeCategory::Consonant,
            },
            Phoneme {
                ipa: "k".to_string(),
                espeak: "k".to_string(),
                description_ja: "無声軟口蓋破裂音「か」".to_string(),
                key: 'k',
                category: PhonemeCategory::Consonant,
            },
            Phoneme {
                ipa: "g".to_string(),
                espeak: "g".to_string(),
                description_ja: "有声軟口蓋破裂音「が」".to_string(),
                key: 'g',
                category: PhonemeCategory::Consonant,
            },
            Phoneme {
                ipa: "m".to_string(),
                espeak: "m".to_string(),
                description_ja: "両唇鼻音「ま」".to_string(),
                key: 'm',
                category: PhonemeCategory::Consonant,
            },
            Phoneme {
                ipa: "n".to_string(),
                espeak: "n".to_string(),
                description_ja: "歯茎鼻音「な」".to_string(),
                key: 'n',
                category: PhonemeCategory::Consonant,
            },
            Phoneme {
                ipa: "ŋ".to_string(),
                espeak: "N".to_string(),
                description_ja: "軟口蓋鼻音 (sing)".to_string(),
                key: 'N',
                category: PhonemeCategory::Consonant,
            },
            Phoneme {
                ipa: "s".to_string(),
                espeak: "s".to_string(),
                description_ja: "無声歯茎摩擦音「さ」".to_string(),
                key: 's',
                category: PhonemeCategory::Consonant,
            },
            Phoneme {
                ipa: "z".to_string(),
                espeak: "z".to_string(),
                description_ja: "有声歯茎摩擦音「ざ」".to_string(),
                key: 'z',
                category: PhonemeCategory::Consonant,
            },
            Phoneme {
                ipa: "ʃ".to_string(),
                espeak: "S".to_string(),
                description_ja: "無声後部歯茎摩擦音「しゃ」".to_string(),
                key: 'S',
                category: PhonemeCategory::Consonant,
            },
            Phoneme {
                ipa: "ʒ".to_string(),
                espeak: "Z".to_string(),
                description_ja: "有声後部歯茎摩擦音 (vision)".to_string(),
                key: 'Z',
                category: PhonemeCategory::Consonant,
            },
            Phoneme {
                ipa: "h".to_string(),
                espeak: "h".to_string(),
                description_ja: "無声声門摩擦音「は」".to_string(),
                key: 'h',
                category: PhonemeCategory::Consonant,
            },
            Phoneme {
                ipa: "f".to_string(),
                espeak: "f".to_string(),
                description_ja: "無声唇歯摩擦音 (fan)".to_string(),
                key: 'f',
                category: PhonemeCategory::Consonant,
            },
            Phoneme {
                ipa: "v".to_string(),
                espeak: "v".to_string(),
                description_ja: "有声唇歯摩擦音 (van)".to_string(),
                key: 'v',
                category: PhonemeCategory::Consonant,
            },
            Phoneme {
                ipa: "l".to_string(),
                espeak: "l".to_string(),
                description_ja: "歯茎側音 (light)".to_string(),
                key: 'l',
                category: PhonemeCategory::Consonant,
            },
            Phoneme {
                ipa: "r".to_string(),
                espeak: "r".to_string(),
                description_ja: "歯茎ふるえ音 (巻き舌)".to_string(),
                key: 'r',
                category: PhonemeCategory::Consonant,
            },
            Phoneme {
                ipa: "w".to_string(),
                espeak: "w".to_string(),
                description_ja: "有声両唇軟口蓋接近音「わ」".to_string(),
                key: 'w',
                category: PhonemeCategory::Consonant,
            },
            Phoneme {
                ipa: "j".to_string(),
                espeak: "j".to_string(),
                description_ja: "有声硬口蓋接近音「や」".to_string(),
                key: 'y',
                category: PhonemeCategory::Consonant,
            },
        ];

        Self { vowels, consonants }
    }

    pub fn get_by_key(&self, key: char) -> Option<&Phoneme> {
        self.vowels
            .iter()
            .chain(self.consonants.iter())
            .find(|p| p.key == key)
    }

    pub fn get_vowels(&self) -> &[Phoneme] {
        &self.vowels
    }

    pub fn get_consonants(&self) -> &[Phoneme] {
        &self.consonants
    }
}

impl Default for PhonemeDatabase {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phoneme_database_creation() {
        let db = PhonemeDatabase::new();
        assert_eq!(db.get_vowels().len(), 8, "Should have 8 vowels");
        assert_eq!(db.get_consonants().len(), 20, "Should have 20 consonants");
    }

    #[test]
    fn test_get_phoneme_by_key() {
        let db = PhonemeDatabase::new();
        let phoneme = db.get_by_key('a').unwrap();
        assert_eq!(phoneme.ipa, "a");
        assert_eq!(phoneme.espeak, "a");
        assert_eq!(phoneme.category, PhonemeCategory::Vowel);
    }

    #[test]
    fn test_get_consonant_by_key() {
        let db = PhonemeDatabase::new();
        let phoneme = db.get_by_key('k').unwrap();
        assert_eq!(phoneme.ipa, "k");
        assert_eq!(phoneme.espeak, "k");
        assert_eq!(phoneme.category, PhonemeCategory::Consonant);
    }

    #[test]
    fn test_invalid_key_returns_none() {
        let db = PhonemeDatabase::new();
        assert!(db.get_by_key('X').is_none());
        assert!(db.get_by_key('1').is_none());
    }

    #[test]
    fn test_all_vowels_have_unique_keys() {
        let db = PhonemeDatabase::new();
        let mut keys: Vec<char> = db.get_vowels().iter().map(|p| p.key).collect();
        keys.sort();
        keys.dedup();
        assert_eq!(keys.len(), 8, "All vowel keys should be unique");
    }

    #[test]
    fn test_all_consonants_have_unique_keys() {
        let db = PhonemeDatabase::new();
        let mut keys: Vec<char> = db.get_consonants().iter().map(|p| p.key).collect();
        keys.sort();
        keys.dedup();
        assert_eq!(keys.len(), 20, "All consonant keys should be unique");
    }
}
