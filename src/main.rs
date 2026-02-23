mod with_enum_dispatch;
mod with_enum_map_dispatch;
mod with_enum_map_trait_objects;
mod with_trait_objects;
mod old_fashioned;

fn main() {
    with_trait_objects::state_test();
    println!("======================================");
    with_enum_dispatch::state_test();
    println!("======================================");
    with_enum_map_trait_objects::state_test();
    println!("======================================");
    with_enum_map_dispatch::state_test();
    println!("======================================");
    old_fashioned::state_test();
}
