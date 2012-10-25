use core::libc::{c_void, size_t};
use core::libc::types::common::c99::{uint32_t, int32_t, uint64_t};
use ll::errors::*;
use ll::stylesheet::*;
use ll::types::*;
use ll::select::*;
use ll::hint::*;
use ll::properties::{css_font_style_e, css_font_variant_e, css_font_weight_e};
use ll_css_stylesheet_create = ll::stylesheet::css_stylesheet_create;
use ll_css_select_ctx_create = ll::select::css_select_ctx_create;
use ptr::{null, to_unsafe_ptr, to_mut_unsafe_ptr};
use cast::transmute;
use properties::*;

use wapcaplet::ll::lwc_string;
use wapcaplet::hl::{LwcStringRef, from_rust_string};

pub type CssStylesheet = c_void;

pub enum CssLanguageLevel {
    CssLevel1 = 0,
    CssLevel2 = 1,
    CssLevel21 = 2,
    CssLevel3 = 3,
    CssLevelDefault = 99 // NB: This is not the same as the ll value
}

pub struct CssStylesheetParams {
    params_version: CssStylesheetParamsVersion,
    level: CssLanguageLevel,
    charset: ~str,
    url: ~str,
    title: ~str,
    allow_quirks: bool,
    inline_style: bool,
    resolve: Option<CssUrlResolutionFn>,
    import: Option<CssImportNotificationFn>,
    color: Option<CssColorResolutionFn>,
    font: Option<CssFontResolutionFn>,
}

pub enum CssStylesheetParamsVersion {
    CssStylesheetParamsVersion1 = 1
}

// FIXME: Need hl reprs of lwc_string
pub type CssUrlResolutionFn = ~fn(base: &str, rel: &lwc_string, abs: & &lwc_string) -> css_error;
pub type CssImportNotificationFn = ~fn(parent: &CssStylesheet, url: &lwc_string, media: &uint64_t) -> css_error;
pub type CssColorResolutionFn = ~fn(name: &lwc_string, color: &CssColor) -> css_error;
pub type CssFontResolutionFn = ~fn(name: &lwc_string, system_font: &CssSystemFont) -> css_error;

pub struct CssColor { r: u8, g: u8, b: u8, a: u8 }

pub mod properties {

    // Similar to css_color_e
    pub enum CssColorProp {
        CssColorInherit,
        CssColorValue(CssColor)
    }
}

pub struct CssSystemFont {
    style: css_font_style_e,
    variant: css_font_variant_e,
    weight: css_font_weight_e,
    size: css_size,
    line_height: css_size,
    family: ~str
}

fn ll_result_to_rust_result<T>(code: css_error, val: T) -> CssResult<T> {
    match code {
        CSS_OK => Ok(move val),
        _ => Err(unsafe { transmute(code) })
    }
}

type CssResult<T> = Result<T, css_error>;

fn require_ok(code: css_error, what: &str) {
    match code {
        CSS_OK => (),
        e => fail fmt!("CSS parsing failed while %s. code: %?", what, e)
    }
}

pub struct CssStylesheetRef {
    priv params: CssStylesheetParams,
    priv sheet: *css_stylesheet,

    drop {
        assert self.sheet.is_not_null();
        let code = css_stylesheet_destroy(self.sheet);
        require_ok(code, "destroying stylesheet");
    }
}

fn css_stylesheet_create(params: CssStylesheetParams) -> CssStylesheetRef {
    let sheet = do params.as_ll |ll_params| {
        let mut sheet: *css_stylesheet = null();
        let code = ll_css_stylesheet_create(
            to_unsafe_ptr(ll_params), realloc, null(), to_mut_unsafe_ptr(&mut sheet));
        require_ok(code, "creating stylesheet");
        assert sheet.is_not_null();
        sheet
    };

    CssStylesheetRef {
        // Store the params to keep their pointers alive
        params: move params,
        sheet: sheet
    }
}

