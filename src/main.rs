use clean_arch::infrastructure::cli;

use clean_arch::adapter::ql::JsonFileApi;

fn main() {
    let db = clean_arch::db::json_file::JsonFile::try_new("test.json").unwrap();
    let api = JsonFileApi::new(&db);
    cli::run();
}
