use std::env;
use std::fs::File;
use std::io::prelude::*;

use sexp::Atom::*;
use sexp::*;

use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug)]
enum Op1 {
    Add1,
    Sub1,
    IsNum,
    IsBool,
    Print,
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
    Call(String, Vec<Expr>),
}

struct Func {
    name: String,
    args: Vec<String>,
    expr: Expr,
}

struct Prog(Vec<Func>, Expr);

const OP1NAMES: [&str; 5] = ["add1", "sub1", "isnum", "isbool", "print"];
const OP2NAMES: [&str; 8] = ["+", "-", "*", "<", ">", "<=", ">=", "="];
const KEYWORDS: [&str; 8] = ["true", "false", "input", "let", "if", "block", "loop", "break"];

fn check_id(s: &str) -> bool {
    s.starts_with(|c: char| c.is_alphabetic()) && s.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') && !OP1NAMES.contains(&s) && !OP2NAMES.contains(&s) && !KEYWORDS.contains(&s)
}

fn parse_bind(s: &Sexp) -> (String, Expr) {
    match s {
        Sexp::List(vec) => match &vec[..] {
            [Sexp::Atom(S(id)), e] if check_id(id.as_str()) => (id.to_string(), parse_expr(e)),
            _ => panic!("Invalid keyword"),
        },
        _ => panic!("Invalid let expression"),
    }
}

