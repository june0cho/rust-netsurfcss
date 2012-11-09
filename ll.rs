/*!

Low-level bindings. These are organized into pub modules mirroring the libcss
include files

*/

use core::libc::{c_char, c_void, size_t};
use core::libc::types::common::c99::{int32_t, uint32_t, uint64_t, uint8_t};

use wapcaplet::ll::lwc_string;

#[cfg(target_os = "macos")]
#[nolink]
#[link_args="-L../libcss -lcss -L../libparserutils -lparserutils -L../libwapcaplet -lwapcaplet -liconv"]
extern mod linking { }

#[cfg(target_os = "linux")]
#[nolink]
#[link_args="-L../libcss -lcss -L../libparserutils -lparserutils -L../libwapcaplet -lwapcaplet"]
extern mod linking { }

// Generally true
type c_enum = uint32_t;
type rust_enum = uint;

pub mod functypes {
    // (ptr: *c_void, size: size_t, pw: *c_void)
    pub type css_allocator_fn = *u8;
}

pub mod types {
    pub type css_language_level = uint32_t;

    pub const CSS_LEVEL_1: css_language_level = 0;
    pub const CSS_LEVEL_2: css_language_level = 1;
    pub const CSS_LEVEL_21: css_language_level = 2;
    pub const CSS_LEVEL_3: css_language_level = 3;
    pub const CSS_LEVEL_DEFAULT: css_language_level = 2;

    pub type css_color = uint32_t;

    pub type css_unit = c_enum;

    pub const CSS_UNIT_PX: css_unit = 0x0;
    pub const CSS_UNIT_EX: css_unit = 0x1;
    pub const CSS_UNIT_EM: css_unit = 0x2;
    pub const CSS_UNIT_IN: css_unit = 0x3;
    pub const CSS_UNIT_CM: css_unit = 0x4;
    pub const CSS_UNIT_MM: css_unit = 0x5;
    pub const CSS_UNIT_PT: css_unit = 0x6;
    pub const CSS_UNIT_PC: css_unit = 0x7;
    pub const CSS_UNIT_PCT: css_unit = 0x8;
    pub const CSS_UNIT_DEG: css_unit = 0x9;
    pub const CSS_UNIT_GRAD: css_unit = 0xa;
    pub const CSS_UNIT_RAD: css_unit = 0xb;
    pub const CSS_UNIT_MS: css_unit = 0xc;
    pub const CSS_UNIT_S: css_unit = 0xd;
    pub const CSS_UNIT_HZ: css_unit = 0xe;
    pub const CSS_UNIT_KHZ: css_unit = 0xf;

    pub type css_origin = c_enum;

    pub const CSS_ORIGIN_UA: css_origin = 0;
    pub const CSS_ORIGIN_USER: css_origin = 1;
    pub const CSS_ORIGIN_AUTHOR: css_origin = 2;

    pub const CSS_MEDIA_AURAL: uint64_t = (1 << 0);
    pub const CSS_MEDIA_BRAILLE: uint64_t = (1 << 1);
    pub const CSS_MEDIA_EMBOSSED: uint64_t = (1 << 2);
    pub const CSS_MEDIA_HANDHELD: uint64_t = (1 << 3);
    pub const CSS_MEDIA_PRINT: uint64_t = (1 << 4);
    pub const CSS_MEDIA_PROJECTION: uint64_t = (1 << 5);
    pub const CSS_MEDIA_SCREEN: uint64_t = (1 << 6);
    pub const CSS_MEDIA_SPEECH: uint64_t = (1 << 7);
    pub const CSS_MEDIA_TTY: uint64_t = (1 << 8);
    pub const CSS_MEDIA_TV: uint64_t = (1 << 9);
    pub const CSS_MEDIA_ALL: uint64_t =
        CSS_MEDIA_AURAL | CSS_MEDIA_BRAILLE |
        CSS_MEDIA_EMBOSSED | CSS_MEDIA_HANDHELD |
        CSS_MEDIA_PRINT | CSS_MEDIA_PROJECTION |
        CSS_MEDIA_SCREEN | CSS_MEDIA_SPEECH |
        CSS_MEDIA_TTY | CSS_MEDIA_TV;

