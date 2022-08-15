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
<link rel='stylesheet' href='css/lit.css'>
<body>
<div class='c'>
<div class='row card'>
<div class='2 col'>
<a href='all_pages.html'> All Pages </a>
</div>
<div class='2 col'>
<a href='all_tags.html'> All Tags </a>
</div>
</div>
</small>
	",
    )
}

pub fn footer() -> String {
    String::from(
        "
    </div>
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
  <a href='{{page}}.html'>{{page}}</a>
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

pub fn all_tags_template() -> Tera {
    let mut tera = Tera::default();
    tera.autoescape_on(vec![]);
    tera.add_raw_template(
        "all_tags.html",
        &template_with_content(
            "
<h1>All Tags</h1>
<ul>
{% for files_for_tag in tag_files %}

	<h3>
	{{ files_for_tag.tag_name }}
	</h3>


	{% for page in files_for_tag.files %}
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

pub fn all_pages_template() -> Tera {
    let mut tera = Tera::default();
    tera.autoescape_on(vec![]);
    tera.add_raw_template(
        "all_pages.html",
        &template_with_content(
            "
<h1>All Pages</h1>
<ul>
{% for page in pages.files %}
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
<hr/>
<h1 class=\"pagename\"><strong>{{title}}</strong></h1>
  <hr/>
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
    What links here: 
	{% for file in backlinks %}
    <a href='{{file.title}}.html' style='margin:10px'>{{ file.title }}</a> 
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
