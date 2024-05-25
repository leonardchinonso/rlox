use first_interpreter::{
    self,
    rlox::{interpreter::Interpreter, parser::Parser, scanner::Scanner},
};

#[test]
fn test_syntax() {
    let source_code = r#"
var a = "global a";
var b = "global b";
var c = "global c";
{
  var a = "outer a";
  var b = "outer b";
  {
    var a = "inner a";
    print a;
    print b;
    print c;
  }
  print a;
  print b;
  print c;
}
print a;
print b;
print c;

var d;
d = "K";
print d;

var a = 1;
{
  var a = a + 2;
  print a;
}
print a;

    "#;

    let mut scanner = Scanner::new(source_code.to_string());

    let res = scanner.scan_tokens();
    assert!(res.is_ok());

    let tokens = res.unwrap();

    let mut parser = Parser::new(tokens);
    let statements = parser.parse();
    assert!(statements.is_ok());

    let mut interpreter = Interpreter::new();
    assert!(interpreter.interpret(statements.unwrap()).is_ok());
}

#[test]
fn test_loop() {
    let source_code = r#"
    // Print first 21 fibonacci numbers
var a = 0;
var temp;
for (var b = 1; a < 10000; b = temp + b) {
  print a;
  temp = a;
  a = b;
}
"#;

    let mut scanner = Scanner::new(source_code.to_string());

    let res = scanner.scan_tokens();
    assert!(res.is_ok());

    let tokens = res.unwrap();

    let mut parser = Parser::new(tokens);
    let statements = parser.parse();
    assert!(statements.is_ok());

    let mut interpreter = Interpreter::new();
    assert!(interpreter.interpret(statements.unwrap()).is_ok());
}

#[test]
fn test_functions() {
    let source_code = r#"
    fun sayHi(firstname, lastname) {
        print "Hi, " + firstname + " " + lastname + "!";
      }
      
      sayHi("Chinonso", "Okoli");
    print sayHi;
"#;

    let mut scanner = Scanner::new(source_code.to_string());

    let res = scanner.scan_tokens();
    assert!(res.is_ok());

    let tokens = res.unwrap();

    let mut parser = Parser::new(tokens);
    let statements = parser.parse();
    assert!(statements.is_ok());

    let mut interpreter = Interpreter::new();
    assert!(interpreter.interpret(statements.unwrap()).is_ok());
}

#[test]
fn test_return_statement() {
    let source_code = r#"
    fun fib(n) {
        if (n <= 1) return n;
        return fib(n - 2) + fib(n - 1);
      }
      
      for (var i = 0; i < 20; i = i + 1) {
        print fib(i);
      }
"#;

    let mut scanner = Scanner::new(source_code.to_string());

    let res = scanner.scan_tokens();
    assert!(res.is_ok());

    let tokens = res.unwrap();

    let mut parser = Parser::new(tokens);
    let statements = parser.parse();
    assert!(statements.is_ok());

    let mut interpreter = Interpreter::new();
    let res = interpreter.interpret(statements.unwrap());
    assert!(res.is_ok());
}

#[test]
fn test_closures() {
    let source_code = r#"
    fun makeCounter() {
      var i = 0;
      fun count() {
        i = i + 1;
        print i;
      }
    
      return count;
    }
    
    var counter = makeCounter();
    counter(); // "1".
    counter(); // "2".
"#;

    let mut scanner = Scanner::new(source_code.to_string());

    let res = scanner.scan_tokens();
    assert!(res.is_ok());

    let tokens = res.unwrap();

    let mut parser = Parser::new(tokens);
    let statements = parser.parse();
    assert!(statements.is_ok());

    let mut interpreter = Interpreter::new();
    let res = interpreter.interpret(statements.unwrap());
    assert!(res.is_ok());
}

#[test]
fn test_dynamic_scoping() {
    let source_code = r#"
    var a = "global";
    {
      fun showA() {
        print a;
      }

      showA();
      var a = "block";
      showA();
    }
"#;

    let mut scanner = Scanner::new(source_code.to_string());

    let res = scanner.scan_tokens();
    assert!(res.is_ok());

    let tokens = res.unwrap();

    let mut parser = Parser::new(tokens);
    let statements = parser.parse();
    assert!(statements.is_ok());

    let mut interpreter = Interpreter::new();
    let res = interpreter.interpret(statements.unwrap());
    assert!(res.is_ok());
}
