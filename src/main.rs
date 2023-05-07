use std::env;
use std::fs::File;
use std::io::prelude::*;

use sexp::Atom::*;
use sexp::*;

use im::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
enum Op1 {
    Add1,
    Sub1,
    IsNum,
    IsBool,
}

#[derive(Debug)]
enum Op2 {
    Plus,
    Minus,
    Times,
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

#[derive(Debug)]
enum Expr {
    Number(i64),
    Boolean(bool),
    Id(String),
    Let(Vec<(String, Expr)>, Box<Expr>),
    UnOp(Op1, Box<Expr>),
    BinOp(Op2, Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Loop(Box<Expr>),
    Break(Box<Expr>),
    Set(String, Box<Expr>),
    Block(Vec<Expr>),
}

struct Func {
    name: String,
    args: Vec<String>,
    expr: Expr,
}

struct Prog(Vec<Func>, Expr);

fn check_id(s: &str) -> bool {
    let keywords = ["true", "false", "input", "let", "if", "block", "loop", "break", "add1", "sub1", "isnum", "isbool"];
    s.starts_with(|c: char| c.is_alphabetic()) && s.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') && !keywords.contains(&s)
}

fn parse_bind(s: &Sexp) -> (String, Expr) {
    match s {
        Sexp::List(vec) => match &vec[..] {
            [Sexp::Atom(S(id)), e] if check_id(id.as_str()) => (id.to_string(), parse_expr(e)),
            _ => panic!("Invalid keyword"),
        },
        _ => panic!("Invalid"),
    }
}

fn parse_expr(s: &Sexp) -> Expr {
    match s {
        Sexp::Atom(I(n)) => {
            let i = i64::try_from(*n).expect("Invalid");
            if i < -4611686018427387904 || i > 4611686018427387903 {
                panic!("Invalid");
            }
            Expr::Number(i)
        },
        Sexp::Atom(S(n)) if n == "false" => Expr::Boolean(false),
        Sexp::Atom(S(n)) if n == "true" => Expr::Boolean(true),
        Sexp::Atom(S(n)) => Expr::Id(n.to_string()),
        Sexp::List(vec) => match &vec[..] {
            [Sexp::Atom(S(op))] if op == "block" => panic!("Invalid"),
            [Sexp::Atom(S(op)), exprs @ ..] if op == "block" => Expr::Block(exprs.into_iter().map(parse_expr).collect()),
            [Sexp::Atom(S(op)), l, e] => match op.as_str() {
                "let" => match l {
                    Sexp::List(b) if !b.is_empty() => Expr::Let(b.iter().map(parse_bind).collect(), Box::new(parse_expr(e))),
                    _ => panic!("Invalid"),
                },
                "set!" => match l {
                    Sexp::Atom(S(n)) => Expr::Set(n.to_string(), Box::new(parse_expr(e))),
                    _ => panic!("Invalid"),
                },
                "+" => Expr::BinOp(Op2::Plus, Box::new(parse_expr(l)), Box::new(parse_expr(e))),
                "-" => Expr::BinOp(Op2::Minus, Box::new(parse_expr(l)), Box::new(parse_expr(e))),
                "*" => Expr::BinOp(Op2::Times, Box::new(parse_expr(l)), Box::new(parse_expr(e))),
                "<" => Expr::BinOp(Op2::Less, Box::new(parse_expr(l)), Box::new(parse_expr(e))),
                ">" => Expr::BinOp(Op2::Greater, Box::new(parse_expr(l)), Box::new(parse_expr(e))),
                "<=" => Expr::BinOp(Op2::LessEqual, Box::new(parse_expr(l)), Box::new(parse_expr(e))),
                ">=" => Expr::BinOp(Op2::GreaterEqual, Box::new(parse_expr(l)), Box::new(parse_expr(e))),
                "=" => Expr::BinOp(Op2::Equal, Box::new(parse_expr(l)), Box::new(parse_expr(e))),
                _ => panic!("Invalid"),
            },
            [Sexp::Atom(S(op)), e] => match op.as_str() {
                "loop" => Expr::Loop(Box::new(parse_expr(e))),
                "break" => Expr::Break(Box::new(parse_expr(e))),
                "add1" => Expr::UnOp(Op1::Add1, Box::new(parse_expr(e))),
                "sub1" => Expr::UnOp(Op1::Sub1, Box::new(parse_expr(e))),
                "isnum" => Expr::UnOp(Op1::IsNum, Box::new(parse_expr(e))),
                "isbool" => Expr::UnOp(Op1::IsBool, Box::new(parse_expr(e))),
                _ => panic!("Invalid"),
            },
            [Sexp::Atom(S(op)), cond, thn, els] if op == "if" => Expr::If(
                Box::new(parse_expr(cond)),
                Box::new(parse_expr(thn)),
                Box::new(parse_expr(els)),
            ),
            _ => panic!("Invalid"),
        },
        _ => panic!("Invalid"),
    }
}

fn parse_func(f: &Sexp) -> Func {
    match f {
        Sexp::List(vec) => match &vec[..] {
            [Sexp::Atom(S(func)), Sexp::List(b), e] if func == "fun" => {
                let args: Vec<String> = b[1..].iter().map(|e| if let Sexp::Atom(S(s)) = e { s.to_string() } else {panic!("Invalid definition")}).collect();
                if let Sexp::Atom(S(n)) = &b[0] {
                    Func {name: n.to_string(), args: args, expr: parse_expr(e)}
                } else {
                    panic!("Invalid definition")
                }
            },
        _ => panic!("Invalid definition"),
        },
        _ => panic!("Invalid definition"),
    }
}

fn parse_program(s: &Sexp) -> Prog {
    match s {
        Sexp::List(vec) if vec.is_empty() => {
            let fs = &vec[0..vec.len() - 1];
            let r: Vec<Func> = fs.into_iter().map(parse_func).collect();
        },
        _ => panic!("Invalid program"),
    }
    return Prog(vec![], Expr::Number(0))
}

#[derive(Debug)]
enum Val {
    Reg(Reg),
    Imm32(i32),
    Imm64(i64),
    RegOffset(Reg, i64),
}

#[derive(Debug)]
enum Reg {
    RAX,
    RBX,
    RDI,
    RSP,
}

#[derive(Debug)]
enum Instr<'a> {
    Label(String),
    Mov(Val, Val),
    Add(Val, Val),
    Sub(Val, Val),
    Imul(Val, Val),
    Xor(Val, Val),
    Sar(Val, Val),
    Cmp(Val, Val),
    Test(Val, Val),
    J(&'a str, String),
    Cmov(&'a str, Val, Val),
}

struct Context<'a> {
    si: i64,
    env: &'a im::HashMap<String, i64>,
    brake: &'a String,
}

struct MutContext {
    label: i64,
}

fn check_num(instrs: &mut Vec<Instr>) {
    instrs.push(Instr::Test(Val::Reg(Reg::RAX), Val::Imm64(1)));
    instrs.push(Instr::J("ne", "invalid_error".to_string()));
}

fn check_bool(instrs: &mut Vec<Instr>) {
    instrs.push(Instr::Test(Val::Reg(Reg::RAX), Val::Imm64(1)));
    instrs.push(Instr::J("e", "invalid_error".to_string()));
}

fn check_overflow(instrs: &mut Vec<Instr>) {
    instrs.push(Instr::J("o", "overflow_error".to_string()));
}

fn new_label(label: &mut i64, s: &str) -> String {
    let cur_label = *label;
    *label += 1;
    format!("{s}_{cur_label}")
}

fn compile_unary_op(o: &Op1, e1: &Expr, c: &Context, mc: &mut MutContext, instrs: &mut Vec<Instr>) {
    compile_to_instrs(e1, c, mc, instrs);
    match o {
        Op1::Add1 => {
            check_num(instrs);
            instrs.push(Instr::Add(Val::Reg(Reg::RAX), Val::Imm32(2)));
            check_overflow(instrs);
        },
        Op1::Sub1 => {
            check_num(instrs);
            instrs.push(Instr::Sub(Val::Reg(Reg::RAX), Val::Imm32(2)));
            check_overflow(instrs);
        },
        Op1::IsNum => {
            instrs.push(Instr::Test(Val::Reg(Reg::RAX), Val::Imm64(1)));
            instrs.push(Instr::Mov(Val::Reg(Reg::RAX), Val::Imm64(1)));
            instrs.push(Instr::Mov(Val::Reg(Reg::RBX), Val::Imm64(3)));
            instrs.push(Instr::Cmov("e", Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
        }
        Op1::IsBool => {
            instrs.push(Instr::Test(Val::Reg(Reg::RAX), Val::Imm64(1)));
            instrs.push(Instr::Mov(Val::Reg(Reg::RAX), Val::Imm64(1)));
            instrs.push(Instr::Mov(Val::Reg(Reg::RBX), Val::Imm64(3)));
            instrs.push(Instr::Cmov("ne", Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
        }
    }
}

fn compile_binary_op(o: &Op2, e1: &Expr, e2: &Expr, c: &Context, mc: &mut MutContext, instrs: &mut Vec<Instr>) {
    if matches!(o, Op2::Equal) {
        compile_to_instrs(e2, c, mc, instrs);
        instrs.push(Instr::Mov(Val::RegOffset(Reg::RSP, -8 * c.si), Val::Reg(Reg::RAX)));
        compile_to_instrs(e1, &Context { si: c.si + 1, ..*c }, mc, instrs);
        instrs.push(Instr::Mov(Val::Reg(Reg::RBX), Val::Reg(Reg::RAX)));
        instrs.push(Instr::Xor(Val::Reg(Reg::RBX), Val::RegOffset(Reg::RSP, -8 * c.si)));
        instrs.push(Instr::Test(Val::Reg(Reg::RBX), Val::Imm32(1)));
        instrs.push(Instr::J("ne", "invalid_error".to_string()));
        instrs.push(Instr::Cmp(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, -8 * c.si)));
        instrs.push(Instr::Mov(Val::Reg(Reg::RBX), Val::Imm32(3)));
        instrs.push(Instr::Mov(Val::Reg(Reg::RAX), Val::Imm32(1)));
        instrs.push(Instr::Cmov("e", Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
    } else {
        compile_to_instrs(e2, c, mc, instrs);
        check_num(instrs);
        instrs.push(Instr::Mov(Val::RegOffset(Reg::RSP, -8 * c.si), Val::Reg(Reg::RAX)));
        compile_to_instrs(e1, &Context { si: c.si + 1, ..*c }, mc, instrs);
        check_num(instrs);
        let i = match o {
            Op2::Plus => Instr::Add(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, -8 * c.si)),
            Op2::Minus => Instr::Sub(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, -8 * c.si)),
            Op2::Times => {
                instrs.push(Instr::Sar(Val::Reg(Reg::RAX), Val::Imm32(1)));
                Instr::Imul(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, -8 * c.si))
            },
            _ => {
                instrs.push(Instr::Cmp(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, -8 * c.si)));
                instrs.push(Instr::Mov(Val::Reg(Reg::RBX), Val::Imm32(3)));
                instrs.push(Instr::Mov(Val::Reg(Reg::RAX), Val::Imm32(1)));
                let c = match o {
                    Op2::Less => "l",
                    Op2::LessEqual => "le",
                    Op2::Greater => "g",
                    Op2::GreaterEqual => "ge",
                    _ => panic!("Impossible Branch"),
                };
                Instr::Cmov(c, Val::Reg(Reg::RAX), Val::Reg(Reg::RBX))
            },
        };
        instrs.push(i);
        if matches!(o, Op2::Plus | Op2::Minus | Op2::Times) {
            check_overflow(instrs);
        }
    }
}

fn compile_let(bs: &Vec<(String, Expr)>, e1: &Expr, c: &Context, mc: &mut MutContext, instrs: &mut Vec<Instr>) {
    let mut ids: HashSet<String> = HashSet::new();
    let mut t = c.env.clone();
    let mut m_si = c.si;
    for (id, ee) in bs {
        if !ids.insert(id.to_string()) {
            panic!("Duplicate binding");
        }
        compile_to_instrs(ee, &Context { si: c.si, env: &t, ..*c }, mc, instrs);
        instrs.push(Instr::Mov(Val::RegOffset(Reg::RSP, -8 * m_si), Val::Reg(Reg::RAX)));
        t = t.update(id.to_string(), m_si);
        m_si += 1;
    }
    compile_to_instrs(e1, &Context { si: c.si, env: &t, ..*c }, mc, instrs);
}

fn compile_if(cond: &Expr, thn: &Expr, els: &Expr, c: &Context, mc: &mut MutContext, instrs: &mut Vec<Instr>) {
    let lend = new_label(&mut mc.label, "ifend");
        let lelse = new_label(&mut mc.label, "ifelse");
        compile_to_instrs(cond, c, mc, instrs);
        instrs.push(Instr::Cmp(Val::Reg(Reg::RAX), Val::Imm32(1)));
        instrs.push(Instr::J("e", lelse.to_string()));
        compile_to_instrs(thn, c, mc, instrs);
        instrs.push(Instr::J("", lend.to_string()));
        instrs.push(Instr::Label(lelse));
        compile_to_instrs(els, c, mc, instrs);
        instrs.push(Instr::Label(lend));
}

fn compile_loop(e1: &Expr, c: &Context, mc: &mut MutContext, instrs: &mut Vec<Instr>) {
    let lst = new_label(&mut mc.label, "loop");
    let led = new_label(&mut mc.label, "loopend");
    instrs.push(Instr::Label(lst.to_string()));
    compile_to_instrs(e1, &Context{ brake: &led, ..*c}, mc, instrs);
    instrs.push(Instr::J("", lst));
    instrs.push(Instr::Label(led));
}

fn compile_to_instrs(e: &Expr, c: &Context, mc: &mut MutContext, instrs: &mut Vec<Instr>) {
    match e {
        Expr::Number(n) => instrs.push(Instr::Mov(Val::Reg(Reg::RAX), Val::Imm64(n << 1))),
        Expr::Boolean(n) => instrs.push(Instr::Mov(Val::Reg(Reg::RAX), Val::Imm32(if *n {3} else {1}))),
        Expr::Id(id) => {
            let v = *c.env.get(id).expect(format!("Unbound variable identifier {id}").as_str());
            instrs.push(Instr::Mov(Val::Reg(Reg::RAX), if v == i64::MAX { Val::Reg(Reg::RDI) } else { Val::RegOffset(Reg::RSP, -8 * v) }))
        },
        Expr::UnOp(o, e1) => compile_unary_op(o, e1, c, mc, instrs),
        Expr::BinOp(o, e1, e2) => compile_binary_op(o, e1, e2, c, mc, instrs),
        Expr::Let(bs, e1) => compile_let(bs, e1, c, mc, instrs),
        Expr::Set(id, e1) => {
            compile_to_instrs(e1, c, mc, instrs);
            instrs.push(Instr::Mov(Val::RegOffset(Reg::RSP, -8 * c.env.get(id).expect(format!("Unbound variable identifier {id}").as_str())), Val::Reg(Reg::RAX)))
        },
        Expr::Block(es) => {
            for e1 in es {
                compile_to_instrs(e1, c, mc, instrs);
            }
        },
        Expr::If(cond, thn, els) => compile_if(cond, thn, els, c, mc, instrs),
        Expr::Loop(e1) => compile_loop(e1, c, mc, instrs),
        Expr::Break(e1) => {
            if c.brake == "" {
                panic!("break");
            }
            compile_to_instrs(e1, c, mc, instrs);
            instrs.push(Instr::J("", c.brake.to_string()));
        },
    }
}

fn instr_to_str(i: &Instr) -> String {
    match i {
        Instr::Mov(u, v) => format!("mov {}, {}\n", val_to_str(u), val_to_str(v)),
        Instr::Add(u, v) => format!("add {}, {}\n", val_to_str(u), val_to_str(v)),
        Instr::Sub(u, v) => format!("sub {}, {}\n", val_to_str(u), val_to_str(v)),
        Instr::Imul(u, v) => format!("imul {}, {}\n", val_to_str(u), val_to_str(v)),
        Instr::Xor(u, v) => format!("xor {}, {}\n", val_to_str(u), val_to_str(v)),
        Instr::Sar(u, v) => format!("sar {}, {}\n", val_to_str(u), val_to_str(v)),
        Instr::Cmp(u, v) => format!("cmp {}, {}\n", val_to_str(u), val_to_str(v)),
        Instr::Test(u, v) => format!("test {}, {}\n", val_to_str(u), val_to_str(v)),
        Instr::J(c, l) if *c == "" => format!("jmp {l}\n"),
        Instr::J(c, l) => format!("j{} {}\n", *c, l),
        Instr::Cmov(c, u, v) => format!("cmov{} {}, {}\n", c, val_to_str(u), val_to_str(v)),
        Instr::Label(l) => format!("{l}:\n"),
    }
}

fn val_to_str(v: &Val) -> String {
    match v {
        Val::Reg(r) => match r {
                Reg::RAX => "rax",
                Reg::RBX => "rbx",
                Reg::RDI => "rdi",
                Reg::RSP => "rsp",
        }.to_string(),
        Val::Imm32(n) => format!("{}", n),
        Val::Imm64(n) => format!("{}", n),
        Val::RegOffset(r, n) => {
            let rs = match r {
                Reg::RAX => "rax",
                Reg::RBX => "rbx",
                Reg::RDI => "rdi",
                Reg::RSP => "rsp",
            };
            if *n > 0 {
                format!("[{} + {}]", rs, n)
            } else {
                format!("[{} - {}]", rs, -n)
            }
        }
    }
}

fn compile(e: &Expr) -> String {
    let mut instrs: Vec<Instr> = Vec::new();
    let mut mc = MutContext{ label: 0 };
    let nul_brake = "".to_string();
    let env: HashMap<String, i64> = HashMap::new();
    compile_to_instrs(e, &Context { si: 2, env: &env, brake: &nul_brake }, &mut mc, &mut instrs);
    instrs.iter().map(instr_to_str).collect::<String>()
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let in_name = &args[1];
    let out_name = &args[2];

    // You will make result hold the result of actually compiling
    let mut in_file = File::open(in_name)?;
    let mut in_contents = String::new();
    in_file.read_to_string(&mut in_contents)?;

    let expr = parse_expr(&parse(&in_contents).expect("Invalid s-expression"));
    let result = compile(&expr);
    let asm_program = format!(
        "
section .text
extern snek_error
global our_code_starts_here
invalid_error:
  push rsp
  mov rdi, 22
  call snek_error
  ret
overflow_error:
  push rsp
  mov rdi, 75
  call snek_error
  ret
our_code_starts_here:
  {}
  ret
",
        result
    );

    let mut out_file = File::create(out_name)?;
    out_file.write_all(asm_program.as_bytes())?;

    Ok(())
}
