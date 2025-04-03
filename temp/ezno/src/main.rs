fn main() -> std::process::ExitCode {
    fn read_from_file(path: &std::path::Path) -> Option<String> {
        std::fs::read_to_string(path).ok()
    }
}
