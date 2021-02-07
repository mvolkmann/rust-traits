use std::fmt;

type Price = u64;

trait Priced {
    fn get_description(&self) -> &str;
    fn get_price(&self) -> Price;
}

struct Book {
    price: Price,
    title: String,
}

#[derive(Default)]
struct Cart {
    items: Vec<Box<dyn Priced>>,
}

impl Cart {
    fn add(&mut self, item: impl Priced + 'static) {
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
        let mut text = "".to_string();
        for boxed_item in self.items {
            let item = *boxed_item;
            text.push_str(&item.get_description());
            text.push_str(" $");
            text.push_str(&item.get_price().to_string());
            text.push('\n');
        }
        write!(f, "{}", text)
    }
}

impl Priced for Book {
    fn get_description(&self) -> &str {
        &self.title
    }
    fn get_price(&self) -> Price {
        self.price
    }
}

struct Food {
    caloriesPerServing: u32,
    description: String,
    price: Price,
}

impl Priced for Food {
    fn get_description(&self) -> &str {
        &self.description
    }
    fn get_price(&self) -> Price {
        self.price
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
        caloriesPerServing: 229,
        price: 75,
    });
    cart.add(Food {
        description: "Coke can".to_string(),
        caloriesPerServing: 140,
        price: 100,
    });
    println!("subtotal = {}", cart.get_subtotal() as f64 / 100.0);
}
