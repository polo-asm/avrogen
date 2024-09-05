mod compare;
use avrogen::Avrogen;
use compare::compare_folders_content;

#[test]
fn convert_simple_enum() {
    Avrogen::new()
        .add_source("test_schemas/simple_enum.avsc")
        .output_folder_from_str("target/tmp/.result/simple_enum/")
        .set_verbosity_debug()
        .execute()
        .expect("No error should appear");

    compare_folders_content(
        "target/tmp/.result/simple_enum/",
        "test_schemas/simple_enum/expected/",
    );
}

#[test]
fn convert_simple_record() {
    Avrogen::new()
        .add_source("test_schemas/simple_record.avsc")
        .output_folder_from_str("target/tmp/.result/simple_record/")
        .set_verbosity_debug()
        .execute()
        .expect("No error should appear");

    compare_folders_content(
        "target/tmp/.result/simple_record/",
        "test_schemas/simple_record/expected/",
    );
}
