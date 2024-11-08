
pub mod rects;
mod sp3exp;

// taken from clipdecode, looking at previous uses to understand endianness
pub mod sp4 {
    use std::sync::LazyLock;

    fn blockdata_tag(s: &'static str) -> Vec<u8> {
        let mut v: Vec<u8> = s.encode_utf16()
            .flat_map(|x| { x.to_be_bytes() })
            .collect();

        let mut sz = (s.len() as u32).to_be_bytes().to_vec();

        sz.append(&mut v);
        sz
    }

    pub(super) const BEGIN_CHUNK: LazyLock<Vec<u8>> = LazyLock::new(|| { blockdata_tag("BlockDataBeginChunk") });
    pub(super) const END_CHUNK: LazyLock<Vec<u8>> = LazyLock::new(|| { blockdata_tag("BlockDataEndChunk") });

    pub(crate) const STATUS: LazyLock<Vec<u8>> = LazyLock::new(|| { blockdata_tag("BlockStatus") });
    pub(crate) const CHECKSUM: LazyLock<Vec<u8>> = LazyLock::new(|| { blockdata_tag("BlockCheckSum") });
}
