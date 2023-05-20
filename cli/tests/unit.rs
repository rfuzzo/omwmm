#[cfg(test)]
mod unit_tests {
    use std::path::{Path, PathBuf};

    use common::parse_cfg;
    use omw_util::get_plugins;

    fn get_cfg() -> (PathBuf, usize, usize) {
        (Path::new("tests/assets/openmw.cfg").into(), 3, 5)
    }
    fn get_cfg_full() -> (PathBuf, usize, usize) {
        (Path::new("tests/assets/openmw_full.cfg").into(), 806, 578)
    }

    #[test]
    fn test_parse() {
        // parse cfg for data dirs
        let (mut in_path, mut d, mut c) = get_cfg();
        let result = parse_cfg(in_path);
        assert!(result.is_some());
        let Some((data_dirs ,plugin_names)) = result else { return };
        assert_eq!(data_dirs.len(), d);
        assert_eq!(plugin_names.len(), c);

        // parse full cfg for data dirs
        (in_path, d, c) = get_cfg_full();
        let result = parse_cfg(in_path);
        assert!(result.is_some());
        let Some((data_dirs ,plugin_names)) = result else { return };
        assert_eq!(data_dirs.len(), d);
        assert_eq!(plugin_names.len(), c);
    }

    #[test]
    fn test_manifest() {
        // parse cfg for data dirs
        let (in_path, d, c) = get_cfg();
        assert!(in_path.exists());
        let result = parse_cfg(in_path);
        assert!(result.is_some());
        let Some((data_dirs ,plugin_names)) = result else { return };
        assert_eq!(data_dirs.len(), d);
        assert_eq!(plugin_names.len(), c);
        // create a manifest
        let files = get_plugins(data_dirs, &plugin_names);
        assert_eq!(files.len(), plugin_names.len());
    }
}
