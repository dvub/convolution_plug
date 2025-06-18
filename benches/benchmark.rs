use std::sync::Arc;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use fundsp::hacker32::*;

use convolution_plug::{
    dsp::nodes::{gain, gain_shared},
    params::PluginParams,
};

fn render_many_params(num_param_nodes: usize, p: &Arc<PluginParams>) -> Wave {
    let mut net = Net::wrap(Box::new(noise()));
    // repeatedly add a float param to the DSP graph
    // (hopefully)
    for _ in 0..num_param_nodes {
        net = net * gain::<U1>(p);
    }

    Wave::render(44100.0, 1.0, &mut net)
}

fn render_many_params_shared(num_param_nodes: usize, p: &Arc<PluginParams>) -> Wave {
    let shared = shared(0.0);

    let init_graph = noise() >> gain_shared(p, &shared);
    let mut net = Net::wrap(Box::new(init_graph));

    // repeatedly add a float param to the DSP graph
    // (hopefully)
    for _ in 0..num_param_nodes {
        net = net * var(&shared);
    }

    Wave::render(44100.0, 1.0, &mut net)
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("render_many_params");

    for x in 0..4 {
        group.bench_with_input(BenchmarkId::from_parameter(x), &x, |b, &x| {
            let default_params = PluginParams::default();
            let p = Arc::new(default_params);

            b.iter(|| render_many_params(x, &p));
        });
    }
    group.finish();

    let mut group = c.benchmark_group("render_many_params_shared");

    for x in 0..4 {
        group.bench_with_input(BenchmarkId::from_parameter(x), &x, |b, &x| {
            let default_params = PluginParams::default();
            let p = Arc::new(default_params);

            b.iter(|| render_many_params_shared(x, &p));
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
