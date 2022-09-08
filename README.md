sato
===
an s-expression based html templating system.

# sato template language examples
## basic template example
```sato
(html
 (head
  (title "basic example")))
```

## tag attributes
```sato
(html
 (head (@ (some thing))
  (title "basic example")))
```

## variables
variables in sato are prefixed with a `$`.
```sato
(html
 (head
  (title $some_variable)))
```

## conditionals
```sato
(html
 (head
  (title (if (is-set $some_variable)
             (div "$some_variable")
             (div "variable is not set")))))
```

## iteration over arrays
```sato
(html
 (body
  (for i in $some_array
       (div "element: " $i))))
```

## iteration over maps
```sato
(html
 (body
  (for k v in $some_map
       (div $k ": " $v))))
```

## switch/case
```sato
(html
 (body
  (switch $blah
          (case asdf
            qwer)
          (case zxcv
            (div what else))
          (case hjkl
            nm))))
```


# basic library example
```rust
use sato::renderer::Renderer;
use sato::context::RenderContext;
use sato::template::Template;

let renderer = Renderer::builder()
    .build();
let expr = r#"(html (head (title "basic example")))"#;
let template = Template::from_str(expr).unwrap();
let html = renderer.render(&template, &RenderContext::default()).unwrap();

assert_eq!(html, "<!doctype html5><html><head><title>basic example</title></head></html>")
```

# using variables
```rust
use sato::renderer::Renderer;
use sato::context::RenderContext;
use sato::template::Template;

let renderer = Renderer::builder()
    .build();
let expr = r#"(html (body (if (eq $asdf qwer) (for i in $array (div $i)))))"#;
let template = Template::from_str(expr).unwrap();
let context = RenderContext::builder()
    .insert("asdf", "qwer")
    .insert("array", vec!["zxc", "xcv", "cvb"])
    .build();
let html = renderer.render(&template, &context).unwrap();

assert_eq!(html, "<!doctype html5><html><body><div>zxc</div><div>xcv</div><div>cvb</div></body></html>")
```

# custom handler functions
```rust
use sato::renderer::{Attributes, Renderer, RenderError};
use sato::context::RenderContext;
use sato::template::{Template, TemplateExprNode};

let post_expr = r##"(div (h2 $title) (span "posted by $author") $content (br) (div (for tag in $tags (span "#$tag"))))"##;
let blogpost_template = Template::from_str(post_expr).unwrap();

let renderer = Renderer::builder()
    .function("blogpost", Box::new(move |attrs, expr, renderer, context| {
        let title = attrs.get("title").unwrap();
        let author = attrs.get("author").unwrap();

        let mut new_context = context.clone();
        new_context.insert("title", title);
        new_context.insert("author", author);
        new_context.insert("content", renderer.evaluate_multiple(expr, &new_context)?);

        Ok(vec![renderer.render(&blogpost_template, &new_context).unwrap()])
    }))
    .build();
let expr = r#"(html (body (blogpost (@ (title faketitle) (author me)) (div "my content here"))))"#;
let template = Template::from_str(expr).unwrap();
let context = RenderContext::builder()
    .insert("tags", vec!["zxc", "xcv", "cvb"])
    .build();
let html = renderer.render(&template, &context).unwrap();

assert_eq!(html, "<!doctype html5><html><body><div><h2>faketitle</h2><span>posted by me</span><div>my content here</div><br /><div><span>#zxc</span><span>#xcv</span><span>#cvb</span></div></div></body></html>")
```

