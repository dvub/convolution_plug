use std::{fs::File, hint::black_box, sync::Arc};

use convolution_plug::{
    dsp::{build_graph, nodes::gain},
    params::PluginParams,
};
use fundsp::hacker32::*;

fn main() {
    let params = Arc::new(PluginParams::default());
    //let (graph, _) = build_graph(&params);
    //let graph = unit::<U2, U2>(graph);

    let mut graph = noise() * gain(&params);

    let wave = Wave::render(44100.0, 1.0, &mut graph);
    wave.save_wav16("test.wav").expect("Could not save wave.");

    // println!("Done rendering.");
}
