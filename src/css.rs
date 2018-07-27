struct Stylesheet {
    rules: Vec<Rule>,
}

struct Rule {
    selectors: Vec<Selector>,
    declarations: Vec<Declaration>,
}

enum Selector {
    Simple(SimpleSelector),
}

struct SimpleSelector {
    tag_name: Option<String>,
    id: Option<String>,
    class: Vec<String>,
}

struct Declaration {
    name: String,
    value: Value,
}

enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color),
}

enum Unit {
    Px,
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

fn parse_simple_selector(&mut self) -> SimpleSelector {
    let mut selector = SimpleSelector { tag_name: None, id: None, class: Vec::new() };
    while !self.eof() {
        match self.next_char() {
            '#' => {
                self.consume_char();
                selector.id = Some(self.parse_identifier());
            }
            '.' => {
                self.consume_char();
                selector.class.push(self.parse_identifier());
            }
            '*' => {
                // universal selector
                self.consume_char();
            }
            c if valid_identifier_char(c) => {
                selector.tag_name = Some(self.parse_identifier());
            }
            _ => break
        }
    }
    return selector;
}

pub type Specificity = (usize, usize, usize);

impl Selector {
    pub fn specificity(&self) -> Specificity {
        // http://www.w3.org/TR/selectors/#specificity
        let Selector::Simple(ref simple) = *self;
        let a = simple.id.iter().count();
        let b = simple.class.len();
        let c = simple.tag_name.iter().count();
        (a, b, c)
    }
}

fn parse_rule(&mut self) -> Rule {
    Rule {
        selectors: self.parse_selectors(),
        declarations: self.parse_declarations()
    }
}

// Parse a comma-separated list of selectors.
fn parse_selectors(&mut self) -> Vec<Selector> {
    let mut selectors = Vec::new();
    loop {
        selectors.push(Selector::Simple(self.parse_simple_selector()));
        self.consume_whitespace();
        match self.next_char() {
            ',' => { self.consume_char(); self.consume_whitespace(); }
            '{' => break, // start of declarations
            c   => panic!("Unexpected character {} in selector list", c)
        }
    }
    // Return selectors with highest specificity first, for use in matching.
    selectors.sort_by(|a,b| b.specificity().cmp(&a.specificity()));
    return selectors;
}
