#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn document_get_implementation(instance: DOMReference) -> i32;
    fn document_set_implementation(instance: DOMReference, value: i32);
}

pub fn get_implementation(instance: DOMReference) -> i32 {
    unsafe { document_get_implementation(instance) }
}

pub fn set_implementation(instance: DOMReference, value: i32) {
    unsafe {
        document_set_implementation(instance, value);
    }
}
extern "C" {
    fn document_get_url(instance: DOMReference) -> CString;
    fn document_set_url(instance: DOMReference, value: CString);
}

pub fn get_url(instance: DOMReference) -> String {
    unsafe { to_string(document_get_url(instance)) }
}

pub fn set_url(instance: DOMReference, value: &str) {
    unsafe {
        document_set_url(instance, to_cstring(value));
    }
}
extern "C" {
    fn document_get_document_uri(instance: DOMReference) -> CString;
    fn document_set_document_uri(instance: DOMReference, value: CString);
}

pub fn get_document_uri(instance: DOMReference) -> String {
    unsafe { to_string(document_get_document_uri(instance)) }
}

pub fn set_document_uri(instance: DOMReference, value: &str) {
    unsafe {
        document_set_document_uri(instance, to_cstring(value));
    }
}
extern "C" {
    fn document_get_compat_mode(instance: DOMReference) -> CString;
    fn document_set_compat_mode(instance: DOMReference, value: CString);
}

pub fn get_compat_mode(instance: DOMReference) -> String {
    unsafe { to_string(document_get_compat_mode(instance)) }
}

pub fn set_compat_mode(instance: DOMReference, value: &str) {
    unsafe {
        document_set_compat_mode(instance, to_cstring(value));
    }
}
extern "C" {
    fn document_get_character_set(instance: DOMReference) -> CString;
    fn document_set_character_set(instance: DOMReference, value: CString);
}

pub fn get_character_set(instance: DOMReference) -> String {
    unsafe { to_string(document_get_character_set(instance)) }
}

pub fn set_character_set(instance: DOMReference, value: &str) {
    unsafe {
        document_set_character_set(instance, to_cstring(value));
    }
}
extern "C" {
    fn document_get_charset(instance: DOMReference) -> CString;
    fn document_set_charset(instance: DOMReference, value: CString);
}

pub fn get_charset(instance: DOMReference) -> String {
    unsafe { to_string(document_get_charset(instance)) }
}

pub fn set_charset(instance: DOMReference, value: &str) {
    unsafe {
        document_set_charset(instance, to_cstring(value));
    }
}
extern "C" {
    fn document_get_input_encoding(instance: DOMReference) -> CString;
    fn document_set_input_encoding(instance: DOMReference, value: CString);
}

pub fn get_input_encoding(instance: DOMReference) -> String {
    unsafe { to_string(document_get_input_encoding(instance)) }
}

pub fn set_input_encoding(instance: DOMReference, value: &str) {
    unsafe {
        document_set_input_encoding(instance, to_cstring(value));
    }
}
extern "C" {
    fn document_get_content_type(instance: DOMReference) -> CString;
    fn document_set_content_type(instance: DOMReference, value: CString);
}

pub fn get_content_type(instance: DOMReference) -> String {
    unsafe { to_string(document_get_content_type(instance)) }
}

pub fn set_content_type(instance: DOMReference, value: &str) {
    unsafe {
        document_set_content_type(instance, to_cstring(value));
    }
}
extern "C" {
    fn document_get_doctype(instance: DOMReference) -> i32;
    fn document_set_doctype(instance: DOMReference, value: i32);
}

pub fn get_doctype(instance: DOMReference) -> i32 {
    unsafe { document_get_doctype(instance) }
}

pub fn set_doctype(instance: DOMReference, value: i32) {
    unsafe {
        document_set_doctype(instance, value);
    }
}
extern "C" {
    fn document_get_document_element(instance: DOMReference) -> i32;
    fn document_set_document_element(instance: DOMReference, value: i32);
}

pub fn get_document_element(instance: DOMReference) -> i32 {
    unsafe { document_get_document_element(instance) }
}

