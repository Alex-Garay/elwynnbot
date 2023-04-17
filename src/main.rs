mod bootstapper;
fn main() -> std::io::Result<()> {
    bootstapper::bootstrap();
    Ok(())
}