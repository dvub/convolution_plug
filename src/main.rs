use std::sync::Arc;

use convolution_plug::{dsp::param_nodes::gain, params::PluginParams};
use fundsp::hacker32::*;

fn main() {
    let params = Arc::new(PluginParams::default());
    //let (graph, _) = build_graph(&params);
    //let graph = unit::<U2, U2>(graph);

    let mut graph = noise() * gain(&params);

    Wave::render(44100.0, 1.0, &mut graph);

    println!("Done rendering.");
}
