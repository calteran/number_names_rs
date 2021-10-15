//! Defines configuration struct Format to control number name formatting.
use crate::languages::Language;
use crate::languages::Language::EnUs;

///
/// Configuration struct to control formatting of the number names.
/// Currently used only to specify the language.
///
/// Examples:
///
/// ```rust
/// // default format
/// let fmt = number_names::Format::default();
///
/// // specify a language
/// let fmt = number_names::Format(number_names::languages::Language::EnUs);
/// ```
pub struct Format {
    /// Language the number name should be formatted in
    pub language: Language,
}

impl Format {
    /// Return a Format with the default values
    /// (currently only language set to American English)
    ///
    /// Example:
    /// ```rust
    /// let fmt = number_names::Format::default();
    /// assert_eq!(fmt, number_names::Format{number_names::languages::Language::EnUs});
    /// ```
    pub fn default() -> Format {
        Format { language: EnUs }
    }
}
