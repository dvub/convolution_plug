// Editor trait implementation.

// module structure/organization based on https://github.com/robbert-vdh/nih-plug/blob/master/nih_plug_vizia/src/editor.rs

use crate::{state::WebviewState, HTMLSource, WebViewEditor, WindowHandler};
use baseview::{Size, WindowHandle, WindowOpenOptions, WindowScalePolicy};
use crossbeam::channel::unbounded;
use nih_plug::prelude::{Editor, GuiContext};
use std::sync::{atomic::Ordering, Arc};
use wry::{WebContext, WebViewBuilder};

impl Editor for WebViewEditor {
    fn spawn(
        &self,
        parent: nih_plug::prelude::ParentWindowHandle,
        context: Arc<dyn GuiContext>,
    ) -> Box<dyn std::any::Any + Send> {
        let options = WindowOpenOptions {
            scale: WindowScalePolicy::SystemScaleFactor,
            size: Size {
                width: self.width.load(Ordering::Relaxed) as f64,
                height: self.height.load(Ordering::Relaxed) as f64,
            },
            title: "Plug-in".to_owned(),
        };

        let width = self.width.clone();
        let height = self.height.clone();
        let developer_mode = self.developer_mode;
        let source = self.source.clone();
        let background_color = self.background_color;
        let custom_protocol = self.custom_protocol.clone();
        let event_loop_handler = self.event_loop_handler.clone();
        let keyboard_handler = self.keyboard_handler.clone();
        let mouse_handler = self.mouse_handler.clone();

        let window_handle = baseview::Window::open_parented(&parent, options, move |window| {
            let (events_sender, events_receiver) = unbounded();

            let mut web_context = WebContext::new(Some(std::env::temp_dir()));

            let mut webview_builder = WebViewBuilder::new_as_child(window)
                .with_bounds(wry::Rect {
                    x: 0,
                    y: 0,
                    width: width.load(Ordering::Relaxed),
                    height: height.load(Ordering::Relaxed),
                })
                .with_accept_first_mouse(true)
                .with_devtools(developer_mode)
                .with_web_context(&mut web_context)
                .with_initialization_script(include_str!("script.js"))
                .with_ipc_handler(move |msg: String| {
                    if let Ok(json_value) = serde_json::from_str(&msg) {
                        let _ = events_sender.send(json_value);
                    } else {
                        panic!("Invalid JSON from web view: {msg}.");
                    }
                })
                .with_background_color(background_color);

            if let Some(custom_protocol) = custom_protocol.as_ref() {
                let handler = custom_protocol.1.clone();
                webview_builder = webview_builder
                    .with_custom_protocol(custom_protocol.0.to_owned(), move |request| {
                        handler(&request).unwrap()
                    });
            }

            let webview = match source.as_ref() {
                HTMLSource::String(html_str) => webview_builder.with_html(*html_str),
                HTMLSource::URL(url) => webview_builder.with_url(url),
            }
            .unwrap()
            .build();

            WindowHandler {
                context,
                event_loop_handler,
                webview: webview.unwrap_or_else(|e| panic!("Failed to construct webview. {e}")),
                events_receiver,
                keyboard_handler,
                mouse_handler,
                width,
                height,
            }
        });

        // !!!
        self.webview_state.open.store(true, Ordering::Release);
        // println!("OPEN");
        Box::new(WebViewEditorHandle {
            window_handle,
            webview_state: self.webview_state.clone(),
        })
    }

    fn size(&self) -> (u32, u32) {
        (
            self.width.load(Ordering::Relaxed),
            self.height.load(Ordering::Relaxed),
        )
    }

    fn set_scale_factor(&self, _factor: f32) -> bool {
        // TODO: implement for Windows and Linux
        false
    }

    fn param_values_changed(&self) {}

    fn param_value_changed(&self, _id: &str, _normalized_value: f32) {}

    fn param_modulation_changed(&self, _id: &str, _modulation_offset: f32) {}
}

struct WebViewEditorHandle {
    window_handle: WindowHandle,
    webview_state: Arc<WebviewState>,
}

impl Drop for WebViewEditorHandle {
    fn drop(&mut self) {
        // !!!!!!!
        // this is like the only reason i made this fork
        // println!("CLOSE");
        self.webview_state.open.store(false, Ordering::Release);
        self.window_handle.close();
    }
}

unsafe impl Send for WebViewEditorHandle {}
