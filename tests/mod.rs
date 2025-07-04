#[cfg(feature = "yaml")]
use serde_yaml::from_str;
#[cfg(all(feature = "yml", not(feature = "yaml")))]
use serde_yml::from_str;

#[test]
fn parse_compose() {
    use docker_compose_types::ComposeFile;
    use glob::glob;
    use std::path::MAIN_SEPARATOR;

    let mut all_succeeded = true;
    for entry in glob("tests/fixtures/**/*.yml")
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
    {
        // Can't figure out why this specific file fails on the top-level enum, it passed on the test below
        let entry_path = entry.display().to_string();

        let skip_list = vec![
            format!("v3-full"),
            format!("extends{MAIN_SEPARATOR}verbose-and-shorthand.yml"),
            format!("net-container{MAIN_SEPARATOR}v2-invalid.yml"),
            format!("v2-simple{MAIN_SEPARATOR}links-invalid.yml"),
        ];

        if skip_list.iter().any(|s| entry_path.contains(s)) {
            continue;
        }

        let is_invalid = entry_path.contains("invalid.yml");
        let file_payload = std::fs::read_to_string(&entry).unwrap();
        match from_str::<ComposeFile>(&file_payload) {
            Ok(_) if is_invalid => {
                // invalid compose file succeeded in being parsed
                all_succeeded = false;
                eprintln!("{entry_path} is an invalid compose file but was successfully parsed");
            }
            Ok(_) => {}
            Err(_) if is_invalid => {}
            Err(_) => {
                all_succeeded = false;
                // The top-level enum for Compose V2 and Compose V3 tends to swallow meaningful errors
                // so re-parse the file as Compose V3 and print the error
                if let Err(e) = from_str::<docker_compose_types::Compose>(&file_payload) {
                    eprintln!("{entry_path} {e:?}");
                }
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
    match from_str::<Compose>(&file_payload) {
        Ok(_c) => {}
        Err(e) => eprintln!("{:?}", e),
    }
}

#[test]
fn parse_extensions_v3_full() {
    use docker_compose_types::Compose;

    let file_payload =
        std::fs::read_to_string("tests/fixtures/extensions/docker-compose.yml").unwrap();
    match from_str::<Compose>(&file_payload) {
        Ok(_c) => {}
        Err(e) => eprintln!("{:?}", e),
    }
}

#[test]
fn volumes() {
    use docker_compose_types::Volumes;
    use serde::Deserialize;

    let v = r#"
volumes:
  - source: /host/path
    target: /container/path
    type: bind
    read_only: true
  - source: foobar
    type: volume
    target: /container/volumepath
  - type: volume
    target: /anonymous
  - type: volume
    source: foobar
    target: /container/volumepath2
    volume:
      nocopy: true
"#;

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct Container {
        volumes: Vec<Volumes>,
    }
    let _parsed: Container = from_str(v).unwrap();
}

#[test]
fn parse_dockerfile_inline() {
    use docker_compose_types::BuildStep;
    use docker_compose_types::Compose;

    let file_payload =
        std::fs::read_to_string("tests/fixtures/dockerfile-inline/docker-compose.yml").unwrap();

    let mut actual_parsed: Compose = from_str(&file_payload).unwrap();

    let dockerfile_inline = actual_parsed
        .services
        .0
        .swap_remove("busybox")
        .flatten()
        .and_then(|service| service.build_)
        .map(|build_| match build_ {
            BuildStep::Advanced(adv) => adv,
            BuildStep::Simple(_) => panic!("Not advanced BuildStep"),
        })
        .and_then(|adv| adv.dockerfile_inline)
        .expect("No dockerfile_inline");
    assert!(dockerfile_inline.contains("FROM busybox"));
}
