type Pointer = *mut u8;

type ChunkSize = u8;

pub struct Arena {
  start: Pointer,
  end: Pointer,
  pub current_ptr_pos: Pointer,
  layout: std::alloc::Layout,
}

impl Arena {
  pub fn try_new(num_chunks: usize) -> Result<Self, std::alloc::LayoutError> {
    unsafe {
      match std::alloc::Layout::array::<ChunkSize>(num_chunks) {
        Ok(layout) => {
          let start = std::alloc::alloc(layout);

          let current_ptr_pos = start.clone();
          let end = start.add(num_chunks);

          Ok(Self {
            start,
            end,
            current_ptr_pos,
            layout,
          })
        }
        Err(e) => Err(e),
      }
    }
  }

  pub fn allocate(&mut self, item: u8) -> Option<Pointer> {
    match self.current_ptr_pos >= self.end {
      true => None,
      false => unsafe {
        let current_ptr = self.current_ptr_pos;
        core::ptr::write(current_ptr, item);

        self.current_ptr_pos = current_ptr.add(1);

        Some(current_ptr)
      },
    }
  }
}

impl Drop for Arena {
  fn drop(&mut self) {
    unsafe {
      std::alloc::dealloc(self.start, self.layout);
    }
  }
}

#[cfg(test)]
mod tests {
  use quickcheck::quickcheck;
  use std::assert_eq;

  use super::*;

  #[test]
  fn no_ops_on_zero_chunk_size() {
    let arena = Arena::try_new(0).unwrap();

    assert_eq!(arena.start, arena.end);
    assert_eq!(arena.start, arena.current_ptr_pos);
    assert_eq!(arena.end, arena.current_ptr_pos);
  }

  quickcheck! {
    fn prop_correctly_offsets_arena_end(num_chunks: usize) -> bool {
      if num_chunks > std::isize::MAX as usize {
        Arena::try_new(num_chunks).is_err()
      } else {
        let arena = Arena::try_new(num_chunks).unwrap();
        unsafe {
          let expected_end = arena.start.add(num_chunks);

          arena.end == expected_end
        }
      }
    }
  }

  #[test]
  fn allocation_returns_correct_pointer_at_start() {
    let mut arena = Arena::try_new(1).unwrap();

    let ptr = arena.allocate(12);

    assert_eq!(Some(arena.start), ptr);
  }

  #[test]
  fn can_allocate_multiple_chunks() {
    let mut arena = Arena::try_new(3).unwrap();

    let ptr1 = arena.allocate(1).unwrap();
    let ptr2 = arena.allocate(2).unwrap();
    let ptr3 = arena.allocate(3).unwrap();

    unsafe {
      println!("start ptr: {:?}", arena.start);
      println!(
        "[chunk 1] ptr: {:?}, content: {:?}",
        ptr1,
        std::ptr::read(ptr1)
      );
      println!(
        "[chunk 2] ptr: {:?}, content: {:?}",
        ptr2,
        std::ptr::read(ptr2)
      );
      println!(
        "[chunk 3] ptr: {:?}, content: {:?}",
        ptr3,
        std::ptr::read(ptr3)
      );
      println!("end ptr: {:?}", arena.end);
    }

    unsafe {
      assert_eq!(1, std::ptr::read(ptr1));
      assert_eq!(2, std::ptr::read(ptr2));
      assert_eq!(3, std::ptr::read(ptr3));
    }
  }

  #[test]
  fn cannot_allocate_more_than_block_size() {
    let mut arena = Arena::try_new(3).unwrap();

    let _ = arena.allocate(1);
    let _ = arena.allocate(2);
    let _ = arena.allocate(3);
    let ptr4 = arena.allocate(4);

    assert_eq!(None, ptr4);
  }
}
