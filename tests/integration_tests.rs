use serde_json::{json, Value};

mod common;



#[actix_web::test]
async fn health_check_works() {

    let port = 7000;
    let ipaddress = "127.0.0.1";
    let suffix = "/";

    // start server
    common::start_server(port);

    // request & response
    let response = common::req_resp(ipaddress, suffix, port, "get", None).await;

    // assert successful request
    assert!(response.status().is_success());

    // expected vs actual
    let expected = "Hello World!";
    let actual = response.text().await.expect("error converting response to text");


    // asssert response
    assert_eq!(expected, actual);

}

#[actix_web::test]
async fn ping_works() {
    let port = 7000;
    let ipaddress = "127.0.0.1";
    let suffix = "/ping";

    // start server
    common::start_server(port);

    // request & response
    let response = common::req_resp(ipaddress, suffix, port, "get", None).await;

    // assert successful request
    assert!(response.status().is_success());

    // expected vs actual
    let expected = json!({
        "ping": "pong"
    });
    let actual: Value = response.json().await.unwrap();
    
    // asssert response
    assert_eq!(expected, actual);
}

#[actix_web::test]
async fn test_empty_menu() {
    let port = 7000;
    let ipaddress = "127.0.0.1";
    let suffix = "/menu";

    // start server
    common::start_server(port);

    // request & response
    let response = common::req_resp(ipaddress, suffix, port, "get", None).await;

    // assert successful request
    assert!(response.status().is_success());

    // expected vs actual
    let expected = json!({
        "pizzas": [],
        "burgers": []
    });
    let actual: Value = response.json().await.unwrap();

    // assert response
    assert_eq!(expected, actual);
}

#[actix_web::test]
async fn test_menu_with_food() {
    let port = 7000;
    let ipaddress = "127.0.0.1";
    let mut suffix = "/menu/add/pizza";

    // start server
    common::start_server(port);

    let cheese_pizza = [
        ("flavor", "cheese"),
        ("description", "the best"),
        ("price", "5.7")
    ];
    

    // request & response
    let mut response = common::req_resp(ipaddress, suffix, port, "post", Some(cheese_pizza)).await;
    assert!(response.status().is_success());

    let pep_pizza: [(&str, &str); 3] = [
        ("flavor", "pepperoni"),
        ("description", "the best for sure"),
        ("price", "10")
    ];

    response = common::req_resp(ipaddress, suffix, port, "post", Some(pep_pizza)).await;
    assert!(response.status().is_success());


    suffix = "/menu/add/burger";
    let burger = [
        ("flavor", "bacon"),
        ("description", "the best burger"),
        ("price", "9.8")
    ];

    response = common::req_resp(ipaddress, suffix, port, "post", Some(burger)).await;
    assert!(response.status().is_success());

    

    suffix = "/menu";
    response = common::req_resp(ipaddress, suffix, port, "get", None).await;
    assert!(response.status().is_success());

    // expected vs actual
    let expected = json!({
        "pizzas": [
            {
                "Pizza": {
                    "flavor": "cheese",
                    "description": "the best",
                    "price": 5.7
                }
            },
            {
                "Pizza": {
                    "flavor": "pepperoni",
                    "description": "the best for sure",
                    "price": 10.0
                }
            }
        ],
        "burgers": [
            {
                "Burger": {
                    "flavor": "bacon",
                    "description": "the best burger",
                    "price": 9.8
                }
            }
        ]
    });
    let actual: Value = response.json().await.unwrap();

    assert_eq!(expected, actual);
}

#[actix_web::test]
async fn test_menu_after_updating_food() {
    let port = 7001;
    let ipaddress = "127.0.0.1";
    let mut suffix = "/menu/add/pizza";

    // start server
    common::start_server(port);

    let cheese_pizza = [
        ("flavor", "cheese"),
        ("description", "the best"),
        ("price", "5.7")
    ];
    

    // request & response
    let mut response = common::req_resp(ipaddress, suffix, port, "post", Some(cheese_pizza)).await;
    assert!(response.status().is_success());

    let pep_pizza: [(&str, &str); 3] = [
        ("flavor", "pepperoni"),
        ("description", "the best for sure"),
        ("price", "10")
    ];

    response = common::req_resp(ipaddress, suffix, port, "post", Some(pep_pizza)).await;
    assert!(response.status().is_success());


    suffix = "/menu/add/burger";
    let burger = [
        ("flavor", "bacon"),
        ("description", "the best burger"),
        ("price", "9.8")
    ];

    response = common::req_resp(ipaddress, suffix, port, "post", Some(burger)).await;
    assert!(response.status().is_success());


    suffix = "/menu/update/pizza";
    let updated_cheese_pizza = [
        ("flavor", "cheese"),
        ("description", "the best cheese ever"),
        ("price", "7.7")
    ];
    
    // request & response
    response = common::req_resp(ipaddress, suffix, port, "put", Some(updated_cheese_pizza)).await;
    assert!(response.status().is_success());

    suffix = "/menu";
    response = common::req_resp(ipaddress, suffix, port, "get", None).await;
    assert!(response.status().is_success());

    // expected vs actual
    let expected = json!({
        "pizzas": [
            {
                "Pizza": {
                    "flavor": "cheese",
                    "description": "the best cheese ever",
                    "price": 7.7
                }
            },
            {
                "Pizza": {
                    "flavor": "pepperoni",
                    "description": "the best for sure",
                    "price": 10.0
                }
            }
        ],
        "burgers": [
            {
                "Burger": {
                    "flavor": "bacon",
                    "description": "the best burger",
                    "price": 9.8
                }
            }
        ]
    });
    let actual: Value = response.json().await.unwrap();

    assert_eq!(expected, actual);
    
}

#[actix_web::test]
async fn test_menu_after_deleting_food() {
    let port = 7002;
    let ipaddress = "127.0.0.1";
    let mut suffix = "/menu/add/pizza";

    // start server
    common::start_server(port);

    let cheese_pizza = [
        ("flavor", "cheese"),
        ("description", "the best"),
        ("price", "5.7")
    ];
    

    // request & response
    let mut response = common::req_resp(ipaddress, suffix, port, "post", Some(cheese_pizza)).await;
    assert!(response.status().is_success());

    let pep_pizza: [(&str, &str); 3] = [
        ("flavor", "pepperoni"),
        ("description", "the best for sure"),
        ("price", "10")
    ];

    response = common::req_resp(ipaddress, suffix, port, "post", Some(pep_pizza)).await;
    assert!(response.status().is_success());


    suffix = "/menu/add/burger";
    let burger = [
        ("flavor", "bacon"),
        ("description", "the best burger"),
        ("price", "9.8")
    ];

    response = common::req_resp(ipaddress, suffix, port, "post", Some(burger)).await;
    assert!(response.status().is_success());

    // delete
    suffix = "/menu/delete/pizza/cheese";
    response = common::req_resp(ipaddress, suffix, port, "delete", None).await;
    assert!(response.status().is_success());


    suffix = "/menu";
    response = common::req_resp(ipaddress, suffix, port, "get", None).await;
    assert!(response.status().is_success());

    // expected vs actual
    let expected = json!({
        "pizzas": [
            {
                "Pizza": {
                    "flavor": "pepperoni",
                    "description": "the best for sure",
                    "price": 10.0
                }
            }
        ],
        "burgers": [
            {
                "Burger": {
                    "flavor": "bacon",
                    "description": "the best burger",
                    "price": 9.8
                }
            }
        ]
    });
    let actual: Value = response.json().await.unwrap();

    assert_eq!(expected, actual);

}



