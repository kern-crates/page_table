# page_table

This crate provides generic, unified, architecture-independent, and OS-free page table structures for various hardware architectures.

The core struct is PageTable64<M, PTE, IF>. OS-functions and architecture-dependent types are provided by generic parameters:

+ M: The architecture-dependent metadata, requires to implement the PagingMetaData trait.
+ PTE: The architecture-dependent page table entry, requires to implement the GenericPTE trait.
+ IF: OS-functions such as physical memory allocation, requires to implement the PagingIf trait.
Currently supported architectures and page table structures:

+ x86: x86_64::X64PageTable
+ ARM: aarch64::A64PageTable
+ RISC-V: riscv::Sv39PageTable, riscv::Sv48PageTable

## Examples

```rust
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use axhal::mem::{memory_regions, phys_to_virt};

#[macro_use]
extern crate axlog2;

#[cfg(target_arch = "riscv64")]
const TEST_ADDRESS: usize = 0x1000_1000;

#[cfg(target_arch = "x86_64")]
const TEST_ADDRESS: usize = 0xfec00000;

#[no_mangle]
pub extern "Rust" fn runtime_main(cpu_id: usize, _dtb_pa: usize) {
    axlog2::init("debug");
    info!("[rt_page_table]: ...");

    axhal::arch_init_early(cpu_id);
    axalloc::init();
    page_table::init();

    info!("Found physcial memory regions:");
    for r in memory_regions() {
        info!(
            "  [{:x?}, {:x?}) {} ({:?})",
            r.paddr,
            r.paddr + r.size,
            r.name,
            r.flags
        );
    }

    // Try to access virtio_mmio space.
    let va = phys_to_virt(TEST_ADDRESS.into()).as_usize();
    let ptr = va as *const u32;
    unsafe {
        info!("Try to access virtio_mmio [{:#X}]", *ptr);
    }

    info!("[rt_page_table]: ok!");
    axhal::misc::terminate();
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    arch_boot::panic(info)
}

```

## Re-exports

### `page_table_entry::GenericPTE`

```rust
pub trait GenericPTE: Debug + Clone + Copy + Sync + Send + Sized {
    // Required methods
    fn new_page(paddr: PhysAddr, flags: MappingFlags, is_huge: bool) -> Self;
    fn new_table(paddr: PhysAddr) -> Self;
    fn paddr(&self) -> PhysAddr;
    fn flags(&self) -> MappingFlags;
    fn set_paddr(&mut self, paddr: PhysAddr);
    fn set_flags(&mut self, flags: MappingFlags, is_huge: bool);
    fn is_unused(&self) -> bool;
    fn is_present(&self) -> bool;
    fn is_huge(&self) -> bool;
    fn clear(&mut self);
}
```

A generic page table entry.
All architecture-specific page table entry types implement this trait.

### `page_table_entry::MappingFlags`

```rust
pub struct MappingFlags(/* private fields */);
```

Generic page table entry flags that indicate the corresponding mapped memory region permissions and attributes.

## Modules

### `aarch64`

AArch64 specific page table structures.

#### Structs

##### `A64PagingMetaData`

```rust
pub struct A64PagingMetaData;
```

Metadata of AArch64 page tables.

#### Type Aliases

##### `A64PageTable`

```rust
pub type A64PageTable<I> = PageTable64<A64PagingMetaData, A64PTE, I>;
```

AArch64 VMSAv8-64 translation table.

### `paging`

Page table manipulation.

#### Re-exports

##### `crate::MappingFlags`

```rust
pub struct MappingFlags(/* private fields */);
```

Generic page table entry flags that indicate the corresponding mapped memory region permissions and attributes.

##### `crate::PageSize`

```rust
#[repr(usize)]
pub enum PageSize {
    Size4K = 4_096,
    Size2M = 2_097_152,
    Size1G = 1_073_741_824,
}
```

The page sizes supported by the hardware page table.

##### `crate::PagingError`

```rust
pub enum PagingError {
    NoMemory,
    NotAligned,
    NotMapped,
    AlreadyMapped,
    MappedToHugePage,
}
```

