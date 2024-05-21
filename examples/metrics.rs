use anyhow::Result;
use concurrency::Metrics;
use rand::Rng;
use std::thread;
use std::time::Duration;

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metrics = Metrics::new();

    //start N workers and M requesters
    println!("{:?}", metrics.snapshot());

    for idx in 0..N {
        task_worker(idx, metrics.clone()); // Metrics {data:Arc::clone(&metrics.data)}
    }

    for _ in 0..M {
        request_worker(metrics.clone())?;
    }

    loop {
        thread::sleep(Duration::from_secs(2));
        println!("{}", metrics);
    }
}

fn task_worker(idx: usize, metrics: Metrics) {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();
        // do long term stuff
        thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
        metrics
            .inc(format!("call.thread.worker.1. {}", idx))
            .unwrap();
    });
}

fn request_worker(metrics: Metrics) -> Result<()> {
    thread::spawn(move || loop {
        // process requests
        let mut rng = rand::thread_rng();
        // do long term stuff
        thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
        let page = rng.gen_range(1..=5);
        metrics.inc(format!("req.page.{}", page));
    });
    #[allow(unreachable_code)]
    Ok::<_, anyhow::Error>(())
}
