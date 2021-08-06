use std::time::Instant;
use tokio::task::JoinHandle;

const NR_TASK: usize = 2000;
const NR_TRY: usize = 1000;

async fn oneshot<F>(f: &F)
where
    F: Fn() -> JoinHandle<()>,
{
    let handles: Vec<JoinHandle<_>> = (0..NR_TASK).map(|_| f()).collect();
    for handle in handles {
        handle.await.unwrap();
    }
}

pub async fn benchmark<F>(f: F)
where
    F: Fn() -> JoinHandle<()>,
{
    oneshot(&f).await;

    let before = Instant::now();
    for _ in 0..NR_TRY {
        oneshot(&f).await;
    }
    let elapsed = before.elapsed();
    println!(
        "{:?} total, {:?} iterations per sec",
        elapsed.as_secs_f32(),
        NR_TRY as f32 / elapsed.as_secs_f32()
    );
}
