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
<link href=\"css/sakura.css\" rel=\"stylesheet\" type=\"text/css\"/>
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
{% for files_by_week in pages %}

	{% if files_by_week.weeks_away == 0 %}
		<h3>
		This week
		</h3>
	{% else %}
		<h3>
		{{ files_by_week.weeks_away }} weeks away
		</h3>
	{% endif %}


	{% for page in files_by_week.files %}
		<li>
		<a href='{{page.title}}.html'>{{page.title}}</a>
		</li>
	{% endfor %}

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
<hr/>
<h1 class=\"pagename\"><strong>{{title}}</strong></h1>
   <small>
   <div>
    {% if tags %}
    Tags: 
	{% for tag in tags %}
    <a href='tag-{{tag }}.html'>{{ tag }}</a>
	{% endfor %}
    {% endif %}
   </div>


   <div>
    {% if backlinks %}
    Backlinks: 
	{% for file in backlinks %}
    <a href='{{file.title}}.html'>{{ file.title }}</a> •
	{% endfor %}
	{% endif %}
   </div>
   </small>

	<div>{{page}}</div>
",
        ),
    )
    .expect("should load raw templat");
    tera
}