pub fn set_document_element(instance: DOMReference, value: i32) {
    unsafe {
        document_set_document_element(instance, value);
    }
}
extern "C" {
    fn document_get_elements_by_tag_name(
        instance: DOMReference,
        get_elements_by_tag_name: CString,
    ) -> i32;
}

pub fn get_elements_by_tag_name(instance: DOMReference, local_name: &str) -> i32 {
    unsafe { document_get_elements_by_tag_name(instance, to_cstring(local_name)) }
}
extern "C" {
    fn document_get_elements_by_tag_name_n_s(
        instance: DOMReference,
        get_elements_by_tag_name_n_s: CString,
        get_elements_by_tag_name_n_s: CString,
    ) -> i32;
}

pub fn get_elements_by_tag_name_n_s(
    instance: DOMReference,
    namespace: &str,
    local_name: &str,
) -> i32 {
    unsafe {
        document_get_elements_by_tag_name_n_s(
            instance,
            to_cstring(namespace),
            to_cstring(local_name),
        )
    }
}
extern "C" {
    fn document_get_elements_by_class_name(
        instance: DOMReference,
        get_elements_by_class_name: CString,
    ) -> i32;
}

pub fn get_elements_by_class_name(instance: DOMReference, class_names: &str) -> i32 {
    unsafe { document_get_elements_by_class_name(instance, to_cstring(class_names)) }
}
extern "C" {
    fn document_get_element_by_id(instance: DOMReference, get_element_by_id: CString) -> i32;
}

pub fn get_element_by_id(instance: DOMReference, element_id: &str) -> i32 {
    unsafe { document_get_element_by_id(instance, to_cstring(element_id)) }
}
extern "C" {
    fn document_create_element(
        instance: DOMReference,
        create_element: CString,
        create_element: i32,
    ) -> i32;
}

pub fn create_element(instance: DOMReference, local_name: &str, options: i32) -> i32 {
    unsafe { document_create_element(instance, to_cstring(local_name), options) }
}
extern "C" {
    fn document_create_element_n_s(
        instance: DOMReference,
        create_element_n_s: CString,
        create_element_n_s: CString,
        create_element_n_s: i32,
    ) -> i32;
}

pub fn create_element_n_s(
    instance: DOMReference,
    namespace: &str,
    qualified_name: &str,
    options: i32,
) -> i32 {
    unsafe {
        document_create_element_n_s(
            instance,
            to_cstring(namespace),
            to_cstring(qualified_name),
            options,
        )
    }
}
extern "C" {
    fn document_create_document_fragment(instance: DOMReference) -> i32;
}

pub fn create_document_fragment(instance: DOMReference) -> i32 {
    unsafe { document_create_document_fragment(instance) }
}
extern "C" {
    fn document_create_text_node(instance: DOMReference, create_text_node: CString) -> i32;
}

pub fn create_text_node(instance: DOMReference, data: &str) -> i32 {
    unsafe { document_create_text_node(instance, to_cstring(data)) }
}
extern "C" {
    fn document_create_comment(instance: DOMReference, create_comment: CString) -> i32;
}

pub fn create_comment(instance: DOMReference, data: &str) -> i32 {
    unsafe { document_create_comment(instance, to_cstring(data)) }
}
extern "C" {
    fn document_create_processing_instruction(
        instance: DOMReference,
        create_processing_instruction: CString,
        create_processing_instruction: CString,
    ) -> i32;
}

pub fn create_processing_instruction(instance: DOMReference, target: &str, data: &str) -> i32 {
    unsafe {
        document_create_processing_instruction(instance, to_cstring(target), to_cstring(data))
    }
}
extern "C" {
    fn document_import_node(instance: DOMReference, import_node: i32, import_node: i32) -> i32;
}

pub fn import_node(instance: DOMReference, node: i32, deep: i32) -> i32 {
    unsafe { document_import_node(instance, node, deep) }
}
extern "C" {
    fn document_adopt_node(instance: DOMReference, adopt_node: i32) -> i32;
}

pub fn adopt_node(instance: DOMReference, node: i32) -> i32 {
    unsafe { document_adopt_node(instance, node) }
}
extern "C" {
    fn document_create_event(instance: DOMReference, create_event: CString) -> i32;
}

