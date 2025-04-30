# Rust FA-Iced

A basic [Font Awesome](https://fontawesome.com/) library written in [Rust](https://www.rust-lang.org/). The basic library functions can be used with any Rust UI framework, but the helper functions are primarily intended for use with the [iced](https://iced.rs/) UI framework. Maybe if I have time I will separate the iced helper functions into a feature.

Currently built against [iced](https://iced.rs/) version 0.13.1.

## Testing

```bash
cargo test                  # Run unit tests
cargo test -- --ignored     # Manually run integration tests
cargo test -- --nocapture   # Show the println!() output
```

## References

- [Rust](https://www.rust-lang.org/)
- [Font Awesome Icons](https://fontawesome.com/)
- [Iced Website](https://iced.rs/)
- [Iced Book](https://book.iced.rs/)
- This project loosely follows [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).
