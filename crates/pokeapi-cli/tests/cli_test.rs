#[cfg(test)]
mod tests {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn cli_should_return_bulbasaur() {
        let mut cmd = Command::cargo_bin("pokeapi-cli").unwrap();

        cmd.arg("1");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("bulbasaur"));
    }

    #[test]
    fn cli_should_return_ivysaur() {
        let mut cmd = Command::cargo_bin("pokeapi-cli").unwrap();

        cmd.arg("2");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("ivysaur"));
    }

    #[test]
    fn cli_should_return_venusaur() {
        let mut cmd = Command::cargo_bin("pokeapi-cli").unwrap();

        cmd.arg("3");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("venusaur"));
    }
}
