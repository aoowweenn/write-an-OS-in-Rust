use memory::{Frame, FrameAllocator};
use multiboot2::{MemoryArea, MemoryAreaIter};

use core::ops::RangeInclusive;

pub struct AreaFrameAllocator {
    next_free_frame: Frame,
    current_area: Option<&'static MemoryArea>,
    areas: MemoryAreaIter,
    kernel_range: RangeInclusive<Frame>,
    multiboot_range: RangeInclusive<Frame>,
}

impl AreaFrameAllocator {
    pub fn new(
        kernel_start: usize,
        kernel_end: usize,
        multiboot_start: usize,
        multiboot_end: usize,
        memory_areas: MemoryAreaIter,
    ) -> AreaFrameAllocator {
        let mut allocator = AreaFrameAllocator {
            next_free_frame: Frame::from_address(0 as usize),
            current_area: None,
            areas: memory_areas,
            kernel_range: RangeInclusive {
                start: Frame::from_address(kernel_start),
                end: Frame::from_address(kernel_end),
            },
            multiboot_range: RangeInclusive {
                start: Frame::from_address(multiboot_start),
                end: Frame::from_address(multiboot_end),
            },
        };
        allocator.choose_next_area();
        allocator
    }

    fn choose_next_free_frame(&mut self) {
        loop {
            let last_frame_in_current_area = if let Some(area) = self.current_area {
                let address = area.base_addr + area.length - 1;
                Frame::from_address(address as usize)
            } else {
                break;
            };

            if self.next_free_frame > last_frame_in_current_area {
                self.choose_next_area();
            } else if self.kernel_range
                .contains(Frame::new(self.next_free_frame.index))
            {
                println!("Frame allocation: Skip kernel range");
                self.next_free_frame.index = self.kernel_range.end.index + 1;
            } else if self.kernel_range
                .contains(Frame::new(self.next_free_frame.index))
            {
                println!("Frame allocation: Skip multiboot range");
                self.next_free_frame.index = self.multiboot_range.end.index + 1;
            } else {
                break;
            }
        }
    }

    fn choose_next_area(&mut self) {
        self.current_area = self.areas
            .clone()
            .filter(|area| {
                let address = area.base_addr + area.length - 1;
                Frame::from_address(address as usize) >= self.next_free_frame
            })
            .min_by_key(|area| area.base_addr);

        if let Some(area) = self.current_area {
            self.next_free_frame = Frame::from_address(area.base_addr as usize);
        } else {
            println!("current_area is None");
        }
    }
}

impl FrameAllocator for AreaFrameAllocator {
    fn allocate(&mut self) -> Option<Frame> {
        self.choose_next_free_frame();
        if let Some(_area) = self.current_area {
            let frame = Frame::new(self.next_free_frame.index);
            self.next_free_frame.index += 1;
            Some(frame)
        } else {
            None
        }
    }

    fn deallocate(&mut self, _frame: Frame) {
        unimplemented!()
    }
}
