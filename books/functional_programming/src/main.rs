mod intro_generics;
mod ownership;
mod errors;

fn main() {
    let (point_1, point_2) = (intro_generics::Point{x: 1, y: 2}, intro_generics::Point{x: 10, y: 20});

    let point_result = point_1.add(&point_2);

    assert_eq!(format!("{} + {} = {}", point_1, point_2, point_result), "(1, 2) + (10, 20) = (11, 22)");

    let point_translate = point_1.translate(2i32);

    assert_eq!(point_translate, intro_generics::Point{ x: 2i32, y: 4i32 });

    let point_translate = point_translate.execute_fn(|x| x.translate(2i32));

    assert_eq!(point_translate, intro_generics::Point{ x: 4i32, y: 8i32 }); 

    intro_generics::removing_warnings();
    intro_generics::dropping_structs();

    ownership::scope();
    ownership::string_type();
    ownership::string_double_free();
    ownership::string_double_free_solution();
    ownership::ownership_on_functions();
    ownership::ownership_on_functions_borrowing();
    ownership::mutable_references();
    ownership::mutable_references_no_more_than_one();
    ownership::mutable_references_in_diff_scopes_but_not_simultaneous();
    ownership::n_immutable_references();
    ownership::n_immutable_reference_sol();
    ownership::lifetime();
    ownership::lifetime_with_the_little_one();
    ownership::lifetime_with_only_one_parameter();
    ownership::lifetime_in_structs();
    ownership::first_word_test();
    ownership::testing_structs_foo();

    // errors::panicking();
    errors::file_not_found();
    errors::file_not_found_matching_error();
    // errors::unwrap_file_not_found();
}