pub fn create_event(instance: DOMReference, name: &str) -> i32 {
    unsafe { document_create_event(instance, to_cstring(name)) }
}
extern "C" {
    fn document_create_range(instance: DOMReference) -> i32;
}

pub fn create_range(instance: DOMReference) -> i32 {
    unsafe { document_create_range(instance) }
}
extern "C" {
    fn document_create_node_iterator(
        instance: DOMReference,
        create_node_iterator: i32,
        create_node_iterator: f32,
        create_node_iterator: i32,
    ) -> i32;
}

pub fn create_node_iterator(
    instance: DOMReference,
    root: i32,
    what_to_show: f32,
    filter: i32,
) -> i32 {
    unsafe { document_create_node_iterator(instance, root, what_to_show, filter) }
}
extern "C" {
    fn document_create_tree_walker(
        instance: DOMReference,
        create_tree_walker: i32,
        create_tree_walker: f32,
        create_tree_walker: i32,
    ) -> i32;
}

pub fn create_tree_walker(
    instance: DOMReference,
    root: i32,
    what_to_show: f32,
    filter: i32,
) -> i32 {
    unsafe { document_create_tree_walker(instance, root, what_to_show, filter) }
}
extern "C" {
    fn document_create_c_d_a_t_a_section(
        instance: DOMReference,
        create_c_d_a_t_a_section: CString,
    ) -> i32;
}

pub fn create_c_d_a_t_a_section(instance: DOMReference, data: &str) -> i32 {
    unsafe { document_create_c_d_a_t_a_section(instance, to_cstring(data)) }
}
extern "C" {
    fn document_create_attribute(instance: DOMReference, create_attribute: CString) -> i32;
}

pub fn create_attribute(instance: DOMReference, name: &str) -> i32 {
    unsafe { document_create_attribute(instance, to_cstring(name)) }
}
extern "C" {
    fn document_create_attribute_n_s(
        instance: DOMReference,
        create_attribute_n_s: CString,
        create_attribute_n_s: CString,
    ) -> i32;
}

pub fn create_attribute_n_s(instance: DOMReference, namespace: &str, name: &str) -> i32 {
    unsafe { document_create_attribute_n_s(instance, to_cstring(namespace), to_cstring(name)) }
}
extern "C" {
    fn document_get_location(instance: DOMReference) -> i32;
    fn document_set_location(instance: DOMReference, value: i32);
}

pub fn get_location(instance: DOMReference) -> i32 {
    unsafe { document_get_location(instance) }
}

pub fn set_location(instance: DOMReference, value: i32) {
    unsafe {
        document_set_location(instance, value);
    }
}
extern "C" {
    fn document_get_referrer(instance: DOMReference) -> CString;
    fn document_set_referrer(instance: DOMReference, value: CString);
}

pub fn get_referrer(instance: DOMReference) -> String {
    unsafe { to_string(document_get_referrer(instance)) }
}

pub fn set_referrer(instance: DOMReference, value: &str) {
    unsafe {
        document_set_referrer(instance, to_cstring(value));
    }
}
extern "C" {
    fn document_get_last_modified(instance: DOMReference) -> CString;
    fn document_set_last_modified(instance: DOMReference, value: CString);
}

pub fn get_last_modified(instance: DOMReference) -> String {
    unsafe { to_string(document_get_last_modified(instance)) }
}

pub fn set_last_modified(instance: DOMReference, value: &str) {
    unsafe {
        document_set_last_modified(instance, to_cstring(value));
    }
}
extern "C" {
    fn document_get_ready_state(instance: DOMReference) -> CString;
    fn document_set_ready_state(instance: DOMReference, value: CString);
}

pub fn get_ready_state(instance: DOMReference) -> String {
    unsafe { to_string(document_get_ready_state(instance)) }
}

pub fn set_ready_state(instance: DOMReference, value: &str) {
    unsafe {
        document_set_ready_state(instance, to_cstring(value));
    }
}
extern "C" {
    fn document_get_title(instance: DOMReference) -> CString;
    fn document_set_title(instance: DOMReference, value: CString);
}

pub fn get_title(instance: DOMReference) -> String {
    unsafe { to_string(document_get_title(instance)) }
}

