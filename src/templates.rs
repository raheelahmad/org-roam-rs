use tera::Tera;

pub fn header() -> String {
    String::from(
        "
<html>
<head> <meta charset='utf-8'/> </head>
<body>
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
All pages for <strong>{{tag_name}}</strong>

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
	{% for tag in tags %}
    <a href='tag-{{tag }}.html'>{{ tag }}</a>
	{% endfor %}
	<div>{{page}}</div>
",
        ),
    )
    .expect("should load raw templat");
    tera
}