impl CssStylesheetRef {
    fn size() -> uint {
        let mut size = 0;
        let code = css_stylesheet_size(self.sheet, to_mut_unsafe_ptr(&mut size));
        require_ok(code, "getting stylesheet size");
        return size as uint;
    }

    fn append_data(data: &[u8]) {
        // FIXME: For some reason to_const_ptr isn't accessible
        let code = css_stylesheet_append_data(self.sheet, unsafe { transmute(vec::raw::to_ptr(data)) }, data.len() as size_t);
        match code {
            CSS_NEEDDATA => { /* fine */ },
            _ => require_ok(code, "appending styleshet data")
        }
    }

    fn data_done() {
        let code = css_stylesheet_data_done(self.sheet);
        require_ok(code, "finishing parsing");
    }
}

extern fn realloc(ptr: *c_void, len: size_t, _pw: *c_void) -> *c_void {
    libc::realloc(ptr, len)
}

struct CssSelectCtxRef {
    priv select_ctx: *css_select_ctx,
    // Whenever a sheet is added to the select ctx we will take ownership of it
    // to ensure that it stays alive
    priv mut sheets: ~[CssStylesheetRef],

    drop {
        assert self.select_ctx.is_not_null();
        let code = css_select_ctx_destroy(self.select_ctx);
        require_ok(code, "destroying select ctx");
    }
}

fn css_select_ctx_create() -> CssSelectCtxRef {
    let mut select_ctx: *css_select_ctx = null();
    let code = ll_css_select_ctx_create(realloc, null(), to_mut_unsafe_ptr(&mut select_ctx));
    require_ok(code, "creating select context");
    assert select_ctx.is_not_null();

    CssSelectCtxRef {
        select_ctx: select_ctx,
        mut sheets: ~[]
    }
}

impl CssSelectCtxRef {
    fn append_sheet(sheet: CssStylesheetRef, origin: css_origin, media: uint64_t) {
        let code = css_select_ctx_append_sheet(self.select_ctx, sheet.sheet, origin, media);
        require_ok(code, "adding sheet to select ctx");

        self.sheets.push(move sheet);
    }

    fn count_sheets() -> uint {
        let mut count = 0;
        let code = css_select_ctx_count_sheets(self.select_ctx, to_mut_unsafe_ptr(&mut count));
        require_ok(code, "counting sheets");
        return count as uint;
    }

    fn select_style<N, H: CssSelectHandler<N>>(node: &N, media: uint64_t,
                                               inline_style: Option<&CssStylesheetRef>,
                                               handler: &H) -> CssSelectResultsRef {
        let raw_handler = build_raw_handler();
        let mut results: *css_select_results = null();
        let code = css_select_style(self.select_ctx,
                                    unsafe { transmute(to_unsafe_ptr(node)) },
                                    media,
                                    null(), // FIXME,
                                    to_unsafe_ptr(&raw_handler),
                                    unsafe { transmute(to_unsafe_ptr(handler)) },
                                    to_mut_unsafe_ptr(&mut results));
        require_ok(code, "selecting style");

        CssSelectResultsRef {
            results: results
        }
    }
}

const CSS_SELECT_HANDLER_VERSION_1: uint32_t = 1;

