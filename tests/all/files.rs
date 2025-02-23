use crate::helpers::{command, get_corpus_path};

#[test]
fn all_log_file_non_strict() {
    let input_path = get_corpus_path().join("all.log");

    let mut cmd = command();
    cmd.arg(input_path);

    cmd.assert().success().stdout(predicates::str::diff(
        r#"# levels
[2012-02-08T22:56:50.856Z] TRACE: myservice/123 on example.com: My message
[2012-02-08T22:56:51.856Z] DEBUG: myservice/123 on example.com: My message
[2012-02-08T22:56:52.856Z]  INFO: myservice/123 on example.com: My message
[2012-02-08T22:56:53.856Z]  WARN: myservice/123 on example.com: My message
[2012-02-08T22:56:54.856Z] ERROR: myservice/123 on example.com: My message
[2012-02-08T22:56:55.856Z] LVL55: myservice/123 on example.com: My message
[2012-02-08T22:56:56.856Z] FATAL: myservice/123 on example.com: My message

# extra fields
[2012-02-08T22:56:52.856Z]  INFO: myservice/123 on example.com: My message (one=short)
[2012-02-08T22:56:52.856Z]  INFO: myservice/123 on example.com: My message (two="short with space")
[2012-02-08T22:56:52.856Z]  INFO: myservice/123 on example.com: My message
    three: multi
    line
[2012-02-08T22:56:52.856Z]  INFO: myservice/123 on example.com: My message
    four: over 50 chars long long long long long long long long long
[2012-02-08T22:56:52.856Z]  INFO: myservice/123 on example.com: My message
    five: {
      "a": "json object"
    }
[2012-02-08T22:56:52.856Z]  INFO: myservice/123 on example.com: My message
    six: [
      "a",
      "json",
      "array"
    ]

# bogus
not a JSON line
{"hi": "there"}
"#,
    ));
}

#[test]
fn all_log_file_strict() {
    let input_path = get_corpus_path().join("all.log");

    let mut cmd = command();
    cmd.arg("--strict").arg(input_path);
    cmd.assert().success().stdout(predicates::str::diff(
        r#"[2012-02-08T22:56:50.856Z] TRACE: myservice/123 on example.com: My message
[2012-02-08T22:56:51.856Z] DEBUG: myservice/123 on example.com: My message
[2012-02-08T22:56:52.856Z]  INFO: myservice/123 on example.com: My message
[2012-02-08T22:56:53.856Z]  WARN: myservice/123 on example.com: My message
[2012-02-08T22:56:54.856Z] ERROR: myservice/123 on example.com: My message
[2012-02-08T22:56:55.856Z] LVL55: myservice/123 on example.com: My message
[2012-02-08T22:56:56.856Z] FATAL: myservice/123 on example.com: My message
[2012-02-08T22:56:52.856Z]  INFO: myservice/123 on example.com: My message (one=short)
[2012-02-08T22:56:52.856Z]  INFO: myservice/123 on example.com: My message (two="short with space")
[2012-02-08T22:56:52.856Z]  INFO: myservice/123 on example.com: My message
    three: multi
    line
[2012-02-08T22:56:52.856Z]  INFO: myservice/123 on example.com: My message
    four: over 50 chars long long long long long long long long long
[2012-02-08T22:56:52.856Z]  INFO: myservice/123 on example.com: My message
    five: {
      "a": "json object"
    }
[2012-02-08T22:56:52.856Z]  INFO: myservice/123 on example.com: My message
    six: [
      "a",
      "json",
      "array"
    ]
"#,
    ));
}

#[test]
fn try_to_open_a_non_regular_file() {
    let input_path = get_corpus_path();

    let mut cmd = command();
    cmd.arg(input_path);

    cmd.assert().failure().stderr(predicates::str::diff(
        "Error: the path provided doesn't point to a file.\n",
    ));
}
