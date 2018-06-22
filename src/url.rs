use urlparse::quote;
use filters::Filters;

pub fn get_url(src: &String, host: &str, compiler: &str, filters: &Filters, args: &String) -> String {
    let codeeditor = json!(
        {
            "type": "component",
            "componentName": "codeEditor",
            "componentState": {
                "id": 1,
                "source": src,
                "options": {
                    "compileOnChange": true,
                    "colouriseAsm": true
                },
            }
        });

    let compilerstate = json!(
        {
            "type": "component",
            "componentName": "compiler",
            "componentState": {
                "source": 1,
                "filters": {
                    "commentOnly": filters.comments,
                    "directives": filters.directives,
                    "intel": filters.intel,
                    "labels": filters.labels,
                    "trim": true
                },
                "options": args,
                "compiler": compiler,
            }
        }    
    );

    let url_parameters = json!({
        "version": 4,
        "content": [
            {
                "type": "row",
                "content" : [
                    codeeditor,
                    compilerstate
                ]
            }
        ]
    });
    format!("{}/#{}", host, quote(url_parameters.to_string(), b"").unwrap())
}
