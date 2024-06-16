use std::{char, env, fs::{self,File}, io::Write};

fn main() {
    let res = AnalizadorLexico::new().analyze_file(&env::args().collect::<Vec<String>>()[1]);
    File::create("Salida.ALX").unwrap().write_all(res.as_bytes()).expect("");
}

struct AnalizadorLexico{
    aa : i32,
    ai : i32,
    state : i32,
    begin_at : i32,
    line : Vec<char>
}

impl AnalizadorLexico{

    const FORBIDDEN_IN_ANY1: [char; 5] = ['\n', '\r', '\t', '"', '\u{FF}'];
    const FORBIDDEN_IN_ANY2: [char; 3] = ['\n', '\r', '\u{FF}'];
    const FORBIDDEN_IN_ANY3: [char; 1] = ['\u{FF}'];
    const RESERVED_WORDS : [&'static str; 28] = ["declara","fin_declara","dato","numerico","cadena","comienza","termina","funciones","fin_funciones","funcion","fin_funcion","mientras","fin_mientras","si","entonces","otro_caso","fin_si","repite","hasta","lee","escribe","escribe_ret","o","y","no","div","mod","abs"];

    pub fn new () -> Self{
        AnalizadorLexico{aa : 0, ai :0, state :0, begin_at : 0, line : vec![]}
    }

    pub fn analyze_file(&mut self, s : &str) -> String{

        let mut list : Vec<String> = Vec::new();
        self.line = self.read_file(s);

        while !self.end_of_line() {
            self.state = 0;
            self.begin_at = 0;
            list.push(self.compute_token().to_string());
        }

        list.push("fin\nfin".to_string());
        list.iter().filter(|s| !s.is_empty()).map(|s| s.to_string()).collect::<Vec<String>>().join("\n")
    }

    fn compute_token(&mut self) -> Entry{

        let mut c : char;

        loop {

            match self.state{

                0 => {
                    c = self.read_char();
                    if (&*self).is_letter(c){
                        self.state = 1;
                    }else{
                        self.state = self.next_diagram();
                    }
                }

                1 => {
                    c = self.read_char();
                    if (&*self).is_letter(c) || (&*self).is_digit(c) {
                        self.state = 1;
                    }else if c == '_'{
                        self.state = 2;
                    }else{
                        self.state = 3;
                    }
                }

                2 => {
                    c = self.read_char();
                    if (&*self).is_letter(c) || (&*self).is_digit(c) {
                        self.state = 1;
                    }else{
                        self.state = self.next_diagram();  //SCARY
                    }
                }

                3 => {
                    self.aa -= 1;
                    return self.get_entry_of("id")
                }

                4 => {
                    if self.read_char() == '"'{
                        self.state = 5;
                    }else {
                        self.state = self.next_diagram();
                    }
                }

                5 => {
                    c = self.read_char();
                    if self.is_in_any1(c){
                        self.state = 6;
                    }else {
                        self.state = self.next_diagram(); //SCARY
                    }
                }

                6 => {
                    c = self.read_char();
                    if self.is_in_any1(c) {
                        self.state = 6;
                    }else if c == '"' {
                        self.state = 7;
                    }else {
                        self.state = self.next_diagram(); //SCARY
                    }
                }

                7 => {
                    return self.get_entry_of("cad");
                }

                8 => {
                    c = self.read_char();
                    if (&*self).is_digit(c){
                        self.state = 9;
                    }else {
                        self.state = self.next_diagram();
                    }
                }

                9 => {
                    c = self.read_char();
                    if (&*self).is_digit(c){
                        self.state = 9;
                    }else if c == '.'{
                        self.state = 10;
                    }else {
                        self.state = self.next_diagram(); //SCARY
                    }
                }

                10 => {
                    c = self.read_char();
                    if (&*self).is_digit(c){
                        self.state = 11;
                    }else {
                        self.state = self.next_diagram(); //SCARY
                    }
                }

                11 => {
                    c = self.read_char();
                    if (&*self).is_digit(c){
                        self.state = 11;
                    }else {
                        self.state = 12;
                    }
                }

                12 => {
                    self.aa -= 1;
                    return self.get_entry_of("num");
                }

                13 => {
                    c = self.read_char();
                    if (&*self).is_digit(c){
                        self.state = 14;
                    }else {
                        self.state = self.next_diagram();
                    }

                }

                14 => {
                    c = self.read_char();
                    if (&*self).is_digit(c){
                        self.state = 14;
                    }else {
                        self.state = 15;
                    }
                }

                15 => {
                    self.aa -= 1;
                    return self.get_entry_of("num");
                }

                16 => {
                    match self.read_char() {
                        '>' => self.state = 17,
                        '<' => self.state = 20,
                        '=' => self.state = 25,
                         _ => self.state = self.next_diagram() //SCARY
                    }
                }

                17 => {
                    if self.read_char() == '='{
                        self.state = 18;
                    }else {
                        self.state = 19;
                    }
                }

                18 => {
                    return self.get_entry_of("mai");
                }

                19 => {
                    self.aa -= 1;
                    return self.get_entry_of(">");
                }

                20 => {
                    match self.read_char() {
                        '=' => self.state = 21,
                        '-' => self.state = 22,
                        '>' => self.state = 23,
                         _  => self.state = 24
                    }
                }

                21 => {
                    return self.get_entry_of("mei");
                }

                22 => {
                    return self.get_entry_of("asig");
                }

                23 => {
                    return self.get_entry_of("dif");
                }

                24 => {
                    self.aa -= 1;
                    return self.get_entry_of("<");
                }

                25 => {
                    match self.read_char() {
                        '<' => self.state = 21,
                        '>' => self.state = 18,
                         _  => self.state = 26
                    }
                }

                26 => {
                    self.aa -= 1;
                    return self.get_entry_of("=");
                }

                27 => {
                    c = self.read_char();
                    if (&*self).is_delimiter(c) {
                        self.state = 28;
                    }else {
                        self.state = self.next_diagram(); //SCARY
                    }
                }

                28 => {
                    c = self.read_char();
                    if (&*self).is_delimiter(c){
                        self.state = 28;
                    }else {
                        self.state = 29;
                    }
                }

                29 => {
                    self.aa -= 1;
                    return self.get_entry_of("OMITE");
                }

                30 => {
                    match self.read_char() {
                        '+' => self.state = 31,
                        '-' => self.state = 32,
                        '*' => self.state = 35,
                        '/' => self.state = 36,
                        ';' => self.state = 43,
                        '(' => self.state = 44,
                        ')' => self.state = 45,
                        ',' => self.state = 46,
                        ']' => self.state = 47,
                        '[' => self.state = 48,
                        '\u{FF}' => self.state = 49,
                        _ => self.state = self.next_diagram() //SCARY
                    }

                }

                31 => {
                    return self.get_entry_of("+");
                }

                32 => {
                    if self.read_char() == '>' {
                        self.state = 33;
                    }else {
                        self.state = 34;
                    }
                }

                33 => {
                    return self.get_entry_of("opdec");
                }

                34 => {
                    self.aa -= 1;
                    return self.get_entry_of("-");
                }

                35 => {
                    return self.get_entry_of("*");
                }

                36 => {
                    match self.read_char() {
                        '/' => self.state = 37,
                        '*' => self.state = 39,
                         _  => self.state = 42
                    }
                }

                37 => {
                    c = self.read_char();
                    if self.is_in_any2(c) {
                        self.state = 37;
                    }else {
                        self.state = 38;
                    }
                }

                38 => {
                    self.aa -= 1;
                    return self.get_entry_of("OMITE");
                }

                39 => {
                    c = self.read_char();
                    if self.is_in_any3(c){
                        self.state = 39;
                    }else if c == '*'{
                        self.state = 40;
                    } else {
                        self.state = self.next_diagram(); // SCARY
                    }
                }

                40 => {
                    c = self.read_char();
                    if self.is_in_any3(c){
                        self.state = 39;
                    } else if c == '*' {
                        self.state = 40;
                    } else if c == '/' {
                        self.state = 41;
                    } else {
                        self.state = self.next_diagram(); // SCARY
                    }
                }

                41 => {
                    return self.get_entry_of("OMITE");
                }

                42 => {
                    self.aa -= 1;
                    return self.get_entry_of("/");
                }

                43 => {
                    return self.get_entry_of(";");
                }

                44 => {
                    return self.get_entry_of("(");
                }

                45 => {
                    return self.get_entry_of(")");
                }

                46 => {
                    return self.get_entry_of(",");
                }

                47 => {
                    return self.get_entry_of("[");
                }

                48 => {
                    return self.get_entry_of("]");
                }

                49 => {
                    return self.get_entry_of("OMITE");
                }

                _ => ()

            }
        }

    }

    fn end_of_line(&self) -> bool{
        self.aa >= (self.line.len() - 1) as i32
    }

    fn error(&self){
        File::create("Salida.ALX").unwrap().write_all("@666".as_bytes()).expect("");
        panic!()
    }

    fn get_entry_of(&mut self, token : &str) -> Entry{
        let e = Entry::new(token.to_string(), self.get_lexeme());
        self.ai = self.aa;
        return e;
    }

    fn get_lexeme(&self) -> String{
        self.line[self.ai as usize .. self.aa as usize]
            .iter()
            .map(|c| format!("{}",c))
            .collect::<Vec<String>>()
            .join("")
    }

    fn is_in_any1(&self, c : char) -> bool{
        !Self::FORBIDDEN_IN_ANY1.contains(&c)
    }

    fn is_in_any2(&self, c : char) -> bool{
        !Self::FORBIDDEN_IN_ANY2.contains(&c)
    }

    fn is_in_any3(&self, c : char) -> bool{
        !Self::FORBIDDEN_IN_ANY3.contains(&c)
    }

    fn is_delimiter(&self, c: char) -> bool {
        let x = c as u32;
        x == 9 || x == 10 || x == 13 || x == 32
    }

    fn is_digit(&self, c: char) -> bool {
        let x = c as u32;
        x >= '0' as u32 && x <= '9' as u32
    }

    fn is_letter(&self, c: char) -> bool {
        let x = c as u32;
        (x >= 'A' as u32 && x <= 'Z' as u32) || (x >= 'a' as u32 && x <= 'z' as u32)
    }

    fn next_diagram(&mut self) -> i32{
        self.aa = self.ai;
        //println!("state = {}, begin_at = {}",self.state,self.begin_at);
        match self.begin_at {
            0 => self.begin_at = 4,
            4 => self.begin_at = 8,
            8 =>self.begin_at = 13,
            13 =>self.begin_at = 16,
            16 =>self.begin_at = 27,
            27 =>self.begin_at = 30,
            _ => self.error()
        }

       self.begin_at
    }

    fn read_char(&mut self) -> char{
        if self.aa < self.line.len() as i32 {
            let i = self.aa as usize;
            self.aa += 1;
            self.line[i]
        }else {
            '\u{FF}'
        }
    }

    fn read_file(&self,s : &str) -> Vec<char>{
        format!("{} ",fs::read_to_string(s).unwrap()).chars().collect()
    }

}

struct Entry{
    token : String,
    lexema : String
}

impl Entry {

    fn new(token : String, lexema : String) -> Self{

        if token.eq("id") && AnalizadorLexico::RESERVED_WORDS.contains(&lexema.to_lowercase().as_str()){
            return Entry{token : lexema.to_lowercase(), lexema}
        }

        return Entry{token, lexema}
    }

    fn to_string(&self) -> String{
        if self.token == "OMITE"{
            "".to_string()
        }else {
            format!("{}\n{}", self.token, self.lexema)
        }
    }

}
