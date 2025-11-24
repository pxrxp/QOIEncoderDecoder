use crate::tags;

pub enum PixelChunk {
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, u8),
    Index(u8),
    Diff { dr: u8, dg: u8, db: u8 },
    Luma { dg: u8, dr_dg: u8, db_dg: u8 },
    Run(u8),
}

pub fn decode_pixel<'a, I>(iter: &mut I) -> Option<PixelChunk>
where
    I: Iterator<Item = &'a u8>,
{
    let tag = iter.next()?;

    match *tag {
        tags::QOI_OP_RGB_TAG => Some(PixelChunk::RGB(*iter.next()?, *iter.next()?, *iter.next()?)),
        tags::QOI_OP_RGBA_TAG => Some(PixelChunk::RGBA(
            *iter.next()?,
            *iter.next()?,
            *iter.next()?,
            *iter.next()?,
        )),
        tags::QOI_OP_INDEX_TAG => Some(PixelChunk::Index(*iter.next()?)),
        tags::QOI_OP_DIFF_TAG => Some(PixelChunk::Diff {
            dr: *iter.next()?,
            dg: *iter.next()?,
            db: *iter.next()?,
        }),
        tags::QOI_OP_LUMA_TAG => Some(PixelChunk::Luma {
            dg: *iter.next()?,
            dr_dg: *iter.next()?,
            db_dg: *iter.next()?,
        }),
        tags::QOI_OP_RUN_TAG => Some(PixelChunk::Run(*iter.next()?)),
        _ => None,
    }
}

pub fn chunk<'a, I>(iter: &mut I, n: usize) -> Vec<u8>
where
    I: Iterator<Item = &'a u8>,
{
    iter.take(n).copied().collect()
}
