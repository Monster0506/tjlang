//! Source span representation

use codespan::{FileId, Span};

/// A source span with file information
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceSpan {
    pub file_id: FileId,
    pub span: Span,
}

impl SourceSpan {
    pub fn new(file_id: FileId, span: Span) -> Self {
        Self { file_id, span }
    }
    
    pub fn start(&self) -> usize {
        self.span.start().into()
    }
    
    pub fn end(&self) -> usize {
        self.span.end().into()
    }
    
    pub fn len(&self) -> usize {
        let start: usize = self.span.start().into();
        let end: usize = self.span.end().into();
        end - start
    }
}
