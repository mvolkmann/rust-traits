use std::fmt;

type Price = u64;

trait Item {
    fn get_description(&self) -> &str;
    fn get_price(&self) -> Price;
}

struct Book {
    price: Price,
    title: String,
}

impl Item for Book {
    fn get_description(&self) -> &str {
        &self.title
    }
    fn get_price(&self) -> Price {
        self.price
    }
}

struct Food {
    calories_per_serving: u32,
    description: String,
    price: Price,
}

impl Item for Food {
    fn get_description(&self) -> &str {
        &self.description
    }
    fn get_price(&self) -> Price {
        self.price
    }
}

#[derive(Default)]
struct Cart {
    // A Cart holds any kind of items that implement the Item trait.
    items: Vec<Box<dyn Item>>,
}

impl Cart {
    // Each Item added to a Cart is guaranteed
    // to live for the duration of the program.
    fn add(&mut self, item: impl Item + 'static) {
        self.items.push(Box::new(item));
    }

    fn get_subtotal(&self) -> Price {
        self.items
            .iter()
            .fold(0, |acc, item| acc + item.get_price())
    }
}

impl fmt::Display for Cart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in &self.items {
            // We don't need to manually extract items from their Box.
            writeln!(f, "{} ${}", item.get_description(), item.get_price())?;
        }
        Ok(())
    }
}

fn main() {
    let mut cart = Cart::default();

    let item = Book {
        title: "Svelte and Sapper in Action".to_string(),
        price: 2000,
    };
    cart.add(item);
    cart.add(Food {
        description: "Snickers bar".to_string(),
        calories_per_serving: 229,
        price: 75,
    });
    cart.add(Food {
        description: "Coke can".to_string(),
        calories_per_serving: 140,
        price: 100,
    });
    println!("subtotal = {}", cart.get_subtotal() as f64 / 100.0);
}
