#[derive(Debug)]
pub struct Money {
    pub penny: f64,
    pub nickel: f64,
    pub dime: f64,
    pub quarter: f64,
    pub one:  u32,
    pub five: u32,
    pub ten: u32,
    pub twenty: u32,
    pub one_hundred: u32,
}

impl Default for Money {
    fn default() -> Self {
        Money{
        penny: 0.0,
        nickel: 0.0,
        dime: 0.0,
        quarter: 0.0,
        one: 0,
        five: 0,
        ten: 0,
        twenty: 0,
        one_hundred: 0,
        }
    }
}

impl PartialEq for Money
{
    fn eq(&self, other: &Self) -> bool {
        self.penny == other.penny &&
        self.nickel == other.nickel &&
        self.dime == other.dime &&
        self.quarter == other.quarter &&
        self.one == other.one &&
        self.five == other.five &&
        self.ten == other.ten &&
        self.twenty == other.twenty &&
        self.one_hundred == other.one_hundred 
    }
}

#[derive(Debug)]
pub struct Shop {
    pub status: String,
    pub change: Money
}

impl PartialEq for Shop
{
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status &&
        self.change == other.change
    }
}


pub fn check_cash_register(price: f64, cash: f64, register: Money) -> Shop {
    Shop {
        status: String::new(),
        change: Money{.. Default::default()},
    }
}


#[cfg(test)]
mod test_check_cash_register {
    use super::*;

    #[test]
    fn test_case_one() {
        let price = 19.5;
        let cash = 20.0;
        let register = Money {
            penny: 1.01,
            nickel: 2.05,
            dime: 3.1,
            quarter: 4.25,
            one:  90,
            five: 55,
            ten: 20,
            twenty: 60,
            one_hundred: 100,
            .. Default::default()
        };
        let change: Money = Money {
            quarter: 0.5,
            .. Default::default()
        };
        let shop = Shop {
            status: String::from("OPEN"),
            change: change,
        };

        assert_eq!(shop, check_cash_register(price, cash, register));

    }

    #[test]

    fn test_case_two() {
        let price = 3.26;
        let cash = 100.0;
        let register = Money {
            penny: 1.01,
            nickel: 2.05,
            dime: 3.1,
            quarter: 4.25,
            one:  90,
            five: 55,
            ten: 20,
            twenty: 60,
            one_hundred: 100,
            .. Default::default()
        };

        let change: Money = Money {
            penny: 0.04,
            dime: 0.2,
            quarter: 0.5,
            one: 1,
            five: 15,
            ten: 20,
            twenty: 60,
            .. Default::default()
        };
        let shop = Shop {
            status: String::from("OPEN"),
            change: change,
        };

        assert_eq!(shop, check_cash_register(price, cash, register));

    }    
}