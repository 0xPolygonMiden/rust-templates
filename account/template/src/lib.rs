// Do not link against libstd (i.e. anything defined in `std::`)
#![no_std]

// However, we could still use some standard library types while
// remaining no-std compatible, if we uncommented the following lines:
//
// extern crate alloc;
// use alloc::vec::Vec;

// Global allocator to use heap memory in no-std environment
#[global_allocator]
static ALLOC: BumpAlloc = miden::BumpAlloc::new();

// Required for no-std crates
#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

use miden::*;

struct Account;

impl Account {
    // Marking the function no_mangle ensures that it is exported
    // from the compiled binary as `receive_asset`, otherwise it would have
    // a mangled name that has no stable form.
    //
    // You can specify a different name from the library than the
    // name in the source code using the `#[export_name = "foo"]`
    // attribute, which will make the function callable as `foo`
    // externally (in this example)
    #[no_mangle]
    fn receive_asset(asset: CoreAsset) {
        miden::account::add_asset(asset);
    }

    #[no_mangle]
    fn send_asset(asset: CoreAsset, tag: Tag, note_type: NoteType, recipient: Recipient) {
        let asset = miden::account::remove_asset(asset);
        miden::tx::create_note(asset, tag, note_type, recipient);
    }
}
