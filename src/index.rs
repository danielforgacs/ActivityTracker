use actix_web::{HttpRequest, get};

const HTML_TMPL: &str = "
<!DOCTYPE html>
<html>
<head>
    <meta charset='utf-8'>
    <meta http-equiv='X-UA-Compatible' content='IE=edge'>
    <title>Page Title</title>
    <meta name='viewport' content='width=device-width, initial-scale=1'>
    <link rel='stylesheet' type='text/css' media='screen' href='main.css'>
    <script src='main.js'></script>
</head>
<body>

</body>
</html>
";

#[get("/")]
async fn index_view(req: HttpRequest) -> String {
    // "ok".to_string()
    HTML_TMPL.to_string()
}
