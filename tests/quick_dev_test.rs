
use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn main_page_test() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:8080")?;

    //client.do_get("/hello?name=apektas").await?.print().await?;
    //client.do_get("/hello2/apektas_var").await?.print().await?;
    // client.do_get("/src/main.rs").await?.print().await?;

    let req_login = client.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome",
        })
    );

    // Disable to see auth exception.
    req_login.await?.print().await?;


    let req_create_ticket = client.do_post(
        "/api/tickets",
        json!({
            "title": "Ticket AAA",
        })

    );

    req_create_ticket.await?.print().await?;

    //client.do_delete("/api/tickets/1").await?.print().await?;

    client.do_get("/api/tickets").await?.print().await?;


    Ok(())

}