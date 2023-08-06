use std::sync::Mutex;
use serde::{Serialize, Deserialize};


#[derive(Serialize)]
pub struct Ping {
    pub ping: &'static str
}


#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct FoodStruct {
    pub flavor: String,
    pub description: Option<String>,
    pub price: f32
}

impl FoodStruct {
    fn default() -> Self {
        Self { flavor: "".to_string(), description: None, price: 0.0 }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Food {
    Burger(FoodStruct),
    Pizza(FoodStruct)
}

impl Food {
    pub fn default(food: String) -> Self {
        let default_food_values = FoodStruct::default();
        match food.as_str()  {
            "burger" => Food::Burger(default_food_values),
            "pizza" => Food::Pizza(default_food_values),
            _ => unimplemented!()
        }
    }

    pub fn create(self, food: FoodStruct) -> Self {
        match self {
            Food::Burger(_) => Food::Burger(food),
            Food::Pizza(_) => Food::Pizza(food)
        }
    }
}


#[derive(Serialize)]
pub struct MenuView {
    pub pizzas: Vec<Food>,
    pub burgers: Vec<Food>,
}

// impl MenuView {
//     fn default() -> Self {
//         MenuView {
//             pizzas: vec![Food::default("pizza".to_string())],
//             burgers: vec![Food::default("burger".to_string())]
//         }
//     }
// }


#[derive(Default)]
pub struct MenuState {
    pub pizzas: Mutex<Vec<Food>>,
    pub burgers: Mutex<Vec<Food>>,
}

// impl MenuState {
//     fn default() -> Self {
//         let default_pizzas = Food::default("pizza".to_string());
//         let default_burgers = Food::default("burger".to_string());

//         let mutexed_pizzas = Mutex::new(vec![default_pizzas]);
//         let mutexed_burgers = Mutex::new(vec![default_burgers]);


//         MenuState { pizzas: mutexed_pizzas, burgers: mutexed_burgers }
//     }
// }


// pub trait FoodTrait {

//     fn set_self(&self, flavor: String, description: Option<String>, price: f32) -> Self;

//     fn add(&mut self, flavor: String, description: Option<String>, price: f32) {
        
//         self.set_self(flavor, description, price);
//     }   
// }

// impl FoodTrait for Food {

//     fn set_self(&self, flavor: String, description: Option<String>, price: f32) -> Self {
//         match self {
//             Food::Pizza(pizza) => {
//                 let pizza = FoodStruct {flavor, description, price};
//                 Food::Pizza(pizza)
//             },
//             Food::Burger(burger) => {
//                 let burger = FoodStruct {flavor, description, price};
//                 Food::Burger(burger)
//             }
//         }
//     }
// }




// #[derive(Serialize, Deserialize, Debug, Clone, Default)]
// pub struct Pizza {
//     pub flavor: String,
//     pub description: Option<String>,
//     pub price: f32
// }

// impl FoodTrait for Pizza {
//     fn default() -> Self {
//         Default::default()
//     }

//     fn set_self(&self, flavor: String, description: Option<String>, price: f32) -> Self {
//         self.flavor = flavor;
//         self.description = description;
//         self.price = price;

//         *self
//     }
// }

// impl Pizza {

//     pub fn add_pizza(&mut self, flavor: String, description: Option<String>, price: f32) -> &mut Pizza {

//         let updated_default = self.add(flavor, description, price);

//         updated_default
//     }
// }


// #[derive(Serialize, Deserialize, Debug, Clone, Default)]
// pub struct Burger {
//     pub flavor: String,
//     pub description: Option<String>,
//     pub price: f32
// }

// impl FoodTrait for Burger {
//     fn set_self(&mut self, flavor: String, description: Option<String>, price: f32) {
//         self.flavor = flavor;
//         self.description = description;
//         self.price = price;
//     }
// }

// impl Burger {

//     pub fn add_burger(&mut self, flavor: String, description: Option<String>, price: f32) -> &mut Burger {

//         let updated_default = self.add(flavor, description, price);

//         updated_default
//     }
// }




