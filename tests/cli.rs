use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn helper(args: &[&str], nl_override: bool) -> TestResult {
    let mut cmd = Command::cargo_bin("echo")?;
    // let string_args = args
    //     .into_iter()
    //     .map(|a| a.to_string())
    //     .collect::<Vec<String>>();
    let expected = args
        .into_iter()
        .filter(|a| **a != "-n" && !nl_override)
        .map(|a| a.to_string())
        .collect::<Vec<String>>();
    cmd.args(args)
        .assert()
        .success()
        .stdout(expected.join(" ") + if nl_override {"\n"}
            else if args.contains(&"-n") {""}
            else {"\n"});
    Ok(())
}

#[test]
fn no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echo")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn one_arg() -> TestResult {
    let mut cmd = Command::cargo_bin("echo")?;
    cmd.arg("hello")
        .assert()
        .success();
    Ok(())
}

#[test]
fn longer_hello() -> TestResult {
    // let mut cmd = Command::cargo_bin("echo")?;
    // let expected = "Hello There\n".to_string();
    // cmd.arg("Hello There")
    //     .assert()
    //     .success()
    //     .stdout(expected);
    // Ok(())
    helper(&["Hello There"], false)
}

// #[test]
// fn vectorized_hello() ->TestResult {
//     let mut cmd = Command::cargo_bin("echo")?;
//     let expected = "Hello There\n".to_string();
//     cmd.arg(vec!["Hello", "There"])
//         .assert()
//         .success()
//         .stdout(expected);
//     Ok(())
// }

#[test]
fn vectorized_hello() -> TestResult {
    helper(&["abc", "dfg"], false)
}

#[test]
fn longer_hello_without_nl() -> TestResult {
    helper(&[ "-n", "Hello There"], false)
}

#[test]
fn vectorized_hello_without_nl() -> TestResult {
    helper(&["-n", "abc", "dfg"], false)
}

#[test]
fn kept_spaces_with_nl() -> TestResult {
    helper(&["abc    ", " dfg"], false)
}

#[test]
fn kept_spaces_without_nl() -> TestResult {
    helper(&[ "-n", "abc     ", "dfg  "], false)
}

#[test]
fn only_n() -> TestResult {
    let mut cmd = Command::cargo_bin("echo")?;
    cmd.arg("-n")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn only_space() -> TestResult {
    helper(&[" "], false)
}

#[test]
fn unknown_args() -> TestResult {
    helper(&["-trf"], false)
}

#[test]
fn unknown_args_without_nl() -> TestResult {
    helper(&["-n", "-trfn"], false)
}
