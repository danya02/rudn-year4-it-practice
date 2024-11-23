use fraction::Fraction;
use rhai::Dynamic;
use rustyline::{config::Configurer, DefaultEditor};

pub mod fraction;

pub fn make_engine() -> rhai::Engine {
    let mut engine = rhai::Engine::new();
    engine.build_type::<fraction::Fraction>();
    engine.register_type::<fraction::Fraction>();
    engine.register_fn("frac", |a: rhai::INT, b: rhai::INT| {
        if b == 0 {}
        if b < 0 {
            fraction::Fraction::new(-a as i128, b as u128)
        } else {
            fraction::Fraction::new(a as i128, b as u128)
        }
    });

    engine.register_fn("plus", |a: fraction::Fraction, b: fraction::Fraction| a + b);
    engine.register_fn("minus", |a: fraction::Fraction, b: fraction::Fraction| {
        a - b
    });
    engine.register_fn("mul", |a: fraction::Fraction, b: fraction::Fraction| a * b);
    engine.register_fn("div", |a: fraction::Fraction, b: fraction::Fraction| a / b);

    println!("Зарегестрированные функции:");
    engine
        .gen_fn_signatures(false)
        .into_iter()
        .for_each(|func| println!("{func}"));

    println!();
    engine
}

fn main() -> Result<(), ()> {
    let engine = make_engine();
    let mut scope = rhai::Scope::new();
    println!("engine готов!");
    println!("Попробуйте: let x = frac(1,2); let y = frac(2,3); plus(a,b)");

    let mut rl = DefaultEditor::new().unwrap();
    rl.set_auto_add_history(true);

    loop {
        let line = rl.readline("> ").map_err(|_| {
            println!("exit...");
            ()
        })?;

        let response = engine.eval_with_scope::<Dynamic>(&mut scope, &line);
        match response {
            Ok(value) => {
                if value.is_unit() {
                } else if value.is_variant() {
                    match value.try_cast_result::<Fraction>() {
                        Ok(value) => println!("{}", value),
                        Err(err) => println!(
                            "Неизвестный тип: {}, значение <{err}> (попробуйте сделать print)",
                            err.type_name()
                        ),
                    }
                } else {
                    println!("{}", value);
                }
            }
            Err(err) => println!("Ошибка: {err}"),
        }
    }
}