pub fn set_title(instance: DOMReference, value: &str) {
    unsafe {
        document_set_title(instance, to_cstring(value));
    }
}
extern "C" {
    fn document_get_dir(instance: DOMReference) -> CString;
    fn document_set_dir(instance: DOMReference, value: CString);
}

pub fn get_dir(instance: DOMReference) -> String {
    unsafe { to_string(document_get_dir(instance)) }
}

pub fn set_dir(instance: DOMReference, value: &str) {
    unsafe {
        document_set_dir(instance, to_cstring(value));
    }
}
extern "C" {
    fn document_get_body(instance: DOMReference) -> i32;
    fn document_set_body(instance: DOMReference, value: i32);
}

pub fn get_body(instance: DOMReference) -> i32 {
    unsafe { document_get_body(instance) }
}

pub fn set_body(instance: DOMReference, value: i32) {
    unsafe {
        document_set_body(instance, value);
    }
}
extern "C" {
    fn document_get_head(instance: DOMReference) -> i32;
    fn document_set_head(instance: DOMReference, value: i32);
}

pub fn get_head(instance: DOMReference) -> i32 {
    unsafe { document_get_head(instance) }
}

pub fn set_head(instance: DOMReference, value: i32) {
    unsafe {
        document_set_head(instance, value);
    }
}
extern "C" {
    fn document_get_images(instance: DOMReference) -> i32;
    fn document_set_images(instance: DOMReference, value: i32);
}

pub fn get_images(instance: DOMReference) -> i32 {
    unsafe { document_get_images(instance) }
}

pub fn set_images(instance: DOMReference, value: i32) {
    unsafe {
        document_set_images(instance, value);
    }
}
extern "C" {
    fn document_get_embeds(instance: DOMReference) -> i32;
    fn document_set_embeds(instance: DOMReference, value: i32);
}

pub fn get_embeds(instance: DOMReference) -> i32 {
    unsafe { document_get_embeds(instance) }
}

pub fn set_embeds(instance: DOMReference, value: i32) {
    unsafe {
        document_set_embeds(instance, value);
    }
}
extern "C" {
    fn document_get_plugins(instance: DOMReference) -> i32;
    fn document_set_plugins(instance: DOMReference, value: i32);
}

pub fn get_plugins(instance: DOMReference) -> i32 {
    unsafe { document_get_plugins(instance) }
}

pub fn set_plugins(instance: DOMReference, value: i32) {
    unsafe {
        document_set_plugins(instance, value);
    }
}
extern "C" {
    fn document_get_links(instance: DOMReference) -> i32;
    fn document_set_links(instance: DOMReference, value: i32);
}

pub fn get_links(instance: DOMReference) -> i32 {
    unsafe { document_get_links(instance) }
}

pub fn set_links(instance: DOMReference, value: i32) {
    unsafe {
        document_set_links(instance, value);
    }
}
extern "C" {
    fn document_get_forms(instance: DOMReference) -> i32;
    fn document_set_forms(instance: DOMReference, value: i32);
}

pub fn get_forms(instance: DOMReference) -> i32 {
    unsafe { document_get_forms(instance) }
}

pub fn set_forms(instance: DOMReference, value: i32) {
    unsafe {
        document_set_forms(instance, value);
    }
}
extern "C" {
    fn document_get_scripts(instance: DOMReference) -> i32;
    fn document_set_scripts(instance: DOMReference, value: i32);
}

pub fn get_scripts(instance: DOMReference) -> i32 {
    unsafe { document_get_scripts(instance) }
}

pub fn set_scripts(instance: DOMReference, value: i32) {
    unsafe {
        document_set_scripts(instance, value);
    }
}
extern "C" {
    fn document_get_elements_by_name(instance: DOMReference, get_elements_by_name: CString) -> i32;
}

pub fn get_elements_by_name(instance: DOMReference, element_name: &str) -> i32 {
    unsafe { document_get_elements_by_name(instance, to_cstring(element_name)) }
}
extern "C" {
    fn document_get_default_view(instance: DOMReference) -> i32;
    fn document_set_default_view(instance: DOMReference, value: i32);
}

pub fn get_default_view(instance: DOMReference) -> i32 {
    unsafe { document_get_default_view(instance) }
}

