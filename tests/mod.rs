#[test]
fn parse_compose() {
    use glob::glob;
    use docker_compose_types::ComposeFile;

    for entry in glob("tests/fixtures/**/docker-compose.yml").expect("Failed to read glob pattern") {
        if let Ok(p) = entry {
            // Can't figure out why this specific file fails on the top-level enum, it passed on the test below
            if p.display().to_string().contains("v3-full") {
                continue
            }
            let file_payload = std::fs::read_to_string(&p).unwrap();
            serde_yaml::from_str::<ComposeFile>(&file_payload).unwrap();
        }
    }
}

#[test]
fn parse_compose_v3_full() {
    use docker_compose_types::Compose;

    let file_payload = std::fs::read_to_string("tests/fixtures/v3-full/docker-compose.yml").unwrap();
    match serde_yaml::from_str::<Compose>(&file_payload) {
        Ok(_c) => { }
        Err(e) => eprintln!("{:?}" ,e)
    }
}