The error type for page table operation failures.

##### `crate::PagingResult`

```rust
pub type PagingResult<T = ()> = Result<T, PagingError>;
```

The specialized Result type for page table operations.

#### Structs

##### `PagingIfImpl`

```rust
pub struct PagingIfImpl;
```

Implementation of PagingIf, to provide physical memory manipulation to the [page_table] crate.

#### Type Aliases

##### `PageTable`

```rust
pub type PageTable = X64PageTable<PagingIfImpl>;
```

The architecture-specific page table.

### `riscv`

RISC-V specific page table structures.

#### Structs

##### `Sv39MetaData`

Metadata of RISC-V Sv39 page tables.

```rust
pub struct Sv39MetaData;
```

##### `Sv48MetaData`

```rust
pub struct Sv48MetaData;
```

Metadata of RISC-V Sv48 page tables.

#### Type Aliases

##### `Sv39PageTable`

```rust
pub type Sv39PageTable<I> = PageTable64<Sv39MetaData, Rv64PTE, I>;
```

Sv39: Page-Based 39-bit (3 levels) Virtual-Memory System.

**Aliased Type**

```rust
struct Sv39PageTable<I> { /* private fields */ }
```

##### `Sv48PageTable`

```rust
pub type Sv48PageTable<I> = PageTable64<Sv48MetaData, Rv64PTE, I>;
```

Sv48: Page-Based 48-bit (4 levels) Virtual-Memory System.

**Aliased Type**

```rust
struct Sv48PageTable<I> { /* private fields */ }
```

### `x86_64`

x86 specific page table structures.

#### Structs

##### `X64PagingMetaData`

```rust
pub struct X64PagingMetaData;
```

metadata of x86_64 page tables.

#### Type Aliases

##### `X64PageTable`

```rust
pub type X64PageTable<I> = PageTable64<X64PagingMetaData, X64PTE, I>;
```

x86_64 page table.

**Aliased Type**

```rust
struct X64PageTable<I> { /* private fields */ }
```

## Structs

### `PageTable64`

```rust
pub struct PageTable64<M: PagingMetaData, PTE: GenericPTE, IF: PagingIf> { /* private fields */ }
```

A generic page table struct for 64-bit platform.
It also tracks all intermediate level tables. They will be deallocated When the PageTable64 itself is dropped.

## Enums

### `PageSize`

```rust
#[repr(usize)]
pub enum PageSize {
    Size4K = 4_096,
    Size2M = 2_097_152,
    Size1G = 1_073_741_824,
}
```

The page sizes supported by the hardware page table.

### `PagingErro`r

```rust
pub enum PagingError {
    NoMemory,
    NotAligned,
    NotMapped,
    AlreadyMapped,
    MappedToHugePage,
}
```

The error type for page table operation failures.

## Traits

### `PagingIf`

```rust
pub trait PagingIf: Sized {
    // Required methods
    fn alloc_frame() -> Option<PhysAddr>;
    fn dealloc_frame(paddr: PhysAddr);
    fn phys_to_virt(paddr: PhysAddr) -> VirtAddr;
}
```

The low-level OS-dependent helpers that must be provided for PageTable64.

### `PagingMetaData`

```rust
pub trait PagingMetaData: Sync + Send + Sized {
    const LEVELS: usize;
    const PA_MAX_BITS: usize;
    const VA_MAX_BITS: usize;
    const PA_MAX_ADDR: usize = _;

    // Provided methods
    fn paddr_is_valid(paddr: usize) -> bool { ... }
    fn vaddr_is_valid(vaddr: usize) -> bool { ... }
}
```

The architecture-dependent metadata that must be provided for PageTable64.

## Type Aliases

### `PagingResult`

```rust
pub type PagingResult<T = ()> = Result<T, PagingError>;
```

The specialized Result type for page table operations.

#### Aliased Type

```rust
enum PagingResult<T = ()> {
    Ok(T),
    Err(PagingError),
}
```

The specialized Result type for page table operations.