    pub type css_computed_style = c_void;

    pub struct css_qname {
        mut ns: *lwc_string,
        mut name: *lwc_string
    }
}

pub mod errors {

    pub type css_error = c_enum;

    pub const CSS_OK: css_error = 0;
    pub const CSS_NOMEM: css_error = 1;
    pub const CSS_BADPARM: css_error = 2;
    pub const CSS_INVALID: css_error = 3;
    pub const CSS_FILENOTFOUND: css_error = 4;
    pub const CSS_NEEDDATA: css_error = 5;
    pub const CSS_BADCHARSET: css_error = 6;
    pub const CSS_EOF: css_error = 7;
    pub const CSS_IMPORTS_PENDING: css_error = 8;
    pub const CSS_PROPERTY_NOT_SET: css_error = 9;
}

pub mod hint {
    use types::css_unit;
    use stylesheet::css_fixed;

    // FIXME: This is not an opaque type
    pub type css_hint = c_void;

    pub struct css_hint_length {
        value: css_fixed,
        unit: css_unit
    }
}

pub mod properties {

    pub type css_properties_e = c_enum;
    
    pub const CSS_PROP_AZIMUTH: css_properties_e			= 0x000;
    pub const CSS_PROP_BACKGROUND_ATTACHMENT: css_properties_e		= 0x001;
    pub const CSS_PROP_BACKGROUND_COLOR: css_properties_e		= 0x002;
    pub const CSS_PROP_BACKGROUND_IMAGE: css_properties_e		= 0x003;
    pub const CSS_PROP_BACKGROUND_POSITION: css_properties_e		= 0x004;
    pub const CSS_PROP_BACKGROUND_REPEAT: css_properties_e		= 0x005;
    pub const CSS_PROP_BORDER_COLLAPSE: css_properties_e		= 0x006;
    pub const CSS_PROP_BORDER_SPACING: css_properties_e			= 0x007;
    pub const CSS_PROP_BORDER_TOP_COLOR: css_properties_e		= 0x008;
    pub const CSS_PROP_BORDER_RIGHT_COLOR: css_properties_e		= 0x009;
    pub const CSS_PROP_BORDER_BOTTOM_COLOR: css_properties_e		= 0x00a;
    pub const CSS_PROP_BORDER_LEFT_COLOR: css_properties_e		= 0x00b;
    pub const CSS_PROP_BORDER_TOP_STYLE: css_properties_e		= 0x00c;
    pub const CSS_PROP_BORDER_RIGHT_STYLE: css_properties_e		= 0x00d;
    pub const CSS_PROP_BORDER_BOTTOM_STYLE: css_properties_e		= 0x00e;
    pub const CSS_PROP_BORDER_LEFT_STYLE: css_properties_e		= 0x00f;
    pub const CSS_PROP_BORDER_TOP_WIDTH: css_properties_e		= 0x010;
    pub const CSS_PROP_BORDER_RIGHT_WIDTH: css_properties_e		= 0x011;
    pub const CSS_PROP_BORDER_BOTTOM_WIDTH: css_properties_e		= 0x012;
    pub const CSS_PROP_BORDER_LEFT_WIDTH: css_properties_e		= 0x013;
    pub const CSS_PROP_BOTTOM: css_properties_e				= 0x014;
    pub const CSS_PROP_CAPTION_SIDE: css_properties_e			= 0x015;
    pub const CSS_PROP_CLEAR: css_properties_e				= 0x016;
    pub const CSS_PROP_CLIP: css_properties_e				= 0x017;
    pub const CSS_PROP_COLOR: css_properties_e				= 0x018;
    pub const CSS_PROP_CONTENT: css_properties_e			= 0x019;
    pub const CSS_PROP_COUNTER_INCREMENT: css_properties_e		= 0x01a;
    pub const CSS_PROP_COUNTER_RESET: css_properties_e			= 0x01b;
    pub const CSS_PROP_CUE_AFTER: css_properties_e			= 0x01c;
    pub const CSS_PROP_CUE_BEFORE: css_properties_e			= 0x01d;
    pub const CSS_PROP_CURSOR: css_properties_e				= 0x01e;
    pub const CSS_PROP_DIRECTION: css_properties_e			= 0x01f;
    pub const CSS_PROP_DISPLAY: css_properties_e			= 0x020;
    pub const CSS_PROP_ELEVATION: css_properties_e			= 0x021;
    pub const CSS_PROP_EMPTY_CELLS: css_properties_e			= 0x022;
    pub const CSS_PROP_FLOAT: css_properties_e				= 0x023;
    pub const CSS_PROP_FONT_FAMILY: css_properties_e			= 0x024;
    pub const CSS_PROP_FONT_SIZE: css_properties_e			= 0x025;
    pub const CSS_PROP_FONT_STYLE: css_properties_e			= 0x026;
    pub const CSS_PROP_FONT_VARIANT: css_properties_e			= 0x027;
    pub const CSS_PROP_FONT_WEIGHT: css_properties_e			= 0x028;
    pub const CSS_PROP_HEIGHT: css_properties_e				= 0x029;
    pub const CSS_PROP_LEFT: css_properties_e				= 0x02a;
    pub const CSS_PROP_LETTER_SPACING: css_properties_e			= 0x02b;
    pub const CSS_PROP_LINE_HEIGHT: css_properties_e			= 0x02c;
    pub const CSS_PROP_LIST_STYLE_IMAGE: css_properties_e		= 0x02d;
    pub const CSS_PROP_LIST_STYLE_POSITION: css_properties_e		= 0x02e;
    pub const CSS_PROP_LIST_STYLE_TYPE: css_properties_e		= 0x02f;
    pub const CSS_PROP_MARGIN_TOP: css_properties_e			= 0x030;
    pub const CSS_PROP_MARGIN_RIGHT: css_properties_e			= 0x031;
    pub const CSS_PROP_MARGIN_BOTTOM: css_properties_e			= 0x032;
    pub const CSS_PROP_MARGIN_LEFT: css_properties_e			= 0x033;
    pub const CSS_PROP_MAX_HEIGHT: css_properties_e			= 0x034;
    pub const CSS_PROP_MAX_WIDTH: css_properties_e			= 0x035;
    pub const CSS_PROP_MIN_HEIGHT: css_properties_e			= 0x036;
    pub const CSS_PROP_MIN_WIDTH: css_properties_e			= 0x037;
    pub const CSS_PROP_ORPHANS: css_properties_e			= 0x038;
    pub const CSS_PROP_OUTLINE_COLOR: css_properties_e			= 0x039;
    pub const CSS_PROP_OUTLINE_STYLE: css_properties_e			= 0x03a;
    pub const CSS_PROP_OUTLINE_WIDTH: css_properties_e			= 0x03b;
    pub const CSS_PROP_OVERFLOW: css_properties_e			= 0x03c;
    pub const CSS_PROP_PADDING_TOP: css_properties_e			= 0x03d;
    pub const CSS_PROP_PADDING_RIGHT: css_properties_e			= 0x03e;
    pub const CSS_PROP_PADDING_BOTTOM: css_properties_e			= 0x03f;
    pub const CSS_PROP_PADDING_LEFT: css_properties_e			= 0x040;
    pub const CSS_PROP_PAGE_BREAK_AFTER: css_properties_e		= 0x041;
    pub const CSS_PROP_PAGE_BREAK_BEFORE: css_properties_e		= 0x042;
    pub const CSS_PROP_PAGE_BREAK_INSIDE: css_properties_e		= 0x043;
    pub const CSS_PROP_PAUSE_AFTER: css_properties_e			= 0x044;
    pub const CSS_PROP_PAUSE_BEFORE: css_properties_e			= 0x045;
    pub const CSS_PROP_PITCH_RANGE: css_properties_e			= 0x046;
    pub const CSS_PROP_PITCH: css_properties_e				= 0x047;
    pub const CSS_PROP_PLAY_DURING: css_properties_e			= 0x048;
    pub const CSS_PROP_POSITION: css_properties_e			= 0x049;
    pub const CSS_PROP_QUOTES: css_properties_e				= 0x04a;
    pub const CSS_PROP_RICHNESS: css_properties_e			= 0x04b;
    pub const CSS_PROP_RIGHT: css_properties_e				= 0x04c;
    pub const CSS_PROP_SPEAK_HEADER: css_properties_e			= 0x04d;
    pub const CSS_PROP_SPEAK_NUMERAL: css_properties_e			= 0x04e;
    pub const CSS_PROP_SPEAK_PUNCTUATION: css_properties_e		= 0x04f;
    pub const CSS_PROP_SPEAK: css_properties_e				= 0x050;
    pub const CSS_PROP_SPEECH_RATE: css_properties_e			= 0x051;
    pub const CSS_PROP_STRESS: css_properties_e				= 0x052;
    pub const CSS_PROP_TABLE_LAYOUT: css_properties_e			= 0x053;
    pub const CSS_PROP_TEXT_ALIGN: css_properties_e			= 0x054;
    pub const CSS_PROP_TEXT_DECORATION: css_properties_e		= 0x055;
    pub const CSS_PROP_TEXT_INDENT: css_properties_e			= 0x056;
    pub const CSS_PROP_TEXT_TRANSFORM: css_properties_e			= 0x057;
    pub const CSS_PROP_TOP: css_properties_e				= 0x058;
    pub const CSS_PROP_UNICODE_BIDI: css_properties_e			= 0x059;
    pub const CSS_PROP_VERTICAL_ALIGN: css_properties_e			= 0x05a;
    pub const CSS_PROP_VISIBILITY: css_properties_e			= 0x05b;
    pub const CSS_PROP_VOICE_FAMILY: css_properties_e			= 0x05c;
    pub const CSS_PROP_VOLUME: css_properties_e				= 0x05d;
    pub const CSS_PROP_WHITE_SPACE: css_properties_e			= 0x05e;
    pub const CSS_PROP_WIDOWS: css_properties_e				= 0x05f;
    pub const CSS_PROP_WIDTH: css_properties_e				= 0x060;
    pub const CSS_PROP_WORD_SPACING: css_properties_e			= 0x061;
    pub const CSS_PROP_Z_INDEX: css_properties_e			= 0x062;
    pub const CSS_PROP_OPACITY: css_properties_e			= 0x063;
    pub const CSS_PROP_BREAK_AFTER: css_properties_e			= 0x064;
    pub const CSS_PROP_BREAK_BEFORE: css_properties_e			= 0x065;
    pub const CSS_PROP_BREAK_INSIDE: css_properties_e			= 0x066;
    pub const CSS_PROP_COLUMN_COUNT: css_properties_e			= 0x067;
    pub const CSS_PROP_COLUMN_FILL: css_properties_e			= 0x068;
    pub const CSS_PROP_COLUMN_GAP: css_properties_e			= 0x069;
    pub const CSS_PROP_COLUMN_RULE_COLOR: css_properties_e		= 0x06a;
    pub const CSS_PROP_COLUMN_RULE_STYLE: css_properties_e		= 0x06b;
    pub const CSS_PROP_COLUMN_RULE_WIDTH: css_properties_e		= 0x06c;
    pub const CSS_PROP_COLUMN_SPAN: css_properties_e			= 0x06d;
    pub const CSS_PROP_COLUMN_WIDTH: css_properties_e			= 0x06e;

