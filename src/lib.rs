/*!
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

let post_expr = r##"(div (h2 $title) (span "posted by " $author) $content (br) (div (for tag in $tags (span "#" $tag))))"##;
let blogpost_template = Template::from_str(post_expr).unwrap();

let renderer = Renderer::builder()
    .function("blogpost", Box::new(move |attrs, expr, renderer, context| {
        let title = attrs.get("title").unwrap();
        let author = attrs.get("author").unwrap();

        let mut new_context = context.clone();
        new_context.insert("title", title);
        new_context.insert("author", author);
        new_context.insert("content", renderer.evaluate_multiple(expr, &new_context)?);

        Ok(renderer.render(&blogpost_template, &new_context).unwrap().into())
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


# builtin functions
## if
`(if [condition] [true code block] [false code block])`
if condition evaluates to true then execute the true block, if false then execute false block.

## get
`(get [array] [index])`

`(get [map] [key])`

gets an element from an array or map

## is-set
`(is-set [variable])`

takes a single argument and returns true or false depending if the variable is set.

## switch/case
`(switch [variable] (case [value] [code block]) (case [value] [code block]) ...)`

## for
`(for [item] in [array] [code block])`

`(for [key] [value] in [map] [code block])`

`(for [item] in (range [min] [max] [step?]) [code block])`

`(for (enumerate [index] [item]) in [array] [code block])`

executes code block for each element in the iterable.

## eq/gt/lt/gte/lte/ne
`(eq [item] [item])`

standard comparison operators, returns true or false

## +, -, *, /, %
standard math operators

`(+ [item] [item])`

*/


mod builtins;
pub mod context;
pub mod renderer;
pub mod template;

pub use crate::renderer::{Renderer, RenderValue, Attribute, Attributes, RenderError};
pub use crate::template::{Template, TemplateExprNode};
pub use crate::context::{RenderContext, ContextValue};


#[cfg(test)]
mod tests {
    use crate::context::{RenderContext, ContextValue};
    use crate::renderer::{Renderer, RenderValue};
    use crate::template::{Template, TemplateExprNode};

    #[test]
    fn test_no_builtins() {
        let renderer = Renderer::builder()
            .build();
        let expr = r#"(head (title "test title"))"#;
        let template = Template::from_str(expr).unwrap();
        let html = renderer.render(&template, &RenderContext::default()).unwrap();

        assert_eq!(html, "<head><title>test title</title></head>")
    }

    #[test]
    fn test_basic_render() {
        let renderer = Renderer::builder()
            .build();
        let expr = r#"(html (head (title "test title")))"#;
        let template = Template::from_str(expr).unwrap();
        let html = renderer.render(&template, &RenderContext::default()).unwrap();

        assert_eq!(html, "<!doctype html5><html><head><title>test title</title></head></html>")
    }

    #[test]
    fn test_attributes() {
        let renderer = Renderer::builder()
            .build();
        let expr = r#"(html (head (@ (asdf qwer) (zxc asd))(title "test title")))"#;
        let template = Template::from_str(expr).unwrap();
        let html = renderer.render(&template, &RenderContext::default()).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><head asdf="qwer" zxc="asd"><title>test title</title></head></html>"#)
    }

    #[test]
    fn test_attributes_on_empty_tag() {
        let renderer = Renderer::builder()
            .build();
        let expr = r#"(html (body (@ (asdf qwer))))"#;
        let template = Template::from_str(expr).unwrap();
        let html = renderer.render(&template, &RenderContext::default()).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><body asdf="qwer" /></html>"#)
    }

    #[test]
    fn test_basic_substitution() {
        let renderer = Renderer::builder()
            .build();
        let expr = r#"(html (head (title $title)))"#;
        let template = Template::from_str(expr).unwrap();
        let context = RenderContext::builder()
            .insert("title", "some sort of title")
            .build();
        let html = renderer.render(&template, &context).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><head><title>some sort of title</title></head></html>"#)
    }