fn build_raw_handler() -> css_select_handler {
    css_select_handler {
        handler_version: CSS_SELECT_HANDLER_VERSION_1,
        node_name: raw_handler::node_name,
        node_classes: raw_handler::node_classes,
        node_id: raw_handler::node_id,
        named_ancestor_node: raw_handler::named_ancestor_node,
        named_parent_node: raw_handler::named_parent_node,
        named_sibling_node: raw_handler::named_sibling_node,
        named_generic_sibling_node: raw_handler::named_generic_sibling_node,
        parent_node: raw_handler::parent_node,
        sibling_node: raw_handler::sibling_node,
        node_has_name: raw_handler::node_has_name,
        node_has_class: raw_handler::node_has_class,
        node_has_id: raw_handler::node_has_id,
        node_has_attribute: raw_handler::node_has_attribute,
        node_has_attribute_equal: raw_handler::node_has_attribute_equal,
        node_has_attribute_dashmatch: raw_handler::node_has_attribute_dashmatch,
        node_has_attribute_includes: raw_handler::node_has_attribute_includes,
        node_has_attribute_prefix: raw_handler::node_has_attribute_prefix,
        node_has_attribute_suffix: raw_handler::node_has_attribute_suffix,
        node_has_attribute_substring: raw_handler::node_has_attribute_substring,
        node_is_root: raw_handler::node_is_root,
        node_count_siblings: raw_handler::node_count_siblings,
        node_is_empty: raw_handler::node_is_empty,
        node_is_link: raw_handler::node_is_link,
        node_is_visited: raw_handler::node_is_visited,
        node_is_hover: raw_handler::node_is_hover,
        node_is_active: raw_handler::node_is_active,
        node_is_focus: raw_handler::node_is_focus,
        node_is_enabled: raw_handler::node_is_enabled,
        node_is_disabled: raw_handler::node_is_disabled,
        node_is_checked: raw_handler::node_is_checked,
        node_is_target: raw_handler::node_is_target,
        node_is_lang: raw_handler::node_is_lang,
        node_presentational_hint: raw_handler::node_presentational_hint,
        ua_default_for_property: raw_handler::ua_default_for_property,
        compute_font_size: raw_handler::compute_font_size
    }
}

