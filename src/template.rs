pub const HTML_TEMPLATE: &str = 
r#"<!DOCTYPE html>
<html>
<head>
    <title> {} </title>
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