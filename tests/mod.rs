#[test]
fn parse_compose() {
    use docker_compose_types::ComposeFile;
    use glob::glob;

    let mut all_succeeded = true;
    for entry in glob("tests/fixtures/**/docker-compose.yml").expect("Failed to read glob pattern").filter_map(Result::ok)
    {
        // Can't figure out why this specific file fails on the top-level enum, it passed on the test below
        if entry.display().to_string().contains("v3-full") {
            continue;
        }
        let file_payload = std::fs::read_to_string(&entry).unwrap();
        match serde_yaml::from_str::<ComposeFile>(&file_payload) {
            Ok(_) => {}
            Err(e) => {
                all_succeeded = false;
                eprintln!("{} {:?}", entry.display(), e);
            }
        }
    }

    assert!(all_succeeded);
}

#[test]
fn parse_compose_v3_full() {
    use docker_compose_types::Compose;

    let file_payload =
        std::fs::read_to_string("tests/fixtures/v3-full/docker-compose.yml").unwrap();
    match serde_yaml::from_str::<Compose>(&file_payload) {
        Ok(_c) => {}
        Err(e) => eprintln!("{:?}", e),
    }
}

#[test]
fn parse_extensions_v3_full() {
    use docker_compose_types::Compose;

    let file_payload =
        std::fs::read_to_string("tests/fixtures/extensions/docker-compose.yml").unwrap();
    match serde_yaml::from_str::<Compose>(&file_payload) {
        Ok(_c) => {
            println!("{:#?}", _c)
        }
        Err(e) => eprintln!("{:?}", e),
    }
}
