use actix_web::{get, post, delete, put, web, Responder};

use crate::models::{Ping, MenuState, Food, MenuView, FoodStruct};


#[get("/")]
async fn index() -> impl Responder {
    println!("in the index route");
    "Hello World!"
}



#[get("/ping")]
async fn ping() -> web::Json<Ping> {
    web::Json(
        Ping { ping: "pong" }
    )
}

#[get("/menu")]
async fn read_menu(data: web::Data<MenuState>) -> web::Json<MenuView> {

    let pizzas = data.pizzas.lock().unwrap();
    let burgers = data.burgers.lock().unwrap();
    

    web::Json(
        MenuView { pizzas: pizzas.clone(), burgers: burgers.clone() }
    )
}

#[get("/menu/{food}")]
async fn read_food(
    data: web::Data<MenuState>,
    path: web::Path<String>
) -> impl Responder {

    let foodlist = match path.into_inner().as_str() {
        "pizza" => data.pizzas.lock().unwrap(),
        "burger" => data.burgers.lock().unwrap(),
        _ => unimplemented!()
    };
    

    web::Json(
        foodlist.clone()
    )
}

// # Restaurant

#[post("/menu/add/{food}")]
pub async fn add_food(
    data: web::Data<MenuState>,
    path: web::Path<String>,
    info: web::Query<FoodStruct>
) -> impl Responder {

    let food_type = path.into_inner();


    let food_vals = FoodStruct {
        flavor: info.flavor.clone(),
        description: info.description.clone(),
        price: info.price
    };

    
    let food = Food::default(food_type.clone()).create(food_vals);

    let mut food_state = match food_type.as_str() {
        "burger" => data.burgers.lock().unwrap(),
        "pizza" => data.pizzas.lock().unwrap(),
        _ => unimplemented!()
    };

    food_state.push(food);


    let res = format!("added {}", food_type);

    res

}

#[put("/menu/update/{food}")]
pub async fn update_food(
    data: web::Data<MenuState>,
    path: web::Path<String>,
    info: web::Query<FoodStruct>,
) -> impl Responder {

    let food_type = path.into_inner();

    let mut food_list = match food_type.as_str() {
        "pizza" => data.pizzas.lock().unwrap(),
        "burger" => data.burgers.lock().unwrap(),
        _ => unimplemented!()
    };
    

    for item in food_list.iter_mut() {
        match item {
            Food::Pizza(st) => {
                if st.flavor == info.flavor {
                    st.description = info.description.clone();
                    st.price = info.price;
                }
            },
            Food::Burger(st) => {
                if st.flavor == info.flavor {
                    st.description = info.description.clone();
                    st.price = info.price;
                }
            },
        }
    }


    let res = format!("updated {}", food_type);

    res

}

#[delete("/menu/delete/{food}/{flavor}")]
pub async fn delete_food(
    data: web::Data<MenuState>,
    path: web::Path<(String, String)>
) -> impl Responder {

    let (food_type, food_flavor) = path.into_inner();

    let mut food_list = match food_type.as_str() {
        "pizza" => data.pizzas.lock().unwrap(),
        "burger" => data.burgers.lock().unwrap(),
        _ => unimplemented!()
    };
    

    for item in food_list.clone().iter() {
        match item {
            Food::Pizza(st) => {
                if st.flavor == food_flavor {
                    let idx = food_list.iter().position(|i| i == item);
                    food_list.remove(idx.unwrap());
                }
            },
            Food::Burger(st) => {
                if st.flavor == food_flavor {
                    let idx = food_list.iter().position(|i| i == item);
                    food_list.remove(idx.unwrap());
                }
            },
        }
    }


    let res = format!("deleted {} {}", food_flavor, food_type);

    res

}
