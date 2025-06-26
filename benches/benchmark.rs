use std::sync::Arc;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use fundsp::hacker32::*;

use convolution_plug::{dsp::nodes::gain, params::PluginParams};

fn render_graph(node: &mut dyn AudioUnit) -> Wave {
    Wave::render(44100.0, 1.0, node)
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("micro_params");

    for n in 0..10 {
        group.bench_with_input(BenchmarkId::new("many_params", n), &n, |b, n| {
            /* SETUP */
            let p = Arc::new(PluginParams::default());
            let mut graph = Net::wrap(Box::new(noise()));
            for _ in 0..*n {
                graph = graph * gain::<U1>(&p);
            }

            /* BENCHING */
            b.iter(|| render_graph(black_box(&mut graph)));
        });

        /*
        group.bench_with_input(BenchmarkId::new("many_params_shared", n), &n, |b, n| {
            /* SETUP */
            let p = Arc::new(PluginParams::default());
            let shared = Shared::new(0.0);
            let mut shared_graph = Net::wrap(Box::new(noise() >> gain_shared(&p, &shared)));

            for _ in 0..*n {
                shared_graph = shared_graph * var(&shared);
            }
            /* BENCHING */
            b.iter(|| render_graph(black_box(&mut shared_graph)));
        });*/
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
