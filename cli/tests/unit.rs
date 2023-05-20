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
        let Some(info) = result else { return };
        assert_eq!(info.data.len(), d);
        assert_eq!(info.plugins.len(), c);

        // parse full cfg for data dirs
        (in_path, d, c) = get_cfg_full();
        let result = parse_cfg(in_path);
        assert!(result.is_some());
        let Some(info) = result else { return };
        assert_eq!(info.data.len(), d);
        assert_eq!(info.plugins.len(), c);
    }

    #[test]
    fn test_manifest() {
        // parse cfg for data dirs
        let (in_path, d, c) = get_cfg();
        assert!(in_path.exists());
        let result = parse_cfg(in_path);
        assert!(result.is_some());
        let Some(info) = result else { return };
        assert_eq!(info.data.len(), d);
        assert_eq!(info.plugins.len(), c);
        // create a manifest
        let files = get_plugins(info.data, &info.plugins);
        assert_eq!(files.len(), info.plugins.len());
    }
}
