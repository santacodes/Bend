use super::{DefId, Name};
use bimap::BiHashMap;
use itertools::Itertools;
use std::fmt;

pub type DefNames = BiHashMap<DefId, Name>;

#[derive(Debug, Clone, Default)]
pub struct DefinitionBook {
  pub def_names: DefNames,
  pub defs: Vec<Definition>,
}

#[derive(Debug, Clone)]
pub struct Definition {
  pub def_id: DefId,
  pub rules: Vec<Rule>,
}

#[derive(Debug, Clone)]
pub struct Rule {
  pub def_id: DefId,
  pub pats: Vec<Pattern>,
  pub body: Term,
}

#[derive(Debug, Clone)]
pub enum Pattern {
  Ctr(Name, Vec<Pattern>),
  U32(u32),
  I32(i32),
  Var(Option<Name>),
}

#[derive(Debug, Clone)]
pub enum Term {
  Lam {
    nam: Option<Name>,
    bod: Box<Term>,
  },
  Var {
    nam: Name,
  },
  /// Like a scopeless lambda, where the variable can occur outside the body
  Chn {
    nam: Name,
    bod: Box<Term>,
  },
  /// The use of a Channel variable.
  Lnk {
    nam: Name,
  },
  /* Let { nam: Name, val: Box<Term>, nxt: Box<Term> }, */
  Ref {
    def_id: DefId,
  },
  App {
    fun: Box<Term>,
    arg: Box<Term>,
  },
  Dup {
    fst: Option<Name>,
    snd: Option<Name>,
    val: Box<Term>,
    nxt: Box<Term>,
  },
  U32 {
    val: u32,
  },
  I32 {
    val: i32,
  },
  /// A numeric operation between built-in numbers.
  Opx {
    op: Opr,
    fst: Box<Term>,
    snd: Box<Term>,
  },
  Sup {
    fst: Box<Term>,
    snd: Box<Term>,
  },
  Era,
}

/// A numeric operator, for built-in machine numbers
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Opr {
  Add,
  Sub,
  Mul,
  Div,
  Mod,
  And,
  Or,
  Xor,
  Shl,
  Shr,
  Ltn,
  Lte,
  Gtn,
  Gte,
  Eql,
  Neq,
}

impl From<Opr> for hvm_core::OP {
  fn from(value: Opr) -> Self {
    match value {
      Opr::Add => hvm_core::OP::ADD,
      _ => todo!(),
    }
  }
}

impl From<hvm_core::OP> for Opr {
  fn from(value: hvm_core::OP) -> Self {
    match value {
      hvm_core::OP::ADD => Opr::Add,
      _ => todo!(),
    }
  }
}

impl From<&hvm_core::OP> for Opr {
  fn from(value: &hvm_core::OP) -> Self {
    match value {
      hvm_core::OP::ADD => Opr::Add,
      hvm_core::OP::SUB => Opr::Sub,
      hvm_core::OP::MUL => Opr::Mul,
      hvm_core::OP::DIV => Opr::Div,
      hvm_core::OP::MOD => Opr::Mod,
      hvm_core::OP::EQ => Opr::Eql,
      hvm_core::OP::NEQ => Opr::Neq,
      hvm_core::OP::LT => Opr::Ltn,
      hvm_core::OP::GT => Opr::Gtn,
      hvm_core::OP::LTE => Opr::Lte,
      hvm_core::OP::GTE => Opr::Gte,
      hvm_core::OP::AND => Opr::And,
      hvm_core::OP::OR => Opr::Or,
    }
  }
}

impl DefinitionBook {
  pub fn new() -> Self {
    Default::default()
  }
}

impl fmt::Display for Opr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Opr::Add => write!(f, "+"),
      Opr::Sub => write!(f, "-"),
      Opr::Mul => write!(f, "*"),
      Opr::Div => write!(f, "/"),
      Opr::Mod => write!(f, "%"),
      Opr::And => write!(f, "&"),
      Opr::Or => write!(f, "|"),
      Opr::Xor => write!(f, "^"),
      Opr::Shl => write!(f, "<<"),
      Opr::Shr => write!(f, ">>"),
      Opr::Ltn => write!(f, "<"),
      Opr::Lte => write!(f, "<="),
      Opr::Gtn => write!(f, ">"),
      Opr::Gte => write!(f, ">="),
      Opr::Eql => write!(f, "=="),
      Opr::Neq => write!(f, "!="),
    }
  }
}

impl Term {
  pub fn to_string(&self, def_names: &DefNames) -> String {
    match self {
      Term::Lam { nam, bod } => {
        format!("λ{} {}", nam.clone().unwrap_or(Name::from_str("*")), bod.to_string(def_names))
      }
      Term::Var { nam } => format!("{nam}"),
      Term::Chn { nam, bod } => format!("λ${} {}", nam, bod.to_string(def_names)),
      Term::Lnk { nam } => format!("${nam}"),
      /* Term::Let { nam, val, nxt } => {
        format!("let {} = {}; {}", nam, val.to_string(def_names), nxt.to_string(def_names))
      } */
      Term::Ref { def_id } => format!("{}", def_names.get_by_left(def_id).unwrap()),
      Term::App { fun, arg } => format!("({} {})", fun.to_string(def_names), arg.to_string(def_names)),
      Term::Dup { fst, snd, val, nxt } => format!(
        "dup {} {} = {}; {}",
        fst.as_ref().map(|x| x.as_str()).unwrap_or("*"),
        snd.as_ref().map(|x| x.as_str()).unwrap_or("*"),
        val.to_string(def_names),
        nxt.to_string(def_names)
      ),
      Term::U32 { val } => format!("{val}"),
      Term::I32 { val } => format!("{val:+}"),
      Term::Opx { op, fst, snd } => {
        format!("({} {} {})", op, fst.to_string(def_names), snd.to_string(def_names))
      }
      Term::Sup { fst, snd } => format!("{{{} {}}}", fst.to_string(def_names), snd.to_string(def_names)),
      Term::Era => "*".to_string(),
    }
  }
}

impl fmt::Display for Pattern {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Pattern::Ctr(name, pats) => write!(f, "({}{})", name, pats.iter().map(|p| format!(" {p}")).join("")),
      Pattern::U32(num) => write!(f, "{num}"),
      Pattern::I32(num) => write!(f, "{num:+}"),
      Pattern::Var(nam) => write!(f, "{}", nam.as_ref().map(|x| x.as_str()).unwrap_or("*")),
    }
  }
}

impl Rule {
  pub fn to_string(&self, def_names: &DefNames) -> String {
    let Rule { def_id, pats, body } = self;
    format!(
      "({}{}) = {}",
      def_names.get_by_left(def_id).unwrap(),
      pats.iter().map(|x| format!(" {x}")).join(""),
      body.to_string(def_names)
    )
  }
}

impl Definition {
  pub fn to_string(&self, def_names: &DefNames) -> String {
    self.rules.iter().map(|x| x.to_string(def_names)).join("")
  }
}

impl DefinitionBook {
  pub fn to_string(&self, def_names: &DefNames) -> String {
    self.defs.iter().map(|x| x.to_string(def_names)).join("\n")
  }
}
