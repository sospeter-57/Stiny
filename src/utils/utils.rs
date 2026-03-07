use chrono::{DateTime, Local, TimeZone};
use std::fs::Metadata;
use std::os::unix::fs::PermissionsExt;

pub fn get_type(mt: Metadata) -> String {
    if is_executable(&mt) {
        return "executable file".to_owned();
    } else if mt.is_file() {
        String::from("regular file")
    } else if mt.is_dir() {
        String::from("directory file")
    } else {
        String::from("symlink file")
    }
}

pub fn get_formatted_date<'a>(raw: i64) -> String {
    let datetime: DateTime<Local> = Local.timestamp_opt(raw, 0).unwrap();
    let formtaed_datetime = datetime.format("%Y-%m-%d  %H:%M:%S:%Z").to_string();
    return formtaed_datetime;
}

pub fn format_memory_blocks(block: u64) -> String {
    if block < 1024 {
        return format!("{} Bits", block);
    } else if block < 1048576 {
        let foo = if block % 1024 == 0 {
            block / 1024
        } else {
            block % 1024
        };
        return format!("{} Kilobytes", foo);
    } else if block < 1073741824 {
        let foo = if block % 1048576 == 0 {
            block / 1048576
        } else {
            block % 1048576
        };
        return format!("{} Megabytes", foo);
    } else if block < 1099511627776 {
        let foo = if block % 1073741824 == 0 {
            block / 1073741824
        } else {
            block % 1073741824
        };
        return format!("{} Gigabytes", foo);
    } else if block < 8796093022208 {
        let foo = if block % 1099511627776 == 0 {
            block / 1099511627776
        } else {
            block % 1099511627776
        };
        return format!("{} Terabytes", foo);
    } else {
        return String::from("blocks more than terabytes are unimplemented");
    }
}

pub fn is_executable(metadata: &Metadata) -> bool {
    let perms = metadata.permissions();
    let mode = perms.mode();
    if mode & 0o111 != 0 { true } else { false }
}
