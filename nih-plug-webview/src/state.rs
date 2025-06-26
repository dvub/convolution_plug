use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

// rewritten from: https://github.com/robbert-vdh/nih-plug/blob/master/nih_plug_vizia/src/lib.rs
pub struct WebviewState {
    /// Whether the editor's window is currently open.
    pub open: AtomicBool,
}

impl WebviewState {
    /// Initialize GUI state.
    pub fn new() -> Arc<WebviewState> {
        Arc::new(WebviewState {
            open: AtomicBool::new(false),
        })
    }

    /// Whether the GUI is currently visible.
    // Called `is_open()` instead of `open()` to avoid ambiguity.
    pub fn is_open(&self) -> bool {
        self.open.load(Ordering::Acquire)
    }
}
impl std::fmt::Debug for WebviewState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WebviewState")
            .field("open", &self.open)
            .finish()
    }
}