    pub type css_font_style_e = c_enum;

    pub const CSS_FONT_STYLE_INHERIT: css_font_style_e = 0;
    pub const CSS_FONT_STYLE_NORMAL: css_font_style_e = 1;
    pub const CSS_FONT_STYLE_ITALIC: css_font_style_e = 2;
    pub const CSS_FONT_STYLE_OBLIQUE: css_font_style_e = 3;

    pub type css_font_variant_e = c_enum;

    pub const CSS_FONT_VARIANT_INHERIT: css_font_variant_e = 0;
    pub const CSS_FONT_VARIANT_NORMAL: css_font_variant_e = 1;
    pub const CSS_FONT_VARIANT_SMALL_CAPS: css_font_variant_e = 2;

    pub type css_font_weight_e = c_enum;

    pub const CSS_FONT_WEIGHT_INHERIT: css_font_weight_e = 0x0;
    pub const CSS_FONT_WEIGHT_NORMAL: css_font_weight_e = 0x1;
    pub const CSS_FONT_WEIGHT_BOLD: css_font_weight_e = 0x2;
    pub const CSS_FONT_WEIGHT_BOLDER: css_font_weight_e = 0x3;
    pub const CSS_FONT_WEIGHT_LIGHTER: css_font_weight_e = 0x4;
    pub const CSS_FONT_WEIGHT_100: css_font_weight_e = 0x5;
    pub const CSS_FONT_WEIGHT_200: css_font_weight_e = 0x6;
    pub const CSS_FONT_WEIGHT_300: css_font_weight_e = 0x7;
    pub const CSS_FONT_WEIGHT_400: css_font_weight_e = 0x8;
    pub const CSS_FONT_WEIGHT_500: css_font_weight_e = 0x9;
    pub const CSS_FONT_WEIGHT_600: css_font_weight_e = 0xa;
    pub const CSS_FONT_WEIGHT_700: css_font_weight_e = 0xb;
    pub const CSS_FONT_WEIGHT_800: css_font_weight_e = 0xc;
    pub const CSS_FONT_WEIGHT_900: css_font_weight_e = 0xd;

