pub fn byte_to_string(bytes: &[u8]) -> String {
    // Turns Bytes into a String
    let binding = String::from_utf8_lossy(bytes);
    // Removes all the extra null zeros in the string. Example: "Wow.exe/0/0/0/0/0/0/0/0"
    let santized_binding = binding.split("\0").next().unwrap();
    santized_binding.to_owned()
}