pub const HTML_TEMPLATE: &str = r#"<!DOCTYPE html>
<html>
<head>
    <title> {} </title>
    <link rel="stylesheet" href="/style/style.css">
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,300;0,700;1,300;1,700&display=swap" rel="stylesheet">
</head>
<body>
<div class="index-header">
    <span class="index-title"><a href="/index.html" class="index-link">{}</a></span>
    <span class="index-box">
        <span><a href = \"{}\" class="index-link">About</a></span>
        <span><a href = \"{}\" class="index-link">Projects</a></span>
    </span>
    <hr/>
</div>
<div class="content">
{}
<hr>
</div>
<div class="index-footer">
{}
</div>
                  
</body>
</html>"#;