    pub type css_font_family_e = c_enum;

    pub const CSS_FONT_FAMILY_INHERIT: css_font_family_e			= 0x0;
    pub const CSS_FONT_FAMILY_SERIF: css_font_family_e			= 0x1;
    pub const CSS_FONT_FAMILY_SANS_SERIF: css_font_family_e		= 0x2;
    pub const CSS_FONT_FAMILY_CURSIVE: css_font_family_e			= 0x3;
    pub const CSS_FONT_FAMILY_FANTASY: css_font_family_e			= 0x4;
    pub const CSS_FONT_FAMILY_MONOSPACE: css_font_family_e		= 0x5;

    pub type css_quotes_e = c_enum;

    pub const CSS_QUOTES_INHERIT: css_quotes_e			= 0x0;
    /* Consult pointer in struct to determine which */
    pub const CSS_QUOTES_STRING: css_quotes_e			= 0x1;
    pub const CSS_QUOTES_NONE: css_quotes_e				= 0x1;

    pub type css_color_e = c_enum;

    pub const CSS_COLOR_INHERIT: css_color_e = 0x0;
    pub const CSS_COLOR_COLOR: css_color_e = 0x1;

    pub type css_border_width_e = c_enum;

    pub const CSS_BORDER_WIDTH_INHERIT: css_border_width_e = 0x0;
    pub const CSS_BORDER_WIDTH_THIN: css_border_width_e = 0x1;
    pub const CSS_BORDER_WIDTH_MEDIUM: css_border_width_e = 0x2;
    pub const CSS_BORDER_WIDTH_THICK: css_border_width_e = 0x3;
    pub const CSS_BORDER_WIDTH_WIDTH: css_border_width_e = 0x4;

