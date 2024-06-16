#![allow(unused)]

mod module_03;
mod module_04;

use module_03::managing_memory_with_ownership;
use module_04::borrowing_values_by_reference;

fn main() {
    // Module 03 - Managing Memory with Ownership
    // managing_memory_with_ownership::copyable_data_types();
    // managing_memory_with_ownership::non_copyable_data_types();
    // managing_memory_with_ownership::copy_and_clone_traits();

    // Module 04 - Borrowing Values by Reference
    borrowing_values_by_reference::immutable_references();
}
