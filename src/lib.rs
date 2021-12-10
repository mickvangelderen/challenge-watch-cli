use std::path::Path;

pub fn watch<'a, I: IntoIterator<Item = &'a Path>>(paths: I) {
    for path in paths {
        // TODO: Actually watch these paths.
        println!(
            "{}: {:#?}",
            Path::new(path).display(),
            std::fs::metadata(path)
        );
    }
}
