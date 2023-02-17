use std::path::{Path, PathBuf};

pub enum FindUpKind {
    File,
    Dir,
}

pub struct FindUpOptions<'a> {
    pub cwd: &'a Path,
    pub kind: FindUpKind,
}

impl<'a> Default for FindUpOptions<'a> {
    fn default() -> Self {
        Self {
            cwd: Path::new("."),
            kind: FindUpKind::File,
        }
    }
}

#[inline]
/// Find a file by walking up parent directories from the current directory
/// at the time the function is invoked.
pub fn find_up<T: AsRef<Path>>(file_name: T) -> std::io::Result<Option<PathBuf>> {
    find_up_with(file_name, Default::default())
}

/// Find a file(default) or directory by walking up parent directories
pub fn find_up_with<T: AsRef<Path>>(
    file_name: T,
    options: FindUpOptions,
) -> std::io::Result<Option<PathBuf>> {
    let target_file_name = file_name.as_ref();
    let cwd_buf = std::env::current_dir().unwrap();
    let cwd = if options.cwd.eq(Path::new(".")) {
        Path::new(&cwd_buf)
    } else {
        options.cwd
    };
    let mut target_dir = Some(cwd);
    let is_search_dir = matches!(options.kind, FindUpKind::Dir);

    while let Some(dir) = target_dir {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if is_search_dir {
                    if let Some(file_name) = path.file_name() {
                        if target_file_name == file_name {
                            return Ok(Some(path));
                        }
                    }
                }
            } else if let Some(file_name) = path.file_name() {
                if target_file_name == file_name {
                    return Ok(Some(path));
                }
            }
            target_dir = dir.parent()
        }
    }
    Ok(None)
}
