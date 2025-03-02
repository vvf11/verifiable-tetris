use vergen::EmitBuilder;

fn main() {
    EmitBuilder::builder()
        .build_timestamp()
        .git_sha(true)
        .emit()
        .unwrap();
}
