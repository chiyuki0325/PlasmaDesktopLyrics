use actix_web::{HttpResponse, Responder};

pub(crate) async fn test_page() -> impl Responder {
    HttpResponse::Ok()
        .append_header(("Content-Type", "text/html; charset=utf-8"))
        .body(
        r#"
        <!DOCTYPE html>
        <html>
        <body>
        <script>
            ws = new WebSocket("ws://localhost:15649/ws")
            ws.onmessage = function(event) {
                console.log(event.data)
            }
            ws.onopen = function(event) {
                console.log("WebSocket connection established")
            }
        </script>
        </body>
        </html>
        "#,
    )
}
