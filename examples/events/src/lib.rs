use web_dom::*;

#[no_mangle]
pub fn callback(_listener: EventListener, _event: Event) -> () {
    let input = document::query_selector(document(), "input");
    let msg = htmlinput::get_value(input);
    window::alert(window(), &msg);
}

#[no_mangle]
pub fn main() -> () {
    let btn = document::query_selector(document(), "button");
    let listener = create_event_listener();
    eventtarget::add_event_listener(btn, "click", listener);
}
