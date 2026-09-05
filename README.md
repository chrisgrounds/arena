# Arena

_This is an LLM free zone_

This is a byte-arena allocator for learning.

An arena is an API which pre-allocates a block of memory and allows the user to sequentially fill up the memory by moving a pointer forward when allocating new memory.

This arena is configurable on number of chunks and has the following API:

```rust
fn Arena::new(num_chunks: usize) -> Result<Arena, std::alloc::LayoutError>;
fn Arena::allocate(self, item: u8) -> *mut u8;
impl Drop for Arena;
```

`Arena::new` creates a new arena with `num_chunks` number of chunks. `Arena::allocate` allocates a byte into the arena and returns the pointer to that chunk. And dropping the arena interally calls the `Arena::deallocate` private function.

## Caveats / TODO

At the moment this arena is hardcoded to storing single bytes and does not take different types or sizes of objects to write to memory. It also only allows configuration of "number of chunks" which are all fixed-size. And even worse, once the user goes over the arena size, the `Arena::allocate` function simply returns `None` and does not create a new arena. Instead the arena should pre-allocate memory, allow different size "chunks", and create a new arena once the user goes over the size of the existing arena.
