use criterion::*;
use madsim::{net::Endpoint, Request, Runtime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Request)]
#[rtype("()")]
struct Req;

fn empty_rpc(c: &mut Criterion) {
    let runtime = Runtime::new();
    let host = runtime.create_host("127.0.0.1:0").build().unwrap();
    let addr = host.local_addr();
    host.spawn(async move {
        let net = Endpoint::current();
        net.add_rpc_handler(|_: Req| async move {});
    })
    .detach();

    c.bench_function("empty RPC", |b| {
        b.iter(|| {
            runtime.block_on(async move {
                let net = Endpoint::current();
                net.call(addr, Req).await.unwrap();
            })
        })
    });
}

fn rpc_data(c: &mut Criterion) {
    let runtime = Runtime::new();
    let host = runtime.create_host("127.0.0.1:0").build().unwrap();
    let addr = host.local_addr();
    host.spawn(async move {
        let net = Endpoint::current();
        net.add_rpc_handler_with_data(|_: Req, data| async move {
            black_box(data);
            ((), vec![])
        });
    })
    .detach();

    let mut group = c.benchmark_group("RPC with data");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
    for size in [16, 256, 4096, 65536, 1048576] {
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let data = vec![0u8; size];
            b.iter(|| {
                runtime.block_on(async {
                    let net = Endpoint::current();
                    net.call_with_data(addr, Req, &data).await.unwrap();
                })
            });
        });
    }
    group.finish();
}

criterion_group!(benches, empty_rpc, rpc_data);
criterion_main!(benches);