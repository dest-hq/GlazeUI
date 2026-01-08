use std::sync::atomic::{AtomicU64, Ordering};

static NODE_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

pub fn next_id() -> u64 {
    NODE_ID_COUNTER.fetch_add(1, Ordering::Relaxed)
}

pub fn sync_with(id: u64) {
    NODE_ID_COUNTER.fetch_max(id, Ordering::Relaxed);
}

pub fn clear_counter() {
    NODE_ID_COUNTER.store(1, Ordering::SeqCst);
}
