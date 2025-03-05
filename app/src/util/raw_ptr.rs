/// RawPtr encapsulation that allows thread exchange
/// by implementing [Sync, Clone, Copy] traits
///
pub struct RawPtr<T> {
    pub ptr: *mut T,
}
unsafe impl<T> Sync for RawPtr<T> {}
impl<T> Clone for RawPtr<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for RawPtr<T> {}
unsafe impl<T> Send for RawPtr<T> {}
