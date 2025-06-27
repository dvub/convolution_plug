pub mod params;

use fundsp::hacker32::*;
use nih_plug::buffer::Buffer;

pub struct PluginDsp<N: Size<f32>> {
    graph: Box<dyn AudioUnit>,
    input_buffer: BufferArray<N>,
    output_buffer: BufferArray<N>,
}

impl<N> Default for PluginDsp<N>
where
    N: Size<f32>,
{
    fn default() -> Self {
        Self {
            graph: Box::new(sink()),
            input_buffer: BufferArray::new(),
            output_buffer: BufferArray::new(),
        }
    }
}

impl<N> PluginDsp<N>
where
    N: Size<f32>,
{
    pub fn process(&mut self, buffer: &mut Buffer) {
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
    }
    pub fn set_graph(&mut self, graph: Box<dyn AudioUnit>) {
        self.graph = graph;
    }
}