pub fn set_default_view(instance: DOMReference, value: i32) {
    unsafe {
        document_set_default_view(instance, value);
    }
}
extern "C" {
    fn document_has_focus(instance: DOMReference) -> i32;
}

pub fn has_focus(instance: DOMReference) -> i32 {
    unsafe { document_has_focus(instance) }
}
extern "C" {
    fn document_get_onreadystatechange(instance: DOMReference) -> i32;
    fn document_set_onreadystatechange(instance: DOMReference, value: i32);
}

pub fn get_onreadystatechange(instance: DOMReference) -> i32 {
    unsafe { document_get_onreadystatechange(instance) }
}

pub fn set_onreadystatechange(instance: DOMReference, value: i32) {
    unsafe {
        document_set_onreadystatechange(instance, value);
    }
}
extern "C" {
    fn document_get_onbeforescriptexecute(instance: DOMReference) -> i32;
    fn document_set_onbeforescriptexecute(instance: DOMReference, value: i32);
}

pub fn get_onbeforescriptexecute(instance: DOMReference) -> i32 {
    unsafe { document_get_onbeforescriptexecute(instance) }
}

pub fn set_onbeforescriptexecute(instance: DOMReference, value: i32) {
    unsafe {
        document_set_onbeforescriptexecute(instance, value);
    }
}
extern "C" {
    fn document_get_onafterscriptexecute(instance: DOMReference) -> i32;
    fn document_set_onafterscriptexecute(instance: DOMReference, value: i32);
}

pub fn get_onafterscriptexecute(instance: DOMReference) -> i32 {
    unsafe { document_get_onafterscriptexecute(instance) }
}

pub fn set_onafterscriptexecute(instance: DOMReference, value: i32) {
    unsafe {
        document_set_onafterscriptexecute(instance, value);
    }
}
extern "C" {
    fn document_get_onselectionchange(instance: DOMReference) -> i32;
    fn document_set_onselectionchange(instance: DOMReference, value: i32);
}

pub fn get_onselectionchange(instance: DOMReference) -> i32 {
    unsafe { document_get_onselectionchange(instance) }
}

pub fn set_onselectionchange(instance: DOMReference, value: i32) {
    unsafe {
        document_set_onselectionchange(instance, value);
    }
}
extern "C" {
    fn document_get_current_script(instance: DOMReference) -> i32;
    fn document_set_current_script(instance: DOMReference, value: i32);
}

pub fn get_current_script(instance: DOMReference) -> i32 {
    unsafe { document_get_current_script(instance) }
}

pub fn set_current_script(instance: DOMReference, value: i32) {
    unsafe {
        document_set_current_script(instance, value);
    }
}
extern "C" {
    fn document_release_capture(instance: DOMReference);
}

pub fn release_capture(instance: DOMReference) {
    unsafe { document_release_capture(instance) }
}
extern "C" {
    fn document_get_document_uri_object(instance: DOMReference) -> i32;
    fn document_set_document_uri_object(instance: DOMReference, value: i32);
}

pub fn get_document_uri_object(instance: DOMReference) -> i32 {
    unsafe { document_get_document_uri_object(instance) }
}

pub fn set_document_uri_object(instance: DOMReference, value: i32) {
    unsafe {
        document_set_document_uri_object(instance, value);
    }
}
extern "C" {
    fn document_get_referrer_policy(instance: DOMReference) -> f32;
    fn document_set_referrer_policy(instance: DOMReference, value: f32);
}

pub fn get_referrer_policy(instance: DOMReference) -> f32 {
    unsafe { document_get_referrer_policy(instance) }
}

pub fn set_referrer_policy(instance: DOMReference, value: f32) {
    unsafe {
        document_set_referrer_policy(instance, value);
    }
}
extern "C" {
    fn document_get_anchors(instance: DOMReference) -> i32;
    fn document_set_anchors(instance: DOMReference, value: i32);
}

pub fn get_anchors(instance: DOMReference) -> i32 {
    unsafe { document_get_anchors(instance) }
}

pub fn set_anchors(instance: DOMReference, value: i32) {
    unsafe {
        document_set_anchors(instance, value);
    }
}
extern "C" {
    fn document_get_applets(instance: DOMReference) -> i32;
    fn document_set_applets(instance: DOMReference, value: i32);
}

