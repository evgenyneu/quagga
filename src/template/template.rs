/// Represents the entire template structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Template {
    pub prompt: PromptSection,
    pub part: PartSection,
}

impl Default for Template {
    fn default() -> Self {
        Template {
            prompt: PromptSection::default(),
            part: PartSection::default(),
        }
    }
}

/// Represents the prompt section, including header, file template, and footer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptSection {
    pub header: String,
    pub file: String,
    pub footer: String,
}

impl Default for PromptSection {
    fn default() -> Self {
        PromptSection {
            header: "".to_string(),
            footer: "".to_string(),
            file: "".to_string(),
        }
    }
}

/// Represents the part section for multi-part outputs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartSection {
    pub header: String,
    pub footer: String,
    pub pending: String,
}

impl Default for PartSection {
    fn default() -> Self {
        PartSection {
            header: "".to_string(),
            pending: "".to_string(),
            footer: "".to_string(),
        }
    }
}
