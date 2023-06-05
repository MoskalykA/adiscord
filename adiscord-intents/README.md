# adiscord-intents

Calculate the number of intents from a list of enums

## Installation

Use the package manager [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) to install adiscord.

```bash
cargo add adiscord-intents
```

## Usage

```rust
use adiscord_intents::{Intent, generate_intent_number};

fn main() {
    let intents = vec![Intent::GuildInvites, Intent::GuildMembers];
    println!("Intent number -> {}", generate_intent_number(intents));
}
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

## License

[MIT](https://choosealicense.com/licenses/mit/)
