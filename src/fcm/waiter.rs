use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::watch;

static ERR_CHANNEL_CLOSED: &str = "Waiter channel closed even! That is not possible!";

pub struct Waiter {
    tx: watch::Sender<()>,
    rx: watch::Receiver<()>,
    is_waiting: AtomicBool,
}

impl Waiter {
    pub fn new() -> Self {
        let (tx, mut rx) = watch::channel(());

        drop(rx.borrow_and_update());

        Self {
            tx,
            rx,
            is_waiting: AtomicBool::new(false),
        }
    }

    pub fn is_waiting(&self) -> bool {
        self.is_waiting.load(Ordering::Acquire)
    }

    pub fn start(&self) {
        self.is_waiting.store(true, Ordering::Release);
    }

    pub fn stop(&self) {
        self.tx.send(()).expect(ERR_CHANNEL_CLOSED);
        self.is_waiting.store(false, Ordering::Release);
    }

    pub async fn wait(&self) {
        self.rx.clone().changed().await.expect(ERR_CHANNEL_CLOSED);
    }
}