mod raw_handler {
    priv fn enter(n: &str) -> css_error {
        debug!("entering raw handler: %s", n);
        CSS_OK
    }
    pub extern fn node_name(pw: *c_void, node: *c_void, qname: *css_qname) -> css_error {
        enter("node_name")
    }
    pub extern fn node_classes(pw: *c_void, node: *c_void, classes: ***lwc_string, n_classes: *uint32_t) -> css_error {
        enter("node_classes")
    }
    pub extern fn node_id(pw: *c_void, node: *c_void, id: **lwc_string) -> css_error {
        enter("node_id")
    }
    pub extern fn named_ancestor_node(pw: *c_void, node: *c_void, qname: *css_qname, parent: **c_void) -> css_error {
        enter("named_ancestor_node")
    }
    pub extern fn named_parent_node(pw: *c_void, node: *c_void, qname: *css_qname, parent: **c_void) -> css_error {
        enter("named_parent_node")
    }
    pub extern fn named_sibling_node(pw: *c_void, node: *c_void, qname: *css_qname, sibling: **c_void) -> css_error {
        enter("named_sibling_node")
    }
    pub extern fn named_generic_sibling_node(pw: *c_void, node: *c_void, qname: *css_qname, sibling: **c_void) -> css_error {
        enter("named_generic_sibling_node")
    }
    pub extern fn parent_node(pw: *c_void, node: *c_void, parent: **c_void) -> css_error {
        enter("parent_node")
    }
    pub extern fn sibling_node(pw: *c_void, node: *c_void, sibling: **c_void) -> css_error {
        enter("sibling_node")
    }
    pub extern fn node_has_name(pw: *c_void, node: *c_void, qname: *css_qname, match_: *bool) -> css_error {
        enter("node_has_name")
    }
    pub extern fn node_has_class(pw: *c_void, node: *c_void, name: *lwc_string, match_: *bool) -> css_error {
        enter("node_has_class")
    }
    pub extern fn node_has_id(pw: *c_void, node: *c_void, name: *lwc_string, match_: *bool) -> css_error {
        enter("node_has_id")
    }
    pub extern fn node_has_attribute(pw: *c_void, node: *c_void, qname: *css_qname, match_: *bool) -> css_error {
        enter("node_has_attribute")
    }
    pub extern fn node_has_attribute_equal(pw: *c_void, node: *c_void, qname: *css_qname, value: *lwc_string, match_: *bool) -> css_error {
        enter("node_has_attribute_equal")
    }
    pub extern fn node_has_attribute_dashmatch(pw: *c_void, node: *c_void, qname: *css_qname, value: *lwc_string, match_: *bool) -> css_error {
        enter("node_has_attribute_dashmatch")
    }
    pub extern fn node_has_attribute_includes(pw: *c_void, node: *c_void, qname: *css_qname, value: *lwc_string, match_: *bool) -> css_error {
        enter("node_has_attribute_includes")
    }
    pub extern fn node_has_attribute_prefix(pw: *c_void, node: *c_void, qname: *css_qname, value: *lwc_string, match_: *bool) -> css_error {
        enter("node_has_attribute_prefix")
    }
    pub extern fn node_has_attribute_suffix(pw: *c_void, node: *c_void, qname: *css_qname, value: *lwc_string, match_: *bool) -> css_error {
        enter("node_has_attribute_suffix")
    }
    pub extern fn node_has_attribute_substring(pw: *c_void, node: *c_void, qname: *css_qname, value: *lwc_string, match_: *bool) -> css_error {
        enter("node_has_attribute_substring")
    }
    pub extern fn node_is_root(pw: *c_void, node: *c_void, match_: *bool) -> css_error {
        enter("node_is_root")
    }
    pub extern fn node_count_siblings(pw: *c_void, node: *c_void, same_name: bool, after: bool, count: *int32_t) -> css_error {
        enter("node_count_siblings")
    }
    pub extern fn node_is_empty(pw: *c_void, node: *c_void, match_: *bool) -> css_error {
        enter("node_is_empty")
    }
    pub extern fn node_is_link(pw: *c_void, node: *c_void, match_: *bool) -> css_error {
        enter("node_is_link")
    }
    pub extern fn node_is_visited(pw: *c_void, node: *c_void, match_: *bool) -> css_error {
        enter("node_is_visited")
    }
    pub extern fn node_is_hover(pw: *c_void, node: *c_void, match_: *bool) -> css_error {
        enter("node_is_hover")
    }
    pub extern fn node_is_active(pw: *c_void, node: *c_void, match_: *bool) -> css_error {
        enter("node_is_active")
    }
    pub extern fn node_is_focus(pw: *c_void, node: *c_void, match_: *bool) -> css_error {
        enter("node_is_focus")
    }
    pub extern fn node_is_enabled(pw: *c_void, node: *c_void, match_: *bool) -> css_error {
        enter("node_is_enabled")
    }
    pub extern fn node_is_disabled(pw: *c_void, node: *c_void, match_: *bool) -> css_error {
        enter("node_is_disabled")
    }
    pub extern fn node_is_checked(pw: *c_void, node: *c_void, match_: *bool) -> css_error {
        enter("node_is_checked")
    }
    pub extern fn node_is_target(pw: *c_void, node: *c_void, match_: *bool) -> css_error {
        enter("node_is_target")
    }
    pub extern fn node_is_lang(pw: *c_void, node: *c_void, lang: *lwc_string, match_: *bool) -> css_error {
        enter("node_is_lang")
    }
    pub extern fn node_presentational_hint(pw: *c_void, node: *c_void, property: uint32_t, hint: *css_hint) -> css_error {
        enter("node_presentational_hint")
    }
    pub extern fn ua_default_for_property(pw: *c_void, property: uint32_t, hint: *css_hint) -> css_error {
        enter("ua_default_for_property")
    }
    pub extern fn compute_font_size(pw: *c_void, parent: *css_hint, size: *css_hint) -> css_error {
        enter("compute_font_size")
    }
}

trait CssSelectHandler<N> {
    
}

struct CssSelectResultsRef {
    priv results: *css_select_results,

    drop {
        assert self.results.is_not_null();
        let code = css_select_results_destroy(self.results);
        require_ok(code, "destroying select results");
    }
}

impl CssSelectResultsRef {
    fn computed_color(element: css_pseudo_element) -> CssColorProp {
        fail
    }
}