pub fn get_applets(instance: DOMReference) -> i32 {
    unsafe { document_get_applets(instance) }
}

pub fn set_applets(instance: DOMReference, value: i32) {
    unsafe {
        document_set_applets(instance, value);
    }
}
extern "C" {
    fn document_get_fullscreen(instance: DOMReference) -> i32;
    fn document_set_fullscreen(instance: DOMReference, value: i32);
}

pub fn get_fullscreen(instance: DOMReference) -> i32 {
    unsafe { document_get_fullscreen(instance) }
}

pub fn set_fullscreen(instance: DOMReference, value: i32) {
    unsafe {
        document_set_fullscreen(instance, value);
    }
}
extern "C" {
    fn document_get_fullscreen_enabled(instance: DOMReference) -> i32;
    fn document_set_fullscreen_enabled(instance: DOMReference, value: i32);
}

pub fn get_fullscreen_enabled(instance: DOMReference) -> i32 {
    unsafe { document_get_fullscreen_enabled(instance) }
}

pub fn set_fullscreen_enabled(instance: DOMReference, value: i32) {
    unsafe {
        document_set_fullscreen_enabled(instance, value);
    }
}
extern "C" {
    fn document_exit_fullscreen(instance: DOMReference);
}

pub fn exit_fullscreen(instance: DOMReference) {
    unsafe { document_exit_fullscreen(instance) }
}
extern "C" {
    fn document_get_onfullscreenchange(instance: DOMReference) -> i32;
    fn document_set_onfullscreenchange(instance: DOMReference, value: i32);
}

pub fn get_onfullscreenchange(instance: DOMReference) -> i32 {
    unsafe { document_get_onfullscreenchange(instance) }
}

pub fn set_onfullscreenchange(instance: DOMReference, value: i32) {
    unsafe {
        document_set_onfullscreenchange(instance, value);
    }
}
extern "C" {
    fn document_get_onfullscreenerror(instance: DOMReference) -> i32;
    fn document_set_onfullscreenerror(instance: DOMReference, value: i32);
}

pub fn get_onfullscreenerror(instance: DOMReference) -> i32 {
    unsafe { document_get_onfullscreenerror(instance) }
}

pub fn set_onfullscreenerror(instance: DOMReference, value: i32) {
    unsafe {
        document_set_onfullscreenerror(instance, value);
    }
}
extern "C" {
    fn document_exit_pointer_lock(instance: DOMReference);
}

pub fn exit_pointer_lock(instance: DOMReference) {
    unsafe { document_exit_pointer_lock(instance) }
}
extern "C" {
    fn document_get_onpointerlockchange(instance: DOMReference) -> i32;
    fn document_set_onpointerlockchange(instance: DOMReference, value: i32);
}

pub fn get_onpointerlockchange(instance: DOMReference) -> i32 {
    unsafe { document_get_onpointerlockchange(instance) }
}

pub fn set_onpointerlockchange(instance: DOMReference, value: i32) {
    unsafe {
        document_set_onpointerlockchange(instance, value);
    }
}
extern "C" {
    fn document_get_onpointerlockerror(instance: DOMReference) -> i32;
    fn document_set_onpointerlockerror(instance: DOMReference, value: i32);
}

pub fn get_onpointerlockerror(instance: DOMReference) -> i32 {
    unsafe { document_get_onpointerlockerror(instance) }
}

pub fn set_onpointerlockerror(instance: DOMReference, value: i32) {
    unsafe {
        document_set_onpointerlockerror(instance, value);
    }
}
extern "C" {
    fn document_get_hidden(instance: DOMReference) -> i32;
    fn document_set_hidden(instance: DOMReference, value: i32);
}

pub fn get_hidden(instance: DOMReference) -> i32 {
    unsafe { document_get_hidden(instance) }
}

pub fn set_hidden(instance: DOMReference, value: i32) {
    unsafe {
        document_set_hidden(instance, value);
    }
}
extern "C" {
    fn document_get_visibility_state(instance: DOMReference) -> i32;
    fn document_set_visibility_state(instance: DOMReference, value: i32);
}

pub fn get_visibility_state(instance: DOMReference) -> i32 {
    unsafe { document_get_visibility_state(instance) }
}