    #[test]
    fn test_vec_substitution() {
        let renderer = Renderer::builder()
            .build();
        let expr = r#"(html (div $vec))"#;
        let template = Template::from_str(expr).unwrap();
        let context = RenderContext::builder()
            .insert("asdf", "qwer")
            .insert("vec", vec!["this", "that", "$asdf"])
            .build();
        let html = renderer.render(&template, &context).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><div>thisthatqwer</div></html>"#)
    }

    #[test]
    fn test_if_is_set() {
        let renderer = Renderer::builder()
            .build();
        let expr = r#"(html (head (if (is-set $title) (title $title) (title "not set"))))"#;
        let template = Template::from_str(expr).unwrap();
        let context = RenderContext::builder()
            .insert("title", "some sort of title")
            .build();
        let html = renderer.render(&template, &context).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><head><title>some sort of title</title></head></html>"#)
    }

    #[test]
    fn test_if_not_is_set() {
        let renderer = Renderer::builder()
            .build();
        let expr = r#"(html (head (if (is-set $title) (title $title) (title "not set"))))"#;
        let template = Template::from_str(expr).unwrap();
        let context = RenderContext::builder()
            .insert("title2", "some sort of title")
            .build();
        let html = renderer.render(&template, &context).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><head><title>not set</title></head></html>"#)
    }

    #[test]
    fn test_array_iteration() {
        let renderer = Renderer::builder()
            .build();
        let expr = r#"(html (body (for i in $asdf (div "iter " $i))))"#;
        let template = Template::from_str(expr).unwrap();
        let context = RenderContext::builder()
            .insert("asdf", vec!["qaz", "wsx", "edc"])
            .build();
        let html = renderer.render(&template, &context).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><body><div>iter qaz</div><div>iter wsx</div><div>iter edc</div></body></html>"#)
    }

    #[test]
    fn test_range_iteration() {
        let renderer = Renderer::builder()
            .build();
        let expr = r#"(html (body (for i in (range 0 3) (div "iter " $i))))"#;
        let template = Template::from_str(expr).unwrap();
        let html = renderer.render(&template, &RenderContext::default()).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><body><div>iter 0</div><div>iter 1</div><div>iter 2</div></body></html>"#)
    }

    #[test]
    fn test_array_index_iteration() {
        let renderer = Renderer::builder()
            .build();
        let expr = r#"(html (body (for (enumerate k i) in $asdf (div $k ": iter " $i))))"#;
        let template = Template::from_str(expr).unwrap();
        let context = RenderContext::builder()
            .insert("asdf", vec!["qaz", "wsx", "edc"])
            .build();
        let html = renderer.render(&template, &context).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><body><div>0: iter qaz</div><div>1: iter wsx</div><div>2: iter edc</div></body></html>"#)
    }

    #[test]
    fn test_object_iteration() {
        let renderer = Renderer::builder()
            .build();
        let expr = r#"(html (body (for k v in $asdf (div "key " $k ", value " $v))))"#;
        let template = Template::from_str(expr).unwrap();
        let internal_obj = RenderContext::builder()
            .insert("as", "df")
            .insert("qw", "er")
            .build();
        let context = RenderContext::builder()
            .insert("asdf", internal_obj)
            .build();
        let html = renderer.render(&template, &context).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><body><div>key as, value df</div><div>key qw, value er</div></body></html>"#)
    }

    #[test]
    fn test_object_access() {
        let renderer = Renderer::builder()
            .build();
        let expr = r#"(html (body $asdf.as))"#;
        let template = Template::from_str(expr).unwrap();

        let internal_obj = RenderContext::builder()
            .insert("as", "df")
            .build();
        let context = RenderContext::builder()
            .insert("asdf", internal_obj)
            .build();
        let html = renderer.render(&template, &context).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><body>df</body></html>"#)
    }

