use anyhow::{anyhow, Result};
use std::{collections::HashMap, fs, path::Path};

// Create a map of tids-vcpus from /proc/pid/task subdirectories
pub fn get_ch_vcpu_tids_by_path(proc_path: &str) -> Result<HashMap<u32, u32>> {
    let mut tids = HashMap::new();

    if proc_path.trim().is_empty() {
        return Err(anyhow!("proc path must be non-empty strings."));
    }

    let src = std::fs::canonicalize(proc_path)
        .map_err(|e| anyhow!("Invalid proc path: {proc_path}: {e}"))?;

    let tid_path = Path::new(&src).join("task");

    // Make the map of TIDs/VCPUs
    for entry in fs::read_dir(tid_path.clone())? {
        let entry = entry?;

        let tid_str = match entry.file_name().into_string() {
            Ok(id) => id,
            Err(_) => continue,
        };

        let tid = tid_str
            .parse::<u32>()
            .map_err(|e| anyhow!(e).context("invalid tid."))?;

        let comm_path = tid_path.join(tid_str.clone()).join("comm");

        if !comm_path.exists() {
            return Err(anyhow!("comm path was not found."));
        }

        let p_name = fs::read_to_string(comm_path)?;

        if !p_name.starts_with("vcpu") {
            continue;
        }

        let vcpu_id = p_name
            .trim_start_matches("vcpu")
            .trim()
            .parse::<u32>()
            .map_err(|e| anyhow!(e).context("Invalid vcpu id."))?;

        tids.insert(tid, vcpu_id);
    }

    if tids.is_empty() {
        return Err(anyhow!("The contents of proc path are not available."));
    }

    Ok(tids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tid_path() {
        #[derive(Debug)]
        struct TestData<'a> {
            proc_path: &'a str,
            result: Result<HashMap<u32, u32>>,
        }

        let tests = &[
            TestData {
                proc_path: "",
                result: Err(anyhow!("proc path must be non-empty strings.")),
            },
            TestData {
                proc_path: "/tmp/12345.6",
                result: Err(anyhow!(
                    "Invalid proc path: /tmp/12345.6: No such file or directory (os error 2)"
                )),
            },
            TestData {
                proc_path: "/usr/lib/os-release",
                result: Err(anyhow!("Not a directory (os error 20)")),
            },
            TestData {
                proc_path: "/proc/1",
                result: Err(anyhow!("The contents of proc path are not available.")),
            },
        ];

        // Run the tests
        for (i, d) in tests.iter().enumerate() {
            let msg = format!("test: [{}]: {:?}", i, d);

            if std::env::var("DEBUG").is_ok() {
                println!("DEBUG: {msg}");
            }

            let result = get_ch_vcpu_tids_by_path(d.proc_path);
            let msg = format!("{}, result: {:?}", msg, result);

            let expected_error = format!("{}", d.result.as_ref().unwrap_err());
            let actual_error = format!("{}", result.unwrap_err());
            assert!(actual_error == expected_error, "{}", msg);
        }
    }
}