    pub type css_margin_e = c_enum;

    pub const CSS_MARGIN_INHERIT: css_margin_e = 0x0;
    pub const CSS_MARGIN_SET: css_margin_e = 0x1;
    pub const CSS_MARGIN_AUTO: css_margin_e = 0x2;

    pub type css_display_e = c_enum;
    
    pub const CSS_DISPLAY_INHERIT: css_display_e = 0x00;
    pub const CSS_DISPLAY_INLINE: css_display_e = 0x01;
    pub const CSS_DISPLAY_BLOCK: css_display_e = 0x02;
    pub const CSS_DISPLAY_LIST_ITEM: css_display_e = 0x03;
    pub const CSS_DISPLAY_RUN_IN: css_display_e = 0x04;
    pub const CSS_DISPLAY_INLINE_BLOCK: css_display_e = 0x05;
    pub const CSS_DISPLAY_TABLE: css_display_e = 0x06;
    pub const CSS_DISPLAY_INLINE_TABLE: css_display_e = 0x07;
    pub const CSS_DISPLAY_TABLE_ROW_GROUP: css_display_e = 0x08;
    pub const CSS_DISPLAY_TABLE_HEADER_GROUP: css_display_e = 0x09;
    pub const CSS_DISPLAY_TABLE_FOOTER_GROUP: css_display_e = 0x0a;
    pub const CSS_DISPLAY_TABLE_ROW: css_display_e = 0x0b;
    pub const CSS_DISPLAY_TABLE_COLUMN_GROUP: css_display_e = 0x0c;
    pub const CSS_DISPLAY_TABLE_COLUMN: css_display_e = 0x0d;
    pub const CSS_DISPLAY_TABLE_CELL: css_display_e = 0x0e;
    pub const CSS_DISPLAY_TABLE_CAPTION: css_display_e = 0x0f;
    pub const CSS_DISPLAY_NONE: css_display_e = 0x10;

