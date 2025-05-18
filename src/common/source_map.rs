use std::sync::OnceLock;

#[derive(Debug)]
pub struct SourceMap {
    lines: Vec<String>,
}

static SOURCE_MAP: OnceLock<SourceMap> = OnceLock::new();

pub fn set_source_map(source: &str) {
    SOURCE_MAP
        .set(SourceMap::new(source))
        .expect("SourceMap already set");
}

pub fn get_source_map() -> &'static SourceMap {
    SOURCE_MAP.get().expect("SourceMap not initialized")
}

impl SourceMap {
    pub fn new(source: &str) -> Self {
        Self {
            lines: source.lines().map(|l| l.to_string()).collect(),
        }
    }

    pub fn get_line(&self, line_number: usize) -> Option<&str> {
        self.lines
            .get(line_number.saturating_sub(1))
            .map(String::as_str)
    }
}
