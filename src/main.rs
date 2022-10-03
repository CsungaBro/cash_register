use cash_register::Money;

fn main() {
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

    let my_shop = cash_register::check_cash_register(price, cash, register);
    println!("{:?}", my_shop)
}