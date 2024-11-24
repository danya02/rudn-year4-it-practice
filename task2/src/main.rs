use enum_display::EnumDisplay;
use enumflags2::{bitflags, make_bitflags, BitFlags};
use inquire::{InquireError, MultiSelect, Select};

#[derive(Copy, Clone, Debug, PartialEq, EnumDisplay)]
pub enum Architecture {
    Type1,
    Type2,
}

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, EnumDisplay)]
pub enum Role {
    DDNS,
    DNS,
    Gateway,
    Router,
}

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, EnumDisplay)]
pub enum Test {
    Test1,
    Test2,
    Test3,
    Test4,
    Test5,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ComputerConfig {
    architecture: Architecture,
    roles: BitFlags<Role>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TestConfig {
    tests: BitFlags<Test>,
    deadline_days: u8,
}

impl ComputerConfig {
    fn only_tests(&self) -> BitFlags<Test> {
        match self.architecture {
            Architecture::Type1 => make_bitflags!(Test::{Test1 | Test2 | Test3}),
            Architecture::Type2 => {
                let mut tests = BitFlags::empty();
                if self.roles.contains(Role::DDNS) {
                    tests |= Test::Test2 | Test::Test3;
                }
                if self.roles.contains(Role::DNS) {
                    tests |= Test::Test4 | Test::Test5;
                }
                if self.roles.contains(Role::Gateway) {
                    tests |= Test::Test3 | Test::Test4;
                }
                if self.roles.contains(Role::Router) {
                    tests |= Test::Test1 | Test::Test3;
                }
                tests
            }
        }
    }

    fn only_deadline_days(tests: BitFlags<Test>) -> u8 {
        if tests.contains(Test::Test1) {
            3
        } else if tests.contains(Test::Test2) {
            7
        } else if tests.contains(Test::Test3) {
            10
        } else if tests.contains(Test::Test4) {
            12
        } else {
            14
        }
    }

    pub fn get_test_config(&self) -> TestConfig {
        let tests = self.only_tests();
        TestConfig {
            tests,
            deadline_days: ComputerConfig::only_deadline_days(tests),
        }
    }
}

pub fn main() -> Result<(), InquireError> {
    let architecture = Select::new(
        "Выберите архитектуру компьютера",
        vec![Architecture::Type1, Architecture::Type2],
    )
    .prompt()?;

    let roles = MultiSelect::new(
        "Выберите роли компьютера",
        vec![Role::DDNS, Role::DNS, Role::Gateway, Role::Router],
    )
    .prompt()?;
    let roles = roles
        .into_iter()
        .fold(BitFlags::empty(), |acc, role| acc | role);

    let config = ComputerConfig {
        architecture,
        roles,
    };

    println!("Ваш компьютер: {config:?}");

    let test_config = config.get_test_config();
    println!("Тесты: {test_config:?}");

    println!();
    println!("Дедлайн: {} дн.", test_config.deadline_days);

    println!();
    println!("Тесты:");
    let tests = test_config.tests;
    if tests.contains(Test::Test1) {
        println!("- Test1");
    }
    if tests.contains(Test::Test2) {
        println!("- Test2");
    }
    if tests.contains(Test::Test3) {
        println!("- Test3");
    }
    if tests.contains(Test::Test4) {
        println!("- Test4");
    }
    if tests.contains(Test::Test5) {
        println!("- Test5");
    }

    Ok(())
}