    pub type css_float_e = c_enum;

    pub const CSS_FLOAT_INHERIT: css_float_e = 0x0;
    pub const CSS_FLOAT_LEFT: css_float_e = 0x1;
    pub const CSS_FLOAT_RIGHT: css_float_e = 0x2;
    pub const CSS_FLOAT_NONE: css_float_e = 0x3;

    pub type css_position_e = c_enum;

    pub const CSS_POSITION_INHERIT: css_position_e = 0x0;
    pub const CSS_POSITION_STATIC: css_position_e = 0x1;
    pub const CSS_POSITION_RELATIVE: css_position_e = 0x2;
    pub const CSS_POSITION_ABSOLUTE: css_position_e = 0x3;
    pub const CSS_POSITION_FIXED: css_position_e = 0x4;

    pub type css_width_e = c_enum;

    pub const CSS_WIDTH_INHERIT: css_width_e = 0x0;
    pub const CSS_WIDTH_SET: css_width_e = 0x1;
    pub const CSS_WIDTH_AUTO: css_width_e = 0x2;

    pub type css_height_e = c_enum;

    pub const CSS_HEIGHT_INHERIT: css_height_e = 0x0;
    pub const CSS_HEIGHT_SET: css_height_e = 0x1;
    pub const CSS_HEIGHT_AUTO: css_height_e = 0x2;

    pub type css_font_size_e = c_enum;

    pub const CSS_FONT_SIZE_INHERIT: css_font_size_e = 0x0;
    pub const CSS_FONT_SIZE_XX_SMALL: css_font_size_e = 0x1;
    pub const CSS_FONT_SIZE_X_SMALL: css_font_size_e = 0x2;
    pub const CSS_FONT_SIZE_SMALL: css_font_size_e = 0x3;
    pub const CSS_FONT_SIZE_MEDIUM: css_font_size_e = 0x4;
    pub const CSS_FONT_SIZE_LARGE: css_font_size_e = 0x5;
    pub const CSS_FONT_SIZE_X_LARGE: css_font_size_e = 0x6;
    pub const CSS_FONT_SIZE_XX_LARGE: css_font_size_e = 0x7;
    pub const CSS_FONT_SIZE_LARGER: css_font_size_e = 0x8;
    pub const CSS_FONT_SIZE_SMALLER: css_font_size_e = 0x9;
    pub const CSS_FONT_SIZE_DIMENSION: css_font_size_e = 0xa;
    
}

pub mod stylesheet {

    use types::{css_language_level, css_unit};
    use properties::{css_font_style_e, css_font_variant_e, css_font_weight_e};
    use functypes::css_allocator_fn;
    use errors::css_error;

    pub struct css_stylesheet_params {
        params_version: uint32_t,
        level: css_language_level,
        charset: *c_char,
        url: *c_char,
        title: *c_char,
        allow_quirks: bool,
        inline_style: bool,
        resolve: css_url_resolution_fn,
        resolve_pw: *c_void,
        import: css_import_notification_fn,
        import_pw: *c_void,
        color: css_color_resolution_fn,
        color_pw: *c_void,
        font: css_font_resolution_fn,
        font_pw: *c_void
    }

    pub const CSS_STYLESHEET_PARAMS_VERSION_1: uint32_t = 1;

