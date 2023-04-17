mod Bootstapper;
fn main() -> std::io::Result<()> {
    Bootstapper::bootstrap();
    Ok(())
}