use guid_create::GUID;
use std::io::stdin;

#[cfg(windows)]
fn win_guid() -> String {
    use windows::Win32::System::Com::CoCreateGuid;

    let guid = unsafe { CoCreateGuid() }.unwrap();
    GUID::build_from_components(guid.data1, guid.data2, guid.data3, &guid.data4).to_string()
}

fn generate_account(phone: &str) -> String {
    let guid = win_guid().replace("-", "");
    let mut check = 0;
    for i in 0..3 {
        check += phone.chars().collect::<Vec<char>>()[i] as u32
            + guid.chars().collect::<Vec<char>>()[i] as u32;
    }
    format!("{}{:0<4}01{}", guid, check, phone).to_ascii_lowercase()
}

fn main() {
    println!("输入11位手机号");
    let mut phone = String::new();
    stdin().read_line(&mut phone).unwrap();
    println!("账户: {}", generate_account(phone.trim()));
    stdin().read_line(&mut phone).unwrap();
}
