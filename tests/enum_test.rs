mod compare;

use avrogen::Avrogen;
use compare::compare_folders_content;

fn standard_test(source_name: &str) {
    let source_file = format!("test_schemas/{}.avsc", source_name);
    let dest_folder = format!("target/tmp/.result/{}/", source_name);
    let expected_folder = format!("test_schemas/{}/expected/", source_name);

    Avrogen::new()
        .add_source(source_file.as_str())
        .output_folder_from_str(dest_folder.as_str())
        .set_verbosity_debug()
        .execute()
        .expect("No error should appear");

    compare_folders_content(dest_folder.as_str(), expected_folder.as_str());
}

#[test]
fn convert_simple_enum() {
    standard_test("simple_enum");
}

#[test]
fn convert_simple_record() {
    standard_test("simple_record");
}