    #[test]
    fn test_deep_object_access() {
        let renderer = Renderer::builder()
            .build();
        let expr = r#"(html (body $nested_object.as.df.qw.er))"#;
        let template = Template::from_str(expr).unwrap();

        let nested3_obj = RenderContext::builder()
            .insert("er", "look at this nested thing")
            .build();
        let nested2_obj = RenderContext::builder()
            .insert("qw", nested3_obj)
            .build();
        let nested1_obj = RenderContext::builder()
            .insert("df", nested2_obj)
            .build();
        let nested0_obj = RenderContext::builder()
            .insert("as", nested1_obj)
            .build();

        let context = RenderContext::builder()
            .insert("nested_object", nested0_obj)
            .build();
        let html = renderer.render(&template, &context).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><body>look at this nested thing</body></html>"#)
    }

    #[test]
    fn test_variable_in_attributes() {
        let renderer = Renderer::builder()
            .build();
        let expr = r#"(html (head (@ (asdf $qwer) (zxc asd))(title "test title")))"#;
        let template = Template::from_str(expr).unwrap();
        let context = RenderContext::builder()
            .insert("qwer", "zxcv")
            .build();
        let html = renderer.render(&template, &context).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><head asdf="zxcv" zxc="asd"><title>test title</title></head></html>"#)
    }

    #[test]
    fn test_object_variable_in_attributes() {
        let renderer = Renderer::builder()
            .build();
        let expr = r#"(html (for qw in $as (div (@ (class $qw.er)) $qw.zx)))"#;
        let template = Template::from_str(expr).unwrap();

        let obj1 = RenderContext::builder()
            .insert("er", "cv")
            .insert("zx", "hj")
            .build();
        let obj2 = RenderContext::builder()
            .insert("er", "df")
            .insert("zx", "nm")
            .build();

        let context = RenderContext::builder()
            .insert("as", vec![obj1, obj2])
            .build();
        let html = renderer.render(&template, &context).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><div class="cv">hj</div><div class="df">nm</div></html>"#)
    }

    #[test]
    fn test_case() {
        let renderer = Renderer::builder()
            .build();
        let expr = r#"(html (div (switch $blah (case asdf qwer) (case zxcv (div what else)) (case hjkl nm))))"#;
        let template = Template::from_str(expr).unwrap();
        let context = RenderContext::builder()
            .insert("blah", "zxcv")
            .build();
        let html = renderer.render(&template, &context).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><div><div>whatelse</div></div></html>"#)
    }

    #[test]
    fn test_custom_closure() {
        let renderer = Renderer::builder()
            .function("blah", Box::new(|_, _, _, _| {
                Ok("hello there".into())
            }))
            .build();
        let expr = r#"(html (div (blah something or other)))"#;
        let template = Template::from_str(expr).unwrap();
        let html = renderer.render(&template, &RenderContext::default()).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><div>hello there</div></html>"#)
    }

    #[test]
    fn test_custom_function() {
        fn blah(_: &crate::renderer::Attributes, _: &[&TemplateExprNode], _: &Renderer, _: &RenderContext) -> Result<RenderValue, crate::renderer::RenderError> {
            Ok("hello there".into())
        }

        let renderer = Renderer::builder()
            .function("blarg", Box::new(blah))
            .build();
        let expr = r#"(html (div (blarg something or other)))"#;
        let template = Template::from_str(expr).unwrap();
        let html = renderer.render(&template, &RenderContext::default()).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><div>hello there</div></html>"#)
    }

    #[test]
    fn test_using_attrs_in_closure() {
        let renderer = Renderer::builder()
            .function("blah", Box::new(|attrs, _, _, _| {
                let mut output: Vec<String> = Vec::new();

                for attr in attrs {
                    output.push("[".into());
                    output.push(attr.0.clone());
                    output.push(" = ".into());
                    output.push(attr.1.clone());
                    output.push("] ".into());
                }

                Ok(output.into())
            }))
            .build();
        let expr = r#"(html (div (blah (@ (this is) (the attr)) something or other)))"#;
        let template = Template::from_str(expr).unwrap();
        let html = renderer.render(&template, &RenderContext::default()).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><div>[this = is] [the = attr] </div></html>"#)
    }

