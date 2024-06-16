#![allow(unused)]

mod module_03;
mod module_04;
mod module_05;

use module_03::managing_memory_with_ownership;
use module_04::borrowing_values_by_reference;
use module_05::using_lifetimes_to_reduce_ambiguity;
use module_05::using_lifetimes_to_reduce_ambiguity_v2;

fn main() {
    // Module 03 - Managing Memory with Ownership
    // managing_memory_with_ownership::copyable_data_types();
    // managing_memory_with_ownership::non_copyable_data_types();
    // managing_memory_with_ownership::copy_and_clone_traits();

    // Module 04 - Borrowing Values by Reference
    // borrowing_values_by_reference::immutable_references();
    // borrowing_values_by_reference::mutable_references();
    // borrowing_values_by_reference::string_slices();

    // Module 05 - Using Lifetimes to Reduce Ambiguity
    // using_lifetimes_to_reduce_ambiguity::lifetimes_in_functions();
    // using_lifetimes_to_reduce_ambiguity_v2::lifetimes_in_structs();
    using_lifetimes_to_reduce_ambiguity_v2::lifetime_elision_rules();
}
