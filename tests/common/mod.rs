use reqwest::Response;

pub fn start_server(port: u16) {

    let server = rusty_restaurant::router::run(port);


    if let Ok(s) = server {
        actix_web::rt::spawn(s);
    }
    
}


pub async fn req_resp(ipaddress: &'static str, suffix: &'static str, port: u16, reqtype: &'static str, queryparams: Option<[(&str, &str); 3]>) -> Response {
    let client = reqwest::Client::new();
    let request_address = format!("http://{ipaddress}:{port}{suffix}");

    let response = match reqtype {
        "get" => {
            client
                .get(request_address)
                .send()
                .await
                .expect("Failed executing request")
        },
        "post" => {
            client
                .post(request_address)
                .query(&queryparams)
                .send()
                .await
                .expect("Failed executing request")
        },
        "put" => {
            client
                .put(request_address)
                .query(&queryparams)
                .send()
                .await
                .expect("Failed executing request")
        },
        "delete" => {
            client
                .delete(request_address)
                .send()
                .await
                .expect("Failed executing request")
        },
        _ => unimplemented!()
    };

    response
}



