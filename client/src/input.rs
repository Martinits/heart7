#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub enum InputRequest {
    SetCursor(usize),
    InsertChar(char),
    GoToPrevChar,
    GoToNextChar,
    DeletePrevChar,
    DeleteNextChar,
}

#[derive(Default, Debug, Clone)]
pub struct Input {
    value: String,
    cursor: usize,
}

impl Input {
    pub fn new(value: String) -> Self {
        let len = value.chars().count();
        Self { value, cursor: len }
    }

    pub fn with_value(mut self, value: String) -> Self {
        self.cursor = value.chars().count();
        self.value = value;
        self
    }

    pub fn with_cursor(mut self, cursor: usize) -> Self {
        self.cursor = cursor.min(self.value.chars().count());
        self
    }

    pub fn reset(&mut self) {
        self.cursor = Default::default();
        self.value = Default::default();
    }

    /// Handle request and emit response.
    pub fn handle(&mut self, req: InputRequest) {
        use InputRequest::*;
        match req {
            SetCursor(pos) => {
                self.cursor = pos.min(self.value.chars().count());
            }

            InsertChar(c) => {
                if self.cursor == self.value.chars().count() {
                    self.value.push(c);
                } else {
                    self.value = self
                        .value
                        .chars()
                        .take(self.cursor)
                        .chain(
                            std::iter::once(c)
                                .chain(self.value.chars().skip(self.cursor)),
                        )
                        .collect();
                }
                self.cursor += 1;
            }

            DeletePrevChar => {
                if self.cursor != 0 {
                    self.cursor -= 1;
                    self.value = self
                        .value
                        .chars()
                        .enumerate()
                        .filter(|(i, _)| i != &self.cursor)
                        .map(|(_, c)| c)
                        .collect();
                }
            }

            DeleteNextChar => {
                if self.cursor != self.value.chars().count() {
                    self.value = self
                        .value
                        .chars()
                        .enumerate()
                        .filter(|(i, _)| i != &self.cursor)
                        .map(|(_, c)| c)
                        .collect();
                }
            }

            GoToPrevChar => {
                if self.cursor != 0 {
                    self.cursor -= 1;
                }
            }

            GoToNextChar => {
                if self.cursor != self.value.chars().count() {
                    self.cursor += 1;
                }
            }
        }
    }

    pub fn value(&self) -> &str {
        self.value.as_str()
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn visual_cursor(&self) -> usize {
        if self.cursor == 0 {
            return 0;
        }

        // Safe, because the end index will always be within bounds
        unicode_width::UnicodeWidthStr::width(unsafe {
            self.value.get_unchecked(
                0..self
                    .value
                    .char_indices()
                    .nth(self.cursor)
                    .map_or_else(|| self.value.len(), |(index, _)| index),
            )
        })
    }

    pub fn visual_scroll(&self, width: usize) -> usize {
        let scroll = (self.visual_cursor()).max(width) - width;
        let mut uscroll = 0;
        let mut chars = self.value().chars();

        while uscroll < scroll {
            match chars.next() {
                Some(c) => {
                    uscroll += unicode_width::UnicodeWidthChar::width(c).unwrap_or(0);
                }
                None => break,
            }
        }
        uscroll
    }
}

impl From<Input> for String {
    fn from(input: Input) -> Self {
        input.value
    }
}

impl From<String> for Input {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        Self::new(value.into())
    }
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}
