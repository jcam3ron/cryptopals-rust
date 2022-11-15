pub fn to_base64(bytes: &[u8]) -> String {

    return String::from_utf8_lossy(bytes).into_owned();
}