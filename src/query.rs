use std::{
    fmt::Display,
    io::{Error, ErrorKind, Result},
};

pub struct ProcessInfo {
    pub name: String,
    pub pid: String,
    pub cmdl: String,
}

impl Display for ProcessInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Process: {} ({})\nCommandLine: {}",
            self.name, self.pid, self.cmdl
        )
    }
}

#[cfg(windows)]
pub fn query_process() -> Result<Vec<ProcessInfo>> {
    use std::collections::HashMap;

    use wmi::{COMLibrary, Variant, WMIConnection, WMIError};

    let err_conv = |err: WMIError| Error::new(ErrorKind::Other, err.to_string());

    let com_con = COMLibrary::new().map_err(err_conv)?;
    let wmi_con = WMIConnection::new(com_con).map_err(err_conv)?;

    let wmi_results: Vec<HashMap<String, Variant>> = wmi_con
        .raw_query(
            r#"SELECT Name,ProcessID,CommandLine FROM Win32_Process WHERE name like '%java%'"#,
        )
        .map_err(err_conv)?;
    Ok(wmi_results
        .into_iter()
        .filter_map(|e| {
            let mut pid = String::new();
            let mut name = String::new();
            let mut cmdl = String::new();

            if let Variant::String(value) = &e["CommandLine"] {
                if !value.contains("minecraft") {
                    return None;
                }

                cmdl = value.clone();
            }

            if let Variant::UI4(value) = &e["ProcessId"] {
                pid = value.to_string();
            };
            if let Variant::String(value) = &e["Name"] {
                name = value.clone();
            };

            Some(ProcessInfo { pid, name, cmdl })
        })
        .collect())
}