    #[test]
    fn test_more_html_in_closure() {
        let renderer = Renderer::builder()
            .function("blah", Box::new(|_, expr, renderer, context| {
                let mut output: Vec<RenderValue> = Vec::new();
                output.push("<blah>".into());
                output.push(renderer.evaluate_multiple(expr, context)?.into());
                output.push("</blah>".into());
                Ok(output.into())
            }))
            .build();
        let expr = r#"(html (div (blah (span hello))))"#;
        let template = Template::from_str(expr).unwrap();
        let html = renderer.render(&template, &RenderContext::default()).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><div><blah><span>hello</span></blah></div></html>"#)
    }

    #[test]
    fn test_renderer_in_closure() {
        let subexpr = r#"(sub str)"#;
        let subtemplate = Template::from_str(subexpr).unwrap();

        let renderer = Renderer::builder()
            .function("blah", Box::new(move |_, _, renderer, _| {
                let mut output: Vec<String> = Vec::new();
                output.push("<blah>".into());
                let suboutput = renderer.render(&subtemplate, &RenderContext::default())?;
                output.push(suboutput);
                output.push("</blah>".into());
                Ok(output.into())
            }))
            .build();
        let expr = r#"(html (div (blah (span hello))))"#;
        let template = Template::from_str(expr).unwrap();
        let html = renderer.render(&template, &RenderContext::default()).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><div><blah><sub>str</sub></blah></div></html>"#)
    }

    #[test]
    fn test_context_in_closure() {
        let renderer = Renderer::builder()
            .function("blah", Box::new(move |_, _, _, context| {
                let s = match context.get("blah").unwrap() {
                    ContextValue::String(s) => s,
                    _ => panic!("not a str")
                }.clone();

                let mut output: Vec<String> = Vec::new();
                output.push("<blah>".into());
                output.push(s);
                output.push("</blah>".into());
                Ok(output.into())
            }))
            .build();
        let expr = r#"(html (div (blah (span hello))))"#;
        let template = Template::from_str(expr).unwrap();
        let context = RenderContext::builder()
            .insert("blah", "zxcv")
            .build();
        let html = renderer.render(&template, &context).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><div><blah>zxcv</blah></div></html>"#)
    }

    #[test]
    fn test_closure_with_everything() {
        let subexpr = r#"(sub $content)"#;
        let subtemplate = Template::from_str(subexpr).unwrap();

        let renderer = Renderer::builder()
            .function("blah", Box::new(move |attr, expr, renderer, context| {
                let mut output: Vec<RenderValue> = Vec::new();
                output.push("<blah>".into());

                let mut subcontext = RenderContext::default();
                subcontext.insert("content", attr.get("something").unwrap().clone());
                let suboutput = renderer.render(&subtemplate, &subcontext)?;
                output.push(suboutput.into());
                output.push(renderer.evaluate_multiple(expr, context)?.into());
                output.push(match context.get("blah").unwrap() {
                    ContextValue::String(s) => s,
                    _ => panic!("not a str")
                }.clone().into());

                output.push("</blah>".into());
                Ok(output.into())
            }))
            .build();
        let expr = r#"(html (div (blah (@ (something extra)) (span hello))))"#;
        let template = Template::from_str(expr).unwrap();
        let context = RenderContext::builder()
            .insert("blah", "zxcv")
            .build();
        let html = renderer.render(&template, &context).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><div><blah><sub>extra</sub><span>hello</span>zxcv</blah></div></html>"#)
    }

    #[test]
    fn test_math_op_mod() {
        let renderer = Renderer::builder()
            .build();
        let expr = r#"(html (body (for i in (range 0 5) (if (eq (% $i 2) 0) (div $i)))))"#;
        let template = Template::from_str(expr).unwrap();
        let context = RenderContext::builder()
            .build();
        let html = renderer.render(&template, &context).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><body><div>0</div><div>2</div><div>4</div></body></html>"#)
    }

