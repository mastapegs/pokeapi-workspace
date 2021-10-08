#[cfg(test)]
mod tests {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn cli_should_return_bulbasaur() {
        let mut cli = Command::cargo_bin("pokeapi-cli").unwrap();

        cli.arg("1");
        cli.assert()
            .success()
            .stdout(predicate::str::contains("bulbasaur"));
    }

    #[test]
    fn cli_should_return_ivysaur() {
        let mut cli = Command::cargo_bin("pokeapi-cli").unwrap();

        cli.arg("2");
        cli.assert()
            .success()
            .stdout(predicate::str::contains("ivysaur"));
    }

    #[test]
    fn cli_should_return_venusaur() {
        let mut cli = Command::cargo_bin("pokeapi-cli").unwrap();

        cli.arg("3");
        cli.assert()
            .success()
            .stdout(predicate::str::contains("venusaur"));
    }
}
