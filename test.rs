use wapcaplet::ll::lwc_string;
use hl::*;

// Based off of libcss's examples/example1.c
mod example1 {
    #[test]
    fn run() {
        let data = "h1 { color: red; }\
                    h4 { color: #321; }\
                    h4, h5 { color: #123456; }";

        let params: CssStylesheetParams = CssStylesheetParams {
            params_version: CssStylesheetParamsVersion1,
            level: CssLevel21,
            charset: ~"UTF-8",
            url: ~"foo",
            title: ~"foo",
            allow_quirks: false,
            inline_style: false,
            resolve: Some(resolve_url),
            import: None,
            color: None,
            font: None,
        };

        let sheet: CssStylesheetRef = css_stylesheet_create(move params);
        debug!("stylesheet: %?", sheet);
        debug!("stylesheet size: %?", sheet.size());

        sheet.append_data(str::to_bytes(data));
        sheet.data_done();
        debug!("stylesheet size: %?", sheet.size());


        let select_ctx: CssSelectCtxRef = css_select_ctx_create();
        assert select_ctx.count_sheets() == 0;
        select_ctx.append_sheet(move sheet, CSS_ORIGIN_AUTHOR, CSS_MEDIA_ALL);
        debug!("count sheets: %?", select_ctx.count_sheets());
        assert select_ctx.count_sheets() == 1;

        for uint::range(1, 7) |hh| {
            let element = fmt!("h%u", hh);
            let element_name: LwcStringRef = from_rust_string(element);
            /*let style: CssSelectResultsRef = select_ctx.select_style(element_name,
                                                                     CSS_MEDIA_SCREEN,
                                                                     None,
                                                                     select_handler);
            match style.computed_color(CSS_PSEUDO_ELEMENT_NONE) {
                CssColorInherit => {
                },
                CssColorValue(color) => {
                }
            }*/
        }
    }

    fn resolve_url(_base: &str, _rel: &lwc_string, _abs: & &lwc_string) -> css_error {
        fail ~"resolving url";
    }

    fn select_handler() {
    }
}