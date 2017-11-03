pub use self::area_frame_allocator::AreaFrameAllocator;

mod area_frame_allocator;

pub const PAGE_SIZE: usize = 4096;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    index: usize,
}

impl Frame {
    fn new(index: usize) -> Frame {
        Frame { index }
    }

    fn from_address(address: usize) -> Frame {
        Frame {
            index: address / PAGE_SIZE,
        }
    }
}

pub trait FrameAllocator {
    fn allocate(&mut self) -> Option<Frame>;
    fn deallocate(&mut self, frame: Frame);
}