pub fn set_visibility_state(instance: DOMReference, value: i32) {
    unsafe {
        document_set_visibility_state(instance, value);
    }
}
extern "C" {
    fn document_get_onvisibilitychange(instance: DOMReference) -> i32;
    fn document_set_onvisibilitychange(instance: DOMReference, value: i32);
}

pub fn get_onvisibilitychange(instance: DOMReference) -> i32 {
    unsafe { document_get_onvisibilitychange(instance) }
}

pub fn set_onvisibilitychange(instance: DOMReference, value: i32) {
    unsafe {
        document_set_onvisibilitychange(instance, value);
    }
}
extern "C" {
    fn document_get_selected_style_sheet_set(instance: DOMReference) -> CString;
    fn document_set_selected_style_sheet_set(instance: DOMReference, value: CString);
}

pub fn get_selected_style_sheet_set(instance: DOMReference) -> String {
    unsafe { to_string(document_get_selected_style_sheet_set(instance)) }
}

pub fn set_selected_style_sheet_set(instance: DOMReference, value: &str) {
    unsafe {
        document_set_selected_style_sheet_set(instance, to_cstring(value));
    }
}
extern "C" {
    fn document_get_last_style_sheet_set(instance: DOMReference) -> CString;
    fn document_set_last_style_sheet_set(instance: DOMReference, value: CString);
}

pub fn get_last_style_sheet_set(instance: DOMReference) -> String {
    unsafe { to_string(document_get_last_style_sheet_set(instance)) }
}

pub fn set_last_style_sheet_set(instance: DOMReference, value: &str) {
    unsafe {
        document_set_last_style_sheet_set(instance, to_cstring(value));
    }
}
extern "C" {
    fn document_get_preferred_style_sheet_set(instance: DOMReference) -> CString;
    fn document_set_preferred_style_sheet_set(instance: DOMReference, value: CString);
}

pub fn get_preferred_style_sheet_set(instance: DOMReference) -> String {
    unsafe { to_string(document_get_preferred_style_sheet_set(instance)) }
}

pub fn set_preferred_style_sheet_set(instance: DOMReference, value: &str) {
    unsafe {
        document_set_preferred_style_sheet_set(instance, to_cstring(value));
    }
}
extern "C" {
    fn document_get_style_sheet_sets(instance: DOMReference) -> i32;
    fn document_set_style_sheet_sets(instance: DOMReference, value: i32);
}

pub fn get_style_sheet_sets(instance: DOMReference) -> i32 {
    unsafe { document_get_style_sheet_sets(instance) }
}

pub fn set_style_sheet_sets(instance: DOMReference, value: i32) {
    unsafe {
        document_set_style_sheet_sets(instance, value);
    }
}
extern "C" {
    fn document_enable_style_sheets_for_set(
        instance: DOMReference,
        enable_style_sheets_for_set: CString,
    );
}

pub fn enable_style_sheets_for_set(instance: DOMReference, name: &str) {
    unsafe { document_enable_style_sheets_for_set(instance, to_cstring(name)) }
}
extern "C" {
    fn document_caret_position_from_point(
        instance: DOMReference,
        caret_position_from_point: f32,
        caret_position_from_point: f32,
    ) -> i32;
}

pub fn caret_position_from_point(instance: DOMReference, x: f32, y: f32) -> i32 {
    unsafe { document_caret_position_from_point(instance, x, y) }
}
extern "C" {
    fn document_get_scrolling_element(instance: DOMReference) -> i32;
    fn document_set_scrolling_element(instance: DOMReference, value: i32);
}

pub fn get_scrolling_element(instance: DOMReference) -> i32 {
    unsafe { document_get_scrolling_element(instance) }
}

pub fn set_scrolling_element(instance: DOMReference, value: i32) {
    unsafe {
        document_set_scrolling_element(instance, value);
    }
}
extern "C" {
    fn document_query_selector(instance: DOMReference, query_selector: CString) -> i32;
}

pub fn query_selector(instance: DOMReference, selectors: &str) -> i32 {
    unsafe { document_query_selector(instance, to_cstring(selectors)) }
}
extern "C" {
    fn document_query_selector_all(instance: DOMReference, query_selector_all: CString) -> i32;
}

