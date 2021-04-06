use tera::Tera;

pub fn header() -> String {
    String::from(
        "
<html>
<head>
 <title>
{{title}}
 </title>
 <meta charset='utf-8'/> </head>
<link href=\"css/tufte.css\" rel=\"stylesheet\" type=\"text/css\"/>
<body>
<h3>
<a href='/all_pages.html'> All Pages </a>
</h3>
</small>
	",
    )
}

pub fn footer() -> String {
    String::from(
        "
	</body></html>
    ",
    )
}

fn template_with_content(content: &str) -> String {
    let mut result = header();
    result.push_str(content);
    result.push_str(&footer());
    result
}

pub fn tag_page_template() -> Tera {
    let mut tera = Tera::default();
    tera.autoescape_on(vec![]);
    let content = template_with_content(
        "
<div>
<h1>
All pages for <strong>{{tag_name}}</strong>
</h1>

<ul>
{% for page in pages %}
<li>
  <a href='{{page.title}}.html'>{{page.title}}</a>
</li>
{% endfor %}
</ul>
</div>

</body></html>
",
    );

    tera.add_raw_template("tag.html", &content)
        .expect("should load raw templat");
    tera
}

pub fn all_pages_template() -> Tera {
    let mut tera = Tera::default();
    tera.autoescape_on(vec![]);
    tera.add_raw_template(
        "all_pages.html",
        &template_with_content(
            "
<h1>All Pages</h1>
<ul>
{% for page in pages %}
<li>
  <a href='{{page.title}}.html'>{{page.title}}</a>
</li>
{% endfor %}
</ul>
",
        ),
    )
    .expect("should load raw templat");
    tera
}

pub fn page_template() -> Tera {
    let mut tera = Tera::default();
    tera.autoescape_on(vec![]);
    tera.add_raw_template(
        "page.html",
        &template_with_content(
            "
<h1><strong>{{title}}</strong></h1>
<h3>
	{% for tag in tags %}
    <a href='tag-{{tag }}.html'>{{ tag }}</a>
	{% endfor %}
</h3>
	<div>{{page}}</div>
",
        ),
    )
    .expect("should load raw templat");
    tera
}
