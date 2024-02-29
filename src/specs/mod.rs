mod num;
mod option;
mod vec;

pub fn specs() {
    option::specs();
    vec::specs();
    num::specs();

    // tests! {
    //     header: "Function `foo`",
    //     ident: wrapping_add,
    //     contract! {
    //         header: "Property `bar`",
    //         inputs: [x: u8],
    //         precondition: x < 20,
    //         postcondition: eval(x + 20) == x + 20,
    //         test_vector: [0, 1, 2]
    //     }
    // }

    // tests! {
    //     header: "Function `foo`",
    //     ident: other,
    //     contract! {
    //         header: "Property `bar`",
    //         inputs: [x: u8, y: u8],
    //         precondition: x < 10 && y < 20,
    //         postcondition: eval(x + 20 + y) == y + x + 20,
    //     }
    // }
}
