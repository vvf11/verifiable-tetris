fn main() {
    println!("cargo:rerun-if-changed=build.rs"); // Пересобрать, если build.rs изменён
}