fn parse_expr(s: &Sexp) -> Expr {
    match s {
        Sexp::Atom(I(n)) => {
            let i = i64::try_from(*n).expect("Invalid literal");
            if i < -4611686018427387904 || i > 4611686018427387903 {
                panic!("Invalid literal");
            }
            Expr::Number(i)
        },
        Sexp::Atom(S(n)) if n == "false" => Expr::Boolean(false),
        Sexp::Atom(S(n)) if n == "true" => Expr::Boolean(true),
        Sexp::Atom(S(n)) => Expr::Id(n.to_string()),
        Sexp::List(vec) => match &vec[..] {
            [Sexp::Atom(S(op))] if op == "block" => panic!("Invalid block expression"),
            [Sexp::Atom(S(op)), exprs @ ..] if op == "block" => Expr::Block(exprs.into_iter().map(parse_expr).collect()),
            [Sexp::Atom(S(op)), e] if OP1NAMES.contains(&op.as_str()) => match op.as_str() {
                "add1" => Expr::UnOp(Op1::Add1, Box::new(parse_expr(e))),
                "sub1" => Expr::UnOp(Op1::Sub1, Box::new(parse_expr(e))),
                "isnum" => Expr::UnOp(Op1::IsNum, Box::new(parse_expr(e))),
                "isbool" => Expr::UnOp(Op1::IsBool, Box::new(parse_expr(e))),
                "print" => Expr::UnOp(Op1::Print, Box::new(parse_expr(e))),
                _ => panic!("Invalid unary operator"),
            },
            [Sexp::Atom(S(op)), e1, e2] if OP2NAMES.contains(&op.as_str()) => match op.as_str() {
                "+" => Expr::BinOp(Op2::Plus, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
                "-" => Expr::BinOp(Op2::Minus, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
                "*" => Expr::BinOp(Op2::Times, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
                "<" => Expr::BinOp(Op2::Less, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
                ">" => Expr::BinOp(Op2::Greater, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
                "<=" => Expr::BinOp(Op2::LessEqual, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
                ">=" => Expr::BinOp(Op2::GreaterEqual, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
                "=" => Expr::BinOp(Op2::Equal, Box::new(parse_expr(e1)), Box::new(parse_expr(e2))),
                _ => panic!("Invalid"),
            },
            [Sexp::Atom(S(op)), e1, e2] if op == "let" => match e1 {
                    Sexp::List(b) if !b.is_empty() => Expr::Let(b.iter().map(parse_bind).collect(), Box::new(parse_expr(e2))),
                    _ => panic!("Invalid let expression"),
                },
            [Sexp::Atom(S(op)), e1, e2] if op == "set!" => match e1 {
                    Sexp::Atom(S(n)) => Expr::Set(n.to_string(), Box::new(parse_expr(e2))),
                    _ => panic!("Invalid set! expression"),
                },
            [Sexp::Atom(S(op)), e] if op == "loop" => Expr::Loop(Box::new(parse_expr(e))),
            [Sexp::Atom(S(op)), e] if op == "break" => Expr::Break(Box::new(parse_expr(e))),
            [Sexp::Atom(S(op)), cond, thn, els] if op == "if" => Expr::If(
                Box::new(parse_expr(cond)),
                Box::new(parse_expr(thn)),
                Box::new(parse_expr(els)),
            ),
            [Sexp::Atom(S(n)), exprs @ ..] => Expr::Call(n.to_string(), exprs.into_iter().map(parse_expr).collect()),
            _ => panic!("Invalid expression"),
        },
        _ => panic!("Invalid expression"),
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

fn parse_prog(s: &Sexp) -> Prog {
    match s {
        Sexp::List(vec) if !vec.is_empty() => {
            let fs = &vec[0..vec.len() - 1];
            let r: Vec<Func> = fs.into_iter().map(parse_func).collect();
            Prog(r, parse_expr(&vec[vec.len() - 1]))
        },
        _ => panic!("Invalid program"),
    }
}

#[derive(Debug)]
enum Val {
    Reg(Reg),
    Imm32(i32),
    Imm64(i64),
    RegOffset(Reg, i32),
}

#[derive(Debug)]
enum Reg {
    RAX,
    RBX,
    RCX,
    RDX,
    RSI,
    RDI,
    RSP,
    RBP,
}

#[derive(Debug)]
enum Instr<'a> {
    Mov(Val, Val),
    Add(Val, Val),
    Sub(Val, Val),
    Imul(Val, Val),
    And(Val, Val),
    Xor(Val, Val),
    Sar(Val, Val),
    Cmp(Val, Val),
    Test(Val, Val),
    Push(Val),
    Pop(Val),
    Call(String),
    Leave,
    Ret,
    J(&'a str, String),
    Cmov(&'a str, Val, Val),
    Label(String),
}

struct Context<'a> {
    si: i32,
    env: &'a im::HashMap<String, i32>,
    brake: &'a String,
    fnames: &'a HashMap<String, i32>,
    aligned: bool,
}

struct MutContext {
    label: i32,
}

fn check_num(instrs: &mut Vec<Instr>) {
    instrs.push(Instr::Test(Val::Reg(Reg::RAX), Val::Imm64(1)));
    instrs.push(Instr::Mov(Val::Reg(Reg::RSI), Val::Imm64(1)));
    instrs.push(Instr::J("ne", "my_error".to_string()));
}

fn check_bool(instrs: &mut Vec<Instr>) {
    instrs.push(Instr::Test(Val::Reg(Reg::RAX), Val::Imm64(1)));
    instrs.push(Instr::Mov(Val::Reg(Reg::RSI), Val::Imm64(1)));
    instrs.push(Instr::J("e", "my_error".to_string()));
}

fn check_overflow(instrs: &mut Vec<Instr>) {
    instrs.push(Instr::Mov(Val::Reg(Reg::RSI), Val::Imm64(2)));
    instrs.push(Instr::J("o", "my_error".to_string()));
}

fn new_label(label: &mut i32, s: &str) -> String {
    let cur_label = *label;
    *label += 1;
    format!("{s}_{cur_label}")
}

fn func_label(s: &str) -> String {
    format!("func_{s}")
}

fn compile_unary_op(o: &Op1, e1: &Expr, c: &Context, mc: &mut MutContext, instrs: &mut Vec<Instr>) {
    compile_expr(e1, c, mc, instrs);
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
        },
        Op1::IsBool => {
            instrs.push(Instr::Test(Val::Reg(Reg::RAX), Val::Imm64(1)));
            instrs.push(Instr::Mov(Val::Reg(Reg::RAX), Val::Imm64(1)));
            instrs.push(Instr::Mov(Val::Reg(Reg::RBX), Val::Imm64(3)));
            instrs.push(Instr::Cmov("ne", Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
        },
        Op1::Print => compile_external_call("snek_print", e1, c, mc, instrs),
    }
}

fn compile_binary_op(o: &Op2, e1: &Expr, e2: &Expr, c: &Context, mc: &mut MutContext, instrs: &mut Vec<Instr>) {
    if matches!(o, Op2::Equal) {
        compile_expr(e2, c, mc, instrs);
        instrs.push(Instr::Mov(Val::RegOffset(Reg::RSP, 8 * c.si), Val::Reg(Reg::RAX)));
        compile_expr(e1, &Context { si: c.si + 1, ..*c }, mc, instrs);
        instrs.push(Instr::Mov(Val::Reg(Reg::RBX), Val::Reg(Reg::RAX)));
        instrs.push(Instr::Xor(Val::Reg(Reg::RBX), Val::RegOffset(Reg::RSP, 8 * c.si)));
        instrs.push(Instr::Test(Val::Reg(Reg::RBX), Val::Imm32(1)));
        instrs.push(Instr::Mov(Val::Reg(Reg::RSI), Val::Imm32(1)));
        instrs.push(Instr::J("ne", "my_error".to_string()));
        instrs.push(Instr::Cmp(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, 8 * c.si)));
        instrs.push(Instr::Mov(Val::Reg(Reg::RBX), Val::Imm32(3)));
        instrs.push(Instr::Mov(Val::Reg(Reg::RAX), Val::Imm32(1)));
        instrs.push(Instr::Cmov("e", Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
    } else {
        compile_expr(e2, c, mc, instrs);
        check_num(instrs);
        instrs.push(Instr::Mov(Val::RegOffset(Reg::RSP, 8 * c.si), Val::Reg(Reg::RAX)));
        compile_expr(e1, &Context { si: c.si + 1, ..*c }, mc, instrs);
        check_num(instrs);
        let i = match o {
            Op2::Plus => Instr::Add(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, 8 * c.si)),
            Op2::Minus => Instr::Sub(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, 8 * c.si)),
            Op2::Times => {
                instrs.push(Instr::Sar(Val::Reg(Reg::RAX), Val::Imm32(1)));
                Instr::Imul(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, 8 * c.si))
            },
            _ => {
                instrs.push(Instr::Cmp(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, 8 * c.si)));
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
        compile_expr(ee, &Context { si: c.si, env: &t, ..*c }, mc, instrs);
        instrs.push(Instr::Mov(Val::RegOffset(Reg::RSP, 8 * m_si), Val::Reg(Reg::RAX)));
        t = t.update(id.to_string(), m_si);
        m_si += 1;
    }
    compile_expr(e1, &Context { si: m_si, env: &t, ..*c }, mc, instrs);
}

fn compile_if(cond: &Expr, thn: &Expr, els: &Expr, c: &Context, mc: &mut MutContext, instrs: &mut Vec<Instr>) {
    let lend = new_label(&mut mc.label, "ifend");
        let lelse = new_label(&mut mc.label, "ifelse");
        compile_expr(cond, c, mc, instrs);
        instrs.push(Instr::Cmp(Val::Reg(Reg::RAX), Val::Imm32(1)));
        instrs.push(Instr::J("e", lelse.to_string()));
        compile_expr(thn, c, mc, instrs);
        instrs.push(Instr::J("", lend.to_string()));
        instrs.push(Instr::Label(lelse));
        compile_expr(els, c, mc, instrs);
        instrs.push(Instr::Label(lend));
}

fn compile_loop(e1: &Expr, c: &Context, mc: &mut MutContext, instrs: &mut Vec<Instr>) {
    let lst = new_label(&mut mc.label, "loop");
    let led = new_label(&mut mc.label, "loopend");
    instrs.push(Instr::Label(lst.to_string()));
    compile_expr(e1, &Context{ brake: &led, ..*c}, mc, instrs);
    instrs.push(Instr::J("", lst));
    instrs.push(Instr::Label(led));
}

fn compile_expr(e: &Expr, c: &Context, mc: &mut MutContext, instrs: &mut Vec<Instr>) {
    match e {
        Expr::Number(n) => instrs.push(Instr::Mov(Val::Reg(Reg::RAX), Val::Imm64(n << 1))),
        Expr::Boolean(n) => instrs.push(Instr::Mov(Val::Reg(Reg::RAX), Val::Imm32(if *n {3} else {1}))),
        Expr::Id(id) => {
            let v = *c.env.get(id).expect(format!("Unbound variable identifier {id}").as_str());
            let target = match v {
                i32::MAX => Val::Reg(Reg::RDI),
                w if w >= 0 => Val::RegOffset(Reg::RSP, 8 * w),
                w => Val::RegOffset(Reg::RBP, -8 * (w - 1) )
            };
            instrs.push(Instr::Mov(Val::Reg(Reg::RAX), target))
        },
        Expr::UnOp(o, e1) => compile_unary_op(o, e1, c, mc, instrs),
        Expr::BinOp(o, e1, e2) => compile_binary_op(o, e1, e2, c, mc, instrs),
        Expr::Let(bs, e1) => compile_let(bs, e1, c, mc, instrs),
        Expr::Set(id, e1) => {
            compile_expr(e1, c, mc, instrs);
            let v = *c.env.get(id).expect(format!("Unbound variable identifier {id}").as_str());
            let target = match v {
                i32::MAX => panic!("Unbound variable identifier {id}"),
                w if w >= 0 => Val::RegOffset(Reg::RSP, 8 * w),
                w => Val::RegOffset(Reg::RBP, -8 * (w - 1))
            };
            instrs.push(Instr::Mov(target, Val::Reg(Reg::RAX)))
        },
        Expr::Block(es) => {
            for e1 in es {
                compile_expr(e1, c, mc, instrs);
            }
        },
        Expr::If(cond, thn, els) => compile_if(cond, thn, els, c, mc, instrs),
        Expr::Loop(e1) => compile_loop(e1, c, mc, instrs),
        Expr::Break(e1) => {
            if c.brake == "" {
                panic!("break");
            }
            compile_expr(e1, c, mc, instrs);
            instrs.push(Instr::J("", c.brake.to_string()));
        },
        Expr::Call(n, args) => compile_call(n, args, c, mc, instrs)
    }
}

fn compile_call(n: &str, args: &Vec<Expr>, c: &Context, mc: &mut MutContext, instrs: &mut Vec<Instr>) {
    match c.fnames.get(n) {
        Some(x) => if *x != args.len() as i32 { panic!("Invalid: {} takes {} arguments but {} were given", n, args.len(), x) },
        None => panic!("Invalid: Function {n} undefined"),
    }
    if (args.len() % 2 == 1) == c.aligned {
        instrs.push(Instr::Sub(Val::Reg(Reg::RSP), Val::Imm32(8)));
    }
    let mut a = args.len() % 2 == 0;
    for e in args.iter().rev() {
        compile_expr(e, &Context { aligned: a, ..*c }, mc, instrs);
        instrs.push(Instr::Push(Val::Reg(Reg::RAX)));
        a = !a;
    }
    instrs.push(Instr::Call(func_label(n)));
    instrs.push(Instr::Add(Val::Reg(Reg::RSP), Val::Imm32(8 * (args.len() + args.len() % 2) as i32)));
}

fn compile_external_call(n: &str, arg1: &Expr, c: &Context, mc: &mut MutContext, instrs: &mut Vec<Instr>) {
    // compile_expr(arg1, c, mc, instrs);
    if c.aligned { instrs.push(Instr::Sub(Val::Reg(Reg::RSP), Val::Imm32(8))); }
    instrs.push(Instr::Push(Val::Reg(Reg::RDI)));
    instrs.push(Instr::Mov(Val::Reg(Reg::RDI), Val::Reg(Reg::RAX)));
    instrs.push(Instr::Call(n.to_string()));
    instrs.push(Instr::Pop(Val::Reg(Reg::RDI)));
    if c.aligned { instrs.push(Instr::Add(Val::Reg(Reg::RSP), Val::Imm32(8))); }
}

fn dep(e: &Expr) -> i32 {
    match e {
        Expr::Number(_) | Expr::Boolean(_) | Expr::Id(_) => 0,
        Expr::UnOp(_, e1) => dep(e1),
        Expr::BinOp(_, e1, e2) => dep(e2).max(dep(e1) + 1),
        Expr::Let(bs, e1) => bs.iter().enumerate().map(|(i, (_, e))| dep(e) + i as i32).max().unwrap_or_default().max(dep(e1) + bs.len() as i32),
        Expr::Set(_, e1) => 0,
        Expr::Block(es) => es.iter().map(dep).max().unwrap_or_default(),
        Expr::If(cond, thn, els) => dep(cond).max(dep(thn)).max(dep(els)),
        Expr::Loop(e1) => dep(e1),
        Expr::Break(e1) => dep(e1),
        Expr::Call(_, es) => es.iter().map(dep).max().unwrap_or_default(),
    }
}

fn compile_func_body(n: &str, e: &Expr, c: &Context, mc: &mut MutContext, instrs: &mut Vec<Instr>) {
    instrs.push(Instr::Label(n.to_string()));
    instrs.push(Instr::Push(Val::Reg(Reg::RBP)));
    instrs.push(Instr::Mov(Val::Reg(Reg::RBP), Val::Reg(Reg::RSP)));
    let d = dep(e);
    instrs.push(Instr::Sub(Val::Reg(Reg::RSP), Val::Imm32(8 * d)));
    compile_expr(e, &Context { si: 0, aligned: d % 2 == 0, ..*c }, mc, instrs);
    instrs.push(Instr::Leave);
    instrs.push(Instr::Ret);
}

fn instr_to_str(i: &Instr) -> String {
    match i {
        Instr::Mov(u, v) => format!("mov {}, {}\n", val_to_str(u), val_to_str(v)),
        Instr::Add(u, v) => format!("add {}, {}\n", val_to_str(u), val_to_str(v)),
        Instr::Sub(u, v) => format!("sub {}, {}\n", val_to_str(u), val_to_str(v)),
        Instr::Imul(u, v) => format!("imul {}, {}\n", val_to_str(u), val_to_str(v)),
        Instr::And(u, v) => format!("and {}, {}\n", val_to_str(u), val_to_str(v)),
        Instr::Xor(u, v) => format!("xor {}, {}\n", val_to_str(u), val_to_str(v)),
        Instr::Sar(u, v) => format!("sar {}, {}\n", val_to_str(u), val_to_str(v)),
        Instr::Cmp(u, v) => format!("cmp {}, {}\n", val_to_str(u), val_to_str(v)),
        Instr::Test(u, v) => format!("test {}, {}\n", val_to_str(u), val_to_str(v)),
        Instr::Push(u) => format!("push {}\n", val_to_str(u)),
        Instr::Pop(u) => format!("pop {}\n", val_to_str(u)),
        Instr::Call(l) => format!("call {l}\n"),
        Instr::Leave => "leave\n".to_string(),
        Instr::Ret => "ret\n".to_string(),
        Instr::Cmov(c, u, v) => format!("cmov{} {}, {}\n", c, val_to_str(u), val_to_str(v)),
        Instr::J(c, l) if *c == "" => format!("jmp {l}\n"),
        Instr::J(c, l) => format!("j{} {}\n", *c, l),
        Instr::Label(l) => format!("{l}:\n"),
    }
}

fn val_to_str(v: &Val) -> String {
    match v {
        Val::Reg(r) => reg_to_str(r).to_string(),
        Val::Imm32(n) => format!("{}", n),
        Val::Imm64(n) => format!("{}", n),
        Val::RegOffset(r, n) => {
            let rs = reg_to_str(r);
            if *n > 0 {
                format!("[{} + {}]", rs, n)
            } else {
                format!("[{} - {}]", rs, -n)
            }
        }
    }
}

fn reg_to_str(r: &Reg) -> &str {
    match r {
        Reg::RAX => "rax",
        Reg::RBX => "rbx",
        Reg::RCX => "rcx",
        Reg::RDX => "rdx",
        Reg::RSI => "rsi",
        Reg::RDI => "rdi",
        Reg::RSP => "rsp",
        Reg::RBP => "rbp",
    }
}

fn compile(p: &Prog) -> String {
    let Prog(fs, e) = p;

    let mut instrs: Vec<Instr> = Vec::new();
    let mut mc = MutContext{ label: 0 };
    let nul_brake = "".to_string();
    
    let mut fnames: HashMap<String, i32> = HashMap::new();
    if fs.iter().any(|f| fnames.insert(f.name.to_string(), f.args.len() as i32).is_some()) {
        panic!("Invalid: Function defined multiple times");
    }

    for f in fs {
        let env: im::HashMap<String, i32> = im::HashMap::from_iter(f.args.iter().enumerate().map(|(i, n)| (n.to_string(), -(i as i32 + 1))));
        if env.len() != f.args.len() { panic!("Invalid: Duplicate arguments in function {}", f.name); }
        compile_func_body(&func_label(f.name.as_str()), &f.expr, &Context { si: 0, env: &env, brake: &nul_brake, fnames: &fnames, aligned: true }, &mut mc, &mut instrs)
    }

    let mut env: im::HashMap<String, i32> = im::HashMap::unit("input".to_string(), i32::MAX);
    compile_func_body("our_code_starts_here", e, &Context { si: 0, env: &env, brake: &nul_brake, fnames: &fnames, aligned: true }, &mut mc, &mut instrs);
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

    let prog = parse_prog(&parse(&format!("({in_contents})")).expect("Invalid s-expression"));
    let result = compile(&prog);
    let asm_program = format!(
        "
section .text
extern snek_error
extern snek_print
my_error:
and rsp, -16
mov rdi, rsi
call snek_error
global our_code_starts_here
  {}
",
        result
    );

    let mut out_file = File::create(out_name)?;
    out_file.write_all(asm_program.as_bytes())?;

    Ok(())
}
