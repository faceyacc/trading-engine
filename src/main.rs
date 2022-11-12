use std::{collections::HashMap};


#[derive(Debug)]
enum BidOrAsk {
    Bid, 
    Ask,
}


#[derive(Debug)]
struct Orderbook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}

impl Orderbook {
    fn new() -> Orderbook {
        Orderbook { 
            asks: HashMap::new(), 
            bids: HashMap::new()
        }
    }

    fn add_order(&mut self, price: f64, order: Order) {
        match order.bid_or_ask {
            BidOrAsk::Ask => {
                todo!()
            },
            BidOrAsk::Bid => {
                let  price = Price::new(price);    

                match self.bids.get_mut(&price) {
                    Some(limit) => {
                        limit.add_order(order);
                    }
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.bids.insert(price, limit);                        
                    },
                }
             },
        };
    }
}


#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64,
}

impl Price {
    fn new(price: f64) -> Price {
        let scalar = 100000;
        let integral = price as u64;
        let fractional = ((price % 1.0) * scalar as f64) as u64;
        Price { 
                integral: integral,
                fractional: fractional,
                scalar: scalar 
        }
    }
}

#[derive(Debug)]
struct Limit {
    price: Price,
    orders: Vec<Order>
}

impl Limit {
    fn new(price: Price) -> Limit {
        Limit { price,  orders: Vec::new() }
    }

    fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }
}

#[derive(Debug)]
struct Order {
    size: f64,
    bid_or_ask: BidOrAsk,
}

impl Order {
    fn new(bid_or_ask: BidOrAsk, size: f64) -> Order {
        Order { size, bid_or_ask }

    }
}

fn main() {
    let buy_order = Order::new(BidOrAsk::Bid, 5.5);
    let sell_order = Order::new(BidOrAsk::Ask, 25.5);

    let mut orderbook = Orderbook::new();

    orderbook.add_order(4.4, buy_order);
    
    println!("{:?}", orderbook);
}