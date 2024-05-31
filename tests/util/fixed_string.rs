use core::fmt::Write;

/// Provides a core-compatible output sink for use with write!(); essentially implements a holder
/// for str with a buffer of fixed size (given by LEN). This is a minimal, core-compatible String 
/// replacement used as part of running comparison tests.
pub struct FixedString<const LEN: usize> {
    buffer: [u8; LEN],
    index: usize
}

impl<const LEN: usize> FixedString<LEN> {
    /// Instantiates a new, empty `FixedString`
    pub fn new() -> Self {
        Self {
            buffer: [0u8; LEN],
            index: 0
        }
    }

    /// Returns the contents of the `FixedString` as a &str
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.buffer[.. self.index]).unwrap()
    }
    
    /// Clears the contents of the `FixedString` to restore it to a new()-like state; this is faster
    /// than instantiating a new one.
    pub fn clear(&mut self) {
        self.index = 0;
    }
}

impl <const LEN: usize> Write for FixedString<LEN> {
    fn write_str(&mut self, text: &str) -> core::fmt::Result {
        // Check for sufficient space
        let end = match self.index.checked_add(text.as_bytes().len()) {
            None => return Err(core::fmt::Error),
            Some(end) if end > LEN => return Err(core::fmt::Error),
            Some(end) => end
        };

        // Found sufficient space, so copy the bytes
        self.buffer[self.index .. end].copy_from_slice(text.as_bytes());
        self.index = end;
        Ok(())
    }
}