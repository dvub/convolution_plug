pub mod params;

use fundsp::hacker32::*;
use nih_plug::buffer::Buffer;

/// This struct is mostly just a wrapper around any FunDSP graph,
/// intended to make things more convenient when used with nih-plug.
/// This struct contains a main graph, as well as input and output buffers used for block processing.
///
/// Mostly copied from: https://github.com/SamiPerttu/fundsp/blob/a4f126bcbb5c6b93c4cd65662035655913e1e830/src/audiounit.rs#L483
pub struct DspAdapter<N: Size<f32>> {
    pub graph: Box<dyn AudioUnit>,
    input_buffer: BufferArray<N>,
    output_buffer: BufferArray<N>,
}

impl<N> Default for DspAdapter<N>
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

impl<N> DspAdapter<N>
where
    N: Size<f32>,
{
    pub fn process(&mut self, buffer: &mut Buffer) {
        let size = buffer.samples();
        let mut i = 0;
        while i < size {
            let n = min(size - i, MAX_BUFFER_SIZE);
            for input_i in 0..self.input_buffer.channels() {
                for j in 0..n {
                    self.input_buffer
                        .set_f32(input_i, j, buffer.as_slice()[input_i][i + j]);
                }
            }
            self.graph.process(
                n,
                &self.input_buffer.buffer_ref(),
                &mut self.output_buffer.buffer_mut(),
            );

            for output_i in 0..self.output_buffer.channels() {
                for j in 0..n {
                    buffer.as_slice()[output_i][i + j] = self.output_buffer.at_f32(output_i, j);
                }
            }
            i += n;
        }
    }
    /// Call this function (or directly update the `graph` field) during your plugin's initialization.
    pub fn set_graph(&mut self, graph: Box<dyn AudioUnit>) {
        self.graph = graph;
    }
}
