#[cfg(not(test))]
use std::sync::atomic::AtomicUsize;
#[cfg(not(test))]
static PIC_ID: AtomicUsize = AtomicUsize::new(1);

#[cfg(not(test))]
pub fn generate_pic_id() -> usize {
    use std::sync::atomic::Ordering;

    let id = PIC_ID.fetch_add(1, Ordering::Relaxed);
    id
}

#[cfg(test)]
pub fn generate_pic_id() -> usize {
    123
}

pub fn create_pic_rid(id: usize) -> String {
    format!("rIdImage{}", id)
}
