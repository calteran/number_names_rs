use crate::format::Capitalization::Title;
use crate::languages::Language;
use crate::languages::Language::EnUs;

pub enum Capitalization {
    Title,
    Upper,
    Lower,
}

pub struct Format {
    pub language: Language,
    pub capitalization: Capitalization,
    pub commas: bool,
    pub dashes: bool,
}

impl Format {
    pub fn default() -> Format {
        Format {
            language: EnUs,
            capitalization: Title,
            commas: true,
            dashes: true,
        }
    }
}
