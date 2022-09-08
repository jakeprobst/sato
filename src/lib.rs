#![feature(once_cell)]

pub mod builtins;
pub mod context;
pub mod renderer;
pub mod template;



#[cfg(test)]
mod tests {
    use crate::context::{RenderContext, ContextValue};
    use crate::renderer::Renderer;
    use crate::template::{Template, TemplateExprNode};
    

    #[test]
    fn basic_render() {
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
    fn test_alternate_array_iteration() {
        let renderer = Renderer::builder()
            .build();
        let expr = r#"(html (body (for (@ (var i) (iterate $asdf)) (div "iter " $i))))"#;
        let template = Template::from_str(expr).unwrap();
        let context = RenderContext::builder()
            .insert("asdf", vec!["qaz", "wsx", "edc"])
            .build();
        let html = renderer.render(&template, &context).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><body><div>iter qaz</div><div>iter wsx</div><div>iter edc</div></body></html>"#)
    }

    #[test]
    fn test_array_index_iteration() {
        let renderer = Renderer::builder()
            .build();
        let expr = r#"(html (body (for (@ (var i) (index k) (iterate $asdf)) (div $k ": iter " $i))))"#;
        let template = Template::from_str(expr).unwrap();
        let context = RenderContext::builder()
            .insert("asdf", vec!["qaz", "wsx", "edc"])
            .build();
        let html = renderer.render(&template, &context).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><body><div>0: iter qaz</div><div>1: iter wsx</div><div>2: iter edc</div></body></html>"#)
    }

    #[test]
    fn test_range_iteration() {
        let renderer = Renderer::builder()
            .build();
        let expr = r#"(html (body (for (@ (var i) (min 0) (max 3)) (div "iter " $i))))"#;
        let template = Template::from_str(expr).unwrap();
        let context = RenderContext::builder()
            .insert("asdf", vec!["qaz", "wsx", "edc"])
            .build();
        let html = renderer.render(&template, &context).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><body><div>iter 0</div><div>iter 1</div><div>iter 2</div></body></html>"#)
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
    fn test_alternate_object_iteration() {
        let renderer = Renderer::builder()
            .build();
        let expr = r#"(html (body (for (@ (key k) (value v) (iterate $asdf)) (div "key " $k ", value " $v))))"#;
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
        //let expr = r#"(html (for qw in $as (div (@ (class $qw.er)) $qw.zx)))"#;
        let expr = r#"(html (for (@ (var qw) (iterate $as)) (div (@ (class $qw.er)) $qw.zx)))"#;
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
                Ok(vec!["hello there".into()])
            }))
            .build();
        let expr = r#"(html (div (blah something or other)))"#;
        let template = Template::from_str(expr).unwrap();
        let html = renderer.render(&template, &RenderContext::default()).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><div>hello there</div></html>"#)
    }

    #[test]
    fn test_custom_function() {
        fn blah(_: &crate::renderer::Attributes, _: &[&TemplateExprNode], _: &Renderer, _: &RenderContext) -> Result<Vec<String>, crate::renderer::RenderError> {
            Ok(vec!["hello there".into()])
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

                Ok(output)
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
                let mut output: Vec<String> = Vec::new();
                output.push("<blah>".into());
                //output.extend(&mut expr.into_iter().cloned().flatten());
                output.append(&mut renderer.evaluate_multiple(expr, context)?);
                output.push("</blah>".into());
                Ok(output)
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
                Ok(output)
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
                Ok(output)
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
                let mut output: Vec<String> = Vec::new();
                output.push("<blah>".into());

                let mut subcontext = RenderContext::default();
                subcontext.insert("content", attr.get("something").unwrap().clone());
                let suboutput = renderer.render(&subtemplate, &subcontext)?;
                output.push(suboutput);
                output.append(&mut renderer.evaluate_multiple(expr, context)?);
                output.push(match context.get("blah").unwrap() {
                    ContextValue::String(s) => s,
                    _ => panic!("not a str")
                }.clone());

                output.push("</blah>".into());
                Ok(output)
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
        //let expr = r#"(html (body (for i in $asdf (div "iter " $i))))"#;
        let expr = r#"(html (body (for (@ (var i) (min 0) (max 5)) (if (eq (% $i 2) 0) (div $i)))))"#;
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
        //let expr = r#"(html (body (for i in $asdf (div "iter " $i))))"#;
        let expr = r#"(html (body (for (@ (var i) (min $a) (max $b)) (div "iter " $i))))"#;
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
    fn test_math_ops_in_for_min_max() {
        let renderer = Renderer::builder()
            .build();
        let expr = r#"(html (body (for (@ (var i) (min (- $b $a)) (max (+ $b 1))) (div "iter " $i))))"#;
        let template = Template::from_str(expr).unwrap();
        let context = RenderContext::builder()
            .insert("a", 4)
            .insert("b", 7)
            .build();
        let html = renderer.render(&template, &context).unwrap();
        assert_eq!(html, r#"<!doctype html5><html><body><div>iter 3</div><div>iter 4</div><div>iter 5</div><div>iter 6</div><div>iter 7</div></body></html>"#)
    }
}
