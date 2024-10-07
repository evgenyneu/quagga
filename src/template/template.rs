/// Represents the entire template structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Template {
    pub prompt: PromptTemplate,
    pub part: PartTemplate,
}

impl Default for Template {
    fn default() -> Self {
        Template {
            prompt: PromptTemplate::default(),
            part: PartTemplate::default(),
        }
    }
}

/// Represents the prompt section, including header, file template, and footer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptTemplate {
    pub header: String,
    pub file: String,
    pub footer: String,
}

impl Default for PromptTemplate {
    fn default() -> Self {
        PromptTemplate {
            header: "".to_string(),
            footer: "".to_string(),
            file: "".to_string(),
        }
    }
}

/// Represents the part section for multi-part outputs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartTemplate {
    pub header: String,
    pub footer: String,
    pub pending: String,
}

impl Default for PartTemplate {
    fn default() -> Self {
        PartTemplate {
            header: "".to_string(),
            pending: "".to_string(),
            footer: "".to_string(),
        }
    }
}
