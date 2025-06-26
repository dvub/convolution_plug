pub mod convolve;
pub mod nodes;
pub mod param;
pub mod switched;

use fundsp::hacker32::*;
use rtrb::Consumer;
use std::sync::Arc;

use nodes::*;

use crate::{
    config::PluginConfig,
    dsp::{convolve::convolver, switched::switched_node},
    params::PluginParams,
    util::{read_samples_from_file, rms_normalize},
    StereoBuffer,
};

// TODO: maybe make some sort of trait?
// would be pretty nice
pub struct PluginDsp {
    // fundsp stuff
    graph: Box<dyn AudioUnit>,
    input_buffer: StereoBuffer,
    output_buffer: StereoBuffer,
    // for updating IR
    slot: Slot,
    /// Receives messages from the GUI thread.
    /// When a message is received, the Slot (frontend) will communicate to the backend to update the convolver/IR
    slot_rx: Option<Consumer<Vec<f32>>>,
}

impl Default for PluginDsp {
    fn default() -> Self {
        Self {
            graph: Box::new(sink()),
            input_buffer: StereoBuffer::new(),
            output_buffer: StereoBuffer::new(),
            slot: Slot::new(Box::new(sink())).0,
            slot_rx: None,
        }
    }
}

impl PluginDsp {
    pub fn set_slot_rx(&mut self, consumer: Consumer<Vec<f32>>) {
        self.slot_rx = Some(consumer);
    }

    // TODO: add arguments like aux inputs that appear in nih-plug's process()
    pub fn process(&mut self, buffer: &mut nih_plug::buffer::Buffer) {
        for (_offset, mut block) in buffer.iter_blocks(MAX_BUFFER_SIZE) {
            // write into input buffer
            for (sample_index, mut channel_samples) in block.iter_samples().enumerate() {
                for channel_index in 0..=1 {
                    // get our input sample
                    let input_sample = *channel_samples.get_mut(channel_index).unwrap();

                    self.input_buffer.buffer_mut().set_f32(
                        channel_index,
                        sample_index,
                        input_sample,
                    );
                }
            }
            // actually do block processing
            self.graph.process(
                block.samples(),
                &self.input_buffer.buffer_ref(),
                &mut self.output_buffer.buffer_mut(),
            );

            // write from output buffer
            for (index, mut channel_samples) in block.iter_samples().enumerate() {
                for n in 0..=1 {
                    *channel_samples.get_mut(n).unwrap() =
                        self.output_buffer.buffer_ref().at_f32(n, index);
                }
            }
        }

        // update IR in our DSP graph

        // note that updating the IR in plugin's persistent data happens on the GUI thread
        // (because that requires locking a mutex which isn't RT safe)
        if let Some(rx) = self.slot_rx.as_mut() {
            // note that these samples should already be processed  from the gui thread
            // (normalized, whatever)
            if let Ok(new_ir_samples) = rx.pop() {
                // TODO: could use stacki here LOL
                let new_convolver =
                    Box::new(convolver(&new_ir_samples) | convolver(&new_ir_samples));
                self.slot.set(Fade::Smooth, 1.0, new_convolver);
            }
        }
    }

    // TODO: is it smart to have this function be here?
    pub fn build_graph(&mut self, params: &Arc<PluginParams>, config: &PluginConfig) {
        // 1. determine if and from where we should load an IR
        let samples = params.persistent_ir_samples.lock().unwrap();
        let slot_element: Box<dyn AudioUnit> = match samples.as_deref() {
            // if an IR was previously loaded, we detect that here and use it again
            Some(samples) => Box::new(convolver(samples) | convolver(samples)),
            None => {
                // if no IR was previously loaded, *then* we check if we should load anything
                // based on config
                if !config.default_ir_path.is_empty() {
                    let mut samples = read_samples_from_file(&config.default_ir_path);
                    if config.normalize_irs {
                        rms_normalize(&mut samples, config.normalization_level);
                    }

                    Box::new(convolver(&samples) | convolver(&samples))
                } else {
                    // no IR is loaded.
                    // we don't even have to convolve by an empty IR, e.g. [1.0, 0.0, 0.0 ... ],
                    // we can simply pass the signal straight through for the best performance
                    Box::new(multipass::<U2>())
                }
            }
        };
        // we want to update the IR/convolver dynamically, so we put it in a Slot
        let convolver_slot = Slot::new(slot_element);
        let slot_frontend = convolver_slot.0;
        let slot_backend = convolver_slot.1;

        let convolver = unit::<U2, U2>(Box::new(slot_backend));
        let eq_wet = convolver
            >> switched_lowpass(params)
            >> switched_bell(params)
            >> switched_highpass(params);

        let wet = eq_wet * dry_wet(params);
        let dry = multipass::<U2>() * (1.0 - dry_wet(params));
        let mixed = wet & dry;

        let graph = mixed * gain(params);

        self.graph = Box::new(graph);
        self.slot = slot_frontend;
        // (Box::new(graph), slot_front)
    }
}

fn lp_with_params(p: &Arc<PluginParams>) -> An<impl AudioNode<Inputs = U1, Outputs = U1>> {
    (pass() | lp_freq::<U1>(p) | lp_q::<U1>(p)) >> lowpass()
}

fn hp_with_params(p: &Arc<PluginParams>) -> An<impl AudioNode<Inputs = U1, Outputs = U1>> {
    (pass() | hp_freq::<U1>(p) | hp_q::<U1>(p)) >> highpass()
}

fn bell_with_params(p: &Arc<PluginParams>) -> An<impl AudioNode<Inputs = U1, Outputs = U1>> {
    (pass() | bell_freq::<U1>(p) | nodes::bell_q::<U1>(p) | bell_gain::<U1>(p)) >> bell()
}

// ...

fn switched_bell(p: &Arc<PluginParams>) -> An<impl AudioNode<Inputs = U2, Outputs = U2>> {
    (multipass::<U2>() | bell_enabled::<U1>(p))
        >> switched_node(stacki::<U2, _, _>(|_| bell_with_params(p)), |x| x == 1.0)
}

fn switched_lowpass(p: &Arc<PluginParams>) -> An<impl AudioNode<Inputs = U2, Outputs = U2>> {
    (multipass::<U2>() | lp_enabled::<U1>(p))
        >> switched_node(stacki::<U2, _, _>(|_| lp_with_params(p)), |x| x == 1.0)
}

fn switched_highpass(p: &Arc<PluginParams>) -> An<impl AudioNode<Inputs = U2, Outputs = U2>> {
    (multipass::<U2>() | hp_enabled::<U1>(p))
        >> switched_node(stacki::<U2, _, _>(|_| hp_with_params(p)), |x| x == 1.0)
}
