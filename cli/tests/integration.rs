#[cfg(test)]
mod integration_tests {
    use std::path::{Path, PathBuf};

    use common::parse_cfg;
    use omw_util::{cleanup, copy_files, get_plugins};
    use omw_util::{export, import};

    // path, data dirs, plugins
    fn get_cfg() -> (PathBuf, usize, usize) {
        (Path::new("tests/assets/openmw.cfg").into(), 3, 5)
    }
    fn get_out_cfg() -> (PathBuf, usize, usize) {
        (Path::new("tests/assets/openmw_out.cfg").into(), 2, 5)
    }

    fn setup_test_env(test_env: &Path) -> PathBuf {
        let data_files_path = test_env.join("Data Files");
        std::fs::create_dir_all(&data_files_path).expect("Failed setup test env: folders");
        std::fs::copy(
            Path::new("tests/assets/Morrowind.ini"),
            test_env.join("Morrowind.ini"),
        )
        .expect("Failed setup test env: ini");
        data_files_path
    }

    #[test]
    fn test_copy() {
        let test_env = Path::new("tests/integration/copy");
        let data_files_path = setup_test_env(test_env);
        let (in_path, d, c) = get_cfg();

        // parse cfg for data dirs
        let result = parse_cfg(in_path);
        assert!(result.is_some());
        let Some(info) = result else { return };
        assert_eq!(info.data.len(), d);
        assert_eq!(info.plugins.len(), c);

        // create a manifest
        let files = get_plugins(info.data, &info.plugins);
        assert_eq!(files.len(), info.plugins.len());

        // now copy the actual files
        let mut manifest = omw_util::Manifest::default();
        copy_files(&files, data_files_path.as_path(), &mut manifest, false);
        assert!(!manifest.files.is_empty());
        let count = manifest.files.len();
        assert_eq!(count, c);

        // destroy test environment
        std::fs::remove_dir_all(test_env).expect("Failed destroy test env");
    }

    #[test]
    fn test_export() {
        // setup test environment
        let test_env = Path::new("tests/integration/export");
        let data_files_path = setup_test_env(test_env);
        let (config_path, _d, c) = get_cfg();

        let result = export(Some(config_path), Some(data_files_path.to_owned()), false);
        assert_eq!(result, Some(c));

        // check order
        let cleanup = cleanup(&Some(data_files_path));
        assert_eq!(cleanup, Some(c));

        // destroy test environment
        std::fs::remove_dir_all(test_env).expect("Failed destroy test env");
    }

    #[test]
    fn test_import() {
        // setup test environment
        let test_env = Path::new("tests/integration/import");
        let data_files_path = setup_test_env(test_env);
        let (config_path, _d, c) = get_cfg();
        // export to set up test
        let result = export(Some(config_path), Some(data_files_path.clone()), false);
        assert_eq!(result, Some(c));

        // modify a file to test import
        let modified_esp = data_files_path.join("mod1.esp");
        assert!(std::fs::write(modified_esp, b"test").is_ok());

        // import
        let (p_out, d_out, c_out) = get_out_cfg();
        let result = import(Some(data_files_path), Some(p_out.clone()), true);
        assert!(result);

        // check cfg
        let result = parse_cfg(p_out);
        assert!(result.is_some());
        let Some(info) = result else { return };
        assert_eq!(info.data.len(), d_out);
        assert_eq!(info.plugins.len(), c_out);

        // destroy test environment
        std::fs::remove_dir_all(test_env).expect("Failed destroy test env");
    }
}