    pub type css_url_resolution_fn = *u8; //extern fn(pw: *c_void, base: *c_char, rel: *lwc_string, abs: **lwc_string) -> css_error;
    pub type css_import_notification_fn = *u8; //extern fn(pw: *c_void, parent: *css_stylesheet, url: *lwc_string, media: *uint64_t) -> css_error;
    pub type css_color_resolution_fn = *u8; //extern fn(pw: *c_void, name: *lwc_string, color: *css_color) -> css_error;
    pub type css_font_resolution_fn = *u8; //extern fn(pw: *c_void, name: *lwc_string, system_font: *css_system_font) -> css_error;

    pub type css_stylesheet = c_void;

    struct css_system_font {
        style: css_font_style_e,
        variant: css_font_variant_e,
        weight: css_font_weight_e,
        size: css_size,
        line_height: css_size,
        family: *lwc_string
    }

    // This isn't in the libcss source. In C it is defined inline with css_system_font
    struct css_size {
        size: css_fixed,
        unit: css_unit
    }

    pub type css_fixed = int32_t;

    extern {
        fn css_stylesheet_create(params: *css_stylesheet_params,
                                 alloc: css_allocator_fn,
                                 alloc_pw: *c_void,
                                 stylesheet: *mut *css_stylesheet) -> css_error;
        fn css_stylesheet_destroy(sheet: *css_stylesheet) -> css_error;
        fn css_stylesheet_size(sheet: *css_stylesheet, size: *mut size_t) -> css_error;
        fn css_stylesheet_append_data(sheet: *css_stylesheet, data: *const uint8_t, len: size_t) -> css_error;
        fn css_stylesheet_data_done(sheet: *css_stylesheet) -> css_error;
    }
}

pub mod select {
    use functypes::css_allocator_fn;
    use errors::css_error;
    use stylesheet::css_stylesheet;
    use types::{css_origin, css_computed_style};

    pub type css_select_ctx = c_void;

    pub type css_pseudo_element = c_enum;

    pub const CSS_PSEUDO_ELEMENT_NONE: css_pseudo_element = 0;
    pub const CSS_PSEUDO_ELEMENT_FIRST_LINE: css_pseudo_element = 1;
    pub const CSS_PSEUDO_ELEMENT_FIRST_LETTER: css_pseudo_element = 2;
    pub const CSS_PSEUDO_ELEMENT_BEFORE: css_pseudo_element = 3;
    pub const CSS_PESUDO_ELEMENT_AFTER: css_pseudo_element = 4;
    pub const CSS_PSEUDO_ELEMENT_COUNT: css_pseudo_element = 5;

    pub struct css_select_results {
        alloc: css_allocator_fn,
        pw: *c_void,
        styles: [*css_computed_style * 5] // 5 == CSS_PSEUDO_ELEMENT_COUNT
    }

    pub type opaque_callback = *u8;

    pub const CSS_SELECT_HANDLER_VERSION_1: uint32_t = 1;

    // See select.h for actual callback signatures
    pub struct css_select_handler {
        handler_version: uint32_t,
        node_name: opaque_callback,
        node_classes: opaque_callback,
        node_id: opaque_callback,
        named_ancestor_node: opaque_callback,
        named_parent_node: opaque_callback,
        named_sibling_node: opaque_callback,
        named_generic_sibling_node: opaque_callback,
        parent_node: opaque_callback,
        sibling_node: opaque_callback,
        node_has_name: opaque_callback,
        node_has_class: opaque_callback,
        node_has_id: opaque_callback,
        node_has_attribute: opaque_callback,
        node_has_attribute_equal: opaque_callback,
        node_has_attribute_dashmatch: opaque_callback,
        node_has_attribute_includes: opaque_callback,
        node_has_attribute_prefix: opaque_callback,
        node_has_attribute_suffix: opaque_callback,
        node_has_attribute_substring: opaque_callback,
        node_is_root: opaque_callback,
        node_count_siblings: opaque_callback,
        node_is_empty: opaque_callback,
        node_is_link: opaque_callback,
        node_is_visited: opaque_callback,
        node_is_hover: opaque_callback,
        node_is_active: opaque_callback,
        node_is_focus: opaque_callback,
        node_is_enabled: opaque_callback,
        node_is_disabled: opaque_callback,
        node_is_checked: opaque_callback,
        node_is_target: opaque_callback,
        node_is_lang: opaque_callback,
        node_presentational_hint: opaque_callback,
        ua_default_for_property: opaque_callback,
        compute_font_size: opaque_callback
    }

