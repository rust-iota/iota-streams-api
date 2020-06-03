use actix_web::{get, web, App, Error, HttpResponse, HttpServer, Responder};


// Example /streams/TYTHYOTDUWIEGSAMQZRJQQWXXOHPLSBPSWJFSQFYRBRLO9TCUEIFJBPTQIODCSZYZDYSZTOKCNLENQFFM/tag/9PJMCTHXCOBLZI9EJRQONPSALVW
#[get("/streams/{channel_address}/tag/{announcement_message_tag}")]
async fn index(channel: web::Path<(String,String)>) -> impl Responder {
    web::HttpResponse::Ok().body(&channel.0)
}

#[get("/get_node_info")]
async fn get_node_info() -> Result<HttpResponse, Error> {
    iota::Client::add_node("https://nodes.comnet.thetangle.org")
        .expect("Error connecting to the node");
    let node_info = iota::Client::get_node_info()
        .await
        .expect("get_node_info - Error connecting to the node");
    println!("{:#?}", node_info);
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&node_info).unwrap()))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(get_node_info))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
