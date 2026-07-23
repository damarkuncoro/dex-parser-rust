use scroll::Pread;

#[derive(Debug, Pread)]
pub struct RawTypeId {
    pub descriptor_idx: u32,
}