    extern {
        fn css_select_ctx_create(alloc: css_allocator_fn, pw: *c_void, result: *mut *css_select_ctx) -> css_error;
        fn css_select_ctx_destroy(ctx: *css_select_ctx) -> css_error;
        fn css_select_ctx_append_sheet(ctx: *css_select_ctx, sheet: *css_stylesheet, origin: css_origin, media: uint64_t) -> css_error;
        fn css_select_ctx_count_sheets(ctx: *css_select_ctx, count: *mut uint32_t) -> css_error;
        fn css_select_style(ctx: *css_select_ctx, node: *c_void, media: uint64_t, inline_style: *css_stylesheet, handler: *css_select_handler, pw: *c_void, result: *mut *css_select_results) -> css_error;
        fn css_select_results_destroy(results: *css_select_results) -> css_error;
    }
}

pub mod computed {
    use types::css_color;

    pub type css_computed_style = c_void;

    pub type compute_font_size_cb = *u8; // (pw: *c_void, parent: *css_hint, size: *mut css_hint) -> css_error

    extern {
        fn css_computed_style_compose(parent: *css_computed_style,
                                      child: *css_computed_style,
                                      compute_font_size: compute_font_size_cb,
                                      pw: *c_void,
                                      result: *mut css_computed_style) -> css_error;

        fn css_computed_color(style: *css_computed_style, color: *mut css_color) -> uint8_t;
        fn css_computed_background_color(style: *css_computed_style, color: *mut css_color) -> uint8_t;
        fn css_computed_border_top_width(style: *css_computed_style, length: *mut css_fixed, unit: *mut css_unit) -> uint8_t;
        fn css_computed_border_right_width(style: *css_computed_style, length: *mut css_fixed, unit: *mut css_unit) -> uint8_t;
        fn css_computed_border_bottom_width(style: *css_computed_style, length: *mut css_fixed, unit: *mut css_unit) -> uint8_t;
        fn css_computed_border_left_width(style: *css_computed_style, length: *mut css_fixed, unit: *mut css_unit) -> uint8_t;
        fn css_computed_border_top_color(style: *css_computed_style, color: *mut css_color) -> uint8_t;
        fn css_computed_border_right_color(style: *css_computed_style, color: *mut css_color) -> uint8_t;
        fn css_computed_border_bottom_color(style: *css_computed_style, color: *mut css_color) -> uint8_t;
        fn css_computed_border_left_color(style: *css_computed_style, color: *mut css_color) -> uint8_t;
        fn css_computed_margin_top(style: *css_computed_style, length: *mut css_fixed, unit: *mut css_unit) -> uint8_t;
        fn css_computed_margin_right(style: *css_computed_style, length: *mut css_fixed, unit: *mut css_unit) -> uint8_t;
        fn css_computed_margin_bottom(style: *css_computed_style, length: *mut css_fixed, unit: *mut css_unit) -> uint8_t;
        fn css_computed_margin_left(style: *css_computed_style, length: *mut css_fixed, unit: *mut css_unit) -> uint8_t;
        fn css_computed_display(style: *css_computed_style, root: bool) -> uint8_t;
        fn css_computed_float(style: *css_computed_style) -> uint8_t;
        fn css_computed_position(style: *css_computed_style) -> uint8_t;
        fn css_computed_width(style: *css_computed_style, length: *mut css_fixed, unit: *mut css_unit) -> uint8_t;
        fn css_computed_height(style: *css_computed_style, length: *mut css_fixed, unit: *mut css_unit) -> uint8_t;
        fn css_computed_font_family(style: *css_computed_style, names: *mut **lwc_string) -> uint8_t;
        fn css_computed_font_size(style: *css_computed_style, length: *mut css_fixed, unit: *mut css_unit) -> uint8_t;
    }
}
