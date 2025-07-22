use std::sync::Arc;

use crossbeam_channel::{Receiver, Sender};

use nih_plug_webview::WebviewState;

// TODO: fix this issue
// due to the way that callback handler currently works,
// parameters MUST be assigned (in default()) in the SAME ORDER they are defined

#[derive(Debug)]
pub struct CallbackHandler {
    counter: usize,
    pub state: Arc<WebviewState>,
    tx: Sender<usize>,
    pub rx: Receiver<usize>,
}
impl Default for CallbackHandler {
    fn default() -> Self {
        let (tx, rx) = crossbeam_channel::bounded::<usize>(1024);
        let state = WebviewState::new();

        Self {
            counter: 0,
            state,
            tx,
            rx,
        }
    }
}

impl CallbackHandler {
    pub fn create_callback<T>(&mut self) -> Arc<impl Fn(T)> {
        let state = self.state.clone();
        let tx = self.tx.clone();
        let parameter_index = self.counter;

        self.counter += 1;

        Arc::new(move |_| {
            if !state.is_open() {
                return;
            }

            tx.try_send(parameter_index)
                .expect("the channel should not be full or try sending if disconnected");
        })
    }
}

#[cfg(test)]
mod tests {
    use super::CallbackHandler;
    use std::sync::atomic::Ordering;

    #[test]
    fn increment_counter() {
        let mut handler = CallbackHandler::default();

        handler.create_callback::<bool>();
        assert_eq!(handler.counter, 1);

        handler.create_callback::<bool>();
        assert_eq!(handler.counter, 2);
    }

    #[test]
    fn skip_when_closed() {
        let mut handler = CallbackHandler::default();

        let callback = handler.create_callback();
        callback(0.0);

        assert!(!handler.state.is_open());
        assert!(handler.rx.is_empty());
    }

    #[test]
    fn send_updates() {
        let mut handler = CallbackHandler::default();
        handler.state.open.store(true, Ordering::Relaxed);

        let callback = handler.create_callback();
        let callback1 = handler.create_callback();

        callback(0.0);
        assert_eq!(handler.rx.recv().unwrap(), 0);

        callback1(0.0);
        assert_eq!(handler.rx.recv().unwrap(), 1);
    }
}
