use actix_web::{HttpRequest, get, HttpResponseBuilder, http, HttpResponse};
use tera::{Tera, Context};

const INDEX_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset='utf-8'>
    <meta http-equiv='X-UA-Compatible' content='IE=edge'>
    <title>Page Title</title>
    <meta name='viewport' content='width=device-width, initial-scale=1'>
    <link rel='stylesheet' type='text/css' media='screen' href='main.css'>
    <script>
        console.log("hey, how cool is this!")
        fetch('api/times')
            .then(response => response.json())
            .then(data => console.log(data));
    </script>
</head>
<body>
{{ message }}
</body>
</html>
"#;

#[get("/")]
async fn index_view(req: HttpRequest) -> HttpResponse {
    let mut tera = Tera::default();
    let template_name = "something-cool.html";
    tera.add_raw_template(template_name, INDEX_TEMPLATE).unwrap();
    let mut ctx = Context::new();
    ctx.insert("message", "Coming from Rust! Check the console!");
    let render = tera.render(template_name, &ctx).unwrap();
    HttpResponse::Ok()
        .body(render)
}
