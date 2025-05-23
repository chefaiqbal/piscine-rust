#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}

impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    pub fn insert_item(&mut self, s: &Store, ele: String) {
      
        if let Some(product) = s.products.iter().find(|(name, _)| *name == ele) {
      
            self.items.push((product.0.clone(), product.1));
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        if self.items.is_empty() {
            return Vec::new();
        }


        let mut prices: Vec<f32> = self.items.iter().map(|(_, price)| *price).collect();
        
        
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
       
        let free_items_count = self.items.len() / 3;
        
        
        let total: f32 = prices.iter().sum();
        
    
        let free_items_value: f32 = prices.iter().take(free_items_count).sum();
   
        let discount_factor = if total > 0.0 { 
            (total - free_items_value) / total 
        } else { 
            1.0 
        };
        
       
        let receipt: Vec<f32> = prices.iter()
            .map(|price| {
                let discounted = price * discount_factor;
                (discounted * 100.0).round() / 100.0  // Round to 2 decimal places
            })
            .collect();
        
    
        self.receipt = receipt.clone();
        receipt
    }
}

/*
You are going to make a shopping system. It will have a store where the products will be saved, and a cart which will contain items from which a receipt will be generated.

"Buy three, get one free".

The store is having a promotion. The cheapest of three items will be free. But there is a problem with the printer interface, it cannot receive any zero values. We can create a workaround. We will reduce all of the values in the cart by a small amount to show the correct total price. You can see the example to see how it works.

You will have to implement for the Cart structure the following functions:

new: that will initialize the cart.
insert_item: will receive a reference to Store and a String. Just like the name says, it will insert the item to the cart.
generate_receipt: returns a vector of sorted floats. This function must generate the receipt just like the example below, using the promotion. It should save the result in the receipt field.
Expected Function
#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    // expected public fields
}
impl Cart {
    pub fn new() -> Cart {}
    pub fn insert_item(&mut self, s: &Store, ele: String) {}
    pub fn generate_receipt(&mut self) -> Vec<f32> {}
}
Example
[1.23, 3.12, 23.1]` => `[1.17, 2.98, 22.06]
Because 1.17 + 2.98 + 22.06 == 0 + 3.12 + 23.1

Floats are rounded with a precision of two decimals which can create small discrepancies as per the example above.

This is a percentage calculation, and it can be applied to a set of three items. If the client purchases 9 items, they will receive three for free, with the discount applied to all items.

[1.23, 23.1, 3.12, 9.75, 1.75, 23.75, 2.75, 1.64, 15.23] => [1.16, 1.55, 1.65, 2.6, 2.94, 9.2, 14.38, 21.8, 22.42]
[3.12, 9.75, 1.75, 23.75, 2.75, 1.64, 15.23] => [1.54, 1.65, 2.59, 2.94, 9.18, 14.34, 22.36]
Hint: Closures are the way.

Usage
Here is a program to test your function,

use sales::*;

fn main() {
    let store = Store::new(vec![
        (String::from("product A"), 1.23),
        (String::from("product B"), 23.1),
        (String::from("product C"), 3.12)]);

    println!("{:?}", store);

    let mut cart = Cart::new();
    cart.insert_item(&store, String::from("product A"));
    cart.insert_item(&store, String::from("product B"));
    cart.insert_item(&store, String::from("product C"));

    println!("{:?}", cart.generate_receipt());

    println!("{:?}", cart);
}
And its output:

$ cargo run
Store { products: [("product A", 1.23), ("product B", 23.1), ("product C", 3.12)] }
[1.17, 2.98, 22.06]
Cart { items: [("product A", 1.23), ("product B", 23.1), ("product C", 3.12)], receipt: [1.17, 2.98, 22.06] }
$
Notions
closures
*/