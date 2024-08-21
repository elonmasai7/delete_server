use std::fs;
use std::io;
use std::path::Path;

fn delete_everything(root_dir: &str) -> io::Result<()> {
    let root_path = Path::new(root_dir);
    let mut stack = vec![root_path.to_path_buf()];

    while let Some(path) = stack.pop() {
        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry_path = entry?.path();
                stack.push(entry_path);
            }
        } else {
            fs::remove_file(path)?;
        }
    }

    Ok(())
}
