pub fn byte_to_string(bytes: &[u8]) -> String {
    // Turns Bytes into a String
    let binding = String::from_utf8_lossy(bytes);
    // Removes null 0 bytes from the string.
    let santized_binding = binding.split("\0").next().unwrap();
    santized_binding.to_owned()
}