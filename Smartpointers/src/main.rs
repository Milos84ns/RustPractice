use ::std::rc::Rc;
use ::std::cell::RefCell;

#[derive(Debug)]
enum MenuItem {
    SALAD,
    SUSHI,
    STEAK,
    ICECREAM,
}

#[derive(Debug)]
struct ItemOrder {
    item: MenuItem,
    quantity: u8,
}

#[derive(Debug)]
struct TableOder {
    items: Vec<ItemOrder>,
}

type Order = Rc<RefCell<Vec<TableOder>>>;

struct Chef(Order);

struct Waiter(Order);

struct Accounting(Order);

fn main() {
    let orders = Rc::new(RefCell::new(vec![]));
    let chef = Chef(Rc::clone(&orders));
    let wait_staff = Chef(Rc::clone(&orders));
    let accounting = Chef(Rc::clone(&orders));

    let order = new_table_order();
    {
        orders.borrow_mut().push(order);
    }
    dbg!(chef.0.borrow());
    dbg!(wait_staff.0.borrow());
    dbg!(accounting.0.borrow());
}

fn new_table_order() -> TableOder {
    TableOder {
        items: vec![ItemOrder {
            item: MenuItem::SALAD,
            quantity: 2,
        }]
    }
}
