/// Represents the entire template structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Template {
    pub prompt: PromptSection,
    pub part: PartSection,
}

/// Represents the prompt section, including header, file template, and footer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptSection {
    pub header: String,
    pub file: String,
    pub footer: String,
}

/// Represents the parts section for multi-part outputs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartSection {
    pub header: String,
    pub footer: String,
    pub pending: String,
}
