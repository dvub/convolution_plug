pub mod params;

use fundsp::hacker32::*;
use nih_plug::buffer::Buffer;

/// This struct is mostly just a wrapper around any FunDSP graph,
/// intended to make things more convenient when used with nih-plug.
/// This struct contains a main graph, as well as input and output buffers used for block processing.
///
/// This struct uses generic FunDSP buffers which supports mono or stereo plugins (or more, I guess).
pub struct PluginDspProcessor<N: Size<f32>> {
    pub graph: Box<dyn AudioUnit>,
    input_buffer: BufferArray<N>,
    output_buffer: BufferArray<N>,
}

impl<N> Default for PluginDspProcessor<N>
where
    N: Size<f32>,
{
    /// A default implementation involves a very cheap graph for quick plugin scanning.
    /// Call this in your plugin's Default impl.
    fn default() -> Self {
        Self {
            graph: Box::new(sink()),
            input_buffer: BufferArray::new(),
            output_buffer: BufferArray::new(),
        }
    }
}

impl<N> PluginDspProcessor<N>
where
    N: Size<f32>,
{
    // TODO: support passing in other nih-plug process() arguments

    /// Process an nih-plug Buffer. This function does 3 things:
    /// 1. Copy samples into FunDSP's buffers.
    /// 2. Perform block processing
    /// 3. Write samples from FunDSP's output buffer, back into the nih-plug buffer.
    ///
    /// In your nih-plug `process()`, call this function first,
    /// then do anything else you might want to do with your plugin afterwards.
    pub fn process(&mut self, buffer: &mut Buffer) {
        for (_offset, mut block) in buffer.iter_blocks(MAX_BUFFER_SIZE) {
            // write into input buffer
            for (sample_index, mut channel_samples) in block.iter_samples().enumerate() {
                for channel_index in 0..=N::USIZE {
                    // get our input sample
                    let input_sample = *channel_samples.get_mut(channel_index).unwrap();

                    self.input_buffer.buffer_mut().set_f32(
                        channel_index,
                        sample_index,
                        input_sample,
                    );
                }
            }

            self.graph.process(
                block.samples(),
                &self.input_buffer.buffer_ref(),
                &mut self.output_buffer.buffer_mut(),
            );

            // write from output buffer
            for (index, mut channel_samples) in block.iter_samples().enumerate() {
                for n in 0..=N::USIZE {
                    *channel_samples.get_mut(n).unwrap() =
                        self.output_buffer.buffer_ref().at_f32(n, index);
                }
            }
        }
    }
    /// Call this function (or directly update the `graph` field) during your plugin's initialization.
    pub fn set_graph(&mut self, graph: Box<dyn AudioUnit>) {
        self.graph = graph;
    }
}
