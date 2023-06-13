use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn helper(args: &[&str]) -> TestResult {
    let mut cmd = Command::cargo_bin("echo")?;
    let string_args = args
        .into_iter()
        .map(|a| a.to_string())
        .collect::<Vec<String>>();
    let expected = args
        .into_iter()
        .filter(|a| **a != "-n")
        .map(|a| a.to_string())
        .collect::<Vec<String>>();
    cmd.args(&string_args)
        .assert()
        .success()
        .stdout(expected.join(" ") + if args.contains(&"-n") {""} else {"\n"});
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
fn longer_hello() ->TestResult {
    // let mut cmd = Command::cargo_bin("echo")?;
    // let expected = "Hello There\n".to_string();
    // cmd.arg("Hello There")
    //     .assert()
    //     .success()
    //     .stdout(expected);
    // Ok(())
    helper(&["Hello There"])
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
fn vectorized_hello() ->TestResult {
    helper(&["abc", "dfg"])
}

#[test]
fn longer_hello_without_nl() ->TestResult {
    helper(&["Hello There", "-n"])
}

#[test]
fn vectorized_hello_without_nl() ->TestResult {
    helper(&["-n", "abc", "dfg", "-n"])
}
