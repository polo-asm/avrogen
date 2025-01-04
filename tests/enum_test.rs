mod compare;

use avrogen::Avrogen;
use compare::compare_folders_content;

fn standard_structured_test(source_name: &str) {
    let source_file = format!("test_schemas/{}/schema.avsc", source_name);
    let dest_folder = format!("target/tmp/.result/{}/structured/", source_name);
    let expected_folder = format!("test_schemas/{}/expected_structured/", source_name);

    Avrogen::new()
        .add_source(source_file.as_str())
        .output_folder_from_str(dest_folder.as_str())
        .set_verbosity_debug()
        .execute()
        .expect("No error should appear");

    compare_folders_content(dest_folder.as_str(), expected_folder.as_str());
}

fn standard_flat_test(source_name: &str) {
    let source_file = format!("test_schemas/{}/schema.avsc", source_name);
    let dest_folder = format!("target/tmp/.result/{}/flat/", source_name);
    let expected_folder = format!("test_schemas/{}/expected_flat/", source_name);

    Avrogen::new()
        .add_source(source_file.as_str())
        .output_folder_from_str(dest_folder.as_str())
        .set_flat_output()
        .set_verbosity_debug()
        .execute()
        .expect("No error should appear");

    compare_folders_content(dest_folder.as_str(), expected_folder.as_str());
}

#[test]
fn convert_simple_enum() {
    standard_structured_test("simple_enum");
}

#[test]
fn convert_simple_record() {
    standard_structured_test("simple_record");
}

#[test]
fn convert_simple_record_flat() {
    standard_flat_test("simple_record");
}

#[test]
fn convert_recursive_record() {
    standard_structured_test("recursive_record");
}

#[test]
fn convert_recursive_record_flat() {
    standard_flat_test("recursive_record");
}
