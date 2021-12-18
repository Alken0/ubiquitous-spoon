pub fn invalid_input<T>(message: String) -> std::io::Result<T> {
    Err(std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        message,
    ))
}

pub fn other<T>(message: String) -> std::io::Result<T> {
    Err(std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        message,
    ))
}