    #[test]
    fn test_variable_range_iteration() {
        let renderer = Renderer::builder()
            .build();
        let expr = r#"(html (body (for i in (range $a $b) (div "iter " $i))))"#;
        let template = Template::from_str(expr).unwrap();
        let context = RenderContext::builder()
            .insert("a", 4)
            .insert("b", 7)
            .build();
        let html = renderer.render(&template, &context).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><body><div>iter 4</div><div>iter 5</div><div>iter 6</div></body></html>"#)
    }

    #[test]
    fn test_math_ops() {
        let renderer = Renderer::builder()
            .build();
        let expr = r#"(html (body (+ (/ (* $b (- $c (+ 2 3))) $c) (+ $b $c))))"#;
        let template = Template::from_str(expr).unwrap();
        let context = RenderContext::builder()
            .insert("a", 4)
            .insert("b", 7)
            .insert("c", 12)
            .build();
        let html = renderer.render(&template, &context).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><body>23</body></html>"#)
    }

    #[test]
    fn test_math_ops_in_attributes() {
        let renderer = Renderer::builder()
            .build();
        let expr = r#"(html (body (@ (blah (+ 2 3)) (asdf (* $a $b)))))"#;
        let template = Template::from_str(expr).unwrap();
        let context = RenderContext::builder()
            .insert("a", 4)
            .insert("b", 7)
            .build();
        let html = renderer.render(&template, &context).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><body blah="5" asdf="28" /></html>"#)
    }

    #[test]
    fn test_math_ops_in_for_range() {
        let renderer = Renderer::builder()
            .build();
        let expr = r#"(html (body (for i in (range (- $b $a) (+ $b 1)) (div "iter " $i))))"#;
        let template = Template::from_str(expr).unwrap();
        let context = RenderContext::builder()
            .insert("a", 4)
            .insert("b", 7)
            .build();
        let html = renderer.render(&template, &context).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><body><div>iter 3</div><div>iter 4</div><div>iter 5</div><div>iter 6</div><div>iter 7</div></body></html>"#)
    }

    #[test]
    fn test_get_from_array() {
        let renderer = Renderer::builder()
            .build();
        let expr = r#"(html (body (get $a 0) (get $a 2)))"#;
        let template = Template::from_str(expr).unwrap();
        let context = RenderContext::builder()
            .insert("a", vec!["asd", "qwe", "zxc"])
            .build();
        let html = renderer.render(&template, &context).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><body>asdzxc</body></html>"#)
    }

    #[test]
    fn test_get_from_object() {
        let renderer = Renderer::builder()
            .build();
        let expr = r#"(html (body (get $asdf as) (get $asdf zx)))"#;
        let template = Template::from_str(expr).unwrap();
        let internal_obj = RenderContext::builder()
            .insert("as", "df")
            .insert("qw", "er")
            .insert("zx", "cv")
            .build();
        let context = RenderContext::builder()
            .insert("asdf", internal_obj)
            .build();
        let html = renderer.render(&template, &context).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><body>dfcv</body></html>"#)
    }

    #[test]
    fn test_nested_gets_from_array() {
        let renderer = Renderer::builder()
            .build();
        let expr = r#"(html (body (get (get $a 2) 0)))"#;
        let template = Template::from_str(expr).unwrap();

        let subvec: ContextValue = vec![ContextValue::from("blah"), "123".into(), "this".into()].into();
        let v: ContextValue = vec!["asd".into(), "qwe".into(), subvec, "zxc".into()].into();
        let context = RenderContext::builder()
            .insert("a", v)
            .build();
        let html = renderer.render(&template, &context).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><body>blah</body></html>"#)
    }

    #[test]
    fn test_multi_type_vec() {
        let renderer = Renderer::builder()
            .build();
        let expr = r#"(html (body $a))"#;
        let template = Template::from_str(expr).unwrap();

        let v: ContextValue = vec!["blah".into(), ContextValue::from(123), "this".into(), ContextValue::from(true)].into();
        let context = RenderContext::builder()
            .insert("a", v)
            .build();
        let html = renderer.render(&template, &context).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><body>blah123thistrue</body></html>"#)
    }
}
