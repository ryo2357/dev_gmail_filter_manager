use regex::Regex;
use serde_json::Value;
use std::io::{self, Write};
use std::process::Command;

pub struct Interface {
    cli_command: String,
    yaml_path: String,
}
enum ConvertAddress {
    From(String),
    Query(String),
}
impl Interface {
    pub fn create_from_env() -> anyhow::Result<Self> {
        let cli_command = std::env::var("CLI_COMMAND")?;
        let yaml_path = std::env::var("YAML_PATH")?;
        Ok(Self {
            cli_command,
            yaml_path,
        })
    }
    pub fn sync_from_yaml(&self) -> anyhow::Result<()> {
        let result = Command::new(&self.cli_command)
            .arg("--sync")
            .arg(&self.yaml_path)
            .output();

        let output = match result {
            Ok(t) => t,
            Err(r) => {
                println!("adb command error :{:?}", r);
                anyhow::bail!("adb command error :{:?}", r);
            }
        };

        println!("status:{:?}", output.status);
        println!("stdoutのデータ長:{:?}", output.stdout.len());
        println!("stderrのデータ長:{:?}", output.stderr.len());

        println!("stdout");
        io::stdout().write_all(&output.stdout).unwrap();
        println!("stderr");
        io::stderr().write_all(&output.stderr).unwrap();
        Ok(())
    }

    pub fn get_deleting_address(&self) -> anyhow::Result<(Vec<String>, Vec<String>)> {
        let result = Command::new(&self.cli_command)
            .arg("--dry-run")
            .arg("--sync")
            .arg(&self.yaml_path)
            .output();

        let output = match result {
            Ok(t) => t,
            Err(r) => {
                println!("adb command error :{:?}", r);
                anyhow::bail!("adb command error :{:?}", r);
            }
        };

        let mut from_list: Vec<String> = Vec::new();
        let mut query_list: Vec<String> = Vec::new();

        // データはなぜがoutput.stderrに入っている
        // デバッグ用
        println!("stdout");
        io::stdout().write_all(&output.stdout).unwrap();

        if output.stderr.is_empty() {
            println!("no deleting");
            return Ok((from_list, query_list));
        }

        let output_str = std::str::from_utf8(&output.stderr)?;
        let output_vec: Vec<String> = output_str.lines().map(|s| s.to_string()).collect();

        for line_string in output_vec.iter() {
            match self.convert_deleting_from_address(line_string) {
                Ok(ConvertAddress::From(t)) => from_list.push(t),
                Ok(ConvertAddress::Query(t)) => query_list.push(t),
                Err(r) => {
                    println!("error:convert_deleting_address:{:?}", r);
                }
            }
        }

        Ok((from_list, query_list))
    }

    // 検証用メソッド
    #[allow(dead_code)]
    pub fn command_test(&self) -> anyhow::Result<()> {
        let result = Command::new(&self.cli_command)
            .arg("--dry-run")
            .arg("--sync")
            .arg(&self.yaml_path)
            .output();

        let output = match result {
            Ok(t) => t,
            Err(r) => {
                println!("adb command error :{:?}", r);
                anyhow::bail!("adb command error :{:?}", r);
            }
        };

        println!("status:{:?}", output.status);
        println!("stdoutのデータ長:{:?}", output.stdout.len());
        println!("stderrのデータ長:{:?}", output.stderr.len());

        println!("stdout");
        io::stdout().write_all(&output.stdout).unwrap();
        println!("stderr");
        io::stderr().write_all(&output.stderr).unwrap();
        Ok(())
    }

    // クラス内メソッド
    // TODO:リファクタリングが必要
    fn convert_deleting_from_address(
        &self,
        output_line: &String,
    ) -> anyhow::Result<ConvertAddress> {
        if !output_line.starts_with("Deleting") {
            println!("not start Deleting :{}", output_line);
            anyhow::bail!("get_deleting_address error ");
        }
        let trimmed_data = output_line.trim_start_matches("Deleting ");
        let trimmed_data = trimmed_data.replace('\'', "\"");
        // println!("trimmed_data:{}", trimmed_data);
        let v: Value = serde_json::from_str(&trimmed_data)?;
        // println!("debug");
        let address: String = v["criteria"]["from"].to_string();
        if &address == "null" {
            let address: String = v["criteria"]["query"].to_string();
            println!("{}", address);
            let re = Regex::new(r"list:\((.+)\)").unwrap();
            let caps = re.captures(&address).unwrap();
            let address = caps[1].to_string();

            Ok(ConvertAddress::Query(address))
        } else {
            let address: String = address.trim_matches('\"').to_string();
            Ok(ConvertAddress::From(address))
        }
    }
}
