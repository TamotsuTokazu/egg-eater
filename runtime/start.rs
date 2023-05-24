use std::env;

#[link(name = "our_code")]
extern "C" {
    // The \x01 here is an undocumented feature of LLVM that ensures
    // it does not add an underscore in front of the name.
    // Courtesy of Max New (https://maxsnew.com/teaching/eecs-483-fa22/hw_adder_assignment.html)
    #[link_name = "\x01our_code_starts_here"]
    fn our_code_starts_here(input : i64, memory : *mut i64) -> i64;
}

#[export_name = "\x01snek_error"]
pub extern "C" fn snek_error(errcode: i64) {
    // TODO: print error message according to writeup
    let err_message = match errcode {
        1 => "invalid argument".to_string(),
        2 => "overflow".to_string(),
        3 => "index out of range".to_string(),
        _ => format!("error code {errcode}"),
    };
    eprintln!("an error ocurred {err_message}");
    std::process::exit(1);
}

fn parse_input(input: &str) -> i64 {
    // TODO: parse the input string into internal value representation
    if input == "true" {7}
    else if input == "false" {3}
    else {
        let i = input.parse::<i64>().unwrap();
        if i < -4611686018427387904 || i > 4611686018427387903 {
            panic!("Invalid");
        }
        i << 1
    }
}

fn snek_str(val: i64, seen: &mut Vec<i64>) -> String {
    if val == 7 { "true".to_string()}
    else if val == 3 { "false".to_string() }
    else if val % 2 == 0 { format!("{}", val >> 1) }
    else if val == 1 { "()".to_string() }
    else if val & 1 == 1 {
        if seen.contains(&val) { "(...)".to_string() }
        else {
            seen.push(val);
            let addr = (val - 1) as *const i64;
            let len = unsafe { *addr } >> 1;
            let s = (1..len as isize + 1).map(|i| snek_str(unsafe {*addr.offset(i)}, seen)).collect::<Vec<_>>().join(" ");
            seen.pop();
            format!("({})", s)
        }
    } else { format!("Unknown value: {}", val) }
}

#[export_name = "\x01snek_print"]
fn snek_print(val: i64) -> i64 {
    let mut seen = Vec::<i64>::new();
    println!("{}", snek_str(val, &mut seen));
    val
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() == 2 { &args[1] } else { "false" };
    let input = parse_input(&input);

    let mut memory = Vec::<i64>::with_capacity(0x1000000);
    let buffer :*mut i64 = memory.as_mut_ptr();

    let i: i64 = unsafe { our_code_starts_here(input, buffer) };
    snek_print(i);
}
