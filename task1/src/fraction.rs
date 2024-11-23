use rhai::{CustomType, TypeBuilder};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, CustomType)]
#[rhai_type(extra = Fraction::build_extra)]
pub struct Fraction {
    numerator: i128,
    denominator: u128,
}

/// Отображение на экран как дробь: 1/2, 13/37
impl Display for Fraction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

/// Преобразование из i32 в дробь
impl From<i32> for Fraction {
    fn from(value: i32) -> Self {
        Self {
            numerator: value as i128,
            denominator: 1,
        }
    }
}

/// Сложение дроби с чем-то, что можно преобразовать в дробь
impl<T> std::ops::Add<T> for Fraction
where
    T: Into<Fraction>,
{
    type Output = Fraction;

    fn add(self, rhs: T) -> Self::Output {
        let rhs: Self = rhs.into();
        // Сложение дробей: (a/b) + (c/d) = (a*d + b*c) / (b*d)
        let numerator =
            (self.numerator * rhs.denominator as i128) + (rhs.numerator * self.denominator as i128);
        let denominator = self.denominator * rhs.denominator;
        Self {
            numerator,
            denominator,
        }
        .normalized()
    }
}

/// Вычитание дроби с чем-то, что можно преобразовать в дробь
/// (то есть сложение с отрицательным другим числом)
impl<T> std::ops::Sub<T> for Fraction
where
    T: Into<Fraction>,
{
    type Output = Fraction;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs: Self = rhs.into();
        self + (-rhs)
    }
}

/// Унарный минус для дроби
impl std::ops::Neg for Fraction {
    type Output = Fraction;

    fn neg(self) -> Self::Output {
        Self {
            numerator: -self.numerator,
            denominator: self.denominator,
        }
        // не требуется нормализация, если исходная дробь уже минимальная
    }
}

/// Умножение дроби на чем-то, что можно преобразовать в дробь
impl<T> std::ops::Mul<T> for Fraction
where
    T: Into<Fraction>,
{
    type Output = Fraction;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs: Self = rhs.into();
        // Умножение дробей: (a/b) * (c/d) = (a*c) / (b*d)
        let numerator = self.numerator * rhs.numerator;
        let denominator = self.denominator * rhs.denominator;
        Self {
            numerator,
            denominator,
        }
        .normalized()
    }
}

/// Деление дроби на чем-то, что можно преобразовать в дробь
/// (умножение на обратную дробь)
impl<T> std::ops::Div<T> for Fraction
where
    T: Into<Fraction>,
{
    type Output = Fraction;

    fn div(self, rhs: T) -> Self::Output {
        let rhs: Self = rhs.into();
        self * rhs.reciprocal()
    }
}

/// Алгоритм Евклида для нахождения наибольшего общего делителя
fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

impl Fraction {
    pub fn new(numerator: i128, denominator: u128) -> Self {
        Self {
            numerator,
            denominator,
        }
        .normalized()
    }

    /// Дробь с таким же числовым значением, но в минимальном виде.
    /// Все внешние API должны возвращать дробь в таком виде.
    fn normalized(self) -> Self {
        let gcd = gcd(self.numerator.abs() as u128, self.denominator);
        Self {
            numerator: self.numerator / gcd as i128,
            denominator: self.denominator / gcd,
        }
    }

    pub fn is_positive(&self) -> bool {
        self.numerator >= 0
    }

    /// Обратная дробь:
    /// x/y -> y/x
    pub fn reciprocal(self) -> Self {
        Self {
            numerator: self.denominator as i128 * (if self.is_positive() { 1 } else { -1 }),
            denominator: self.numerator.abs() as u128,
        }
        // не требуется нормализация, если исходная дробь уже минимальная
    }

    /// Добавляет дополнительные методы к типу в Rhai
    pub(crate) fn build_extra(builder: &mut TypeBuilder<Self>) {
        builder
            .with_name("Fraction")
            .with_fn("new", Self::new)
            .with_fn("reciprocal", Self::reciprocal)
            .with_fn("is_positive", Self::is_positive)
            .on_print(|v| format!("{}", v))
            .on_debug(|v| format!("{:?}", v));
    }
}
