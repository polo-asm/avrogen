mod compare;

use avrogen::Avrogen;
use compare::compare_folders_content;

fn standard_test(avrogen: Avrogen,source_name: &str, expected_folder_name: &str) {
    let source_file = format!("test_schemas/{}/schema.avsc", source_name);
    let dest_folder = format!("target/tmp/.result/{}/{}/", source_name, expected_folder_name);
    let expected_folder = format!("test_schemas/{}/expected_{}/", source_name, expected_folder_name);

    avrogen
        .add_source(source_file.as_str())
        .output_folder_from_str(dest_folder.as_str())
        .set_verbosity_debug()
        .execute()
        .expect("No error should appear");

    compare_folders_content(dest_folder.as_str(), expected_folder.as_str());
}

#[test]
fn convert_date_record_with_chrono() {
    let avrogen=Avrogen::new().set_flat_output();
    standard_test(avrogen, "date_record", "flat_chrono");
}
#[test]
fn convert_date_record_with_jiff() {
    let avrogen=Avrogen::new().set_flat_output().use_jiff();
    standard_test(avrogen, "date_record", "flat_jiff");
}

#[test]
fn convert_simple_enum() {
    let avrogen=Avrogen::new();
    standard_test(avrogen, "simple_enum", "structured");
}

#[test]
fn convert_simple_record() {
    let avrogen=Avrogen::new();
    standard_test(avrogen, "simple_record", "structured");
}

#[test]
fn convert_simple_record_flat() {
    let avrogen=Avrogen::new().set_flat_output();
    standard_test(avrogen, "simple_record", "flat");
}

#[test]
fn convert_recursive_record() {
    let avrogen=Avrogen::new();
    standard_test(avrogen, "recursive_record", "structured");
}

#[test]
fn convert_recursive_record_flat() {
    let avrogen=Avrogen::new().set_flat_output();
    standard_test(avrogen, "recursive_record", "flat");
}

#[test]
fn convert_type_with_reference() {
    let avrogen=Avrogen::new()
        .add_source("test_schemas/type_by_reference/*.avsc");
    standard_test(avrogen, "type_by_reference", "structured");
}

#[test]
fn convert_type_with_reference_flat() {
    let avrogen=Avrogen::new()
        .set_flat_output()
        .add_source("test_schemas/type_by_reference/*.avsc");
    standard_test(avrogen, "type_by_reference", "flat");
}