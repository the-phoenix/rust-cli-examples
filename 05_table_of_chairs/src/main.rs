use std::fmt::Display;
use cli_table::{Table, WithTitle, format::Justify, print_stdout};

struct Price(f32);
impl Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${:.2}", self.0)
    }
}

#[derive(Table)]
struct Chair {
    #[table(title="Name")]
    name: &'static str,

    #[table(title="Price")]
    price: Price,

    #[table(title="Color", justify="Justify::Center")]
    color: &'static str,

    #[table(title="Quantity", justify="Justify::Right")]
    quantity: u32
}

fn chairs() -> Vec<Chair> {
    vec![
        Chair {
            name: "Ergonomic Office Chair",
            price: Price(199.99),
            color: "Black",
            quantity: 20,
        },
        Chair {
            name: "Bucket Seat Gaming Chair",
            price: Price(249.99),
            color: "Turquoise",
            quantity: 3,
        },
        Chair {
            name: "Curl Swivel Accent Chair",
            price: Price(407.96),
            color: "Orange",
            quantity: 2,
        },
        Chair {
            name: "Velvet High Back Rocking Chair",
            price: Price(113.99),
            color: "Blue",
            quantity: 1,
        },
        Chair {
            name: "Velvet High Back Rocking Chair",
            price: Price(27.99),
            color: "Grey",
            quantity: 5,
        },
    ]
}
fn main() {
    let chairs = chairs();

    let _ = print_stdout(chairs.with_title());

    // Ok(())
}