pub fn query_selector_all(instance: DOMReference, selectors: &str) -> i32 {
    unsafe { document_query_selector_all(instance, to_cstring(selectors)) }
}
extern "C" {
    fn document_get_timeline(instance: DOMReference) -> i32;
    fn document_set_timeline(instance: DOMReference, value: i32);
}

pub fn get_timeline(instance: DOMReference) -> i32 {
    unsafe { document_get_timeline(instance) }
}

pub fn set_timeline(instance: DOMReference, value: i32) {
    unsafe {
        document_set_timeline(instance, value);
    }
}
extern "C" {
    fn document_get_animations(instance: DOMReference) -> i32;
}

pub fn get_animations(instance: DOMReference) -> i32 {
    unsafe { document_get_animations(instance) }
}
extern "C" {
    fn document_get_root_element(instance: DOMReference) -> i32;
    fn document_set_root_element(instance: DOMReference, value: i32);
}

pub fn get_root_element(instance: DOMReference) -> i32 {
    unsafe { document_get_root_element(instance) }
}

pub fn set_root_element(instance: DOMReference, value: i32) {
    unsafe {
        document_set_root_element(instance, value);
    }
}
extern "C" {
    fn document_get_is_srcdoc_document(instance: DOMReference) -> i32;
    fn document_set_is_srcdoc_document(instance: DOMReference, value: i32);
}

pub fn get_is_srcdoc_document(instance: DOMReference) -> i32 {
    unsafe { document_get_is_srcdoc_document(instance) }
}

pub fn set_is_srcdoc_document(instance: DOMReference, value: i32) {
    unsafe {
        document_set_is_srcdoc_document(instance, value);
    }
}
extern "C" {
    fn document_get_sandbox_flags_as_string(instance: DOMReference) -> CString;
    fn document_set_sandbox_flags_as_string(instance: DOMReference, value: CString);
}

pub fn get_sandbox_flags_as_string(instance: DOMReference) -> String {
    unsafe { to_string(document_get_sandbox_flags_as_string(instance)) }
}

pub fn set_sandbox_flags_as_string(instance: DOMReference, value: &str) {
    unsafe {
        document_set_sandbox_flags_as_string(instance, to_cstring(value));
    }
}
extern "C" {
    fn document_insert_anonymous_content(
        instance: DOMReference,
        insert_anonymous_content: i32,
    ) -> i32;
}

pub fn insert_anonymous_content(instance: DOMReference, a_element: i32) -> i32 {
    unsafe { document_insert_anonymous_content(instance, a_element) }
}
extern "C" {
    fn document_remove_anonymous_content(instance: DOMReference, remove_anonymous_content: i32);
}

pub fn remove_anonymous_content(instance: DOMReference, a_content: i32) {
    unsafe { document_remove_anonymous_content(instance, a_content) }
}
extern "C" {
    fn document_get_selection(instance: DOMReference) -> i32;
}

pub fn get_selection(instance: DOMReference) -> i32 {
    unsafe { document_get_selection(instance) }
}
extern "C" {
    fn document_get_user_has_interacted(instance: DOMReference) -> i32;
    fn document_set_user_has_interacted(instance: DOMReference, value: i32);
}

pub fn get_user_has_interacted(instance: DOMReference) -> i32 {
    unsafe { document_get_user_has_interacted(instance) }
}

pub fn set_user_has_interacted(instance: DOMReference, value: i32) {
    unsafe {
        document_set_user_has_interacted(instance, value);
    }
}
extern "C" {
    fn document_notify_user_gesture_activation(instance: DOMReference);
}

pub fn notify_user_gesture_activation(instance: DOMReference) {
    unsafe { document_notify_user_gesture_activation(instance) }
}
extern "C" {
    fn document_get_document_flash_classification(instance: DOMReference) -> i32;
    fn document_set_document_flash_classification(instance: DOMReference, value: i32);
}

pub fn get_document_flash_classification(instance: DOMReference) -> i32 {
    unsafe { document_get_document_flash_classification(instance) }
}

pub fn set_document_flash_classification(instance: DOMReference, value: i32) {
    unsafe {
        document_set_document_flash_classification(instance, value);
    }
}
