use std::collections::HashMap;
use regex::Regex;
use crate::option;

fn group(file_paths: &Vec<String>) -> HashMap<String, Vec<String>> {
    let re = Regex::new(r"^(.*/)([^/]+)$").unwrap();

    let mut grouped_paths = HashMap::new();

    for paths in file_paths {
        if let Some(capture) = re.captures(paths) {
            let key = capture.get(1).unwrap().as_str().to_string();
            grouped_paths
                .entry(key)
                .or_insert(Vec::new())
                .push(paths.to_string());
        }
    }

    grouped_paths
}

pub fn get_file_groups() -> HashMap<String, Vec<String>> {
    let files = &option::INSTANCE.get().unwrap().files;
    group(files)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::files::group;

    #[test]
    fn test_case() {
        let files = vec![
            "./src/feature-x/user-list/i18n/en.json".to_string(),
            "./src/feature-x/user-list/i18n/ja.json".to_string(),
            "./src/feature-x/user-detail/i18n/en.json".to_string(),
            "./src/feature-x/user-detail/i18n/ja.json".to_string(),
            "./src/feature-y/user-list/i18n/en.json".to_string(),
            "./src/feature-y/user-list/i18n/ja.json".to_string(),
            "./src/deprecated/feature-x/user-list/i18n/en.json".to_string(),
            "./src/deprecated/feature-x/user-list/i18n/ja.json".to_string(),
        ];

        let mut expected = HashMap::new();
        expected.insert(
            "./src/feature-x/user-list/i18n/".to_string(),
            vec![
                "./src/feature-x/user-list/i18n/en.json".to_string(),
                "./src/feature-x/user-list/i18n/ja.json".to_string(),
            ],
        );
        expected.insert(
            "./src/feature-x/user-detail/i18n/".to_string(),
            vec![
                "./src/feature-x/user-detail/i18n/en.json".to_string(),
                "./src/feature-x/user-detail/i18n/ja.json".to_string(),
            ],
        );
        expected.insert(
            "./src/feature-y/user-list/i18n/".to_string(),
            vec![
                "./src/feature-y/user-list/i18n/en.json".to_string(),
                "./src/feature-y/user-list/i18n/ja.json".to_string(),
            ],
        );
        expected.insert(
            "./src/deprecated/feature-x/user-list/i18n/".to_string(),
            vec![
                "./src/deprecated/feature-x/user-list/i18n/en.json".to_string(),
                "./src/deprecated/feature-x/user-list/i18n/ja.json".to_string(),
            ],
        );
        assert_eq!(group(&files), expected);
    }
}
